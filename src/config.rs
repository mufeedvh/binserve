/*
    `config.rs` - Configuration Functions
    generate, save, and get config
*/
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Write};
use std::path::Path;
use std::process;

use serde::{Deserialize, Serialize};
use serde_json::{from_str, Value};

use lazy_static::lazy_static;

// config filename
static CONFIG_FILE: &str = "binserve.json";

/*
    TODO: Store config in cache
*/

// save the config to an environment variable
fn load_config() -> std::io::Result<ConfigData> {
    let config_file = File::open(CONFIG_FILE)?;
    let mut buf_reader = BufReader::new(config_file);
    let mut json_string = String::new();
    buf_reader.read_to_string(&mut json_string)?;
    let config: ConfigData = from_str(&json_string)?;
    Ok(config)
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ConfigData {
    pub server: ServerConfig,
    pub static_directory: String,
    pub routes: HashMap<String, String>,
    pub template_variables: serde_json::Map<String, Value>,
    pub error_pages: HashMap<String, String>,
    pub enable_logging: bool,
    pub directory_listing: bool,
    pub follow_symlinks: bool,
    pub minify: bool,
}

// generate the config file for binserve - `binserve.json`
fn generate_config_file() -> std::io::Result<()> {
    let config_obj = serde_json::json!({
        "server": {
            "host": "127.0.0.1",
            "port": 1337
        },
        "static_directory": "static",
        "routes": {
            "/": "index.html",
            "/example": "example.html"
        },
        "template_variables": {
            "load_static": "/static/",
            "name": "Binserve"
        },
        "error_pages": {
            "404": "404.html",
            "500": "500.html"
        },
        "enable_logging": true,
        "directory_listing": false,
        "follow_symlinks": false,
        "minify": false
    });

    let contents = serde_json::to_string_pretty(&config_obj).unwrap();

    let mut file = File::create(CONFIG_FILE)?;
    file.write_all(contents.as_bytes())?;

    Ok(())
}

pub fn setup_config() -> std::io::Result<ConfigData> {
    // only generate the config file if it doesn't exist already
    if !Path::new(CONFIG_FILE).exists() {
        generate_config_file().ok();
    }
    load_config()
}

fn abort(message: String) -> ! {
    println!("{}", message);
    process::exit(1)
}
fn get_err_pages() -> (String, String) {
    let error_pages = &CONFIG.error_pages;
    match (&error_pages.get("404"), &error_pages.get("500")) {
        (Some(p_404), Some(p_500)) => (p_404.to_string(), p_500.to_string()),
        (Some(_), None) => abort("500 page not specified".to_string()),
        (None, Some(_)) => abort("404 page not specified".to_string()),
        (None, None) => {
            abort("required 404 and 500 error page templates not specified".to_string())
        }
    }
}
lazy_static! {
    static ref ERR_PAGES: (String, String) = get_err_pages();
    pub static ref CONFIG: ConfigData = setup_config().unwrap();
    pub static ref PAGE_404: String = ERR_PAGES.0.to_string();
    pub static ref PAGE_500: String = ERR_PAGES.1.to_string();
}
