use assert_cmd::Command;
use std::fs;
use tempfile::NamedTempFile;

#[test]
fn quickstart_end_to_end() {
    let md = "- [Example](https://example.com)\n- [Rust](https://www.rust-lang.org)\n";
    let input = NamedTempFile::new().unwrap();
    fs::write(input.path(), md).unwrap();

    let output = NamedTempFile::new().unwrap();

    let mut cmd = Command::cargo_bin("markdown-to-netscape").unwrap();
    cmd.arg(input.path()).arg(output.path());
    cmd.assert().success();

    let out = fs::read_to_string(output.path()).unwrap();
    assert!(out.contains("https://example.com"));
    assert!(out.contains("https://www.rust-lang.org"));
}
