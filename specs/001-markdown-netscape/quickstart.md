# Phase 1 Quickstart: Markdown→Netscape CLI

## 開発環境セットアップ
1. VS Codeでdevcontainerを開く
2. Rust最新版がインストールされていることを確認
3. cargo build / cargo test でビルド・テスト

## 変換手順
1. Markdownファイル（links.md）を用意
2. CLIコマンド例:
   ```sh
   cargo run -- links.md bookmarks.html
   ```
3. bookmarks.htmlがNetscape Bookmark File Formatで生成される

## テスト実行
```sh
cargo test
```
