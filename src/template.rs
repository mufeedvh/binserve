/*
    `template.rs` - Templating Engine
    generates templates of static HTML files with `Handlebars`
*/
mod content_buffer;
use std::collections::HashMap;
use std::fs;
use std::fs::{create_dir_all, read_dir, File};
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
pub const TEMPLATE_DIR: &str = "rendered_templates";

/// read the configured layout file
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

// iterate over the files in the directory, recursively adding their content to the list of partials
fn read_partials_impl(partials: &mut HashMap<String, String>, dir: &String) -> Result<()> {
    for f in read_dir(dir)? {
        if let Ok(f) = f {
            if let Ok(ft) = f.file_type() {
                if ft.is_dir() {
                    read_partials_impl(
                        partials,
                        &String::from(
                            f.path()
                                .to_str()
                                .ok_or(Error::from(format!("invalid path {:?}", f)))?,
                        ),
                    )?;
                } else if ft.is_file() {
                    let path = f.path();
                    if let Some(path_str) = path.to_str() {
                        let mut file = File::open(&path)?;
                        let mut string = String::new();
                        file.read_to_string(&mut string)?;
                        partials.insert(String::from(path_str), string);
                    } else {
                        println!("WARNING: invalid file path at {}", path.display())
                    }
                }
            }
        }
    }
    Ok(())
}

///
fn read_partials() -> Result<HashMap<String, String>> {
    let mut partials = HashMap::new();
    if let Some(templates) = &CONFIG.templates {
        read_partials_impl(&mut partials, &templates.partials_directory)?;
    }
    Ok(partials)
}

lazy_static! {
    /// The content of the configured layout file. Empty if no such
    /// configuration.
    pub static ref LAYOUT_CONTENT: String = read_layout();
    pub static ref PARTIALS: HashMap<String, String> = read_partials().expect("failed to read partials directory");
}

struct TemplateWriter {
    base_dir: Box<Path>,
    out_dir: Box<Path>,
}

impl TemplateWriter {
    fn new(base_dir: Box<Path>, out_dir: Box<Path>) -> Self {
        Self {
            base_dir: base_dir,
            out_dir: out_dir,
        }
    }
}

impl TemplateWriter {
    /// iterate over `base_dir`, rendering each file to `out_dir`.
    fn write(&self) -> Result<()> {
        if !self.base_dir.exists() {
            create_dir_all(&self.base_dir)?;
        }
        for path in self.base_dir.read_dir()? {
            let path = path?.path();
            println!("rendering template at {}", path.display());
            let basename = path.as_path().strip_prefix(&self.base_dir)?;
            let result = if path.is_dir() {
                let out_dir = self.out_dir.join(basename);
                create_dir_all(&out_dir)?;
                let writer = Self::new(path.clone().into(), out_dir.into());
                writer.write()
            } else if path.is_file() {
                self.write_file(&path)
            } else {
                Err(
                    std::io::Error::new(
                        ErrorKind::Other,
                        format!("file '{:?}' is not a file or directory. Is it a device, symlink, or other 'fake' file?", path),
                    ).into(),
                )
            };
            if result.is_err() {
                return result;
            }
        }
        Ok(())
    }
    /// render the file at the given path
    fn write_file(&self, path: &Path) -> Result<()> {
        let mut output = File::create(self.out_dir.join(path.strip_prefix(&self.base_dir)?))?;
        let mut bars = Handlebars::new();
        if CONFIG.templates.is_some() {
            let mut file = File::open(path)?;
            bars.register_template_string("layout", LAYOUT_CONTENT.as_str())?;
            let mut content = LayoutBuffer::new(
                br#"{{#*inline "content"}}"#,
                &mut file,
                b"{{/inline}}\n{{> layout}}",
            );
            bars.register_template_source("content", &mut content)?;
            for (path, content) in PARTIALS.iter() {
                bars.register_template_string(path, content)?;
            }
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
        let text = if CONFIG.minify { minify(&text) } else { text };
        Ok(output.write_all(text.as_bytes())?)
    }
}

/// pass all the static HTML files to validate and render
pub fn render_templates() -> Result<()> {
    // create binserve templates directory if it doesn't exist
    if !Path::new(&TEMPLATE_DIR).exists() {
        fs::create_dir(TEMPLATE_DIR)?;
    }

    TemplateWriter::new(
        Path::new(&CONFIG.static_directory).into(),
        Path::new(TEMPLATE_DIR).into(),
    )
    .write()
}
