/*
    `setup_static.rs` - Setup Default Static Files
    creates boilerplate static files for binserve
*/
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use crate::config::CONFIG;
use crate::error::Result;

/// Generate the default static-files directories.
fn create_directory_structure() -> std::io::Result<()> {
    let static_dir = &CONFIG.static_directory;
    let static_dir = Path::new(static_dir);
    let assets_dir = static_dir.join("assets");
    let images_dir = assets_dir.join("images");
    let css_dir = assets_dir.join("css");
    let js_dir = assets_dir.join("js");
    // Create templates partials directory
    if let Some(templates) = &CONFIG.templates {
        fs::create_dir_all(&templates.partials_directory)?;
    }

    if !static_dir.exists() {
        fs::create_dir_all(static_dir)?;
    }

    // create `assets/` directory
    if !assets_dir.exists() {
        fs::create_dir(&assets_dir)?;
    }
    // create `assets/images/` directory
    if !images_dir.exists() {
        fs::create_dir(&images_dir)?;
    }
    // create `assets/css/` directory
    if !css_dir.exists() {
        fs::create_dir(&css_dir)?;
    }
    // create `assets/js/` directory
    if !js_dir.exists() {
        fs::create_dir(&js_dir)?;
    }
    Ok(())
}

/// Generate the default templates. If a "templates" entry is in the config, a
/// layout and error template will be created. Otherwise, a static index.html
/// template will be generated.
pub fn generate_templates() -> Result<()> {
    if Path::new(&CONFIG.static_directory).exists() {
        return Ok(());
    }
    create_directory_structure()?;

    if let Some(templates) = &CONFIG.templates {
        // Default layout
        const MAIN_TEMPLATE_CONTENT: &[u8] = b"\
<!DOCTYPE html>
<html>
    <title>{{name}}</title>
    <body>
        {{> content}}
    </body>
</html>";
        // Default index content
        const INDEX_CONTENT: &[u8] = b"
<h1>Hello, universe!</h1>
<h2>{{name}} is up and running!</h2>
<hr>
<p><i>binserve v0.1.0</i></p>\n";
        File::create(&templates.layout)?.write_all(MAIN_TEMPLATE_CONTENT)?;
        Ok(
            File::create(format!("{}/index.html", &CONFIG.static_directory))?
                .write_all(INDEX_CONTENT)?,
        )
    } else {
        // default `index.html` template
        const INDEX_CONTENT: &[u8] = b"<html>
    <title>{{name}}</title>
    <body>
        <h1>Hello, universe!</h1>
        <h2>{{name}} is up and running!</h2>
        <hr>
        <p><i>binserve v0.1.0</i></p>
    </body>
</html>";

        Ok(
            File::create(format!("{}/index.html", CONFIG.static_directory))?
                .write_all(INDEX_CONTENT)?,
        )
    }
}

// setup all the default static files
pub fn setup_static() -> Result<()> {
    generate_templates()
}
