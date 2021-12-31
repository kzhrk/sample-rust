## MEMO

- rust は[crates.io](https://crates.io/)というレジストラを使っている
- Cargo.toml ファイルで crate のバージョン管理を行っている
- `cargo build` を実行すると、Cargo.toml を参照したバージョンの crate がインストールされる
- crate は一度ダウンロードするとグローバルでキャッシュされるようになる
- `cargo update` を実行すると、Cargo.lock のマイナーバージョンが更新される
