use crate::error::{ErrorType, PaError};
use crate::hdr::*;

#[derive(Debug)]
pub enum Proto {
    Arp(ArpHdr),
    Eth(EthHdr),
    IPv4(IPv4Hdr),
    ICMP,
    Unknown,
}

pub enum EthType {
    Arp,
    IPv4,
    Unknown,
}

impl Proto {
    pub fn unwrap_arp(self) -> Result<ArpHdr, PaError> {
        if let Proto::Arp(hdr) = self {
            Ok(hdr)
        } else {
            Err(PaError::new(
                "Unable to unwrap Arp Header. Enum may be of any other type.",
                ErrorType::UnwrapHeaderError,
            ))
        }
    }
}
