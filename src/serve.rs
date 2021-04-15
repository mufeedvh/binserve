/*
    `serve.rs` - HTTP Request/Response Management, Route Validation & Serving
    validates routes & serves HTTP responses with the corresponding route files
*/
use actix_files::NamedFile;
use actix_web::http::StatusCode;
use actix_web::{HttpRequest, Result};

use crate::config::get_config;

// directory of rendered templates
static TEMPLATE_DIR: &str = "rendered_templates";

pub async fn serve_content(req: HttpRequest) -> Result<NamedFile> {
    // get the HTTP request path
    let req_path = format!("/{}", req.match_info().query("route"));

    let config = get_config();

    let status_code;
    let response_file = if let Some(path) = config.routes.get(&req_path) {
        status_code = StatusCode::OK;
        format!("{}/{}", TEMPLATE_DIR, path)
    } else {
        /*
            404 Not Found Handler
            `config["routes"][...]` returns 'null' if the route entry doesn't exist
        */
        status_code = StatusCode::NOT_FOUND;
        format!("{}/{}", TEMPLATE_DIR, *crate::config::PAGE_404)
    };

    Ok(NamedFile::open(response_file)?
        .set_status_code(status_code)
        .prefer_utf8(true)
        .use_last_modified(true))
}
