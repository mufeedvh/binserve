use std::cmp::min;
use std::{
  fmt,
  fmt::{Display, Formatter},
};

/// This is used to create a wrapper around template files to inject handlebars
/// instructions before and after the template data provided by the user.
/// Essentially, this eliminates boilerplate that would have to be repeated in
/// every content template.
pub struct LayoutBuffer<'a> {
  /// the part that comes before the template
  prologue: &'static [u8],
  /// The template data
  content: &'a mut dyn std::io::Read,
  /// the part that comes after the template
  epilogue: &'static [u8],
  /// what part of the process we are in, see enum `Stage`
  stage: Stage,
  /// used if a partial copy is needed for the prologe and epilogue stages,
  /// this keeps track of how much has been read of that buffer.
  pos: usize,
}

impl<'a> LayoutBuffer<'a> {
  pub fn new(
    prologue: &'static [u8],
    content: &'a mut dyn std::io::Read,
    epilogue: &'static [u8],
  ) -> Self {
    Self {
      prologue: prologue,
      content: content,
      epilogue: epilogue,
      stage: Prologue,
      pos: 0,
    }
  }
  /// performs `read()` on either the prologue or the epilogue, depending on
  /// the given `OtherStage` value.
  fn read_prologue_or_epilogue(
    self: &mut Self,
    buf: &mut [u8],
    stage: OtherStage,
  ) -> std::io::Result<usize> {
    let (relevant_buffer, next_stage) = match stage {
      OtherStage::Prologue => (&self.prologue, Content),
      OtherStage::Epilogue => (&self.epilogue, EOF),
    };
    let size = min(buf.len(), relevant_buffer[self.pos..].len());
    buf[..size].copy_from_slice(&relevant_buffer[self.pos..self.pos + size]);
    self.pos += size;
    if self.pos >= relevant_buffer.len() {
      self.stage = next_stage;
      self.pos = 0;
    }
    Ok(size)
  }
}

impl<'a> std::io::Read for LayoutBuffer<'a> {
  fn read(self: &mut Self, buf: &mut [u8]) -> std::io::Result<usize> {
    match self.stage {
      Prologue => self.read_prologue_or_epilogue(buf, OtherStage::Prologue),
      Content => match self.content.read(buf) {
        Ok(read_count) => {
          if read_count == 0 {
            self.stage = Epilogue;
            self.read(buf)
          } else {
            Ok(read_count)
          }
        }
        err => err,
      },
      Epilogue => self.read_prologue_or_epilogue(buf, OtherStage::Epilogue),
      EOF => Ok(0),
    }
  }
}
/// Stage used by `LayoutBuffer::read()`.
enum Stage {
  Prologue,
  Content,
  Epilogue,
  EOF,
}
/// Stage passed to `LayoutBuffer::read_prologue_or_epilogue` by
/// `LayoutBuffer::read()`. This ensures at compile-time that
/// `LayoutBuffer::read_prologue_or_epilogue` is never passed one of the
/// irrellevant `Stage`s.
enum OtherStage {
  Prologue,
  Epilogue,
}

/// 👌
use Stage::*;

impl Display for Stage {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Prologue => "Prologue",
        Content => "Content",
        Epilogue => "Epilogue",
        EOF => "EOF",
      }
    )
  }
}
