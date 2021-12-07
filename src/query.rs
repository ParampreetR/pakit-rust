use crate::hdr::{ArpQuery, EthQuery, IPv4Query};
use crate::Pdu;
use std::collections::HashMap;

#[derive(Clone, Hash, PartialEq, Eq)]
pub enum QueryHdr {
    IPv4(IPv4Query),
    Eth(EthQuery),
    Arp(ArpQuery),
}

pub struct Rules {
    pub rules: HashMap<QueryHdr, fn(Pdu) -> Pdu>,
}

impl Rules {
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
        }
    }

    pub fn add_rule(&mut self, query: QueryHdr, callback: fn(Pdu) -> Pdu) {
        self.rules.insert(query, callback);
    }
}
