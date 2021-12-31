## MEMO

```
$ cargo build
$ ./target/debug/hello_cargo
$ cargo run
$ cargo build --release
```

- `cargo build` で実行ファイルを出力
- `cargo run` は `cargo build` で出力された `./target/debug/hello_cargo` を実行するのと同義
- `cargo build --release` で最適化されたリリース用の実行ファイルを `./target/release` ディレクトリに出力
