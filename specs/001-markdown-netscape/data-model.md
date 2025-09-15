# Phase 1 Data Model: Markdown→Netscape CLI

## Entity: Link
- title: String（必須、MarkdownリンクのテキストまたはURL）
- url: String（必須、MarkdownリンクのURL）

### Validation Rules
- urlは有効なURL形式であること
- titleが空の場合はurlをタイトルとして使用

## State Transitions
- 入力: Markdownリンクリスト
- 変換: Link構造体へパース
- 出力: Netscape Bookmark HTMLへシリアライズ
