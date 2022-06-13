use clap::{Command, Arg, ArgMatches};

use crate::core::VERSION;

/// Prints an ASCII art banner to look cool!
pub fn banner() {
    eprintln!(
        "{}", format!("{} {}\n", include_str!("banner"), VERSION)
    )
}

/// Command-line arguments
pub fn args() -> ArgMatches {
    Command::new("binserve")
        .version(VERSION)
        .author("Mufeed VH <mufeed@lyminal.space>")
        .about("A fast static web server with Automatic HTTPs, routing, templating, and security in a single binary you can setup with zero code.")
        .arg(Arg::new("command")
            .help("Command to run.")
            .value_name("COMMAND")
            .required(false)
            .index(1))
        .arg(Arg::new("host")
            .short('h')
            .long("host")
            .value_name("HOST IP/DOMAIN:PORT")
            .help("Host to run binserve on.")
            .required(false)
            .takes_value(true))
        .arg(Arg::new("tls_key")
            .short('k')
            .long("key")
            .value_name("TLS KEY")
            .help("TLS key file.")
            .required(false)
            .takes_value(true))            
        .arg(Arg::new("tls_cert")
            .short('c')
            .long("cert")
            .value_name("TLS CERT")
            .help("TLS cert file.")
            .required(false)
            .takes_value(true))           
        .get_matches()
}