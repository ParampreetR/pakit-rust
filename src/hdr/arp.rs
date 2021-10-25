use super::traits::Hdr;
use crate::error::*;
use crate::proto::Proto;
use crate::utility::*;

pub const REQ: u16 = 1;
pub const REP: u16 = 2;

#[derive(Clone)]
pub struct ArpHdr {
  pub hw_type: u16,
  pub proto_type: u16,
  pub hw_addr_len: u8,
  pub proto_addr_len: u8,
  pub opr: u16,
  pub src_hw_addr: [u8; 6],
  pub src_proto_addr: [u8; 4],
  pub dst_hw_addr: [u8; 6],
  pub dst_proto_addr: [u8; 4],
}

impl ArpHdr {
  pub fn new() -> Self {
    Self {
      hw_type: 0,
      proto_type: 0,
      hw_addr_len: 0,
      proto_addr_len: 0,
      opr: 0,
      src_hw_addr: [0; 6],
      src_proto_addr: [0; 4],
      dst_hw_addr: [0; 6],
      dst_proto_addr: [0; 4],
    }
  }

  pub fn from<T: ToString, U: ToString>(
    sender_mac: T,
    sender_ip: U,
    receiver_mac: T,
    receiver_ip: U,
  ) -> Result<Self, Error> {
    let src_ip = parse_ip(sender_ip)?;
    let dst_ip = parse_ip(receiver_ip)?;
    let src_mac = parse_mac(sender_mac)?;
    let dst_mac = parse_mac(receiver_mac)?;

    Ok(Self {
      hw_type: 1,
      proto_type: 0x0800,
      hw_addr_len: 6,
      proto_addr_len: 4,
      opr: REQ,
      src_hw_addr: src_mac,
      src_proto_addr: src_ip,
      dst_hw_addr: dst_mac,
      dst_proto_addr: dst_ip,
    })
  }

  pub fn set_arp_reply(&mut self) {
    self.opr = REP;
  }
}

impl Hdr for ArpHdr {
  fn parse(bytes: &[u8]) -> Self {
    let mut src_hw_addr = [0; 6];
    let mut dst_hw_addr = [0; 6];
    let mut src_proto_addr = [0; 4];
    let mut dst_proto_addr = [0; 4];
    for i in 0..6 {
      src_hw_addr[i] = bytes[i + 8];
      dst_hw_addr[i] = bytes[i + 18];
    }
    for i in 0..4 {
      src_proto_addr[i] = bytes[i + 14];
      dst_proto_addr[i] = bytes[i + 24];
    }

    Self {
      hw_type: ((bytes[0] as u16) << 8) + bytes[1] as u16,
      proto_type: ((bytes[2] as u16) << 8) + bytes[3] as u16,
      hw_addr_len: bytes[4],
      proto_addr_len: bytes[5],
      opr: ((bytes[6] as u16) << 8) + bytes[7] as u16,
      src_hw_addr: src_hw_addr,
      src_proto_addr: src_proto_addr,
      dst_hw_addr: dst_hw_addr,
      dst_proto_addr: dst_proto_addr,
    }
  }
  fn create(&self) -> Result<Vec<u8>, Error> {
    let mut packet_data: Vec<u8> = Vec::with_capacity(28);
    packet_data.push(((self.hw_type & 0xff00) >> 8) as u8);
    packet_data.push((self.hw_type & 0x00ff) as u8);
    packet_data.push(((self.proto_type & 0xff00) >> 8) as u8);
    packet_data.push((self.proto_type & 0x00ff) as u8);
    packet_data.push(self.hw_addr_len);
    packet_data.push(self.proto_addr_len);
    packet_data.push(((self.opr & 0xff00) >> 8) as u8);
    packet_data.push((self.opr & 0x00ff) as u8);
    for i in 0..6 {
      packet_data.push(self.src_hw_addr[i]);
    }

    for i in 0..4 {
      packet_data.push(self.src_proto_addr[i]);
    }

    for i in 0..6 {
      packet_data.push(self.dst_hw_addr[i]);
    }

    for i in 0..4 {
      packet_data.push(self.dst_proto_addr[i]);
    }

    Ok(packet_data)
  }
  fn get(&self) -> Proto {
    Proto::Arp(self.clone())
  }
}

impl PartialEq for ArpHdr {
  fn eq(&self, other: &Self) -> bool {
    if self.hw_type == other.hw_type
      || self.proto_type == other.proto_type
      || self.hw_addr_len == other.hw_addr_len
      || self.proto_addr_len == other.proto_addr_len
      || self.opr == other.opr
      || self.src_hw_addr == other.src_hw_addr
      || self.src_proto_addr == other.src_proto_addr
      || self.dst_hw_addr == other.dst_hw_addr
      || self.dst_proto_addr == other.dst_proto_addr
    {
      true
    } else {
      false
    }
  }
}

impl std::fmt::Debug for ArpHdr {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    f.write_str(
      format!(
        "Hardware type: {},
Protocol type: {},
Hardware Address Length: {},
Protocol Address Length: {},
Operation: {},
Source Hardware Address: {},
Source Protocol Address: {},
Destination Hardware Address: {},
Destination Protocol Address: {}",
        self.hw_type,
        self.proto_type,
        self.hw_addr_len,
        self.proto_addr_len,
        self.opr,
        mac_to_string(&self.src_hw_addr),
        ip_to_string(&self.src_proto_addr),
        mac_to_string(&self.dst_hw_addr),
        ip_to_string(&self.dst_proto_addr),
      )
      .as_str(),
    )
  }
}

impl std::fmt::Display for ArpHdr {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    f.write_str(format!("{:?}", self.create().unwrap()))
  }
}
