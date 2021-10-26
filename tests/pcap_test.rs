#[test]
fn save_to_pcap_test() {
  use pakit::sock::MyChannel;
  MyChannel::from("lo")
    .unwrap()
    .capture_to_pcap("./test.pcap", Some(5));
  assert!(std::path::Path::new("./test.pcap").exists());
}
