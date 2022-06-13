use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};

use std::fs::File;
use std::io::BufReader;

use anyhow::{Context, Result};

use super::config::CONFIG_STATE;

use crate::cli::messages::{Type, push_message};

/// Load TLS configuration
pub fn load_rustls_config() -> Result<rustls::ServerConfig> {
    let config_state = &*CONFIG_STATE.lock();

    // init server config builder with safe defaults
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();

    let cert_file_path = &config_state.server.tls.cert;
    let cert_key_path = &config_state.server.tls.key;

    // load TLS key/cert files
    let cert_file = &mut BufReader::new(
        File::open(cert_file_path)
            .with_context(|| format!("Failed to read file {:?}", cert_file_path.to_string_lossy()))?
    );

    let key_file = &mut BufReader::new(
        File::open(cert_key_path)
            .with_context(|| format!("Failed to read file {:?}", cert_key_path.to_string_lossy()))?
    );

    // convert files to key/cert objects
    let cert_chain = certs(cert_file)
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect();

    let mut keys: Vec<PrivateKey> = pkcs8_private_keys(key_file)
        .unwrap()
        .into_iter()
        .map(PrivateKey)
        .collect();

    // exit if no keys could be parsed
    if keys.is_empty() {
        push_message(Type::Error, "Could not locate PKCS 8 private keys.");
        std::process::exit(1);
    }

    Ok(
        config.with_single_cert(cert_chain, keys.remove(0))?
    )
}