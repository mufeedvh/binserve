use actix_web::{
    http::{
        header::{
            HeaderMap, HeaderValue, CACHE_CONTROL, ETAG, IF_MODIFIED_SINCE, IF_NONE_MATCH,
            LAST_MODIFIED, SERVER,
        },
        KeepAlive,
    },
    middleware::{self, Compress, Condition, Logger},
    web, App, HttpRequest, HttpResponse, HttpServer, Result,
};

use actix_files::{self, NamedFile};
use actix_web_lab::middleware::RedirectHttps;

use std::path::{Path, PathBuf};

use super::{
    config::{BinserveConfig, CONFIG_STATE},
    routes::{Type, ROUTEMAP},
    tls,
};

use crate::cli::messages::{push_message, Type as MsgType};

/// Check for `If-None-Matched` and `If-Modified-Since` headers
/// for enabling browser caching.
///
/// All patterns are exhaustively checked to correctly handle contradicting values.
async fn request_client_is_cached(
    headers: &HeaderMap,
    etag: &HeaderValue,
    last_modified: &HeaderValue,
) -> bool {
    match (headers.get(IF_NONE_MATCH), headers.get(IF_MODIFIED_SINCE)) {
        (None, None) => return false,
        (Some(req_etag), Some(req_last_modified)) => {
            // if both of them match the requested file's state,
            // return `304` to respond with the browser cache
            if req_etag == etag && req_last_modified == last_modified {
                return true;
            }
        }
        (Some(req_etag), None) => {
            // if the etag value match the requested file's state,
            // return `304` to respond with the browser cache
            if req_etag == etag {
                return true;
            }
        }
        (None, Some(req_last_modified)) => {
            // if the last modified system time match the requested file's state,
            // return `304` to respond with the browser cache
            if req_last_modified == last_modified {
                return true;
            }
        }
    }

    false
}

/// Route matcher and handles all HTTP requests. (registered as the `default_service`)
async fn router(req: HttpRequest) -> Result<HttpResponse> {
    match ROUTEMAP.get(req.path()) {
        Some(handler) => {
            let handler = handler.value();

            // the response body in `Bytes`
            let body = handler.response.bytes.to_owned();
            // the mime type (`Content-Type`) derived from the file
            let mime_type = handler.response.mime.as_ref().unwrap();
            // the etag derived from the file metadata
            let etag = handler.response.etag.as_ref().unwrap();
            // the last modified timestamp of the file
            let last_modified = handler.response.last_modified.as_ref().unwrap();

            // if it's not stored in memory, read from disk
            if handler.r#type == Type::File {
                let path = &handler.response.path;

                // handlebars templates are always pre-rendered at initialization and stored in-memory
                if path.to_string_lossy().ends_with(".hbs") {
                    // get the request headers
                    let headers = req.headers();

                    // if the request client is cached, respond with the cache content (304 Not Modified)
                    if request_client_is_cached(headers, etag, last_modified).await {
                        // 304 Not Modified
                        return Ok(HttpResponse::NotModified()
                            .insert_header((LAST_MODIFIED, last_modified))
                            .insert_header((ETAG, etag))
                            .finish());
                    }

                    return Ok(HttpResponse::Ok()
                        .insert_header((LAST_MODIFIED, last_modified))
                        .insert_header((ETAG, etag))
                        .content_type(mime_type)
                        .body(handler.response.hbs_bytes.to_owned()));
                }

                return Ok(NamedFile::open(&handler.response.path)?
                    .prefer_utf8(true)
                    .use_etag(true)
                    .use_last_modified(true)
                    .into_response(&req));
            }

            // get the request headers
            let headers = req.headers();

            // if the request client is cached, respond with the cache content (304 Not Modified)
            if request_client_is_cached(headers, etag, last_modified).await {
                // 304 Not Modified
                return Ok(HttpResponse::NotModified()
                    .insert_header((LAST_MODIFIED, last_modified))
                    .insert_header((ETAG, etag))
                    .finish());
            }

            // fallback to returning the current state by default
            Ok(HttpResponse::Ok()
                .insert_header((LAST_MODIFIED, last_modified))
                .insert_header((ETAG, etag))
                .content_type(mime_type)
                .body(body)) // NOTE: should we `stream` body here, testing didn't show much changes?
        }
        None => {
            let not_found_handler = ROUTEMAP.get("{{404}}").unwrap();
            let handle = not_found_handler.value();
            let not_found_response = &handle.response.bytes;
            Ok(HttpResponse::NotFound().body(not_found_response.to_owned()))
        }
    }
}

/// Run the actix-web server.
#[actix_web::main]
pub async fn run_server(config_state: BinserveConfig) -> std::io::Result<()> {
    let mut http_server = HttpServer::new(move || {
        let mut app_instance = App::new()
            .wrap({
                // by default env has to be initialized to log events
                let mut logger = Logger::new("");

                // enable logging middleware
                if config_state.config.enable_logging {
                    env_logger::try_init_from_env(env_logger::Env::new().default_filter_or("info"))
                        .unwrap_or_default();

                    logger = Logger::new("%a \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T");
                }

                logger
            })
            .wrap({
                let mut headers_middleware = middleware::DefaultHeaders::new();

                // binserve server header
                headers_middleware = headers_middleware
                    .add((SERVER, format!("binserve/{}", env!("CARGO_PKG_VERSION"))));

                // Add the `Cache-Control` header if enabled in config.
                //
                // On the `no-cache` choice:
                // https://jakearchibald.com/2016/caching-best-practices/
                if config_state.config.enable_cache_control {
                    headers_middleware = headers_middleware.add((CACHE_CONTROL, "no-cache"));
                }

                // overwrite specified headers
                if !config_state.insert_headers.is_empty() {
                    for (header, value) in config_state.insert_headers.iter() {
                        headers_middleware =
                            headers_middleware.add((header.as_str(), value.as_str()));
                    }
                }

                headers_middleware
            })
            .wrap(Compress::default())
            // enable TLS autoredirect to HTTPs
            .wrap({
                // enable default redirect
                let mut middleware =
                    Condition::new(config_state.server.tls.enable, RedirectHttps::default());

                // if it's a port like 8443, resolve to that instead
                let tls_host = &config_state.server.tls.host;

                // note: this is better than deriving from
                // socket address lookup even if it unwraps
                // since lookups won't complete in debug stages anyway
                let tls_address = tls_host.split(':');
                let tls_address = tls_address.collect::<Vec<&str>>();
                let mut tls_port = "443";
                if tls_address.len() > 1 {
                    tls_port = tls_address[1]
                }

                if tls_port != "443" {
                    middleware = Condition::new(
                        config_state.server.tls.enable,
                        RedirectHttps::default().to_port(tls_port.parse::<u16>().unwrap()),
                    );
                }

                middleware
            });

        let static_served_from = &config_state.r#static.served_from;
        let static_directory = &config_state.r#static.directory;

        if !static_served_from.is_empty() && static_directory != &PathBuf::new() {
            app_instance = app_instance.service({
                let mut static_file_service =
                    actix_files::Files::new(static_served_from, static_directory)
                        // don't follow symlinks unless explicitly stated otherwise
                        .path_filter(|path, _| {
                            let config_state = &*CONFIG_STATE.lock();

                            // if configured to follow symlinks
                            if config_state.config.follow_symlinks {
                                false
                            } else {
                                Path::new(&config_state.r#static.directory)
                                    .join(path)
                                    .symlink_metadata()
                                    .map(|m| !m.file_type().is_symlink())
                                    .unwrap_or(false)
                            }
                        })
                        .prefer_utf8(true)
                        .use_etag(true)
                        .use_last_modified(true);

                // if configured to allow directory listing or not
                // for the static files.
                if config_state.config.enable_directory_listing {
                    static_file_service = static_file_service.show_files_listing()
                }

                static_file_service
            });
        }

        app_instance.default_service(web::get().to(router))
    })
    .bind({
        // port doesn't have to be explicitly specified
        let mut host = config_state.server.host.to_owned();

        let address = host.split(':');
        let address = address.collect::<Vec<&str>>();
        if address.len() == 1 {
            host = format!("{}:{}", host, "80")
        }

        push_message(
            MsgType::Success,
            &format!("Your server is up and running at {} 🚀", host),
        );

        host
    })?
    .max_connection_rate(500)
    .keep_alive(KeepAlive::Os);

    // enable TLS connection
    let config_state = BinserveConfig::read()?;

    if config_state.server.tls.enable {
        let tls_host = &config_state.server.tls.host;
        let tls_config = tls::load_rustls_config().unwrap();

        // bind the TLS host and the rustls configuration
        http_server = http_server.bind_rustls_0_22(tls_host, tls_config)?;
    }

    http_server.run().await
}
