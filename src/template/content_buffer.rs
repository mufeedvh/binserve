use std::cmp::min;

enum Stage {
  Prologue,
  Content,
  Epilogue,
  EOF,
}
use Stage::*;

pub struct LayoutBuffer<'a> {
  prologue: &'static [u8],
  content: &'a mut dyn std::io::Read,
  epilogue: &'static [u8],
  stage: Stage,
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
}

impl<'a> std::io::Read for LayoutBuffer<'a> {
  fn read(self: &mut Self, buf: &mut [u8]) -> std::io::Result<usize> {
    match self.stage {
      Prologue => {
        let size = min(buf.len(), self.prologue[self.pos..].len());
        buf[..size].copy_from_slice(&self.prologue[self.pos..self.pos + size]);
        self.pos += size;
        if self.pos >= self.prologue.len() {
          self.stage = Stage::Content;
          self.pos = 0;
        }
        Ok(size)
      }
      Content => match self.content.read(buf) {
        Ok(read_count) => {
          if read_count == 0 {
            self.stage = Epilogue;
            self.read(buf)
          } else {
            Ok(read_count)
          }
        }
        err => {
          println!("{:?}", err);
          err
        }
      },
      Epilogue => {
        let size = min(buf.len(), self.epilogue[self.pos..].len());
        buf[..size].copy_from_slice(&self.epilogue[self.pos..self.pos + size]);
        self.pos += size;
        if self.pos >= self.epilogue.len() {
          self.stage = Stage::EOF;
          self.pos = 0;
        }
        Ok(size)
      }
      EOF => Ok(0),
    }
  }
}
