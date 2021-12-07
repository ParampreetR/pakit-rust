use crate::dstructs::Bits;
use crate::error::PaError;
use crate::hdr::ArpHdr;
use crate::utility::*;
use crate::{debug, proto::Proto, Pdu};

macro_rules! ifeq {
    ($lhs:expr, $rhs:expr) => {
        if $lhs.is_some() {
            if $lhs.unwrap() != $rhs {
                debug!("{:?} != {:?} \t Unmatched", $lhs, $rhs);
                return false;
            }
        }
    };
}

/// Used as query of finding particular ARP data
///
/// Members of structs are `Option<_>`
/// * `None` - will match ANY data.
/// * `Some(data)` - will match only to data similar to data.
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct ArpQuery {
    pub hw_type: Option<Bits>,
    pub proto_type: Option<Bits>,
    pub hw_addr_len: Option<Bits>,
    pub proto_addr_len: Option<Bits>,
    pub opr: Option<Bits>,
    pub src_hw_addr: Option<[u8; 6]>,
    pub src_proto_addr: Option<[u8; 4]>,
    pub dst_hw_addr: Option<[u8; 6]>,
    pub dst_proto_addr: Option<[u8; 4]>,
}

impl PartialEq<ArpHdr> for ArpQuery {
    fn eq(&self, rhs: &ArpHdr) -> bool {
        ifeq!(self.hw_type.clone(), rhs.hw_type);
        ifeq!(self.proto_type.clone(), rhs.proto_type);
        ifeq!(self.hw_addr_len.clone(), rhs.hw_addr_len);
        ifeq!(self.proto_addr_len.clone(), rhs.proto_addr_len);
        ifeq!(self.opr.clone(), rhs.opr);
        ifeq!(self.src_hw_addr.clone(), rhs.src_hw_addr);
        ifeq!(self.src_proto_addr.clone(), rhs.src_proto_addr);
        ifeq!(self.dst_hw_addr.clone(), rhs.dst_hw_addr);
        ifeq!(self.dst_proto_addr.clone(), rhs.dst_proto_addr);

        return true;
    }
}

impl ArpQuery {
    pub fn new() -> Self {
        Self {
            hw_type: None,
            proto_type: None,
            hw_addr_len: None,
            proto_addr_len: None,
            opr: None,
            src_hw_addr: None,
            src_proto_addr: None,
            dst_hw_addr: None,
            dst_proto_addr: None,
        }
    }

    pub fn from(
        sender_mac: Option<&str>,
        sender_ip: Option<&str>,
        receiver_mac: Option<&str>,
        receiver_ip: Option<&str>,
    ) -> Result<Self, PaError> {
        let mut src_ip = None;
        let mut dst_ip = None;
        let mut src_mac = None;
        let mut dst_mac = None;

        if sender_ip.is_some() {
            src_ip = Some(parse_ip(sender_ip.unwrap()).unwrap());
        }

        if receiver_ip.is_some() {
            dst_ip = Some(parse_ip(receiver_ip.unwrap()).unwrap());
        }

        if sender_mac.is_some() {
            src_mac = Some(parse_mac(sender_mac.unwrap()).unwrap());
        }

        if receiver_mac.is_some() {
            dst_mac = Some(parse_mac(receiver_mac.unwrap()).unwrap())
        }

        Ok(Self {
            hw_type: Some(Bits::from(1, 16)),
            proto_type: Some(Bits::from(0x0800, 16)),
            hw_addr_len: Some(Bits::from(6, 8)),
            proto_addr_len: Some(Bits::from(4, 8)),
            opr: Some(Bits::from(1, 16)),
            src_hw_addr: src_mac,
            src_proto_addr: src_ip,
            dst_hw_addr: dst_mac,
            dst_proto_addr: dst_ip,
        })
    }

    pub fn set_arp_reply(&mut self) {
        self.opr = Some(Bits::from(2, 16));
    }
}

impl PartialEq<Pdu> for ArpQuery {
    fn eq(&self, other: &Pdu) -> bool {
        if other.headers.get(&3).is_none() {
            return false;
        }
        return if let Proto::Arp(hdr) = other.headers.get(&3).unwrap() {
            debug!("ARP headers found in PDU Group");
            if self == hdr {
                debug!("ARP headers matched with PDU Query");
                true
            } else {
                debug!("ARP headers not matched with PDU Query");
                false
            }
        } else {
            false
        };
    }
}
