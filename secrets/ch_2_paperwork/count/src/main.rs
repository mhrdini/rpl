// use std::io::BufRead;
use std::{io::stdin, process};

use count::count_lines;

fn main() {
    /* ---------------------------------------------------------------------- */
    /* Prototype                                                              */
    /* ---------------------------------------------------------------------- */
    // stdin() -> handle to standard input, stdout() -> std output
    // lock() -> mutex (mutually exclusive) lock, ensure no other thread can
    // read from stdin

    // StdinLock<'_> -> gets BufRead trait, gives buffered reader on input,
    // which gives us lines method that returns an iterator over input data one
    // line at a time, and iterators have count method to count its length

    // let input = stdin().lock();
    // let lines = input.lines().count();
    // println!("{lines} lines");

    /* ---------------------------------------------------------------------- */
    /* Moved to lib.rs                                                        */
    /* ---------------------------------------------------------------------- */
    let lines = count_lines(stdin().lock());
    match lines {
        Ok(lines) => println!("{lines} lines"),
        Err(e) => {
            // print errors to stderr stream not stdout
            eprintln!("Error: {e}");
            // set exit status to some non-zero value to indicate error/failure
            process::exit(1);
        }
    }
}
