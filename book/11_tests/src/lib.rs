/* -------------------------------------------------------------------------- */
/* Tests                                                                      */
/* -------------------------------------------------------------------------- */
// 1. Set up any needed data or state
// 2. Run the code you want to test
// 3. Assert that the results are what you expect

pub mod adder; // to make it visible for tests/integration_test.rs

/* -------------------------------------------------------------------------- */
#[derive(Debug)]
struct Rectangle {
    width: u32,

    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
/* -------------------------------------------------------------------------- */
pub fn add_two(a: u64) -> u64 {
    a + 2
}
/* -------------------------------------------------------------------------- */
fn greeting(name: &str) -> String {
    format!("Hello {name}!")
}
/* -------------------------------------------------------------------------- */
struct Guess {
    value: i32,
}

impl Guess {
    fn new(value: i32) -> Guess {
        assert!(
            (1..=100).contains(&value),
            "Guess must be between 1 and 100, got {value}."
        );
        Guess { value }
    }
}

/* -------------------------------------------------------------------------- */
/* Unit tests                                                                 */
/* -------------------------------------------------------------------------- */
// - live in same file as code
// - inside module named tests
// - annotate tests module with #[cfg(test)] ->  test code is only compiled
//   for `cargo test`, not `cargo build`, to save compile time and binary size
#[cfg(test)]
mod tests {
    use crate::adder::add;
    /* ---------------------------------------------------------------------- */
    /* use super::*                                                           */
    /* ---------------------------------------------------------------------- */
    // - tests module is an inner module
    // - so using super with glob makes it so that anything defined in outer
    //   module is available in this tests module
    // - makes it able to test private functions
    use super::*;

    /* ---------------------------------------------------------------------- */
    /* Using assert! macro to check for truth                                 */
    /* ---------------------------------------------------------------------- */
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller)); // assert! to assert exp is true
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger)); // prepend with ! to assert false
    }

    /* ---------------------------------------------------------------------- */
    /* Using assert_eq! and assert_ne! macros to check for in/equality        */
    /* ---------------------------------------------------------------------- */
    #[test]
    fn it_adds_two() {
        let result = add_two(2);
        // left and right -> order doesn't matter
        assert_eq!(result, 4);
        assert_eq!(4, result);
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{result}`"
        );
    }

    /* ---------------------------------------------------------------------- */
    /* should_panic attribute + expected panic message                        */
    /* ---------------------------------------------------------------------- */
    // passes if tests panic with correct expected message
    #[test]
    #[should_panic = "Guess must be between 1 and 100, got 200."] // to pass tests that panic
    fn guess_greater_than_100() {
        Guess::new(200);
    }

    /* ---------------------------------------------------------------------- */
    /* Using Result<T, E> as test return type                                 */
    /* ---------------------------------------------------------------------- */
    // tests that return Result<T, E> can let you use ? inside tests
    #[test]
    // cannot use should_panic attribute
    fn it_works() -> Result<(), String> {
        let result = add(2, 2);
        if result == 4 {
            Ok(()) // return to pass
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
