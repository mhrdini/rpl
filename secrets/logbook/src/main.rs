use std::env;

/* When using writeln!                                                        */
// write_fmt is a method defined on the trait Write,
// not an inherent method on File.
// In Rust, to call a trait's method with dot syntax (including macro-generated
// calls like writeln!), that trait must be in scope in the current module.
// Without use std::io::Write;, the compiler won't consider write_fmt a valid
// method on logbook at all.
// Requiring the use makes the choice explicit and visible for which
// implementation is being used.
// use std::io::Write;

use anyhow::Result;

fn main() -> Result<()> {
    /* Take message on command line                                           */
    let args: Vec<_> = env::args().skip(1).collect();
    if args.is_empty() {
        // bail!("Usage: logbook <MESSAGE>");

        /* Create if logbook text file doesn't exist                          */
        if let Some(text) = logbook::read("logbook.txt")? {
            print!("{text}");
        } else {
            println!("Logbook is empty");
        }
    } else {
        logbook::append("logbook.txt", &args.join(" "))?;
    }
    Ok(())
}
