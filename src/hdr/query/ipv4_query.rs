use crate::dstructs::Bits;
use crate::hdr::IPv4Hdr;
use crate::utility::*;
use crate::Error;

macro_rules! ifeq {
  ($lhs:expr, $rhs:expr) => {
    if $lhs.is_some() {
      if $lhs.unwrap() != $rhs {
        return false;
      }
    }
  };
}

/// Used as query of finding particular IPv4 Data data
///
/// Members of structs are `Option<_>`
/// * `None` - will match ANY data.
/// * `Some(qdata)` - will match only to data similar to qdata.
#[derive(Clone)]
pub struct IPv4Query {
  pub ver: Option<Bits>,
  pub ihl: Option<Bits>,
  pub tos: Option<Bits>,
  pub total_len: Option<Bits>,
  pub id: Option<Bits>,
  pub flags: Option<Bits>,
  pub frag_offset: Option<Bits>,
  pub ttl: Option<Bits>,
  pub proto: Option<Bits>,
  pub hdr_checksum: Option<Bits>,
  pub src_ip_addr: Option<[u8; 4]>,
  pub dst_ip_addr: Option<[u8; 4]>,
}

impl PartialEq<IPv4Hdr> for IPv4Query {
  fn eq(&self, rhs: &IPv4Hdr) -> bool {
    ifeq!(self.ver.clone(), rhs.ver);
    ifeq!(self.ihl.clone(), rhs.ihl);
    ifeq!(self.tos.clone(), rhs.tos);
    ifeq!(self.total_len.clone(), rhs.total_len);
    ifeq!(self.id.clone(), rhs.id);
    ifeq!(self.flags.clone(), rhs.flags);
    ifeq!(self.frag_offset.clone(), rhs.frag_offset);
    ifeq!(self.ttl.clone(), rhs.ttl);
    ifeq!(self.proto.clone(), rhs.proto);
    ifeq!(self.hdr_checksum.clone(), rhs.hdr_checksum);
    ifeq!(self.src_ip_addr.clone(), rhs.src_ip_addr);
    ifeq!(self.dst_ip_addr.clone(), rhs.dst_ip_addr);

    return true;
  }
}

impl IPv4Query {
  pub fn new() -> Self {
    Self {
      ver: None,
      ihl: None,
      tos: None,
      total_len: None,
      id: None,
      flags: None,
      frag_offset: None,
      ttl: None,
      proto: None,
      hdr_checksum: None,
      src_ip_addr: None,
      dst_ip_addr: None,
    }
  }

  pub fn from(
    src_addr: Option<impl ToString>,
    dst_addr: Option<impl ToString>,
    proto: Option<u8>,
  ) -> Result<Self, Error> {
    let mut src_ip = None;
    let mut dst_ip = None;
    let mut pro = None;
    if src_addr.is_some() {
      src_ip = Some(parse_ip(src_addr.unwrap())?)
    }

    if dst_addr.is_some() {
      dst_ip = Some(parse_ip(dst_addr.unwrap())?)
    }

    if proto.is_some() {
      pro = Some(Bits::from(proto.unwrap().into(), 8))
    }

    Ok(Self {
      ver: Some(Bits::from(4, 4)),
      ihl: Some(Bits::from(5, 4)),
      tos: Some(Bits::from(0, 8)),
      total_len: Some(Bits::from(0, 16)),
      id: Some(Bits::from(0, 16)),
      flags: Some(Bits::from(0, 2)),
      frag_offset: Some(Bits::from(0, 14)),
      ttl: Some(Bits::from(64, 8)),
      proto: pro,
      hdr_checksum: Some(Bits::from(0, 16)),
      src_ip_addr: src_ip,
      dst_ip_addr: dst_ip,
    })
  }
}
