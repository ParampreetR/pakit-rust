use crate::dstructs::Packet;
use crate::error::PaError;
use crate::hdr::*;
use crate::proto::{EthType, Proto};
use crate::sock::Channel;
use std::collections::HashMap;

pub struct Pdu {
    pub headers: HashMap<u8, Proto>,
    pub buffer: Vec<u8>,
}

impl Pdu {
    pub fn new() -> Self {
        Self {
            headers: HashMap::with_capacity(6),
            buffer: Vec::new(),
        }
    }

    pub fn parse(bits: &[u8]) -> Self {
        let mut pack = Self::new();
        let eth_hdr = EthHdr::parse((&bits[0..14]).into());
        pack.headers.insert(2, Proto::Eth(eth_hdr.clone()));
        match eth_hdr.get_data_type() {
            EthType::Arp => {
                let arp_hdr = ArpHdr::parse((&bits[14..]).into());
                pack.headers.insert(3, Proto::Arp(arp_hdr));
            }
            EthType::IPv4 => {
                let ipv4_hdr = IPv4Hdr::parse((&bits[14..]).into());
                pack.headers.insert(3, Proto::IPv4(ipv4_hdr));
            }
            EthType::Unknown => {}
        };

        pack
    }

    pub fn header(mut self, hdr: impl Hdr) -> Self {
        match hdr.get() {
            Proto::Arp(arp_hdr) => self.headers.insert(3, Proto::Arp(arp_hdr)),
            Proto::Eth(eth_hdr) => self.headers.insert(2, Proto::Eth(eth_hdr)),
            Proto::IPv4(ipv4_hdr) => self.headers.insert(3, Proto::IPv4(ipv4_hdr)),
            _ => None,
        };

        self
    }

    pub fn set_header(&mut self, hdr: impl Hdr) {
        match hdr.get() {
            Proto::Arp(arp_hdr) => self.headers.insert(3, Proto::Arp(arp_hdr)),
            Proto::Eth(eth_hdr) => self.headers.insert(2, Proto::Eth(eth_hdr)),
            Proto::IPv4(ipv4_hdr) => self.headers.insert(3, Proto::IPv4(ipv4_hdr)),
            _ => None,
        };
    }

    pub fn build(&mut self) -> Result<(), PaError> {
        match self.headers.get(&3) {
            Some(hdr) => match hdr {
                Proto::Arp(arp) => match self.headers.get(&2) {
                    Some(hdr2) => match hdr2 {
                        Proto::Eth(eth) => {
                            self.buffer = eth.encapsulate(arp.clone())?;
                        }
                        _ => {
                            self.buffer = vec![];
                        }
                    },
                    _ => {}
                },
                Proto::IPv4(ipv4) => match self.headers.get(&2) {
                    Some(hdr2) => match hdr2 {
                        Proto::Eth(eth) => {
                            self.buffer = eth.encapsulate(ipv4.clone())?;
                        }
                        _ => {
                            self.buffer = vec![];
                        }
                    },
                    _ => {} /* Currently in Development */
                },
                _ => {}
            },
            _ => {}
        };
        Ok(())
    }

    pub fn send_and_recv(&self, interface_name: Option<String>) -> Result<Packet, PaError> {
        let mut c;
        if interface_name.is_none() {
            c = Channel::new()?;
        } else {
            c = Channel::from(interface_name.unwrap())?;
        }
        //TODO: Check if received raw data is really a response of your send data.
        c.send_packet(&self.buffer);
        let bits = c.recv();
        println!("{:?}", bits);
        Ok(bits.into())
    }

    pub fn send(&self, interface_name: Option<String>) -> Result<usize, PaError> {
        let mut c;
        if interface_name.is_none() {
            c = Channel::new()?;
        } else {
            c = Channel::from(interface_name.unwrap())?;
        }
        c.send_packet(&self.buffer);
        Ok(self.buffer.len())
    }
}
