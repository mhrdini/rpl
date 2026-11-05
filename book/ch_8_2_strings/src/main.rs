/* -------------------------------------------------------------------------- */
/* String type                                                                */
/* -------------------------------------------------------------------------- */
// - collection of bytes, UTF-8 encoded, part of standard library
// - different to string slices (str or borrowed form &str) of the core language
//   which are references to some UTF-8 encoded data stored elsewhere,
//   e.g. string literals (which is stored in program binary) are string slices
// - String is implemented as wrapper around a vector of bytes, Vec<u8>
//    +++ other guarantees, restrictions, capabilities
fn main() {
    /* ---------------------------------------------------------------------- */
    /* Creating a String                                                      */
    /* ---------------------------------------------------------------------- */
    // using new method
    let _s = String::new();

    // using to_string() method on string literal
    let data = "initial contents";
    let _s = data.to_string();
    let _s = "initial contents".to_string();

    /* ---------------------------------------------------------------------- */
    /* Appending                                                              */
    /* ---------------------------------------------------------------------- */
    // using push_str to append string slice, push_str borrows
    let mut s = String::from("foo");
    s.push_str("bar");
    s.push_str(&String::from("bar")); // deref coercion: &String -> [String -> str] -> &str

    // using push to append a single char
    let mut s = String::from("lo");
    s.push('l');

    /* ---------------------------------------------------------------------- */
    /* Concatenating                                                          */
    /* ---------------------------------------------------------------------- */
    /* using + to combine strings                                             */
    // because + uses add(self, &str) method, leftmost operand is moved
    // therefore LH addend is moved
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // takes ownership of s1, appends copy of s2, returns ownership of result
    // so s1 no longer usable here
    println!("{s3}");

    let mut s4 = s3;
    s4.push_str(" <3");
    println!("{s4}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let _s = s1 + "-" + &s2 + "-" + &s3;
    // let s = s1 + &s2 + &s3;

    /* using format! macro to combine without giving up leftmost operand      */
    // uses reference so no ownership is moved
    let s1 = String::from("tic");
    let s = format!("{s1}-{s2}-{s3}");

    /* ---------------------------------------------------------------------- */
    /* Indexing                                                               */
    /* ---------------------------------------------------------------------- */
    // - no indexing using &s[i] where i is a single integer

    /* only by slicing                                                        */
    // - must be used with caution, because it will get the number of bytes
    // - this will create a string slice containing particular bytes
    // - so in this example, it will get the first 4 bytes of the string
    //   these characters have 2 bytes each, so s will be "Зд"
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{s}");

    /* ---------------------------------------------------------------------- */
    /* Iterating                                                              */
    /* ---------------------------------------------------------------------- */
    // - must explicitly say if you want the bytes or the chars of the string
    for c in hello.chars() {
        println!("{c}");
    }

    for b in hello.bytes() {
        println!("{b}");
    }
}
