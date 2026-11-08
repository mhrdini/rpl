use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
fn binary_with_no_args_prints_usage() {
    Command::cargo_bin("count")
        .unwrap()
        .assert()
        .failure() // expect program to exit with failure status code
        .stderr(predicate::str::contains("Usage"));
}

#[test]
fn binary_with_valid_args_prints_line_and_word_count() {
    Command::cargo_bin("count")
        .unwrap()
        .args(["test.txt", "test_1.txt"])
        .assert()
        .success()
        .stdout("test.txt: 3 lines, 6 words\ntest_1.txt: 0 lines, 0 words\n");
}

#[test]
fn binary_with_w_flag_counts_words() {
    Command::cargo_bin("count")
        .unwrap()
        .args(["-w", "test.txt"])
        .assert()
        .success()
        .stdout("test.txt: 6 words\n");
}
