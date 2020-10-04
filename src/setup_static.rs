/*
    `setup_static.rs` - Setup Default Static Files
    creates boilerplate static files for binserve
*/
use std::fs;
use std::io::prelude::*;
use std::path::Path;

use crate::config::get_config;

// setup all the default static files
pub fn setup_static() -> std::io::Result<()> {
    let config = get_config();

    // default `index.html` template
    const index_contents: &str = "<html>
    <title>{{name}}</title>
    <body>
        <h1>Hello, universe!</h1>
        <h2>{{name}} is up and running!</h2>
        <hr>
        <p><i>binserve v0.1.0</i></p>
    </body>
</html>";

    // create binserve static directories and files
    let static_dir = config["static_directory"].to_string().replace("\"", "");
    if !Path::new(&static_dir).exists() {
        let index_html_path = format!("{}/index.html", static_dir);

        // create `static/` directory
        fs::create_dir(static_dir).ok();

        // create `index.html` homepage inside static directory
        if !Path::new(&index_html_path).exists() {
            let mut index_html = fs::File::create(index_html_path)?;
        index_html.write_all(index_contents.as_bytes())?;
        }
    }

    let static_dir = config["static_directory"].to_string().replace("\"", "");
    let assets_dir = format!("{}/assets", static_dir);
    let images_dir = format!("{}/assets/images", static_dir);
    let css_dir = format!("{}/assets/css", static_dir);
    let js_dir = format!("{}/assets/js", static_dir);

    // create `assets/` directory
    if !Path::new(&assets_dir).exists() {
        fs::create_dir(assets_dir).ok();
    }
    // create `assets/images/` directory
    if !Path::new(&images_dir).exists() {
        fs::create_dir(images_dir).ok();
    }
    // create `assets/css/` directory
    if !Path::new(&css_dir).exists() {
        fs::create_dir(css_dir).ok();
    }
    // create `assets/js/` directory
    if !Path::new(&js_dir).exists() {
        fs::create_dir(js_dir).ok();
    }

    Ok(())
}
