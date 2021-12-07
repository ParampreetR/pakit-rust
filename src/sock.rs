use crate::error::*;
use crate::{debug, QueryHdr};
use crate::{Pdu, Rules};

#[cfg(feature = "pcap")]
use pcap_file::PcapWriter;
#[cfg(feature = "pcap")]
use std::{fs::File, time::Instant};

use pnet_datalink::{
    channel, interfaces, Channel as PChannel, Config, DataLinkReceiver, DataLinkSender,
    NetworkInterface,
};

pub struct Channel {
    rx: Box<dyn DataLinkReceiver>,
    tx: Box<dyn DataLinkSender>,
    interf: NetworkInterface,
}

impl Channel {
    pub fn send_packet(&mut self, packet: &[u8]) {
        self.tx.send_to(packet, Some(self.interf.clone()));
    }

    pub fn new() -> Result<Self, PaError> {
        let interfaces_list = interfaces();
        let default_interface: Option<&NetworkInterface> = interfaces_list
            .iter()
            .find(|e| e.is_up() && !e.is_loopback() && !e.ips.is_empty());

        return if let Some(default_interface) = default_interface {
            let my_channel = channel(default_interface, Config::default())?;
            match my_channel {
                PChannel::Ethernet(tx, rx) => Ok(Channel {
                    tx,
                    rx,
                    interf: default_interface.clone(),
                }),
                _ => Err(PaError::new("Unknown Channel", ErrorType::ChannelError)),
            }
        } else {
            Err(PaError::new(
                "Error in getting default interface",
                ErrorType::ChannelError,
            ))
        };
    }

    pub fn get_interface_list() -> Vec<NetworkInterface> {
        interfaces()
    }

    pub fn from(interface: impl ToString) -> Result<Self, PaError> {
        let mut interf_selected: Option<&NetworkInterface> = None;
        let interf_list: Vec<NetworkInterface> = Self::get_interface_list();
        for interf in interf_list.iter() {
            if interf.name == interface.to_string() {
                interf_selected = Some(interf);
            }
        }
        if interf_selected.is_none() {
            return Err(PaError::new(
                "Interface not found",
                ErrorType::InterfaceError,
            ));
        }

        let interf_selected = interf_selected.unwrap();

        let my_channel = channel(&interf_selected, Config::default())?;
        return match my_channel {
            PChannel::Ethernet(tx, rx) => Ok(Channel {
                tx,
                rx,
                interf: interf_selected.clone(),
            }),
            _ => Err(PaError::new("Unknown Channel", ErrorType::ChannelError)),
        };
    }

    pub fn recv(&mut self) -> Vec<u8> {
        let mut bits: Vec<u8> = Vec::new();
        if let Ok(byte) = self.rx.next() {
            for bit in byte {
                bits.push(*bit);
            }
        }

        bits
    }

    #[cfg(feature = "pcap")]
    pub fn capture_to_pcap(
        &mut self,
        pcap_path: impl ToString,
        length: Option<usize>,
    ) -> Result<usize, PaError> {
        let pcap_path = pcap_path.to_string();
        let pcap = File::create(pcap_path);
        match pcap {
            Err(e) => Err(PaError::new(e.to_string(), ErrorType::PcapFileError)),
            Ok(p) => {
                let mut total_bytes_written = 0;
                let mut pcap_writer = PcapWriter::new(p).expect("Error writing file");
                match length {
                    Some(l) => {
                        let time_start = Instant::now();

                        for _ in 0..l {
                            let raw_packet = self.recv();

                            pcap_writer
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
                            //FIXME: Time written to pcap file is not accurate
                            pcap_writer
                                .write(
                                    time_start.elapsed().as_secs() as u32,
                                    time_start.elapsed().as_millis() as u32
                                        % time_start.elapsed().as_secs() as u32,
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

    pub fn auto_reply(&mut self, rules: Rules, limit: Option<usize>) {
        let mut total_send = 0;
        let mut reply: Pdu;
        loop {
            if limit.is_some() {
                if limit.unwrap() == total_send {
                    break;
                }
            }
            let mut matched: bool = false;
            let recvd = self.recv();
            let pdu = Pdu::parse(&recvd);
            for (key, value) in &rules.rules {
                match key {
                    QueryHdr::Arp(query) => {
                        if query == &pdu {
                            matched = true;
                            reply = value(pdu);
                            reply.build();
                            self.send_packet(&reply.buffer);
                            debug!("Send crafted response for ARP Query");
                            break;
                        }
                    }
                    QueryHdr::Eth(query) => {
                        debug!("Got Eth, matching with query");
                        if query == &pdu {
                            debug!("Eth matched with Eth Query");
                            matched = true;
                            reply = value(pdu);
                            reply.build();
                            self.send_packet(&reply.buffer);
                            debug!("Send crafted response for Eth Query");
                            break;
                        }
                    }
                    QueryHdr::IPv4(query) => {
                        debug!("Got IPv4, matching with query");
                        if query == &pdu {
                            debug!("Eth matched with IPv4 Query");
                            matched = true;
                            reply = value(pdu);
                            reply.build();
                            self.send_packet(&reply.buffer);
                            debug!("Send crafted response for IPv4 Query");
                            break;
                        }
                    }
                }
            }

            if matched {
                total_send += 1;
            }
        }
    }
}
