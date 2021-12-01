use crate::dstructs::Bits;
use crate::dstructs::Packet;
use crate::error::{Error, ErrorType};
use crate::hdr::Hdr;
use crate::proto::Proto;
use crate::utility::{ip_to_string, parse_ip};

pub mod ip_proto {
  pub const ICMP: u8 = 0x01;
  pub const TCP: u8 = 0x06;
  pub const UDP: u8 = 0x11;
}

/// IPv4 header according to [RFC 791](https://datatracker.ietf.org/doc/html/rfc791)
#[derive(Clone)]
pub struct IPv4Hdr {
  pub ver: Bits,
  pub ihl: Bits,
  pub tos: Bits,
  pub total_len: Bits,
  pub id: Bits,
  pub flags: Bits,
  pub frag_offset: Bits,
  pub ttl: Bits,
  pub proto: Bits,
  pub hdr_checksum: Bits,
  pub src_ip_addr: [u8; 4],
  pub dst_ip_addr: [u8; 4],
}

impl IPv4Hdr {
  pub fn new() -> Self {
    Self {
      ver: Bits::from(4, 4),
      ihl: Bits::from(5, 4),
      tos: Bits::from(0, 8),
      total_len: Bits::from(0, 16),
      id: Bits::from(0, 16),
      flags: Bits::from(0, 2),
      frag_offset: Bits::from(0, 14),
      ttl: Bits::from(64, 8),
      proto: Bits::from(0, 8),
      hdr_checksum: Bits::from(0, 16),
      src_ip_addr: [0; 4],
      dst_ip_addr: [0; 4],
    }
  }

  pub fn from(src_addr: impl ToString, dst_addr: impl ToString, proto: u8) -> Result<Self, Error> {
    Ok(Self {
      ver: Bits::from(4, 4),
      ihl: Bits::from(5, 4),
      tos: Bits::from(0, 8),
      total_len: Bits::from(0, 16),
      id: Bits::from(0, 16),
      flags: Bits::from(0, 2),
      frag_offset: Bits::from(0, 14),
      ttl: Bits::from(64, 8),
      proto: Bits::from(proto.into(), 8),
      hdr_checksum: Bits::from(0, 16),
      src_ip_addr: parse_ip(src_addr.to_string())?,
      dst_ip_addr: parse_ip(dst_addr.to_string())?,
    })
  }
  pub fn encapsulate(&self, data: impl Hdr) -> Result<Vec<u8>, Error> {
    let mut encapsulated: Vec<u8> = self.create()?.into();
    let mut data: Vec<u8> = data.create()?.into();

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

  pub fn length(&self) -> usize {
    65535
  }
}

impl Hdr for IPv4Hdr {
  fn create(&self) -> Result<Packet, Error> {
    let mut packet_data = Packet::new();
    packet_data.append(self.ver.clone().into());
    packet_data.append(self.ihl.clone().into());
    packet_data.append(self.tos.clone().into());
    packet_data.append(self.total_len.clone().into());
    packet_data.append(self.id.clone().into());
    packet_data.append(self.flags.clone().into());
    packet_data.append(self.frag_offset.clone().into());
    packet_data.append(self.ttl.clone().into());
    packet_data.append(self.proto.clone().into());
    packet_data.append(self.hdr_checksum.clone().into());

    for i in 0..4 {
      packet_data.push(self.src_ip_addr[i]);
    }

    for i in 0..4 {
      packet_data.push(self.dst_ip_addr[i]);
    }

    Ok(packet_data.into())
  }

  fn parse(bytes: Packet) -> Self {
    let src_ip_vec = bytes.get_slice(96, 128);
    let dst_ip_vec = bytes.get_slice(128, 160);

    let mut src_ip = [0; 4];
    let mut dst_ip = [0; 4];
    for i in 0..4 {
      src_ip[i] = src_ip_vec[i];
      dst_ip[i] = dst_ip_vec[i];
    }

    Self {
      ver: bytes.get_bin_slice(0, 4).into(),
      ihl: bytes.get_bin_slice(4, 8).into(),
      tos: bytes.get_bin_slice(8, 14).into(),
      total_len: bytes.get_bin_slice(14, 16).into(),
      id: bytes.get_bin_slice(16, 32).into(),
      flags: bytes.get_bin_slice(32, 48).into(),
      frag_offset: bytes.get_bin_slice(48, 64).into(),
      ttl: bytes.get_bin_slice(64, 72).into(),
      proto: bytes.get_bin_slice(72, 80).into(),
      hdr_checksum: bytes.get_bin_slice(80, 96).into(),
      src_ip_addr: src_ip,
      dst_ip_addr: dst_ip,
    }
  }

  fn get(&self) -> Proto {
    Proto::IPv4(self.clone())
  }
}

impl PartialEq for IPv4Hdr {
  fn eq(&self, other: &Self) -> bool {
    if self.ver == other.ver
      || self.ihl == other.ihl
      || self.tos == other.tos
      || self.total_len == other.total_len
      || self.id == other.id
      || self.flags == other.flags
      || self.frag_offset == other.frag_offset
      || self.ttl == other.ttl
      || self.proto == other.proto
      || self.hdr_checksum == other.hdr_checksum
      || self.src_ip_addr == other.src_ip_addr
      || self.dst_ip_addr == other.dst_ip_addr
    {
      true
    } else {
      false
    }
  }
}

impl std::fmt::Debug for IPv4Hdr {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    f.write_str(
      format!(
        "
Version: {}
Internet Header Length: {}
TOS: {}
Total Length: {}
ID: {}
Flags: {}
Frag Offset: {}
TTL: {}
Protocol: {}
Header Checksum: {}
Source IP Address: {}
Destination IP Address: {}",
        self.ver,
        self.ihl,
        self.tos,
        self.total_len,
        self.id,
        self.flags,
        self.frag_offset,
        self.ttl,
        self.proto,
        self.hdr_checksum,
        ip_to_string(&self.src_ip_addr),
        ip_to_string(&self.dst_ip_addr),
      )
      .as_str(),
    )
  }
}

impl std::fmt::Display for IPv4Hdr {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let packet_vec: Vec<u8> = self.create().unwrap().into();
    f.write_str(format!("{:?}", packet_vec).as_str())
  }
}
