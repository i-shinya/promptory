# Tauri + React + Typescript

This template should help get you started developing with Tauri, React and Typescript in Vite.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## DB

### migration

アプリ起動時に自動的に migration するようにした

#### add migration file

頑張って手動で作成する

### generate entity

```bash
sea-orm-cli generate entity \
    -u 'sqlite:data/db/database.db?mode=rwc' \
    -o src-tauri/src/infra/repository/entities
```

## run

```bash
npm run tauri dev
```

ウィンドウが立ち上がる

また、以下 URL でブラウザから確認することも可能
`http://localhost:1420/`
