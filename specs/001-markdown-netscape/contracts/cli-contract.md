# CLI Contract: Markdown→Netscape

## コマンド仕様
```
cargo run -- <input.md> <output.html>
```
- <input.md>: Markdown形式のリンクリストファイル
- <output.html>: Netscape Bookmark File Formatで出力

## オプション
- --help: ヘルプ表示
- --version: バージョン表示

## エラーケース
- 入力ファイルが存在しない場合
- 入力ファイルがMarkdown形式でない場合
- リンクリストが空の場合
- URLが不正な場合

## 出力
- 標準出力に進捗・完了メッセージ
- 標準エラー出力にエラー内容
