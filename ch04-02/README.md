## MEMO

https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

- 関数の引数に `&` を宣言すると immutable な引数として扱われる
- 関数内で引数の String を変更する場合は、引数に `&mut` 宣言が必要
  - 関数の宣言先でも `&mut` で mutable 宣言をする必要がある
- `&変数名` で変数の参照になる
  - デフォルトでimmutableな変数として扱われる
  - mutableにしたい場合は、 `&mut 変数名` の宣言が必要になる
- 同じ変数を参照するmutableな変数を2つ以上宣言するとコンパイルエラーになる
- mutableな変数参照をしたあとにimmutableな変数参照をした場合もコンパイルエラーになる
