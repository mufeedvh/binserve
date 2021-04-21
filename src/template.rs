/*
    `template.rs` - Templating Engine
    generates templates of static HTML files with `Handlebars`
*/
mod content_buffer;
use std::fs;
use std::fs::{create_dir_all, File};
use std::io::prelude::*;
use std::io::ErrorKind;
use std::path::Path;

use handlebars::Handlebars;
use lazy_static::lazy_static;
use minify::html::minify;

use crate::config::CONFIG;
use crate::error::{Error, Result};
use crate::template::content_buffer::LayoutBuffer;

// directory to save rendered templates
const TEMPLATE_DIR: &str = "rendered_templates";

fn read_layout() -> String {
    if let Some(templates) = &CONFIG.templates {
        let mut text = String::new();
        match File::open(&templates.layout) {
            Ok(mut file) => {
                if let Err(err) = file.read_to_string(&mut text) {
                    println!("error reading global template file");
                    Error::from(err).fatal()
                } else {
                    text
                }
            }
            Err(error) => {
                println!("error opening global template file");
                Error::from(error).fatal()
            }
        }
    } else {
        String::new()
    }
}

lazy_static! {
    pub static ref LAYOUT_CONTENT: String = read_layout();
}

struct TemplateWriter {
    base_dir: Box<Path>,
    out_dir: Box<Path>,
}

impl TemplateWriter {
    fn new(base_dir: Box<Path>, out_dir: Box<Path>) -> Result<Self> {
        Ok(Self {
            base_dir: base_dir,
            out_dir: out_dir,
        })
    }
}

impl TemplateWriter {
    fn basename(path: &Path) -> Result<&Path> {
        match path.file_name() {
            Some(name) => {
                if let Some(name) = name.to_str() {
                    Ok(Path::new(name))
                } else {
                    Err(std::io::Error::new(
                        ErrorKind::Other,
                        format!("invalid filename: '{:?}'", name),
                    )
                    .into())
                }
            }
            None => Ok(path),
        }
    }
    fn write(self: &Self) -> Result<()> {
        if !self.base_dir.exists() {
            create_dir_all(&self.base_dir)?;
        }
        for path in self.base_dir.read_dir()? {
            let path = path?.path();
            println!("rendering template at {}", path.display());
            let basename = Self::basename(path.as_path())?;
            let result = if path.is_dir() {
                let writer = Self::new(path.clone().into(), self.out_dir.join(basename).into())?;
                writer.write()
            } else if path.is_file() {
                self.write_file(&path)
            } else {
                Err(std::io::Error::new(ErrorKind::Other, format!("file '{:?}' is not a file or directory. Is it a device, symlink, or other 'fake' file?", path)).into())
            };
            if result.is_err() {
                return result;
            }
        }
        Ok(())
    }
    fn write_file(self: &Self, path: &Path) -> Result<()> {
        let mut output = File::create(self.out_dir.join(Self::basename(path)?))?;
        let mut bars = Handlebars::new();
        if CONFIG.templates.is_some() {
            let mut file = File::open(path)?;
            bars.register_template_string("layout", LAYOUT_CONTENT.as_str())?;
            let mut content = LayoutBuffer::new(
                br#"{{#*inline "content"}}"#,
                &mut file,
                b"{{/inline}}\n{{> layout}}",
            );
            let mut text = String::new();
            println!("{}\n{}", content.read_to_string(&mut text)?, text);
            // bars.register_template_source("content", &mut content)?;
            bars.register_template_string("content", text)?;
        } else {
            bars.register_template_file("layout", path)?;
        }
        /*
         * TODO!
         *
         * This string-based implementation will fail (due to the OOM
         * condition) for very large files, consumes an unnecessary amount
         * of memory, and is innefficient. It should be replaced by
         * something which pipes the output of `bars.render_to_write()`
         * into `minify_from_read()` and then into `&mut file`.
         */
        let text = bars.render("content", &CONFIG.template_variables)?;
        let text = minify(&text);
        Ok(output.write_all(text.as_bytes())?)
    }
}

fn render_404() -> Result<()> {
    if let Some(templates) = &CONFIG.templates {
        let mut bars = Handlebars::new();
        bars.register_template_file("page", &templates.error)?;
        let file = File::create(Path::new(TEMPLATE_DIR).join("404.html"))?;
        Ok(bars.render_to_write(
            "page",
            &serde_json::json!({
                "status_code":  "404",
                "status_message": "Not Found"
            }),
            file,
        )?)
    } else {
        Ok(())
    }
}

// pass all the static HTML files to validate and render
pub fn render_templates() -> Result<()> {
    // create binserve templates directory if it doesn't exist
    if !Path::new(&TEMPLATE_DIR).exists() {
        fs::create_dir(TEMPLATE_DIR).ok();
    }

    TemplateWriter::new(
        Path::new(&CONFIG.static_directory).into(),
        Path::new(TEMPLATE_DIR).into(),
    )?
    .write()?;

    render_404()
}
