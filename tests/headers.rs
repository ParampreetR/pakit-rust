#[test]
fn arp_create_parse() {
  use pakit::hdr::ArpHdr;
  use pakit::hdr::Hdr;
  let hdr = ArpHdr::from(
    "aa:aa:aa:aa:aa:aa",
    "192.168.1.100",
    "bb:bb:bb:bb:bb:bb",
    "192.168.1.101",
  )
  .unwrap();

  let raw_hdrs = hdr.create().unwrap();
  let hdr2 = ArpHdr::parse(&raw_hdrs);
  assert_eq!(hdr, hdr2);
}

#[test]
fn eth_create_parse() {
  use pakit::hdr::EthHdr;
  use pakit::hdr::Hdr;
  let hdr = EthHdr::from("aa:aa:aa:aa:aa:bb", "cc:cc:cc:cc:cc:dd", 1).unwrap();
  let raw_hdrs = hdr.create().unwrap();
  let hdr2 = EthHdr::parse(&raw_hdrs);
  assert_eq!(hdr, hdr2);
}

#[test]
fn ipv4_create_parse() {
  use pakit::hdr::Hdr;
  use pakit::hdr::{ip_proto, IPv4Hdr};

  let hdr = IPv4Hdr::from("192.168.1.1", "192.168.10.2", ip_proto::TCP);
  let raw_hdrs = hdr.create().unwrap();
  let hdr2 = IPv4Hdr::parse(&raw_hdrs);
  assert_eq!(hdr, hdr2);
}
