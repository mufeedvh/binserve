use colored::*;

/// CLI message types
pub enum Type {
    _Warning,
    _Skipped,
    Error,
    Info,
    Success,
}

/// Outputs CLI messages
// NOTE: this is not logging, just meant to be messages for the CLI user
pub fn push_message(log_type: Type, message: &str) {
    let prefix = match log_type {
        Type::_Warning => format!("{}{}{}", "[".bold(), "WARN".bold().yellow(), "]".bold()),
        Type::_Skipped => format!("{}{}{}", "[".bold(), "SKIPPED".bold().yellow(), "]".bold()),
        Type::Error => format!("{}{}{}", "[".bold(), "ERROR".bold().red(), "]".bold()),
        Type::Info => format!("{}{}{}", "[".bold(), "INFO".bold().cyan(), "]".bold()),
        Type::Success => format!("{}{}{}", "[".bold(), "SUCCESS".bold().green(), "]".bold()),
    };
    eprintln!("{} {}", prefix, message)
}
