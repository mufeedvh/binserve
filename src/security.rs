/*
    `security.rs` - Security Validation Functions
    validates 'route' and 'destination' in the configuration file for potential attacks
*/
use std::fs;
use std::path::Path;
use std::process;

use crate::config::get_config;

/*
    reading symlink files pointing to a sensitive file can leak information,
    this function validates a file is a symlink or not
*/
fn safe_symlink(file: &str) -> i32 {
    let config = get_config();

    let follow_symlink = config["follow_symlinks"].to_string();
    let static_dir = config["static_directory"].to_string().replace("\"", "");

    let mut vulns_found = 0;

    // if `follow_symlinks` is enabled in config
    if follow_symlink != "true" {
        let file_path = format!("{}/{}", static_dir, file);

        if Path::new(&file_path).exists() {
            // check if file is a symlink
            let file_metadata = fs::symlink_metadata(file_path);
            let is_symlink = file_metadata.unwrap().file_type().is_symlink();
            if is_symlink {
                // increment vulnerabilities found
                vulns_found += 1;
                // print out help and error message to rectify the issues
                println!(
                    "\n[!] ERROR::FOUND_SYMLINK: The `{}/{}` file is a symlink.\n",
                    static_dir, file
                );
                println!(
                    "\n[-] INFO: You've disabled symlinks in your configuration as it can lead to potential attacks.\n"
                );
                println!(
                    "\n[?] WHAT TO DO: You can either allow symlinks or delete the symlink file at `{}/{}`\n", static_dir, file
                );
            }
        }
    }

    vulns_found
}

/*
    reading files outside the process directory can leak information,
    this function checks if the file is outside the process directory
*/
fn path_traversal(route: &str) -> i32 {
    let mut vulns_found = 0;

    if route.contains("..") {
        // increment vulnerabilities found
        vulns_found += 1;
        // print out help and error message to rectify the issues
        println!(
            "\n[!] ERROR::PATH_TRAVERSAL: The `{}` file is pointed outside the `static`\
            directory you specified in your configuration.\n",
            route
        );
    }

    vulns_found
}

// iterate through all the route files and pass to security validation functions
fn validate_file(routes: serde_json::map::Iter) -> std::io::Result<()> {
    // total vulnerabilities found
    let mut vulns_found = 0;

    // validate route files
    for (_key, value) in routes {
        let mut file = value.to_string().replace("\"", "");
        // this function returns an `int` of vulnerabilities found
        vulns_found += safe_symlink(&file);
        file = value.to_string().replace("\"", "");
        // this function returns an `int` of vulnerabilities found
        vulns_found += path_traversal(&file);
    }

    // print out notice message and exit the process in case of any potential vulnerabilities
    if vulns_found > 0 {
        println!(
            "\n[!] TOTAL POTENTIAL VULNERABILITIES FOUND: {}\n",
            vulns_found
        );
        println!(
            "\n[-] INFO: Please fix all the potential vulnerable configurations \
            in your `binserve.json` to run server.\n"
        );
        process::exit(1);
    }

    Ok(())
}

// pass all the route files for validation
pub fn is_config_secure() {
    let config = get_config();

    // converting JSON structures to iterable structures
    let routes = config["routes"].as_object().unwrap().into_iter();
    let error_pages = config["error_pages"].as_object().unwrap().into_iter();

    validate_file(routes).ok();
    validate_file(error_pages).ok();
}
