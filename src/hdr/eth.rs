use super::traits::Hdr;
use crate::error::{Error, ErrorType};
use crate::proto::{EthType, Proto};
use crate::utility::from_ethtype;

#[derive(Debug, Clone)]
pub struct EthHdr {
  pub src_hw_addr: [u8; 6],
  pub dst_hw_addr: [u8; 6],
  pub eth_type: u16,
}

impl EthHdr {
  pub fn new() -> Self {
    Self {
      src_hw_addr: [0; 6],
      dst_hw_addr: [0; 6],
      eth_type: 0,
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
        eth_type: eth_type,
      })
    }
  }

  pub fn from_raw(src_addr: [u8; 6], dst_addr: [u8; 6], eth_type: u16) -> Self {
    Self {
      src_hw_addr: src_addr,
      dst_hw_addr: dst_addr,
      eth_type: eth_type,
    }
  }

  fn length(&self) -> usize {
    60
  }

  pub fn encapsulate(&self, data: impl Hdr) -> Result<Vec<u8>, Error> {
    let mut encapsulated: Vec<u8> = self.create()?;
    let mut data = data.create()?;

    encapsulated.append(&mut data);

    if encapsulated.len() > self.length() {
      return Err(Error::new(
        "Too much data in Packet",
        ErrorType::ConstructError,
      ));
    }

    while encapsulated.len() < self.length() {
      encapsulated.push(0);
    }
    Ok(encapsulated)
  }

  pub fn get_data_type(&self) -> EthType {
    from_ethtype(self.eth_type)
  }
}

impl Hdr for EthHdr {
  fn create(&self) -> Result<Vec<u8>, Error> {
    let mut data: Vec<u8> = Vec::with_capacity(14);
    for i in 0..6 {
      data.push(self.dst_hw_addr[i]);
    }
    for i in 0..6 {
      data.push(self.src_hw_addr[i]);
    }
    data.push(((self.eth_type & 0xff00) >> 8) as u8);
    data.push((self.eth_type & 0x00ff) as u8);
    Ok(data)
  }

  fn parse(bytes: &[u8]) -> Self {
    let mut src_hw_addr = [0; 6];
    let mut dst_hw_addr = [0; 6];
    for i in 0..6 {
      src_hw_addr[i] = bytes[i + 6];
      dst_hw_addr[i] = bytes[i];
    }
    let eth_type = ((bytes[12] as u16) << 8) + bytes[13] as u16;
    Self {
      src_hw_addr: src_hw_addr,
      dst_hw_addr: dst_hw_addr,
      eth_type: eth_type,
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
