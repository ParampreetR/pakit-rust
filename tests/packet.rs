#[test]
fn create_packet() {
  use pakit::hdr::{ArpHdr, EthHdr};
  use pakit::Pakit;
  let mut packet = Pakit::new()
    .header(
      ArpHdr::from(
        "aa:aa:aa:aa:aa:aa",
        "192.168.1.100",
        "bb:bb:bb:bb:bb:bb",
        "192.168.1.101",
      )
      .unwrap(),
    )
    .header(EthHdr::from("aa:aa:aa:aa:aa:bb", "cc:cc:cc:cc:cc:dd", 1).unwrap());
  packet.build_packet();
  println!("{:?}", packet.buffer);
  assert_eq!(1, 1);
}
