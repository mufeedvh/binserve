/*
    `error_page.rs` - Error Page Generation
    generate error pages
*/
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use actix_web::http::StatusCode;
use handlebars::Handlebars;
use maplit::hashmap;

use crate::config::CONFIG;
use crate::error::{Error, Result};
use crate::template::TEMPLATE_DIR;

/// Generate the default error template
pub fn generate_error_template() -> Result<()> {
    const CONTENTS: &'static str = "
    <!DOCTYPE html>
    <html>
    <title>{{status_code}} {{status_message}}</title>
    <body>
        <h1>{{status_code}} {{status_message}}</h1>
        <hr />
        <p><i>binserve v0.2.0</i></p>
    </body>
    </html>";
    if let Some(templates) = &CONFIG.templates {
        Ok(File::create(&templates.error)?.write_all(CONTENTS.as_bytes())?)
    } else {
        Err(Error::config(
            "trying to generate templates, but the location is not configured",
        ))
    }
}

/// Return the location of a rendered error page. If the page was not
/// previously rendered, it will be rendered. If static files are being used
/// instead, the config will be checked for a file. If the file is not found in
/// the configuration, or there is an error rendering the template, an error is
/// returned.
pub fn error_page(status: StatusCode) -> Result<PathBuf> {
    if let Some(templates) = &CONFIG.templates {
        let mut page_path = Path::new(TEMPLATE_DIR).join(status.as_str());
        page_path.set_extension("html");
        if !page_path.exists() {
            let file = File::create(&page_path)?;
            let mut template = File::open(&templates.error)?;
            let bars = Handlebars::new();
            bars.render_template_source_to_write(
                &mut template,
                &hashmap!(
                    "status_code" => status.as_str(),
                    "status_message" => status.canonical_reason().unwrap_or("")
                ),
                file,
            )?;
        }
        Ok(page_path)
    } else if let Some(error_pages) = &CONFIG.error_pages {
        if let Some(path) = error_pages.get(status.as_str()) {
            // let path = ;
            Ok(Path::new(path).into())
        } else {
            Err(Error::config(
                format!("error page not specified for {}", status).as_str(),
            ))
        }
    } else {
        Err(Error::config(
            "must specify either error template or error pages",
        ))
    }
}
