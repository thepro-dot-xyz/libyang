use libyang::Yang;
use std::path::PathBuf;

#[test]
fn scan_dir() {
    let yang = Yang::new();
    let path = PathBuf::from("tests/ietf-dhcp@2017-03-02.yang");

    if let Ok(p) = yang.scan_dir("tests", "ietf-dhcp.yang", false) {
        assert_eq!(p, path);
    } else {
        panic!("scan_dir should match to yang file.");
    }
}
