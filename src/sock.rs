use crate::error::*;
use pcap_file::PcapWriter;
use pnet_datalink::{
  channel, interfaces, Channel as PChannel, Config, DataLinkReceiver, DataLinkSender,
  NetworkInterface,
};
use std::{fs::File, time::Instant};

pub struct Channel {
  rx: Box<dyn DataLinkReceiver>,
  tx: Box<dyn DataLinkSender>,
  interf: NetworkInterface,
}

impl Channel {
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
        PChannel::Ethernet(tx, rx) => {
          return Ok(Channel {
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
      PChannel::Ethernet(tx, rx) => {
        return Ok(Channel {
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

  pub fn capture_to_pcap(
    &mut self,
    pcap_path: impl ToString,
    length: Option<usize>,
  ) -> Result<usize, Error> {
    let pcap_path = pcap_path.to_string();
    let pcap = File::create(pcap_path);
    match pcap {
      Err(e) => Err(Error::new(e.to_string(), ErrorType::PcapFileError)),
      Ok(p) => {
        let mut total_bytes_written = 0;
        let mut pcap_writter = PcapWriter::new(p).expect("Error writing file");
        match length {
          Some(l) => {
            let time_start = Instant::now();

            for _ in 0..l {
              println!("{:?}", time_start.elapsed().as_millis());
              let raw_packet = self.recv();

              pcap_writter
                .write(
                  time_start.elapsed().as_secs() as u32,
                  time_start.elapsed().as_millis() as u32
                    % ((time_start.elapsed().as_secs() as u32) * 1000),
                  &raw_packet,
                  raw_packet.len() as u32,
                )
                .unwrap();
              total_bytes_written += raw_packet.len();
            }
            Ok(total_bytes_written)
          }
          None => {
            let raw_packet = self.recv();
            let time_start = Instant::now();

            loop {
              pcap_writter
                .write(
                  time_start.elapsed().as_secs() as u32,
                  time_start.elapsed().as_millis() as u32 % time_start.elapsed().as_secs() as u32,
                  &raw_packet,
                  raw_packet.len() as u32,
                )
                .unwrap();
              total_bytes_written += raw_packet.len();
            }
          }
        }
      }
    }
  }
}
