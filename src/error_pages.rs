/*
    `error_page.rs` - Error Page Generation
    generate error pages
*/
use crate::config::CONFIG;
use std::fs::File;
use std::io::Write;
use std::path::Path;

// create a default `404 Not Found` error page if it doesn't exist
fn generate_404_page(static_dir: &str) -> std::io::Result<()> {
    const CONTENTS: &str = "<html>
    <title>404 Not Found</title>
    <body>
        <h1>404 Not Found</h1>
        <hr>
        <p><i>binserve v0.1.0</i></p>
    </body>
</html>";

    let file_path = format!("{}/404.html", static_dir);
    if !Path::new(&file_path).exists() {
        let mut file = File::create(file_path)?;
        file.write_all(CONTENTS.as_bytes())?;
    }
    Ok(())
}

fn generate_500_page(static_dir: &str) -> std::io::Result<()> {
    const CONTENTS: &str = "<html>
    <title>500 Internal Server Error</title>
    <body>
        <h1>500 Internal Server Error</h1>
        <hr>
        <p><i>binserve v0.1.0</i></p>
    </body>
</html>";

    let file_path = format!("{}/500.html", static_dir);
    if !Path::new(&file_path).exists() {
        let mut file = File::create(file_path)?;
        file.write_all(CONTENTS.as_bytes())?;
    }
    Ok(())
}

pub fn generate_error_pages() {
    generate_404_page(&CONFIG.static_directory).ok();
    generate_500_page(&CONFIG.static_directory).ok();
}
