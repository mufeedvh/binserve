use actix_web::{
    http::header::{HeaderValue, HttpDate},
    web::Bytes,
};

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, prelude::*};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use etag::EntityTag;

use handlebars::{Context as HbsContext, Handlebars};

use minify_html_onepass::Cfg;

use super::config::{CONFIG_FILE, CONFIG_STATE};

/// Represents a static file
#[derive(Debug)]
pub struct StaticFile {
    pub mime: Option<HeaderValue>,          // mime type of the file
    pub bytes: Bytes,                       // contents of the file in bytes
    pub path: PathBuf,                      // path to the file in disk
    pub etag: Option<HeaderValue>,          // etag header value (RFC 7232 §2.3)
    pub last_modified: Option<HeaderValue>, // last modified system time (RFC 7232 §2.2)
    pub hbs_bytes: Bytes,                   // to read pre-rendered handlebars content
}

/// Max file size allowed to be cached in memory
const MAX_FILE_SIZE: u64 = 104_857_600;

impl StaticFile {
    /// Creates a static file instance
    pub fn create(path: &PathBuf, handlebars_handle: &(Handlebars, HbsContext)) -> Result<Self> {
        // read the current config state
        let config_state = &*CONFIG_STATE.lock();

        // read the file
        let file = File::open(path)
            .with_context(|| format!("Failed to read file {:?}", path.to_string_lossy()))?;

        let file_size = file.metadata()?.len();

        // file contents as Bytes object
        let mut contents = Bytes::from(fs::read(path)?);

        let file_metadata = std::fs::metadata(path)?;

        // if configured not to follow symlinks
        if file_metadata.is_symlink() && !config_state.config.follow_symlinks {
            return Ok(Self {
                mime: None,
                bytes: Bytes::new(),
                path: path.to_path_buf(),
                etag: None,
                last_modified: None,
                hbs_bytes: Bytes::new(),
            });
        }

        // derive an etag from the file's metadata
        let etag = EntityTag::from_file_meta(&file_metadata);

        // derive the last modified time of the file
        let last_modified: HttpDate = file_metadata.modified()?.into();

        // get the mime type of the file
        // by default the mime type falls to `application/octet-stream`
        let mut mime_type = new_mime_guess::from_path(path)
            .first_raw()
            .unwrap_or("application/octet-stream");

        // render handlebars templates (.hbs templates)
        let mut hbs_prerendered_bytes = Bytes::new();

        let ext = path.extension();

        if ext.is_some() {
            let extension: &str = ext.unwrap().to_str().unwrap();

            // identify handlebars template
            if extension == "hbs" {
                mime_type = "text/html"; // it's rendered to HTML at initialization

                // handlebars registered handle and context with the template variables
                let (hbs_reg, hbs_ctx) = handlebars_handle;

                // render the template
                contents = Bytes::from(hbs_reg.render_template_with_context(
                    &String::from_utf8_lossy(&contents[..]),
                    hbs_ctx,
                )?);

                hbs_prerendered_bytes = contents.to_owned();
            }
        }

        // minify html if configured
        if mime_type == "text/html" && config_state.config.minify_html {
            if let Ok(minified_html) = minify_html_onepass::copy(&contents, &Cfg::new()) {
                contents = Bytes::from(minified_html);
            }
        }

        // prepared header values
        let mime = Some(HeaderValue::from_str(mime_type)?);
        let etag = Some(HeaderValue::from_str(&etag.to_string())?);
        let last_modified = Some(HeaderValue::from_str(&last_modified.to_string())?);

        // only save the file in-memory if the size is less than 100 MB
        //
        // So what if multiple files cumulatively make up to a bigger size?
        // That's the reason `fast_mem_cache` feature exists. Disable it in
        // those scenarios. It's a very rare scenario and only bound to happen
        // when it's a website with thousands of static files in which case
        // one should totally disable in-memory caching of files unless you
        // have a chonker of a RAM.
        //
        // TODO: How about caching frequently accessed files? Like a priority cache?
        //
        // It skips this whole step if the `fast_mem_cache` feature is disabled.
        if file_size < MAX_FILE_SIZE && config_state.config.fast_mem_cache {
            return Ok(Self {
                mime,
                bytes: contents,
                path: path.to_path_buf(),
                etag,
                last_modified,
                hbs_bytes: Bytes::new(),
            });
        }

        // fallbacks to file from disk by default
        Ok(Self {
            mime,
            bytes: Bytes::new(),
            path: path.to_path_buf(),
            etag,
            last_modified,
            hbs_bytes: hbs_prerendered_bytes,
        })
    }
}

/// Generate the 404 Not Found template.
pub fn generate_not_found() -> Result<StaticFile> {
    let config = &*CONFIG_STATE.lock();

    // default not found template
    let mut not_found_template = Bytes::from(include_bytes!("../starter/404.html").to_vec());

    // user defined error pages
    let error_page_map = &config.r#static.error_pages;

    // for defined error page templates
    if !error_page_map.is_empty() {
        if let Some(template_path) = error_page_map.get(&404) {
            let read_file = fs::read(template_path).with_context(|| {
                format!("Failed to read file {:?}", template_path.to_string_lossy())
            })?;
            not_found_template = Bytes::from(read_file)
        }
    }

    Ok(StaticFile {
        mime: None,
        bytes: not_found_template,
        path: PathBuf::new(),
        etag: None,
        last_modified: None,
        hbs_bytes: Bytes::new(),
    })
}

/// Generate the starter boilerplate.
pub fn generate_starter_boilerplate() -> io::Result<()> {
    // if the config file is not there, that means it's the first run
    // the configuration boilerplate generation does the same thing
    if !Path::new(CONFIG_FILE).exists() && !Path::new("public").exists() {
        // contain the boilerplate starter code in binary
        let starter_directory: HashMap<PathBuf, Vec<u8>> = HashMap::from([
            // public root directory
            (
                "public/index.html".into(),
                include_bytes!("../starter/index.html").to_vec(),
            ),
            (
                "public/404.html".into(),
                include_bytes!("../starter/404.html").to_vec(),
            ),
            (
                "public/usage.hbs".into(),
                include_bytes!("../starter/usage.hbs").to_vec(),
            ),
            (
                "public/header.hbs".into(),
                include_bytes!("../starter/header.hbs").to_vec(),
            ),
            // assets directory
            (
                "public/assets/css/styles.css".into(),
                include_bytes!("../starter/assets/css/styles.css").to_vec(),
            ),
            (
                "public/assets/images/binserve.webp".into(),
                include_bytes!("../starter/assets/images/binserve.webp").to_vec(),
            ),
        ]);

        // prepare the default public directories
        let directories = ["public/assets/css", "public/assets/images"];

        for entry in directories {
            std::fs::create_dir_all(entry)?;
        }

        // write the static files to disk
        for (path, contents) in starter_directory {
            let mut file = File::create(path)?;
            file.write_all(&contents)?;
        }
    }

    Ok(())
}
