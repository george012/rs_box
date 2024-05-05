// tests/version_tests.rs
extern crate rust_box;  // `your_crate_name` 是你的包名

#[test]
fn test_get_version() {
    let version = rust_box::get_version().unwrap();

    assert_eq!(version, env!("CARGO_PKG_VERSION"));
    println!("{}", version);
}