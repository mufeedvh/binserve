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

    let routes = config["routes"][req_path].to_string().replace("\"", "");

    let status_code;
    /*
        404 Not Found Handler
        `routes` returns 'null' if the route entry doesn't exist
    */
    let response_file = if routes == "null" {
        let page_404 = config["error_pages"]["404"].to_string().replace("\"", "");
        status_code = StatusCode::NOT_FOUND;
        format!("{}/{}", TEMPLATE_DIR, page_404)
    } else {
        status_code = StatusCode::OK;
        format!("{}/{}", TEMPLATE_DIR, routes)
    };

    Ok(NamedFile::open(response_file)?
        .set_status_code(status_code)
        .prefer_utf8(true)
        .use_last_modified(true))
}
