#[test]
#[cfg(feature = "pcap")]
fn save_to_pcap_test() {
    use pakit::Channel;
    Channel::from("lo")
        .unwrap()
        .capture_to_pcap("./test.pcap", Some(5))
        .unwrap();
    assert!(std::path::Path::new("./test.pcap").exists());
}
