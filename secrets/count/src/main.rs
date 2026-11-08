// use std::io::BufRead;
// use std::io::stdin;
// use std::io::Error;
// use std::process;
// use std::fs::File;
// use std::io::BufReader;
use std::env;

use count::count_in_path;
// use count::count_lines;
// use count::count_lines_in_path;

/* -------------------------------------------------------------------------- */
/* Using anyhow                                                               */
/* -------------------------------------------------------------------------- */
/* anyhow::Result                                                             */
// - allows for an error type that can accept different kinds of standard
//   library errors AND our own custom messages
// - pretty printable as it implements Display
use anyhow::Result;

/* anyhow::Context                                                            */
// - trait to allow to call context method on Option value
// - so using .context(message)? on Err value will return early with message
// use anyhow::Context;

/* anyhow::bail                                                               */
// - bail! macro to return an appropriate error, hinting to user what they
//   should type to fix the problem
// - "return with this error message"
use anyhow::bail;

/* -------------------------------------------------------------------------- */
/* Using clap                                                                 */
/* -------------------------------------------------------------------------- */
use clap::Parser;
#[derive(Parser)]
/// Counts lines and/or words in the specified files
struct Args {
    /// Counts words
    #[arg(short, long)]
    words: bool,
    /// Counts lines
    #[arg(short, long)]
    lines: bool,
    /// Files to be counted
    #[arg(required = true)]
    files: Vec<String>,
}

fn main() -> Result<()> {
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
    /* From stdin                                                             */
    /* ---------------------------------------------------------------------- */
    // print errors to stderr stream not stdout
    // set exit status to some non-zero value to indicate error/failure

    // let lines = count_lines(stdin().lock());
    // match lines {
    //     Ok(lines) => println!("{lines} lines"),
    //     Err(e) => {
    //         eprintln!("Error: {e}");
    //         process::exit(1);
    //     }
    // }

    /* ---------------------------------------------------------------------- */
    /* From file                                                              */
    /* ---------------------------------------------------------------------- */
    // return Result<_> from main to use ? operator so we can:
    // - use ok_or to transform Option into Result, and bail out using ?,
    //   instead of using let...else to check if arg is None and return error
    // - use ? instead of .map_err(|e| { handle error + exit }).unwrap()

    // let path = env::args()
    //     .nth(1)
    //     .ok_or(Error::other("Usage: count <FILE>"))?;

    // or for automatic bail out with error containing context
    // let path = env::args().nth(1).context("Usage: count <FILE>")?;

    // let args: Vec<_> = env::args().skip(1).collect();
    let args = Args::parse();

    // if args.is_empty() {
    //     bail!("Usage: count [-w] <FILE>...");
    // }

    // let mut words = false;

    // for path in args {
    for path in args.files {
        // we want to provide context for the possible error in this fn
        // but in order to be able to capture values from the current environment,
        // - without consuming it: clone the context,
        // - even going beyond its lifetime: use with_context, which takes a closure,
        //   which won't be evaluated unless it's needed

        // let file = File::open(&path).with_context(|| path.clone())?;
        // let file = BufReader::new(file);
        // let lines = count_lines(file).with_context(|| path.clone())?;

        // let lines = count_lines_in_path(&path)?;
        // println!("{lines} lines");

        let count = count_in_path(&path)?;
        // without clap
        // if !words && path == "-w" {
        //     words = true;
        //     continue;
        // }
        // if words {
        //     println!("{path}: {} words", count.words);
        // } else {
        //     println!("{path}: {} lines, {} words", count.lines, count.words);
        // }

        // with clap
        if (args.words && args.lines) || !(args.words || args.lines) {
            println!("{path}: {} lines, {} words", count.lines, count.words);
        } else if args.words {
            println!("{path}: {} words", count.words);
        } else if args.lines {
            println!("{path}: {} lines", count.lines);
        }
    }

    Ok(())
}
