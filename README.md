# Markdown to Netscape

## 概要

このプログラムは、Markdown形式で書かれたリンクリストを「Netscape Bookmark HTML」形式に変換するコマンドラインツールです。  
Webブラウザのブックマークインポート機能などで利用できるHTMLファイルを簡単に生成できます。

## 主な特徴

- Markdownのリンクリスト（例: `[タイトル](URL)`）をNetscape Bookmark HTMLに変換
- 無効なリンクやURLは自動的に無視し、警告として表示

## 使い方

### 1. ビルド

```bash
cargo build --release
```

### 2. 変換の実行

例として、`examples/links.md` を Netscape形式HTMLに変換し `/tmp/bookmarks.html` に出力します。

```bash
./target/release/markdown-to-netscape examples/links.md /tmp/bookmarks.html
```

### 3. 結果の確認

出力された `/tmp/bookmarks.html` を `examples/expected_bookmarks.html` と比較してください。

---


## ライセンス

このプロジェクトはMITライセンスで公開されています。  
自由に利用・改変・再配布が可能です。

---

