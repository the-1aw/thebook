mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(tests::add_two(4), 6);
}
