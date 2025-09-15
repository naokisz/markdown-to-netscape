# Phase 0 Research: Markdown→Netscape CLI (Rust/devcontainer)

## タイトルなしリンクの扱い
- Decision: Netscape Bookmark File Formatではタイトルが必須。タイトルがない場合はURLをタイトルとして使用する。
- Rationale: Netscape仕様上、<A>タグのテキストがタイトルとなるため、空欄はユーザービリティ低下。
- Alternatives considered: 空欄、固定文字列（"No Title"）

## Markdown以外の入力ファイルのエラー処理
- Decision: 拡張子が.md以外の場合はエラーを返す。ファイル内容がMarkdownでない場合もエラー。
- Rationale: 入力形式の明確化と誤動作防止。
- Alternatives considered: 拡張子無視、内容判定のみ

## RustでNetscape Bookmark HTML生成ベストプラクティス
- Decision: HTMLテンプレートを定義し、各リンクを<A>タグで出力。pulldown-cmarkでMarkdownパース。
- Rationale: RustでのHTML生成はテンプレートエンジン不要。シンプルな文字列連結で十分。
- Alternatives considered: Tera等テンプレートエンジン利用

## devcontainerでRust開発環境の最適化
- Decision: devcontainer.jsonでRust最新版、cargo、clippy、rustfmtをインストール。Dockerfileで必要ツール追加。
- Rationale: VS Code上で一貫した開発・テスト・ビルド環境を提供。
- Alternatives considered: ローカル環境、GitHub Codespaces
