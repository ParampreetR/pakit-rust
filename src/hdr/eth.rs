use super::traits::Hdr;
use crate::dstructs::{Bits, Packet};
use crate::error::{Error, ErrorType};
use crate::proto::{EthType, Proto};
use crate::utility::{from_ethtype, mac_to_string};
use std::convert::TryInto;

/// The internal structure of an Ethernet frame is specified in IEEE 802.3
#[derive(Clone)]
pub struct EthHdr {
  pub src_hw_addr: [u8; 6],
  pub dst_hw_addr: [u8; 6],
  pub eth_type: Bits,
}

impl EthHdr {
  pub fn new() -> Self {
    Self {
      src_hw_addr: [0; 6],
      dst_hw_addr: [0; 6],
      eth_type: Bits::from(0, 16),
    }
  }

  pub fn from<T: ToString>(src_addr: T, dst_addr: T, eth_type: u16) -> Result<Self, Error> {
    let src_addr = src_addr.to_string();
    let src_addr: Vec<&str> = src_addr.split(":").collect();
    let dst_addr = dst_addr.to_string();
    let dst_addr: Vec<&str> = dst_addr.split(":").collect();

    if src_addr.len() != 6 || dst_addr.len() != 6 {
      Err(Error::new(
        "Error in formatting address in Ethernet Header",
        ErrorType::ConstructError,
      ))
    } else {
      let mut src_hw_addr: [u8; 6] = [0; 6];
      let mut dst_hw_addr: [u8; 6] = [0; 6];
      for i in 0..6 {
        src_hw_addr[i] = u8::from_str_radix(src_addr[i], 16)
          .unwrap_or_else(|err| panic!("{:?}", Error::new(err.to_string(), ErrorType::ParseError)));
        dst_hw_addr[i] = u8::from_str_radix(dst_addr[i], 16)
          .unwrap_or_else(|err| panic!("{:?}", Error::new(err.to_string(), ErrorType::ParseError)));
      }

      Ok(Self {
        src_hw_addr: src_hw_addr,
        dst_hw_addr: dst_hw_addr,
        eth_type: Bits::from(eth_type.into(), 16),
      })
    }
  }

  pub fn from_raw(src_addr: [u8; 6], dst_addr: [u8; 6], eth_type: u16) -> Self {
    Self {
      src_hw_addr: src_addr,
      dst_hw_addr: dst_addr,
      eth_type: Bits::from(eth_type.into(), 16),
    }
  }

  fn length(&self) -> usize {
    60
  }

  pub fn encapsulate(&self, data: impl Hdr) -> Result<Vec<u8>, Error> {
    let mut encapsulated: Vec<u8> = self.create()?.into();
    let mut data = data.create()?.into();

    encapsulated.append(&mut data);

    if encapsulated.len() > self.length() {
      return Err(Error::new(
        "Too much data in Packet",
        ErrorType::ConstructError,
      ));
    }

    Ok(encapsulated)
  }

  pub fn get_data_type(&self) -> EthType {
    from_ethtype(u16::from_str_radix(&self.eth_type.to_bits(), 2).unwrap())
  }
}

impl Hdr for EthHdr {
  fn create(&self) -> Result<Packet, Error> {
    let mut packet_data: Packet = Packet::new();
    for i in 0..6 {
      packet_data.push(self.dst_hw_addr[i]);
    }
    for i in 0..6 {
      packet_data.push(self.src_hw_addr[i]);
    }
    packet_data.append(self.eth_type.clone().into());
    Ok(packet_data.into())
  }

  fn parse(bytes: Packet) -> Self {
    let src_hw_addr: [u8; 6] = bytes.get_slice(48, 96).try_into().unwrap();
    let dst_hw_addr: [u8; 6] = bytes.get_slice(0, 48).try_into().unwrap();
    Self {
      src_hw_addr: src_hw_addr,
      dst_hw_addr: dst_hw_addr,
      eth_type: bytes.get_bin_slice(96, 112).into(),
    }
  }

  fn get(&self) -> Proto {
    Proto::Eth(self.clone())
  }
}

impl PartialEq for EthHdr {
  fn eq(&self, other: &Self) -> bool {
    if self.src_hw_addr == other.src_hw_addr
      || self.dst_hw_addr == other.dst_hw_addr
      || self.eth_type == other.eth_type
    {
      true
    } else {
      false
    }
  }
}

impl std::fmt::Debug for EthHdr {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    f.write_str(
      format!(
        "Ethernet type: {},
Source Hardware Address: {},
Destination Hardware Address: {}",
        mac_to_string(&self.src_hw_addr),
        mac_to_string(&self.dst_hw_addr),
        self.eth_type,
      )
      .as_str(),
    )
  }
}

impl std::fmt::Display for EthHdr {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let packet_vec: Vec<u8> = self.create().unwrap().into();
    f.write_str(format!("{:?}", packet_vec).as_str())
  }
}
