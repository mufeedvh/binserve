/*
    `config.rs` - Configuration Functions
    generate, save, and get config
*/
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_json::{from_str, Value};

use lazy_static::lazy_static;

// config filename
static CONFIG_FILE: &str = "binserve.json";

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
pub struct TemplatesConfig {
    pub error: String,
    pub layout: String,
    pub partials_directory: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ConfigData {
    pub server: ServerConfig,
    pub static_directory: String,
    pub routes: HashMap<String, String>,
    pub template_variables: serde_json::Map<String, Value>,
    pub enable_logging: bool,
    pub directory_listing: bool,
    pub follow_symlinks: bool,
    pub minify: bool,
    pub templates: Option<TemplatesConfig>,
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
        "enable_logging": true,
        "directory_listing": false,
        "follow_symlinks": false,
        "minify": false,
        "templates": {
            "error": "templates/error.html",
            "layout": "templates/layout.html",
            "partials_directory": "templates/partials",
        },
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

lazy_static! {
    pub static ref CONFIG: ConfigData = setup_config().unwrap();
}
