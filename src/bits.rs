use crate::error::*;
use std::fmt::{Binary, Display, Formatter, Result as ResultFmt, Write};

#[derive(Debug, Clone)]
pub struct Bits {
  /// This is a length of string but it contains data in binary like '0110'
  length: u8, /* I don't want something to be more than 255 bits */
  data: String, /* Is something more better than String in this Structure? */
}

impl Bits {
  pub fn new(len: u8) -> Self {
    Self {
      length: len,
      data: String::new(),
    }
  }

  pub fn from_bin<T: ToString>(data: T, len: u8) -> Result<Self, Error> {
    let data = data.to_string();

    if data.len() > len.into() {
      Err(Error::new(
        "data provided is larger than provided length",
        ErrorType::LengthError,
      ))
    } else {
      Ok(Self {
        length: len,
        data: data.to_string(),
      })
    }
  }

  pub fn check_error(&self) {
    if self.data.len() > self.length.into() {
      panic!("data provided is larger than provided length")
    }
  }

  pub fn from(data: usize, len: u8) -> Self {
    let bin = format!("{:b}", data).to_string();
    if bin.len() > len.into() {
      panic!("data provided is larger than provided length")
    } else {
      Self {
        length: len,
        data: bin,
      }
    }
  }

  pub fn to_bits(&self) -> String {
    self.check_error();
    let mut len = self.data.len() as u8;
    let mut val = String::new();
    while self.length > len {
      val.push('0');
      len += 1;
    }

    val.push_str(&self.data);
    val
  }
}

impl Display for Bits {
  fn fmt(&self, f: &mut Formatter) -> ResultFmt {
    self.check_error();
    let val = usize::from_str_radix(&self.data, 2).unwrap();
    f.write_str(&format!("{}", val))
  }
}

impl Binary for Bits {
  fn fmt(&self, f: &mut Formatter) -> ResultFmt {
    self.check_error();
    let mut len = self.data.len() as u8;
    while self.length > len {
      f.write_char('0').unwrap();
      len += 1;
    }

    f.write_str(&self.data)
  }
}

impl Into<String> for Bits {
  fn into(self) -> String {
    self.check_error();
    self.data
  }
}

impl Into<usize> for Bits {
  fn into(self) -> usize {
    self.check_error();
    usize::from_str_radix(&self.data, 2).unwrap()
  }
}

impl From<usize> for Bits {
  fn from(val: usize) -> Self {
    let bin = format!("{:b}", val).to_string();
    if bin.len() > 64 {
      panic!("Data too large!");
    } else {
      Self {
        length: 64,
        data: bin,
      }
    }
  }
}

impl PartialEq for Bits {
  fn eq(&self, other: &Bits) -> bool {
    self.check_error();
    if self.length == other.length {
      if self.data == other.data {
        true
      } else {
        false
      }
    } else {
      false
    }
  }
}
