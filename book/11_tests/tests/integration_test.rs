/* -------------------------------------------------------------------------- */
/* Integration tests                                                          */
/* -------------------------------------------------------------------------- */
// can only access from library crate (lib.rs), not binary crate (main.rs)

use tests_lib::add_two; // valid -> straight from lib.rs
use tests_lib::adder::add; // valid ->  pub mod adder was written in lib.rs

#[test]
fn it_adds_two() {
    assert_eq!(add_two(2), 4);
}

#[test]
fn it_adds() {
    assert_eq!(add(2, 3), 5);
}
