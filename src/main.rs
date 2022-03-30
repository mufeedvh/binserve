use actix_files::Files;
use actix_web::{middleware, web, App, HttpServer};

use std::env::set_var;

mod config;
mod error_pages;
mod security;
mod serve;
mod setup_static;
mod template;

use error_pages::generate_error_pages;
use security::is_config_secure;
use serve::serve_content;
use setup_static::setup_static;
use template::render_templates;

fn binserve_init() {
    // setup binserve configuration
    config::setup_config();

    // setup static files
    setup_static().ok();

    // validate routes for security vulnerabilities
    is_config_secure();

    // generate static error pages
    generate_error_pages();

    // render templates
    render_templates();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // init binserve server config & files
    binserve_init();

    let config = config::get_config();

    // enable/disable logging
    let enable_logging = config["enable_logging"].to_string();
    if enable_logging == "true" {
        set_var("RUST_LOG", "actix_web=info");
        env_logger::init();
    }

    let host = config["server"]["host"].to_string().replace("\"", "");
    let port = config["server"]["port"].to_string();

    // ASCII art banner always looks cool
    println!(
        "                            
         _   _                         
        | |_|_|___ ___ ___ ___ _ _ ___ 
        | . | |   |_ -| -_|  _| | | -_|
        |___|_|_|_|___|___|_|  \\_/|___| v0.1.0
    "
    );

    // print out `host` and `port` of the server
    println!(
        "\nYour server is up and running at http://{}:{}/\n",
        host, port
    );

    HttpServer::new(move || {
        let mut files = Files::new("/static", "static/assets/")
            .prefer_utf8(true)
            .use_last_modified(true);

        // Hide directories by default
        if config["directory_listing"].as_bool().unwrap_or(false) {
            files = files.show_files_listing()
        }

        App::new()
            // enable the logger middleware
            .wrap(middleware::Logger::default())
            .service(files)
            // serve static files
            .route("/{route:.*}", web::get().to(serve_content))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
