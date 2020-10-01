/*
    `template.rs` - Templating Engine
    generates templates of static HTML files with `Handlebars`
*/
use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

use handlebars::Handlebars;

use crate::config::get_config;

// directory to save rendered templates
static TEMPLATE_DIR: &str = "rendered_templates";

// render each template and saves it to `TEMPLATE_DIR`
fn engine_write_templates(templates: serde_json::map::Iter) -> std::io::Result<()> {
    let config = get_config();

    let static_dir = config["static_directory"].to_string().replace("\"", "");

    let template_variables = config["template_variables"].as_object().unwrap();

    // iterates through all the static files and renders it
    for (_key, value) in templates {
        let filename = value.to_string().replace("\"", "");

        let file_path = format!("{}/{}", static_dir, filename);
        let static_file = fs::File::open(file_path)?;
        let mut buf_reader = BufReader::new(static_file);
        let mut file_content = String::new();
        buf_reader.read_to_string(&mut file_content)?;

        // Handlebars template engine register
        let reg = Handlebars::new();

        // render the template with the `template_variables` from `binserve.json`
        let rendered_template = reg
            .render_template(&file_content, &serde_json::json!(template_variables))
            .unwrap();

        // write the templates to `TEMPLATE_DIR`
        let template_to_write = format!("{}/{}", TEMPLATE_DIR, filename);
        let mut file = fs::File::create(template_to_write)?;
        file.write_all(rendered_template.as_bytes())?;
    }

    Ok(())
}

// pass all the static HTML files to validate and render
pub fn render_templates() {
    // create binserve templates directory if it doesn't exist
    if !Path::new(&TEMPLATE_DIR).exists() {
        fs::create_dir(TEMPLATE_DIR).ok();
    }

    let config = get_config();

    // converting JSON structures to iterable structures
    let templates = config["routes"].as_object().unwrap().into_iter();
    let error_page_templates = config["error_pages"].as_object().unwrap().into_iter();

    engine_write_templates(templates).ok();
    engine_write_templates(error_page_templates).ok();
}
