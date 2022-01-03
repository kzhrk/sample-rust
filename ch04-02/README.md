## MEMO

https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

- 関数の引数に `&` を宣言すると immutable な引数として扱われる
- 関数内で引数の String を変更する場合は、引数に `&mut` 宣言が必要
  - 関数の宣言先でも `&mut` で mutable 宣言をする必要がある
