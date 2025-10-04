# Markdown to Netscape

Quickstart: convert the example markdown to a Netscape bookmark HTML file.

1. Build the binary:

```bash
cargo build --release
```

2. Run the converter on the example:

```bash
./target/release/markdown-to-netscape examples/links.md /tmp/bookmarks.html
```

3. Compare `/tmp/bookmarks.html` with `examples/expected_bookmarks.html`.
