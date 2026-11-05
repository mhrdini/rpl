use std::io::{BufRead, Error, Read, Result};

/// Counts the lines in `input`.
///
/// # Errors
///
/// Returns an error if reading from input fails, including when a line
/// isn't valid UTF-8.
pub fn count_lines(input: impl BufRead) -> Result<usize> {
    let mut count = 0;
    for line in input.lines() {
        line?; // return early with Error if no line
        count += 1;
    }
    Ok(count)
}

/// Counts the lines in input using `lines()` iter and `try_fold()` method.
///
/// # Errors
///
/// Returns an error if reading from input fails, including when a line
/// isn't valid UTF-8.
pub fn count_lines_iter(input: impl BufRead) -> Result<usize> {
    input
        .lines()
        .try_fold(0, |count, line| line.map(|_| count + 1))
}

pub struct ErrorReader;

impl Read for ErrorReader {
    fn read(&mut self, _buf: &mut [u8]) -> Result<usize> {
        // Err(Error::new(ErrorKind::Other, "oh no"))
        Err(Error::other("oh no"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ErrorReader;
    use std::io::{BufReader, Cursor};

    /* ---------------------------------------------------------------------- */
    /* Test the HAPPY PATH: that it works on a guaranteed reader              */
    /* ---------------------------------------------------------------------- */
    #[test]
    fn count_lines_fn_counts_lines_in_input() -> Result<()> {
        // panics with the io::Error's Debug output if it's Err
        let input = Cursor::new("line 1\nline 2\n");
        assert_eq!(count_lines(input).unwrap(), 2);

        // assert as Option
        let input = Cursor::new("line 1\nline 2\n");
        assert_eq!(count_lines(input).ok(), Some(2));

        // assert as Result
        let input = Cursor::new("line 1\nline 2\n");
        assert!(matches!(count_lines(input), Ok(2)));

        // the way I want
        let input = Cursor::new("line 1\nline 2\n");

        let want = 2;
        let got = count_lines(input)?;

        assert_eq!(
            want, got,
            "count_lines: wrong line count: want {want}, got {got}"
        );
        Ok(())
    }

    /* ---------------------------------------------------------------------- */
    /* Test the UNHAPPY PATH: that it handles errors correctly                */
    /* ---------------------------------------------------------------------- */
    #[test]
    fn count_lines_fn_returns_any_read_error() {
        let reader = BufReader::new(ErrorReader);
        let result = count_lines(reader);
        assert!(result.is_err(), "count_lines: no error returned");
    }
}
