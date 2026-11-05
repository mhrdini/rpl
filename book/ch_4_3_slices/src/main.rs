fn main() {
    let s = String::from("hello world");
    let n = 0;

    // immutable borrow of s here
    let word = nth_word(n, &s);

    // mutable borrow of s here
    // error: cannot simultaneously have an immutable and a mutable borrow
    // error: cannot make mutable borrow of s when s is immutable (not mut)
    // s.clear();

    // using the immutable borrow
    println!("nth word: n = {} -> {}", n, word);
}

// zero-based, n = 0 -> first word, n = 1 -> second word
// &str:
// - allows slices of string literals (with indices)
// - allows references to Strings (= full length of String)
// - allows references to Strings with indices (= becomes string literal &str)
fn nth_word(n: i32, s: &str) -> &str {
    let bytes = s.as_bytes();

    let mut n_count = 0;

    let mut start = 0;

    for (i, &item) in bytes.iter().enumerate() {
        // println!("{:?} {}, {} {}", from_utf8(&[item]), n_count, start, i);
        if item == b' ' {
            if n_count == n {
                return &s[start..i];
            }
            start = i + 1;
            n_count += 1;
        }
    }

    if n_count < n {
        start = 0
    }
    &s[start..]
}
