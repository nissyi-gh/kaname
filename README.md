# kaname

AI開発コックピット（管制塔）デスクトップアプリケーション。

AIエージェント(Claude Code等)へのタスク委譲を管理し、開発プロセスを効率化します。

## 技術スタック

- **Desktop Framework**: [Tauri v2](https://tauri.app/) (Rust backend)
- **Frontend**: React + TypeScript + Vite
- **将来的な追加**: SQLite, ACP (Agent Client Protocol)

## 開発環境の前提条件

- [Node.js](https://nodejs.org/) (v18+)
- [Rust](https://www.rust-lang.org/tools/install) (stable)
- [Tauri Prerequisites](https://tauri.app/start/prerequisites/)

## セットアップ

```bash
npm install
```

## 開発

```bash
npm run tauri dev
```

## ビルド

```bash
npm run tauri build
```

## プロジェクト構成

```
src/           # React frontend
src-tauri/     # Rust backend (Tauri)
docs/          # 構想ドキュメント
```

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
