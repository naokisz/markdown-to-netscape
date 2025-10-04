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

Building for Apple Silicon macOS (locally)
----------------------------------------

If you have an Apple Silicon Mac, you can build a native `aarch64-apple-darwin` release binary locally:

```bash
# make script executable once
chmod +x scripts/build-macos-aarch64.sh
./scripts/build-macos-aarch64.sh
```

The resulting binary will be at:

```
target/aarch64-apple-darwin/release/markdown-to-netscape
```

Alternatives when not on macOS
------------------------------
- Use the GitHub Actions workflow (on this repo) which builds and uploads an `aarch64-apple-darwin` artifact.
- Cross-compilation on Linux is possible but involved (osxcross, SDKs, etc.) — see project docs or CI artifacts.

