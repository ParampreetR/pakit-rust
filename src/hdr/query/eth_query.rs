use crate::dstructs::Bits;
use crate::hdr::EthHdr;
use crate::{Error, ErrorType};

macro_rules! ifeq {
  ($lhs:expr, $rhs:expr) => {
    if $lhs.is_some() {
      if $lhs.unwrap() != $rhs {
        return false;
      }
    }
  };
}

/// Used as query of finding particular Ethernet data
///
/// Members of structs are `Option<_>`
/// * `None` - will match ANY data.
/// * `Some(qdata)` - will match only to data similar to qdata.
#[derive(Clone)]
pub struct EthQuery {
  pub src_hw_addr: Option<[u8; 6]>,
  pub dst_hw_addr: Option<[u8; 6]>,
  pub eth_type: Option<Bits>,
}

impl PartialEq<EthHdr> for EthQuery {
  fn eq(&self, rhs: &EthHdr) -> bool {
    ifeq!(self.src_hw_addr, rhs.src_hw_addr);
    ifeq!(self.dst_hw_addr, rhs.dst_hw_addr);
    ifeq!(self.eth_type.clone(), rhs.eth_type);

    return true;
  }
}

impl EthQuery {
  pub fn new() -> Self {
    Self {
      src_hw_addr: None,
      dst_hw_addr: None,
      eth_type: None,
    }
  }

  pub fn from<T: ToString>(
    src_addr: Option<T>,
    dst_addr: Option<T>,
    eth_type: Option<u16>,
  ) -> Result<Self, Error> {
    let mut src_hw_addr: Option<[u8; 6]> = None;
    let mut dst_hw_addr: Option<[u8; 6]> = None;
    let mut etype: Option<Bits> = None;

    if src_addr.is_some() {
      src_hw_addr = Some([0; 6]);
      let s_addr = src_addr.unwrap().to_string();
      let s_addr: Vec<&str> = s_addr.split(":").collect();

      if s_addr.len() != 6 {
        return Err(Error::new(
          "Error in formatting address in Ethernet Header",
          ErrorType::ConstructError,
        ));
      }
      for i in 0..6 {
        src_hw_addr.unwrap()[i] = u8::from_str_radix(s_addr[i], 16)
          .unwrap_or_else(|err| panic!("{:?}", Error::new(err.to_string(), ErrorType::ParseError)));
      }
    }

    if dst_addr.is_some() {
      dst_hw_addr = Some([0; 6]);
      let d_addr = dst_addr.unwrap().to_string();
      let d_addr: Vec<&str> = d_addr.split(":").collect();
      if d_addr.len() != 6 {
        return Err(Error::new(
          "Error in formatting address in Ethernet Header",
          ErrorType::ConstructError,
        ));
      }
      for i in 0..6 {
        dst_hw_addr.unwrap()[i] = u8::from_str_radix(d_addr[i], 16)
          .unwrap_or_else(|err| panic!("{:?}", Error::new(err.to_string(), ErrorType::ParseError)));
      }
    }

    if eth_type.is_some() {
      etype = Some(Bits::from(eth_type.unwrap().into(), 16));
    }

    return Ok(Self {
      src_hw_addr: src_hw_addr,
      dst_hw_addr: dst_hw_addr,
      eth_type: etype,
    });
  }
}
