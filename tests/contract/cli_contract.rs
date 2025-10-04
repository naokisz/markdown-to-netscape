use assert_cmd::Command;
use predicates::prelude::predicate;
use std::fs;
use tempfile::NamedTempFile;

#[test]
fn cli_runs_and_generates_file() {
    let input = NamedTempFile::new().unwrap();
    fs::write(input.path(), "- [Rust](https://www.rust-lang.org)\n").unwrap();

    let output = NamedTempFile::new().unwrap();

    let cmd = Command::cargo_bin("markdown-to-netscape").unwrap();
    cmd.arg(input.path()).arg(output.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Conversion completed"));

    let out_contents = fs::read_to_string(output.path()).unwrap();
    assert!(out_contents.contains("https://www.rust-lang.org"));
}
