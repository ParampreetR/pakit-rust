use crate::error::*;
use pnet_datalink::{
  channel, interfaces, Channel, Config, DataLinkReceiver, DataLinkSender, NetworkInterface,
};

pub struct MyChannel {
  rx: Box<dyn DataLinkReceiver>,
  tx: Box<dyn DataLinkSender>,
  interf: NetworkInterface,
}

impl MyChannel {
  pub fn send_packet(&mut self, packet: &[u8]) {
    self.tx.send_to(packet, Some(self.interf.clone()));
  }

  pub fn new() -> Result<Self, Error> {
    let interfaces_list = interfaces();
    let default_interface: Option<&NetworkInterface> = interfaces_list
      .iter()
      .find(|e| e.is_up() && !e.is_loopback() && !e.ips.is_empty());

    if let Some(default_interface) = default_interface {
      let my_channel = channel(default_interface, Config::default()).unwrap();
      match my_channel {
        Channel::Ethernet(tx, rx) => {
          return Ok(MyChannel {
            tx: tx,
            rx: rx,
            interf: default_interface.clone(),
          })
        }
        _ => return Err(Error::new("Unknown Channel", ErrorType::ChannelError)),
      }
    } else {
      return Err(Error::new(
        "Error in getting default interface",
        ErrorType::ChannelError,
      ));
    }
  }

  pub fn get_interface_list() -> Vec<NetworkInterface> {
    interfaces()
  }

  pub fn from(interface: impl ToString) -> Result<Self, Error> {
    let mut interf_selected: Option<&NetworkInterface> = None;
    let interf_list: Vec<NetworkInterface> = Self::get_interface_list();
    for interf in interf_list.iter() {
      if interf.name == interface.to_string() {
        interf_selected = Some(interf);
      }
    }
    if interf_selected.is_none() {
      return Err(Error::new("Interface not found", ErrorType::InterfaceError));
    }

    let interf_selected = interf_selected.unwrap();

    let my_channel = channel(&interf_selected, Config::default()).unwrap();
    match my_channel {
      Channel::Ethernet(tx, rx) => {
        return Ok(MyChannel {
          tx: tx,
          rx: rx,
          interf: interf_selected.clone(),
        })
      }
      _ => return Err(Error::new("Unknown Channel", ErrorType::ChannelError)),
    }
  }

  pub fn recv(&mut self) -> Vec<u8> {
    let mut bits: Vec<u8> = Vec::new();
    if let Ok(byte) = self.rx.next() {
      println!("{:?}", byte);
      for bit in byte {
        bits.push(*bit);
      }
    }

    bits
  }
}
