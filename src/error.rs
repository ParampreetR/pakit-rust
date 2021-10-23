#[derive(Debug)]
pub enum ErrorType {
  ParseError,
  ConstructError,
  ChannelError,
  InterfaceError,
  UnwrapHeaderError,
}

#[derive(Debug)]
pub struct Error {
  pub err_type: ErrorType,
  pub msg: String,
}

impl Error {
  pub fn new<T: ToString>(msg: T, er_type: ErrorType) -> Self {
    Self {
      err_type: er_type,
      msg: msg.to_string(),
    }
  }
}
