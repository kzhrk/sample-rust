## MEMO

https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

string

- 文字列を変数に直接代入するのではなく、 `String::from` で宣言するとメソッドが定義された String が代入できる
- String を equal で参照渡しをするとコンパイルエラーになる
- String の変数を他の変数に代入ときは、`.clone()`メソッドを使ってメモリ領域を確保した上で代入する必要がある
- 大体の型には`.clone()` メソッドが含まれているが、Array には`.clone()`が生えていない
  - `clone_*`, `copy_*` といったメソッドが生えている
- 関数の返り値で変数の参照渡しを行った場合、古い参照を持つ変数を参照するとコンパイルエラーになる
