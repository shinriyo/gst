# gst (Git Skip-worktree Tool)

tigライクなGitのskip-worktreeフラグを管理するためのTUI（ターミナルユーザーインターフェース）ツールです。

## 特徴

- skip-worktreeフラグを管理するための対話的なTUI
- 簡単なナビゲーションとトグル機能
- 一括操作のサポート
- クイック操作のためのコマンドラインインターフェース

## インストール

### ソースからのインストール

```bash
cargo install --path .
```

### Homebrewを使用

```bash
brew tap shinriyo/gst
brew install gst
```

## 使い方

### TUIモード

Gitリポジトリで以下のコマンドを実行するだけです：

```bash
gst
```

#### キーバインド

- `j` / `↓`: カーソルを下に移動
- `k` / `↑`: カーソルを上に移動
- `u`: 選択したファイルのskip-worktreeフラグを切り替え
- `!`: すべてのskip-worktreeフラグをクリア（確認あり）
- `q`: 終了

### CLIモード

ファイルをスキップ：
```bash
gst skip file1 file2
```

トラッキングを再開：
```bash
gst resume file1 file2
```

## なぜskip-worktreeを使うのか？

skip-worktreeフラグは、追跡されているファイルのローカルな変更をGitに無視させたい場合に便利です。一般的な使用例：

- ローカルで変更が必要な設定ファイル
- 環境固有の設定
- ローカル開発用の調整

`.gitignore`とは異なり、skip-worktreeは既にGitで追跡されているファイルに対して機能します。

## 必要要件

- Git
- Rust 2021エディション以降

## ライセンス

MIT

## 作者

shinriyo 