use std::fmt::{Display, Formatter, Result as FmtResult};
use std::path::StripPrefixError;
use std::process::exit;

/// Unified error type across the crate. Any errors which may be returned by
/// any function should be integrated as a variant of this enum, in order to
/// enable consistent use of the `?` operator across the crate.
#[derive(Debug)]
pub enum Error {
  IOError(std::io::Error),
  HandlebarsParseError(handlebars::TemplateFileError),
  HandlebarsRenderError(handlebars::RenderError),
  HandlebarsParseStringError(handlebars::TemplateError),
  HandlebarsParseRenderError(handlebars::TemplateRenderError),
  ConfigError(String),
  PathStripPrefix(StripPrefixError),
  Generic(String),
}

impl Error {
  /// An error indicating invalid configuration
  pub fn config(message: &str) -> Self {
    Self::ConfigError(String::from(message))
  }
  /// An error which causes the program to abort.
  pub fn fatal(&self) -> ! {
    println!("{}", self);
    exit(1);
  }
}

impl From<std::io::Error> for Error {
  fn from(err: std::io::Error) -> Self {
    Self::IOError(err)
  }
}

impl From<handlebars::TemplateFileError> for Error {
  fn from(err: handlebars::TemplateFileError) -> Self {
    Self::HandlebarsParseError(err)
  }
}

impl From<handlebars::RenderError> for Error {
  fn from(err: handlebars::RenderError) -> Self {
    Self::HandlebarsRenderError(err)
  }
}

impl From<String> for Error {
  fn from(err: String) -> Self {
    Self::Generic(err)
  }
}

impl From<handlebars::TemplateError> for Error {
  fn from(err: handlebars::TemplateError) -> Self {
    Self::HandlebarsParseStringError(err)
  }
}

impl From<handlebars::TemplateRenderError> for Error {
  fn from(err: handlebars::TemplateRenderError) -> Self {
    Self::HandlebarsParseRenderError(err)
  }
}

impl From<StripPrefixError> for Error {
  fn from(err: StripPrefixError) -> Self {
    Self::PathStripPrefix(err)
  }
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      Self::IOError(err) => write!(f, "io error: {}", err),
      Self::HandlebarsParseError(err) => write!(f, "handlebars parse error: {}", err),
      Self::HandlebarsRenderError(err) => write!(f, "handlebars render error: {}", err),
      Self::HandlebarsParseStringError(err) => write!(f, "handlebars parse error: {}", err),
      Self::HandlebarsParseRenderError(err) => write!(f, "handlebars parse error: {}", err),
      Self::PathStripPrefix(err) => write!(f, "error stripping prefix of path: {}", err),
      Self::ConfigError(err) => write!(f, "error in config: {}", err),
      Self::Generic(err) => write!(f, "error: {}", err),
    }
  }
}

/// allow `Error` variants to be returned from the actix handler
impl actix_web::ResponseError for Error {}

/// crate-unified `std::result::Result` variant, using the crate-unified
/// `Error` type.
pub type Result<T> = std::result::Result<T, Error>;
