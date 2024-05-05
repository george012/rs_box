
#[test]
fn test_get_version() {
    let version = crate::get_version().unwrap();
    println!("{}", version);
}