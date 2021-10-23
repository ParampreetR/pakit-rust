use crate::error::{Error, ErrorType};
use crate::hdr::*;
use crate::proto::{EthType, Proto};
use crate::sock::MyChannel;
use std::collections::HashMap;

pub struct Packet {
  pub headers: HashMap<u8, Proto>,
  pub buffer: Vec<u8>,
}

impl Packet {
  pub fn new() -> Self {
    Self {
      headers: HashMap::with_capacity(7),
      buffer: Vec::new(),
    }
  }

  pub fn parse(bits: &[u8]) -> Self {
    let mut pack = Self::new();
    let eth_hdr = EthHdr::parse(&bits[0..14]);
    pack.headers.insert(2, Proto::Eth(eth_hdr.clone()));
    match eth_hdr.get_data_type() {
      EthType::Arp => {
        let arp_hdr = ArpHdr::parse(&bits[14..]);
        pack.headers.insert(3, Proto::Arp(arp_hdr));
      }
      EthType::IP => {}
      EthType::Unknown => {}
    };

    pack
  }

  pub fn header(mut self, hdr: impl Hdr) -> Self {
    match hdr.get() {
      Proto::Arp(arp_hdr) => self.headers.insert(3, Proto::Arp(arp_hdr)),
      Proto::Eth(eth_hdr) => self.headers.insert(2, Proto::Eth(eth_hdr)),
      _ => None,
    };

    self
  }

  pub fn set_header(&mut self, hdr: impl Hdr) {
    match hdr.get() {
      Proto::Arp(arp_hdr) => self.headers.insert(3, Proto::Arp(arp_hdr)),
      Proto::Eth(eth_hdr) => self.headers.insert(2, Proto::Eth(eth_hdr)),
      _ => None,
    };
  }

  pub fn build_packet(&mut self) {
    match self.headers.get(&3) {
      Some(hdr) => match hdr {
        Proto::Arp(arp) => match self.headers.get(&2) {
          Some(hdr2) => {
            match hdr2 {
              Proto::Eth(eth) => {
                self.buffer = eth.encapsulate(arp.clone()).unwrap();
              }
              _ => {
                self.buffer = vec![];
              }
            }
            println!("{:?}", self.buffer)
          }
          _ => {}
        },
        _ => {}
      },
      _ => {}
    }
  }

  pub fn send_and_recv(&self) -> Vec<u8> {
    let mut c = MyChannel::from("wlp0s19f2u3").unwrap();
    c.send_packet(&self.buffer);
    let bits = c.recv();
    println!("{:?}", bits);
    bits
  }

  // pub fn recv(&self) {
  //   let mut packet =
  // }
}

use std::convert::TryInto;

// impl TryInto<Vec<u8>> for Packet {
//   type Error = Error;
// }
