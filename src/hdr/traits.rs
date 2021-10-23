use crate::error::Error;
use crate::proto::Proto;

pub trait Hdr {
  fn create(&self) -> Result<Vec<u8>, Error>;
  fn parse(bytes: &[u8]) -> Self;
  fn get(&self) -> Proto;
}
