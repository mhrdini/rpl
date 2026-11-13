//! Records observations in a logbook file, or lists previous
//! observations.

use anyhow::Result;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

/// Reads the contents of the logbook file at `path`.
///
/// Returns [`None`] if the file does not exist or is empty.
///
/// # Errors
///
/// Returns any error from [`fs::exists`] or [`fs::read_to_string`].
pub fn read(path: &str) -> Result<Option<String>> {
    if let Ok(text) = fs::read_to_string(path) {
        if text.is_empty() {
            Ok(None)
        } else {
            Ok(Some(text))
        }
    } else {
        Ok(None)
    }
}

/* -------------------------------------------------------------------------- */
/* AsRef<T> trait                                                             */
/* -------------------------------------------------------------------------- */
// generic AsRef<T> -> "can be turned into/obtained as a reference to T"

/// Appends `msg` to the logbook file at `path`, creating the file
/// if necessary.
///
/// # Errors
///
/// Returns any error from [`open`](fs::OpenOptions::open) or
/// [`writeln!`].
pub fn append(path: impl AsRef<Path>, message: &str) -> Result<()> {
    let mut logbook = File::options().create(true).append(true).open(path)?;
    writeln!(logbook, "{message}")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn read_returns_none_if_file_does_not_exist() {
        let want = None;
        let got = read("tests/data/does_not_exist.txt").unwrap();
        assert_eq!(want, got, "read: want {want:?}, got {got:?}");
    }

    #[test]
    fn read_reads_contents_of_file_as_string() {
        let want = "hello world";
        let got = read("tests/data/logbook.txt").unwrap().unwrap();
        assert_eq!(want, got, "read: want {want:?}, got {got:?}");
    }

    #[test]
    fn append_creates_file_if_necessary() {
        // dir variable exists to ensure temp dir doesn't get dropped from memory
        let dir = tempdir().unwrap();
        let path = dir.path().join("new_log.txt");

        let message = "hello logbook";
        append(&path, message).unwrap();

        let want = format!("{message}\n");
        let got = fs::read_to_string(path).unwrap();

        assert_eq!(want, got, "append: wrong text");
    }
}
