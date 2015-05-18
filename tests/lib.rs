/// extern test-lib for zwreec
/// to test the whole zwreec library
/// for the future: real twee and z-code test-data

extern crate zwreec;

#[test]
fn file_test() {
    assert_eq!("hello from file", zwreec::utils::file::temp_hello());
}

#[test]
fn frontend_test() {
}

#[test]
fn backend_test() {
    assert_eq!("hello from backend", zwreec::backend::temp_hello());
}
