use crate::dstructs::Packet;
use crate::error::Error;
use crate::proto::Proto;

pub trait Hdr {
  fn create(&self) -> Result<Packet, Error>;
  fn parse(bytes: Packet) -> Self;
  fn get(&self) -> Proto;
}
