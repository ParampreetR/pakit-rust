use super::traits::Hdr;
use crate::dstructs::{Bits, Packet};
use crate::error::*;
use crate::proto::Proto;
use crate::utility::*;
use std::convert::TryInto;

#[path = "query/arp_query.rs"]
mod arp_query;
pub use arp_query::*;

pub const REQ: u16 = 1;
pub const REP: u16 = 2;

#[derive(Clone)]
pub struct ArpHdr {
    pub hw_type: Bits,
    pub proto_type: Bits,
    pub hw_addr_len: Bits,
    pub proto_addr_len: Bits,
    pub opr: Bits,
    pub src_hw_addr: [u8; 6],
    pub src_proto_addr: [u8; 4],
    pub dst_hw_addr: [u8; 6],
    pub dst_proto_addr: [u8; 4],
}

impl ArpHdr {
    pub fn new() -> Self {
        Self {
            hw_type: Bits::from(1, 16),
            proto_type: Bits::from(0x0800, 16),
            hw_addr_len: Bits::from(6, 8),
            proto_addr_len: Bits::from(4, 8),
            opr: Bits::from(REQ.into(), 16),
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
    ) -> Result<Self, PaError> {
        let src_ip = parse_ip(sender_ip)?;
        let dst_ip = parse_ip(receiver_ip)?;
        let src_mac = parse_mac(sender_mac)?;
        let dst_mac = parse_mac(receiver_mac)?;

        Ok(Self {
            hw_type: Bits::from(1, 16),
            proto_type: Bits::from(0x0800, 16),
            hw_addr_len: Bits::from(6, 8),
            proto_addr_len: Bits::from(4, 8),
            opr: Bits::from(REQ.into(), 16),
            src_hw_addr: src_mac,
            src_proto_addr: src_ip,
            dst_hw_addr: dst_mac,
            dst_proto_addr: dst_ip,
        })
    }

    pub fn set_arp_reply(&mut self) {
        self.opr = Bits::from(REP.into(), 16);
    }
}

impl Hdr for ArpHdr {
    fn create(&self) -> Result<Packet, PaError> {
        let mut packet_data: Packet = Packet::new();
        packet_data.append(self.hw_type.clone().into());
        packet_data.append(self.proto_type.clone().into());
        packet_data.append(self.hw_addr_len.clone().into());
        packet_data.append(self.proto_addr_len.clone().into());
        packet_data.append(self.opr.clone().into());
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

        Ok(packet_data.into())
    }
    fn parse(bytes: Packet) -> Self {
        let src_hw_addr: [u8; 6] = bytes.get_slice(64, 112).try_into().unwrap();
        let dst_hw_addr: [u8; 6] = bytes.get_slice(144, 192).try_into().unwrap();
        let src_proto_addr: [u8; 4] = bytes.get_slice(112, 144).try_into().unwrap();
        let dst_proto_addr: [u8; 4] = bytes.get_slice(192, 224).try_into().unwrap();

        Self {
            hw_type: bytes.get_bin_slice(0, 16).into(),
            proto_type: bytes.get_bin_slice(16, 32).into(),
            hw_addr_len: bytes.get_bin_slice(32, 40).into(),
            proto_addr_len: bytes.get_bin_slice(40, 48).into(),
            opr: bytes.get_bin_slice(48, 64).into(),
            src_hw_addr,
            src_proto_addr,
            dst_hw_addr,
            dst_proto_addr,
        }
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
        let packet_vec: Vec<u8> = self.create().unwrap().into();
        f.write_str(format!("{:?}", packet_vec).as_str())
    }
}
