//! Contains `Error` struct and `ErrorType` enum

/// Enum containing possible errors thrown from any function from library
#[derive(Debug)]
pub enum ErrorType {
    ParseError,
    ConstructError,
    ChannelError,
    InterfaceError,
    UnwrapHeaderError,
    PcapFileError,
    LengthError,
}

/// This error struct is used in error handling of this library
#[derive(Debug)]
pub struct PaError {
    pub err_type: ErrorType,
    pub msg: String,
}

impl PaError {
    /// Creates new instance of Error Struct
    pub fn new<T: ToString>(msg: T, er_type: ErrorType) -> Self {
        Self {
            err_type: er_type,
            msg: msg.to_string(),
        }
    }
}
