use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};

use compact_str::CompactString;

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::time::Duration;

use super::config::{BinserveConfig, CONFIG_FILE};
use super::routes::{RouteHandle, Type, ROUTEMAP};
use super::templates;

/// Watch for filesystem for updates/writes and hot reload the server state.
pub fn hot_reload_files() -> anyhow::Result<()> {
    let config_state = BinserveConfig::read()?;

    // check if hot reload is enabled or not
    if !config_state.config.enable_hot_reload {
        return Ok(());
    }

    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Create a watcher object, delivering debounced events.
    // The notification back-end is selected based on the platform.
    let mut watcher = watcher(tx, Duration::from_secs(1))?;

    let mut file_mapping: HashMap<PathBuf, CompactString> = HashMap::with_capacity(ROUTEMAP.len());

    // add the binserve config file to the hot reloader
    let config_file_path = PathBuf::from(CONFIG_FILE);
    let abs_config_path = fs::canonicalize(config_file_path)?;
    watcher.watch(CONFIG_FILE, RecursiveMode::Recursive)?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    for route in ROUTEMAP.iter() {
        let handler = route.value();

        if handler.r#type == Type::Bytes {
            let key = route.key();
            let file_path = &handler.response.path;

            if *file_path == PathBuf::new() {
                continue;
            }

            let abs_file_path = fs::canonicalize(file_path)?;

            // add to the system filesystem events watch list
            watcher.watch(file_path, RecursiveMode::Recursive)?;

            // map them to the corresponding keys in the routemap
            file_mapping.insert(abs_file_path, key.to_owned());
        }
    }

    loop {
        match rx.recv() {
            Ok(event) => {
                match event {
                    DebouncedEvent::Write(file_path)
                    | DebouncedEvent::Create(file_path)
                    | DebouncedEvent::Remove(file_path) => {
                        if file_path == abs_config_path {
                            // read the configuration file
                            let config = BinserveConfig::read()?;

                            // prepare template partials
                            let handlebars_handle = templates::render_templates(&config)?;

                            // prepare routes table
                            RouteHandle::add_routes(&config.routes, &handlebars_handle)?;
                        }

                        if let Some(route_key) = file_mapping.get(&file_path) {
                            // read the configuration file
                            let config = BinserveConfig::read()?;

                            // prepare template partials
                            let handlebars_handle = templates::render_templates(&config)?;

                            // reload the file state and update the global program state
                            RouteHandle::associate_files_to_routes(
                                &route_key.to_string(),
                                &file_path,
                                &handlebars_handle,
                            )?;
                        }
                    }
                    _ => (),
                }
            }
            Err(e) => {
                println!("[!] filesystem watch error (binserve hot reload): {:?}", e)
            }
        }
    }
}
