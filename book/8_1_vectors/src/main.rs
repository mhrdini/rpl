/* -------------------------------------------------------------------------- */
/* Vectors: Vec<T>                                                            */
/* -------------------------------------------------------------------------- */
// - Single data structure to store multiple values next to each other in memory
// - All values must have same type
// - Implemented using generics, so need type annotation if init without values

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // without initial values
    let v: Vec<i32> = Vec::new();

    // with initial values, using vec! macro
    let v = vec![1, 2, 3];

    // adding elements onto vector using push
    let mut v: Vec<i32> = Vec::new();
    println!("v before push: {v:?}");
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("v after push: {v:?}");

    /* ---------------------------------------------------------------------- */
    /* Accessing values                                                       */
    /* ---------------------------------------------------------------------- */
    /* Accessing values via Indexing/slicing syntax                           */
    // - using & and [] gives us reference to element(s) at index value(s)
    // - will panic if trying to reference non-existent element
    // - therefore best used to enforce crash on such an attempt
    let vs = &v[2..4];
    let third = &vs[0];
    println!("The third element is {third}");

    /* Accessing values via get method                                        */
    // using get method gives us Option<&T> that can be used with match
    // - returns None without panic when attempting to referencing non-existent element
    let third = vs.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }

    /* Rules of references must still apply                                   */
    // - upon accessing and obtaining a valid reference, borrow checker makes sure
    //   of ownership and borrowing rules, so that the reference and any other
    //   references to the contents of the vector remain valid
    // - rules of references:
    //   (1) at any given time, at most ONE mutable reference, OR
    //                          ANY number of immutable references
    //   (2) automatic invalidation of references to deallocated data
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0]; // immutable borrow here

    // v.push(6); // cannot have mutable borrow here
    println!("The first element is: {first}"); // since immutable borrow is later used here

    /* ---------------------------------------------------------------------- */
    /* Iterating over vector values                                           */
    /* ---------------------------------------------------------------------- */
    // - no modification to whole vector (no insertion/removal)
    // - since an I/M borrow occurs for the vector, so having an M borrow for pushing
    //   onto the vector is impossible, because you cannot have I+M nor M+M borrows
    let v = vec![100, 32, 57];
    // immutable reference
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];
    // mutable reference
    for i in &mut v {
        // must use * deref op when mutating mutable reference, to get the value
        *i += 50;
    }

    /* ---------------------------------------------------------------------- */
    /* Using enum variants to store multiple types in vector                  */
    /* ---------------------------------------------------------------------- */
    // used when you know the exhaustive set of types
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{row:?}");
}
