use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader, Error, Read},
};

use anyhow::{Context, Result};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Count {
    pub lines: usize,
    pub words: usize,
}

impl Display for Count {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(count: line: {}, words: {})", self.lines, self.words)
    }
}

pub fn count_lines(input: impl BufRead) -> Result<usize, Error> {
    let mut count = 0;
    for line in input.lines() {
        line?; // return early with Error if no line
        count += 1;
    }
    Ok(count)
}

pub fn count_lines_iter(input: impl BufRead) -> Result<usize, Error> {
    input
        .lines()
        .try_fold(0, |count, line| line.map(|_| count + 1))
}

pub fn count(mut input: impl BufRead) -> Result<Count, Error> {
    let mut count = Count::default();
    let mut line = String::new();
    while input.read_line(&mut line)? > 0 {
        count.lines += 1;
        count.words += line.split_whitespace().count();
        line.clear();
    }
    Ok(count)
}

pub fn count_in_path(path: &str) -> Result<Count> {
    let file = File::open(path).with_context(|| path.to_owned())?;
    let file = BufReader::new(file);
    count(file).with_context(|| path.to_owned())
}

pub struct ErrorReader;

impl Read for ErrorReader {
    fn read(&mut self, _buf: &mut [u8]) -> Result<usize, Error> {
        // Err(Error::new(ErrorKind::Other, "oh no"))
        Err(Error::other("oh no"))
    }
}

pub fn count_lines_in_path(path: &str) -> Result<usize> {
    let file = File::open(path).with_context(|| path.to_owned())?;
    let file = BufReader::new(file);
    count_lines(file).with_context(|| path.to_owned())
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

    #[test]
    fn count_lines_in_path_fn_counts_lines_in_file_path() -> Result<()> {
        let path = "test.txt";

        let want: usize = 3;
        let got = count_lines_in_path(path)?;
        assert_eq!(
            want, got,
            "count_lines_in_path: incorrect line count: want {want}, got {got}"
        );
        Ok(())
    }

    #[test]
    fn count_fn_counts_lines_and_words_in_input() -> Result<()> {
        let input = Cursor::new("line 1\nline 2\nline 3");

        let want = Count { lines: 3, words: 6 };
        let got = count(input)?;
        assert_eq!(
            want, got,
            "count_words: incorrect word count: want {want}, got {got}"
        );
        Ok(())
    }

    #[test]
    fn count_in_path_fn_counts_lines_and_words_in_file_path() -> Result<()> {
        let path = "test.txt";

        let want = Count { lines: 3, words: 6 };
        let got = count_in_path(path)?;
        assert_eq!(
            want, got,
            "count_in_path: incorrect line/word count: want {want}, got {got}"
        );
        Ok(())
    }
}
