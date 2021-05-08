#![feature(box_syntax)]
use actix_files::Files;
use actix_web::{middleware, web::get as GET, App, HttpServer};

use std::env::{set_var, var};

mod config;
mod error;
mod error_pages;
mod security;
mod serve;
mod setup_static;
mod template;

use crate::config::CONFIG;
use error_pages::generate_error_template;
use security::is_config_secure;
use serve::serve_content;
use setup_static::setup_static;
use template::render_templates;

fn binserve_init() {
    // setup static files
    if let Err(err) = setup_static() {
        err.fatal();
    }

    // validate routes for security vulnerabilities
    is_config_secure();

    // generate static error pages
    if let Err(err) = generate_error_template() {
        err.fatal();
    }

    // render templates
    if let Err(err) = render_templates() {
        err.fatal();
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // init binserve server config & files
    binserve_init();

    // enable/disable logging
    if CONFIG.enable_logging && var("RUST_LOG").is_err() {
        set_var("RUST_LOG", "actix_web=info");
        env_logger::init();
    }

    // ASCII art banner always looks cool
    println!(
        "                            
         _      _                         
        | |         ___   ____  _____   ____  _  _ _____
        | |__   |  || || |    ||     | | || | |  ||     |
        |    |  |  || || |  ~~|| /-__| | ||_| |  || /-__|
        | -- |  |  || || |~~  || |___  | |    |  || |___
        |____| _|_ \\/ \\/ |____||_____| |_|    \\__/|_____| v0.1.0
    "
    );

    // print out `host` and `port` of the server
    println!(
        "\nYour server is up and running at http://{}:{}/\n",
        CONFIG.server.host, CONFIG.server.port
    );
    let server_config = CONFIG.server.clone();

    HttpServer::new(move || {
        App::new()
            // enable the logger middleware
            .wrap(middleware::Logger::default())
            .service(setup_file_service())
            // serve static files
            .route("/{route:.*}", GET().to(serve_content))
    })
    .bind(format!("{}:{}", server_config.host, server_config.port))?
    .run()
    .await
}

fn setup_file_service() -> Files {
    let fs = Files::new("/", "rendered_templates/");
    //`.show_files_listing()` mode is set if directory listing is enabled in config
    if CONFIG.directory_listing {
        fs.show_files_listing()
    } else {
        fs
    }
    .prefer_utf8(true)
    .use_last_modified(true)
}
