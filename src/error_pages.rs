/*
    `error_page.rs` - Error Page Generation
    generate error pages
*/
use crate::config::CONFIG;
use crate::error::{Error, Result};
use std::fs::File;
use std::io::Write;

pub fn generate_error_template() -> Result<()> {
    const CONTENTS: &[u8] = b"
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
        Ok(File::create(&templates.error)?.write_all(CONTENTS)?)
    } else {
        Err(Error::config(
            "trying to generate templates, but the location is not configured",
        ))
    }
}
