use crate::dstructs::Packet;
use crate::error::PaError;
use crate::proto::Proto;

pub trait Hdr {
    fn create(&self) -> Result<Packet, PaError>;
    fn parse(bytes: Packet) -> Self;
    fn get(&self) -> Proto;
}
