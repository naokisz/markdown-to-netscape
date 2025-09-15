# Tasks: Markdownリンクリスト→NetscapeブックマークCLI (Rust/devcontainer)

## 実行ガイド
- [P]は並列実行可能なタスク
- 各タスクは依存関係順に並び、TDD（テスト→実装）を徹底
- ファイルパスは絶対パスで記載

---

### T001: devcontainerセットアップ
- /devcontainer/devcontainer.json, /devcontainer/Dockerfile を作成し、Rust最新版・cargo・clippy・rustfmtをインストール
- 依存: なし

### T002: Rustプロジェクト初期化
- cargo init で src/ ディレクトリを作成
- 依存: T001

### T003: 必要ライブラリ追加
- clap, pulldown-cmark, anyhow, log をCargo.tomlに追加
- 依存: T002

### T004: Link構造体の定義 [P]
- src/models/link.rs に title, url フィールドを持つ構造体を定義
- 依存: T003

### T005: Markdownパースロジックのテスト作成 [P]
- tests/unit/markdown_parse.rs にMarkdownリンクリスト→Link構造体への変換テスト（RED状態）
- 依存: T004

### T006: Markdownパースロジック実装 [P]
- src/lib/markdown_parse.rs にMarkdown→Link構造体変換処理を実装
- 依存: T005

### T007: Netscape Bookmark HTML生成ロジックのテスト作成 [P]
- tests/unit/html_gen.rs にLink構造体→Netscape Bookmark HTML変換テスト（RED状態）
- 依存: T004

### T008: Netscape Bookmark HTML生成ロジック実装 [P]
- src/lib/html_gen.rs にLink構造体→HTML変換処理を実装
- 依存: T007

### T009: CLIコマンド仕様テスト作成
- tests/contract/cli_contract.rs に cargo run -- <input.md> <output.html> の正常系・異常系テスト（RED状態）
- 依存: T006, T008

### T010: CLIコマンド実装
- src/cli/main.rs にコマンド引数処理・ファイル入出力・エラー処理を実装
- 依存: T009

### T011: 統合テスト（クイックスタートシナリオ）作成 [P]
- tests/integration/quickstart.rs に quickstart.md の手順を再現する統合テスト（RED状態）
- 依存: T010

### T012: 統合テスト実装 [P]
- tests/integration/quickstart.rs のテストをパスするように実装
- 依存: T011

### T013: ログ・エラーハンドリング強化 [P]
- src/lib/logging.rs, src/lib/error.rs で構造化ログ・エラー管理を追加
- 依存: T010

### T014: パフォーマンステスト・最適化 [P]
- tests/unit/performance.rs に1万リンク/秒以上の変換速度を検証するテスト
- 依存: T008, T010

### T015: ドキュメント・README作成 [P]
- README.md, quickstart.md, contracts/cli-contract.md を整備
- 依存: T012, T013, T014

---

## 並列実行例
- T004, T005, T007 は同時に着手可能 [P]
- T006, T008, T013, T014, T015 はそれぞれ依存解消後に並列実行可能 [P]

## 依存関係まとめ
- T001→T002→T003→(T004, T005, T007)→(T006, T008)→T009→T010→T011→T012→(T013, T014, T015)

---

## 実行コマンド例
- cargo test
- cargo run -- links.md bookmarks.html
