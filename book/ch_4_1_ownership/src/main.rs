fn main() {
    /* ---------------------------------------------------------------------- */
    /* Ownership                                                              */
    /* - Assigning a value to another variable moves it.                      */
    /* - When a variable that includes data on the heap goes out of scope,    */
    /*   the value will be cleaned up by drop unless ownership of the data    */
    /*   has been moved to another variable.                                  */
    /* ---------------------------------------------------------------------- */

    let s = String::from("yo!!!!");
    println!("immutable\n\t{s}");
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("mutable\n\t{s}");

    /* ---------------------------------------------------------------------- */

    let s1 = String::from("hello");
    let s2 = s1; // shallow copy s1 stack data, drop s1 stack data, "hello" remains in heap

    // println!("s1: {s1}, world!"); // error: borrow of moved value, s1
    println!("shallow copied s2\n\t{s2}, world!");

    /* -------------------------------------------------------------------------- */

    let mut s = String::from("hello");
    s = String::from("ahoy"); // drop "hello" heap data, s ptr to point to new "ahoy" heap data
    println!("new heap data s\n\t{s}, world!");

    /* ---------------------------------------------------------------------- */

    let s1 = String::from("hello");
    let s2 = s1.clone(); // deep copy "hello" heap data, s2 ptr to point to new heap data
    println!("original heap data s1\n\t{s1}");
    println!("deep copy heap data s2\n\t{s2}");

    /* ---------------------------------------------------------------------- */

    let x = 5;
    let y = x; // known-size types are stored on stack -> so copy is cheap and done -> therefore no drop needed
    // these stack-only data types are annotated with Copy trait:
    // integers, boolean, floating point, char, tuples with only Copy-types
    // types implementing Drop trait can never be annotated with Copy trait
    println!("original stack data x\n\t{x}");
    println!("copy trait stack data y\n\t{y}");

    /* ---------------------------------------------------------------------- */

    let s = String::from("hello"); // s comes into scope
    takes_ownership(s); // s's value moves into function
    // so s is no longer valid here -> error: borrow of moved value, s

    let x = 5; // x comes into scope
    makes_copy(x); // x's value does not move into function
    // so x can be used here

    /* ---------------------------------------------------------------------- */

    let s1 = gives_ownership();
    println!("given ownership from fn s1\n\t{s1}");
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); // s2's value moves into function, then into s3
    // so s2 is no longer valid here
    // println!("s2\n\t{s2}"); // error: borrow of moved value, s2
    println!("moved ownership from s2 via fn s3\n\t{s3}");

    /* ---------------------------------------------------------------------- */

    // problem: tediously having to pass back anything we pass in
    // using multi-valued tuple
    let s1 = String::from("hello");
    let (s2, len) = calculate_length_by_move(s1);
    println!(
        "ownership forcing return of moved value of s1 from fn into s2 + len\n\t{s2} -> {len}"
    );

    // solution: borrowing by passing references! see 4_2_borrowing/main.rs
}

/* Ownership helper functions                                                 */

fn takes_ownership(some_string: String) {
    println!("ownership of heap data moved/taken\n\t{some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("ownership of stack data not moved, just copied\n\t{some_integer}");
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length_by_move(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
