use std::{
    fmt::Display,
    fs::{self, File},
    io::{BufReader, BufWriter},
    path::{Path, PathBuf},
};

use std::io::Result;

use serde::{Deserialize, Serialize};

pub struct Memos {
    path: PathBuf,
    pub inner: Vec<Memo>,
}

impl Memos {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let mut memos = Self {
            path: PathBuf::from(path.as_ref()),
            inner: Vec::new(),
        };
        if fs::exists(&path)? {
            let file = File::open(&path)?;
            // de-serialise using serde_json::from_reader
            memos.inner = serde_json::from_reader(BufReader::new(file))?;
        }
        Ok(memos)
    }

    pub fn sync(&self) -> Result<()> {
        let file = File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&self.path)?;
        // serialise using serde_json::to_writer
        serde_json::to_writer(BufWriter::new(file), &self.inner)?;
        Ok(())
    }

    pub fn find_all(&mut self, prefix: &str) -> Vec<&mut Memo> {
        self.inner
            .iter_mut()
            .filter(|m| m.text.starts_with(prefix))
            .collect()
    }

    pub fn purge_done(&mut self) {
        self.inner.retain(|m| m.status != Status::Done);
    }
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Memo {
    pub text: String,
    pub status: Status,
}

impl Display for Memo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.status, self.text)
    }
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub enum Status {
    Pending,
    Done,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pending => write!(f, "[ ]"),
            Self::Done => write!(f, "[x]"),
        }
    }
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn open_returns_empty_vec_for_missing_file() {
        let memos = Memos::open("tests/data/does_not_exist.json").unwrap();
        assert!(memos.inner.is_empty(), "open: vec must be empty");
    }

    #[test]
    fn open_returns_data_from_given_file() {
        let memos = Memos::open("tests/data/memos.json").unwrap();
        assert_eq!(
            memos.inner,
            vec![
                Memo {
                    text: "foo".to_string(),
                    status: Status::Pending,
                },
                Memo {
                    text: "bar".to_string(),
                    status: Status::Pending,
                }
            ],
            "open: wrong data"
        );
    }

    #[test]
    fn sync_saves_vector_into_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("memos.json");

        let memos = Memos {
            path: path.clone(),
            inner: vec![
                Memo {
                    text: "foo".to_string(),
                    status: Status::Pending,
                },
                Memo {
                    text: "bar".to_string(),
                    status: Status::Pending,
                },
            ],
        };

        memos.sync().unwrap();

        let memos_2 = Memos::open(&path).unwrap();
        assert_eq!(memos.inner, memos_2.inner, "sync: wrong data");
    }

    #[test]
    fn round_trip_via_sync_and_open_preserves_data() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("memos.json");

        let memos = Memos {
            path: path.clone(),

            inner: vec![
                Memo {
                    text: "foo".to_string(),
                    status: Status::Pending,
                },
                Memo {
                    text: "bar".to_string(),
                    status: Status::Pending,
                },
            ],
        };

        memos.sync().unwrap();
        let memos_2 = Memos::open(&path).unwrap();
        assert_eq!(memos.inner, memos_2.inner, "wrong data");
    }

    #[test]
    fn find_all_returns_all_matching_prefix() {
        let mut memos = Memos {
            path: PathBuf::from("dummy"),
            inner: vec![
                Memo {
                    text: "foo".to_string(),
                    status: Status::Pending,
                },
                Memo {
                    text: "bar".to_string(),
                    status: Status::Pending,
                },
                Memo {
                    text: "food".to_string(),
                    status: Status::Pending,
                },
                Memo {
                    text: "eat food".to_string(),
                    status: Status::Pending,
                },
            ],
        };

        let found = memos.find_all("fo");

        assert_eq!(found.len(), 2, "find_all: wrong number of matches");
        assert_eq!(found[0].text, "foo", "find_all: wrong match");
        assert_eq!(found[1].text, "food", "find_all: wrong match");
    }

    #[test]
    fn purge_done_removes_all_completed_memos() {
        let mut memos = Memos {
            path: PathBuf::from("dummy"),
            inner: vec![
                Memo {
                    text: "foo".to_string(),
                    status: Status::Done,
                },
                Memo {
                    text: "bar".to_string(),
                    status: Status::Pending,
                },
            ],
        };

        memos.purge_done();

        let want = vec![Memo {
            text: "bar".to_string(),
            status: Status::Pending,
        }];

        assert_eq!(memos.inner, want, "purge_done: did not purge as expected");
    }
}
