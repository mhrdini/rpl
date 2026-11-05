use std::{
    error::Error,
    fs::{self, File, OpenOptions},
    io::{Error as IOError, ErrorKind, Read, Seek, SeekFrom, Write},
};

fn main() {
    /* ---------------------------------------------------------------------- */
    /* Unrecoverable errors with panic! macro                                 */
    /* ---------------------------------------------------------------------- */
    /* Unwinding                                                              */
    // - By default, when panic occurs, Rust program starts unwinding
    //   i.e. walking back up the stack and cleaning up data from each fn it
    //   encounters
    // - Another option is to immediate Abort without clean up, letting the OS
    //    do the clean up of memory the program was using
    // - In Cargo.toml:
    //   [profile.release]
    //   panic = 'abort'
    // panic!("crash and burn"); // uncomment this line for panic!!!!!

    /* ---------------------------------------------------------------------- */
    /* Recoverable errors with Result<T, E>                                   */
    /* ---------------------------------------------------------------------- */
    // relative paths are resolved against current working dir, so not in src
    // but its parent dir
    match fs::read_to_string("hello.txt") {
        Ok(contents) => println!("{contents}"),
        Err(error) => {
            // panic!("Problem opening the file: {error:?}")
            println!("Problem opening the file: {error:?}")
        }
    }

    /* ---------------------------------------------------------------------- */
    /* Matching on different errors with ErrorKind                            */
    /* ---------------------------------------------------------------------- */
    let greeting_file_result = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("hello_world.txt");

    let mut greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello_world.txt") {
                Ok(file) => file,
                Err(error) => panic!("Problem creating the file: {error:?}"),
            },
            // add other ErrorKind variants to match here
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };

    let _ = greeting_file.write_all(b"Hello, world!");
    let _ = greeting_file.seek(SeekFrom::Start(0)); // rewind, or you'll read nothing

    let mut contents = String::new();
    greeting_file
        .read_to_string(&mut contents)
        .expect("Failed to read hello_world.txt into string");

    println!("{contents}");

    /* ---------------------------------------------------------------------- */
    /* Alternatives for match with Result<T, E>                               */
    /* ---------------------------------------------------------------------- */
    let mut greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });

    let mut contents = String::new();
    greeting_file
        .read_to_string(&mut contents)
        .expect("Failed to read hello_world.txt into string");

    /* ---------------------------------------------------------------------- */
    /* Shortcuts for panic on error                                           */
    /* ---------------------------------------------------------------------- */
    // unwrap = shortcut to transform from Result and either: return Ok variant,
    // or panic! macro if Result is Err variant
    // let greeting_file = File::open("goodbye.txt").unwrap();
    let greeting_file = File::open("hello.txt").unwrap();
}

/* -------------------------------------------------------------------------- */
/* Propagating errors                                                         */
/* -------------------------------------------------------------------------- */
// propagating = returning error to calling code and let it decide what to do
fn read_username_from_file() -> Result<String, IOError> {
    let username_file_result = File::open("hello.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e), // returns error
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e), // returns error
    }
}

/* -------------------------------------------------------------------------- */
/* Propagating errors by using the ? operator on Result                       */
/* -------------------------------------------------------------------------- */
// - how it works similarly to match above:
//   - an expression calling ? on Result::Ok will return the value inside the Ok
//     from the expression
//   - an expression calling ? on Result::Err will return the Err itself from
//     the whole function (as if we had used the return keyword),
//     therefore propagating the error to the calling code
// - how it works differently from match above:
//   - calling ? on Error values invokes `from` function (defined in std::from::From
//     trait) on them, which converts the error type received by the Error value
//     to the error type defined in the RETURN type of the current FUNCTION
// - therefore the function's return type must be Result<T, E> compatible to all
//   Results that any ?'s in the function is called on
fn read_username_from_file_shortcut() -> Result<String, IOError> {
    let mut username_file = File::open("hello.txt")?; // here
    let mut username = String::new();

    // if Ok -> will return Ok to username_file var
    // if Err -> return early out of whole fn and give any Err value to calling code
    username_file.read_to_string(&mut username)?;

    // can be chained like:
    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn read_username_from_file_even_shorter() -> Result<String, IOError> {
    fs::read_to_string("hello.txt")
}

/* -------------------------------------------------------------------------- */
/* Using ? operator on Option<T>                                              */
/* -------------------------------------------------------------------------- */
// - if Option is None -> returns None early
// - if Option is Some -> extract value inside Some as resultant value of expression,
//   and function continues
fn last_char_of_first_line(text: &str) -> Option<char> {
    // gets first line with .lines().next()
    // calling ? on:
    // - None -> return the None early if there's no first line
    // - Some -> expression evaluates to the first line (&'a str type)
    // then it just simply gets the last char
    text.lines().next()?.chars().last()
}

/* -------------------------------------------------------------------------- */
/* When and how to use ? operator                                             */
/* -------------------------------------------------------------------------- */
// - Only within functions that return Result, Option, or another type
//   implementing FromResidual
// - 2 choices to fix errors regarding not being able to use ?:
//   (1) change return type of function to be compatible with value you're using
//   the ? on
//   (2) use match or one of the Result<T, E> methods to handle Result<T, E>
fn open_file() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;
    Ok(())
}
