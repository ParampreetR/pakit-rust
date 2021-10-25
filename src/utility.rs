//! Contains utility functions
//! often called helper functions

use crate::error::{Error, ErrorType};
use crate::proto::EthType;

/// Parses IP Address in string to array of bytes of length 4
pub fn parse_ip<T: ToString>(ip_addr: T) -> Result<[u8; 4], Error> {
  let ip_addr = ip_addr.to_string();
  let ip_addr: Vec<&str> = ip_addr.split(".").collect();

  if ip_addr.len() != 4 {
    Err(Error::new("", ErrorType::ParseError))
  } else {
    let mut parsed_ip_addr: [u8; 4] = [0; 4];
    for i in 0..4 {
      /* TODO: Return Error here if parsing fail but
      present implementation will panic on error */
      parsed_ip_addr[i] = ip_addr[i].trim().parse().unwrap();
    }
    Ok(parsed_ip_addr)
  }
}

/// Parses IP Address in string to array of bytes of length 6
pub fn parse_mac<T: ToString>(mac_addr: T) -> Result<[u8; 6], Error> {
  let mac_addr = mac_addr.to_string();
  let mac_addr: Vec<&str> = mac_addr.split(":").collect();

  if mac_addr.len() != 6 {
    Err(Error::new("", ErrorType::ParseError))
  } else {
    let mut parsed_mac_addr: [u8; 6] = [0; 6];
    for i in 0..6 {
      /* TODO: Return Error here if parsing fail but
      present implementation will panic on error */
      parsed_mac_addr[i] = u8::from_str_radix(mac_addr[i], 16).unwrap();
    }
    Ok(parsed_mac_addr)
  }
}

/// Parses IP address in array of bytes to String eg "192.168.1.1"
pub fn ip_to_string(ip_addr: &[u8]) -> String {
  format!(
    "{}.{}.{}.{}",
    ip_addr[0], ip_addr[1], ip_addr[2], ip_addr[3]
  )
}

/// Parses MAC address in array of bytes to String eg "aa:aa:aa:aa:aa"
pub fn mac_to_string(mac_addr: &[u8]) -> String {
  let mut string_mac_addr = String::new();
  for octet in mac_addr {
    string_mac_addr.push_str(format!("{:02x}:", octet).as_str());
  }
  string_mac_addr.trim_end_matches(":").to_string()
}

/// Get Ethernet type from 16 byte unsigned integer
pub fn from_ethtype(ethtype: u16) -> EthType {
  match ethtype {
    0x806 => EthType::Arp,
    _ => EthType::Unknown,
  }
}
