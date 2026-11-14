/// Library that tracks a value against a max value and sends messages based on
/// how close to the max value the current value is
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

/* -------------------------------------------------------------------------- */
/* RefCell<T>: Creating mocks where immutable references can be mutated       */
/* -------------------------------------------------------------------------- */
// mocks to create:
// (1) mock for the Messenger trait, so our mock object can be used the same way a
//     real object is
// (2) mock object that, instead of sending the messages, stores the to-be-sent
//     messages so that we can check that we've got the correct messages based on
//     what value was given to set_value

/* How RefCell<T> works:                                                      */
// - use borrow() and borrow_mut() methods -> part of safe API belonging to RefCell<T>
// - returns Ref<T> and RefMut<T> respectively -> smart pointers, treatable as
//   regular references
// - RefCell<T> keeps track of 2 counts: how many Ref<T> and RefMut<T>
//   smart pointers are currently active, increasing count for every call to
//   borrow() or borrow_mut(), decreasing count for every smart pointer going
//   out of scope
// - same borrowing rules -> can have any # of Ref<T>s or only one RefMut<T> at
//   any point in time -> breaking the rules causes a panic!

// Practical use of RefCell<T> for mocks:
// Allows us to write mock object that can modify itself to keep track of
// changes while using it in a context where only immutable values are allowed

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;

    struct MockMessenger {
        // sent_messages: Vec<String>,

        // use RefCell<T> on to-be-mutated fields, even when they are accessed
        // via an immutable reference
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                // sent_messages: vec![],
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        // still remains an immutable ref parameter
        fn send(&self, msg: &str) {
            // self.sent_messages.push(String::from(msg));

            // but by using RefCell::borrow_mut() here for mutable borrow
            // to the sent_messages vector, it can be mutated
            self.sent_messages.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        // assert_eq!(mock_messenger.sent_messages.len(), 1);

        // use RefCell::borrow() here for immutable borrow
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
