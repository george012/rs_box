

#[test]
fn test_get_version() {
    let version = crate::LIB_VERSION;
    println!("{}", version.to_string());
}

#[test]
fn test_rs_box_setup() {
    crate::rs_box_setup("test_setup_rs_box",crate::RunMode::RunModeTest,"",7,60)
}