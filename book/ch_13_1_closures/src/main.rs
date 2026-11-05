/* -------------------------------------------------------------------------- */
/* Closures                                                                   */
/* -------------------------------------------------------------------------- */
// - anonymous functions you can save in variable or pass as arguments
// - can be defined in one context and then callable elsewhere,
//   therefore evaluated in a separate context
// - can capture values from the scope in which they are DEFINED
// - aim: code reuse, behaviour customisation

use std::thread;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

/* -------------------------------------------------------------------------- */
/* Closure expressions                                                        */
/* -------------------------------------------------------------------------- */
// - vertical pipes || with optional parameters between them, and a body of the
//   closure right after
// - types are inferred, so usually no type annotations for parameters or return
//   needed
// - inferred types are locked in upon first call, so cannot call same closure
//   with different type thereafter
fn main() {
    /* ---------------------------------------------------------------------- */
    /* Function vs Closure: Syntax                                            */
    /* ---------------------------------------------------------------------- */
    fn add_one_v1(x: usize) -> usize {
        x + 1
    }

    let add_one_v1 = |x: usize| -> usize { x + 1 };
    let add_one_v2 = |x: usize| x + 1;

    /* ---------------------------------------------------------------------- */
    /* Types locked in after first call                                       */
    /* ---------------------------------------------------------------------- */
    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    // let n = example_closure(5); // error: mismatched types

    /* ---------------------------------------------------------------------- */
    /* 3 ways to capture values from environment                              */
    /* ---------------------------------------------------------------------- */
    // - depends on what the body does with the captured value

    /* (1) Borrow immutably                                                   */
    // - other immutable uses of closure variable is allowed
    let list = vec![1, 2, 3];
    println!("Immutable borrow - Before closure: {list:?}");
    let only_borrows = || println!("Immutable borrow - From closure: {list:?}");
    only_borrows();
    println!("Immutable borrow - After closure: {list:?}");

    /* (2) Borrow mutably                                                     */
    // - closure variable must be mut
    // - no other borrows of closure variable allowed between closure def and
    //   last use
    let mut list = vec![1, 2, 3];
    println!("Mutable borrow - Before closure: {list:?}");
    let mut borrows_mutably = || list.push(7);
    borrows_mutably();
    println!("Mutable borrow - After closure: {list:?}");

    /* (3) Take ownership: Using move keyword before (||) vertical lines      */
    // - force ownership of closure variable even if closure only needs ref
    // - mostly needed when handing data to a new thread, so the data outlives
    //   the spawning scope
    let list = vec![1, 2, 3];
    println!("Take ownership - Before closure: {list:?}");
    thread::spawn(move || println!("Take ownership - From thread: {list:?}"))
        .join()
        .unwrap();

    /* ---------------------------------------------------------------------- */
    /* Traits that closures implement                                         */
    /* ---------------------------------------------------------------------- */

    /* Closure body                                                           */
    // - dictates what happens to references or values when closure is evaluated
    //   later
    // - can do any of the following:
    //   - (1) move captured value out of closure
    //   - (2) mutate captured value
    //   - (3) neiher move nor mutate value
    //   - (4) capture nothing to begin with
    // - this affects what traits a closure automatically implements

    /* Closure-implemented Fn traits                                          */
    // - additive -> can automatically implement one, two, or all three Fn traits
    // - used to specify what kinds of closures other functions and structs can use

    /* (1) FnOnce                                                             */
    // - can be called once
    //   - every closure implements it (because all are callable)
    // - closures that move captured values out of its body -> implements
    //   FnOnce ONLY, because they can ONLY be called once
    // - most flexible

    /* (2) FnMut                                                              */
    // - doesn't move captured values out of body
    // - AND may mutate captured values
    // - can be called more than once

    /* (3) Fn                                                                 */
    // - doesn't move captured values out of body
    // - AND doesn't mutate captured values
    // - OR capture nothing to begin with
    // - can be called more than once, even concurrently, without mutating
    //   environment

    /* Using a function in lieu of Fn trait implementing construct                */
    // If what we want to do doesn't require capturing a value from environment,
    // -> we can use name of a function rather than a closure wherever we need
    // something that implements one of the Fn traits.
    // (Compiler will auto-implement whichever Fn traits is applicable)
    // e.g. On an Option<Vec<T>> value, we can call unwrap_or_else(Vec::new) to
    // get a new empty vector if value is None.

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    // let mut sort_operations = vec![];
    // let value = String::from("closure called");
    let mut num_sort_operations = 0;

    list.sort_by_key(|r| {
        /* error: moves String value out of closure into sort_operations      */
        // - on first call, value is moved out
        // - on second call, attempt to push value is invalid because it's
        //   already gone -> illegal "use-after-move"
        // - this makes the closure FnOnce, which is incompatible with
        //   sort_by_key's parameter type of FnMut
        // sort_operations.push(value);
        num_sort_operations += 1;
        r.width * r.height
    });

    println!("{list:#?}, sorted in {num_sort_operations} operations");
}
