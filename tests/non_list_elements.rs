use assert_cmd::Command;
use std::fs;
use tempfile::NamedTempFile;

#[test]
fn links_in_non_list_elements_are_extracted() {
    let md = r#"
# Heading

This is a paragraph with a [link in paragraph](https://para.example).

> A blockquote with a [quote link](https://quote.example)

Some inline `code` and a [code link](https://code.example) in text.

| Col | Link |
| --- | ---- |
| 1   | [table link](https://table.example) |

Normal list should still work:
- [List link](https://list.example)

"#;

    let input = NamedTempFile::new().unwrap();
    fs::write(input.path(), md).unwrap();

    let output = NamedTempFile::new().unwrap();

    let mut cmd = Command::cargo_bin("markdown-to-netscape").unwrap();
    cmd.arg(input.path()).arg(output.path());
    cmd.assert().success();

    let out = fs::read_to_string(output.path()).unwrap();
    // assert that links from various contexts are present
    assert!(out.contains("https://para.example"));
    assert!(out.contains("https://quote.example"));
    assert!(out.contains("https://code.example"));
    assert!(out.contains("https://table.example"));
    assert!(out.contains("https://list.example"));
}
