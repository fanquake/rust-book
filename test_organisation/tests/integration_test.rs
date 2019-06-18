use test_organisation;

mod common;

#[test]
fn it_adds_two() {
    assert_eq!(4, test_organisation::add_two(2));
}