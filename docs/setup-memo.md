

## インストール

```bash
brew install rustup-init
rustup-init

cargo install tauri-cli
```

### tauri-apps/cli installで警告

https://github.com/advisories/GHSA-2rcp-jvr4-r259

npm installでdependency errorでてnpm auditで以下のような表示
"@tauri-apps/cli": "^2.0.0-alpha.17"を指定したけど色々追いついていないみたい
一旦警告は無視して1.5.6で進める

```
# npm audit report
@tauri-apps/cli  <2.0.0-alpha.16
Severity: high
Tauri's Updater Private Keys Possibly Leaked via Vite Environment Variables - https://github.com/advisories/GHSA-2rcp-jvr4-r259
No fix available
node_modules/@tauri-apps/cli
1 high severity vulnerability
Some issues need review, and may require choosing
a different dependency
```
