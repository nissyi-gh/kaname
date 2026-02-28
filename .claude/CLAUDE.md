# kaname

AI開発コックピット（管制塔）デスクトップアプリ。

## Tech Stack

- Frontend: React 19 + TypeScript 5.8 + Vite 7
- Backend: Tauri v2 (Rust)
- DB: SQLite (planned)

## Development Commands

### Frontend

- `npm run dev` -- Viteの開発サーバーを起動
- `npm run build` -- TypeScriptのコンパイルとViteビルド
- `npm run tauri dev` -- Tauriアプリとして起動（開発モード）
- `npm run lint` -- ESLintによるコードチェック
- `npm run format` -- Prettierによるコードフォーマット
- `npm run format:check` -- フォーマットのチェック（CI用）

### Rust

- `cd src-tauri && cargo build` -- Rustバックエンドのビルド
- `cd src-tauri && cargo fmt` -- Rustコードのフォーマット
- `cd src-tauri && cargo fmt --check` -- フォーマットのチェック
- `cd src-tauri && cargo test` -- テスト実行

## Project Structure

```
kaname/
  src/              # React frontend
  src-tauri/        # Rust backend (Tauri)
    src/
      acp/          # ACP (Agent Client Protocol) 連携
        mod.rs      # モジュール公開
        client.rs   # KanameClient: acp::Client トレイト実装
        connection.rs # サブプロセス起動、stdio接続、initialize
      lib.rs        # Tauriアプリのメインロジック
      main.rs       # エントリーポイント
  docs/             # 設計ドキュメント
```

## Code Style

### TypeScript / React

- ESLint + Prettier で統一
- ダブルクォート、セミコロンあり
- インデント: 2スペース
- React 19 の新しいJSX Transform使用（`import React` 不要）

### Rust

- `rustfmt` でフォーマット（edition 2021）
- `cargo fmt --check` でCI検証

## Conventions

- ブランチ命名: `{step番号}/{連番}-{短い説明}` (例: `step1/02-dev-environment`)
- PRマージ: Squash merge
- コミットメッセージ: Conventional Commits形式
