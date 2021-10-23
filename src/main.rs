fn main() {
  use pakit::hdr::{ArpHdr, EthHdr};
  use pakit::{proto::Proto, Packet};
  let mut packet = Packet::new()
    .header(
      ArpHdr::from(
        "00:36:76:53:ee:bd",
        "192.168.43.18",
        "ac:5f:3e:47:b1:bc",
        "192.168.43.1",
      )
      .unwrap(),
    )
    .header(EthHdr::from("00:36:76:53:ee:bd", "ac:5f:3e:47:b1:bc", 0x0806).unwrap());
  packet.build_packet();
  let reply = packet.send_and_recv();
  let reply_parsed = Packet::parse(&reply);
  println!("{:?}", reply_parsed.headers.get(&3).unwrap());
  println!("{:?}", packet.buffer);
}
