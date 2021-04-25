/*
    `serve.rs` - HTTP Request/Response Management, Route Validation & Serving
    validates routes & serves HTTP responses with the corresponding route files
*/
use std::path::Path;

use actix_files::NamedFile;
use actix_web::http::StatusCode;
use actix_web::{HttpRequest, Result};

use crate::config::CONFIG;
use crate::error_pages::error_page;

/// directory of rendered templates
static TEMPLATE_DIR: &str = "rendered_templates";

/// serve pages which have route mappings and weren't caught by the static-file
/// handler. Render a 404 if the route was not configured.
pub async fn serve_content(req: HttpRequest) -> Result<NamedFile> {
    // get the HTTP request path
    let req_path = format!("/{}", req.match_info().query("route"));

    let status_code;
    let response_file: &Path;
    let page;
    let path_str: String;
    if let Some(path) = CONFIG.routes.get(&req_path) {
        status_code = StatusCode::OK;
        path_str = format!("{}/{}", TEMPLATE_DIR, path);
        response_file = Path::new(&path_str)
    } else {
        /*
            404 Not Found Handler
            `config["routes"][...]` returns 'null' if the route entry doesn't exist
        */
        status_code = StatusCode::NOT_FOUND;
        page = error_page(status_code)?;
        response_file = &page;
    };

    Ok(NamedFile::open(response_file)?
        .set_status_code(status_code)
        .prefer_utf8(true)
        .use_last_modified(true))
}
