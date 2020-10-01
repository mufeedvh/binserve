/*
    `error_page.rs` - Error Page Generation
    generate error pages
*/
use crate::config::get_config;
use std::fs::File;
use std::io::Write;
use std::path::Path;

// create a default `404 Not Found` error page if it doesn't exist
fn generate_404_page(static_dir: &str) -> std::io::Result<()> {
    let contents = b"<html>\
    <title>404 Not Found</title>\
    <body>\
    <h1>404 Not Found</h1>\
    <hr>\
    <p><i>binserve v0.1.0</i></p>\
    </body>\
    </html>";

    let file_path = format!("{}/404.html", static_dir);
    if !Path::new(&file_path).exists() {
        let mut file = File::create(file_path)?;
        file.write_all(contents)?;
    }
    Ok(())
}

pub fn generate_error_pages() {
    let config = get_config();
    let static_dir = config["static_directory"].to_string().replace("\"", "");
    generate_404_page(&static_dir).ok();
}
