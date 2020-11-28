/*
    `config.rs` - Configuration Functions
    generate, save, and get config
*/
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Write};
use std::path::Path;

use serde_json::{Value, from_str};

// config filename
static CONFIG_FILE: &str = "binserve.json";

/*
    TODO: Store config in cache
*/

// save the config to an environment variable
fn save_config() -> std::io::Result<()> {
    let config_file = File::open(CONFIG_FILE)?;
    let mut buf_reader = BufReader::new(config_file);
    let mut json_string = String::new();
    buf_reader.read_to_string(&mut json_string)?;
    
    /*
        TODO: verify valid config structure
        https://github.com/mufeedvh/binserve/issues/6
    */

    env::set_var("JSON_CONFIG", json_string);
    Ok(())
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
            "404": "404.html"
        },
        "enable_logging": true,
        "directory_listing": false,
        "follow_symlinks": false
    });

    let contents = serde_json::to_string_pretty(&config_obj).unwrap();

    let mut file = File::create(CONFIG_FILE)?;
    file.write_all(contents.as_bytes())?;

    Ok(())
}

pub fn setup_config() {
    // only generate the config file if it doesn't exist already
    if !Path::new(CONFIG_FILE).exists() {
        generate_config_file().ok();
    }
    save_config().ok();
}

// this function returns the JSON config
pub fn get_config() -> Value {
    let bs_config = env::var("JSON_CONFIG").unwrap();

    let json_config: Value = from_str(&bs_config).expect("JSON was not well-formatted");

    json_config
}
