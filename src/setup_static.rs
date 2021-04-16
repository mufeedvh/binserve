/*
    `setup_static.rs` - Setup Default Static Files
    creates boilerplate static files for binserve
*/
use std::fs;
use std::io::prelude::*;
use std::path::Path;

use crate::config::CONFIG;

// setup all the default static files
pub fn setup_static() -> std::io::Result<()> {
    // default `index.html` template
    const INDEX_CONTENTS: &str = "<html>
    <title>{{name}}</title>
    <body>
        <h1>Hello, universe!</h1>
        <h2>{{name}} is up and running!</h2>
        <hr>
        <p><i>binserve v0.1.0</i></p>
    </body>
</html>";

    // create binserve static directories and files
    let static_dir = &CONFIG.static_directory;
    if !Path::new(&static_dir).exists() {
        let index_html_path = format!("{}/index.html", static_dir);

        // create `static/` directory
        fs::create_dir(&static_dir).ok();

        // create `index.html` homepage inside static directory
        if !Path::new(&index_html_path).exists() {
            let mut index_html = fs::File::create(index_html_path)?;
            index_html.write_all(INDEX_CONTENTS.as_bytes())?;
        }
    }

    let static_dir = &static_dir;
    let static_dir = Path::new(static_dir);
    let assets_dir = static_dir.join("assets");
    let images_dir = assets_dir.join("images");
    let css_dir = assets_dir.join("css");
    let js_dir = assets_dir.join("js");

    // create `assets/` directory
    if !Path::new(&assets_dir).exists() {
        fs::create_dir(assets_dir)?;
    }
    // create `assets/images/` directory
    if !Path::new(&images_dir).exists() {
        fs::create_dir(images_dir)?;
    }
    // create `assets/css/` directory
    if !Path::new(&css_dir).exists() {
        fs::create_dir(css_dir)?;
    }
    // create `assets/js/` directory
    if !Path::new(&js_dir).exists() {
        fs::create_dir(js_dir)?;
    }

    Ok(())
}
