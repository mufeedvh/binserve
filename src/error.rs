use std::fmt::{Display, Formatter, Result as FmtResult};
use std::process::exit;

#[derive(Debug)]
pub enum Error {
  IOError(std::io::Error),
  HandlebarsParseError(handlebars::TemplateFileError),
  HandlebarsRenderError(handlebars::RenderError),
  HandlebarsParseStringError(handlebars::TemplateError),
  ConfigError(String),
  Generic(String),
}

impl Error {
  pub fn config(message: &str) -> Self {
    Self::ConfigError(String::from(message))
  }
  pub fn fatal(self: &Self) -> ! {
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

impl Display for Error {
  fn fmt(self: &Self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      Self::IOError(err) => write!(f, "io error: {}", err),
      Self::HandlebarsParseError(err) => write!(f, "handlebars parse error: {}", err),
      Self::HandlebarsRenderError(err) => write!(f, "handlebars render error: {}", err),
      Self::HandlebarsParseStringError(err) => write!(f, "handlebars parse error: {}", err),
      Self::ConfigError(err) => write!(f, "error in config: {}", err),
      Self::Generic(err) => write!(f, "error: {}", err),
    }
  }
}

pub type Result<T> = std::result::Result<T, Error>;
