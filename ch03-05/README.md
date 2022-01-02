## MEMO

https://doc.rust-lang.org/book/ch03-05-control-flow.html

### if

- JavaScript で変数の存在確認をする `if number {}` のような書き方はコンパイルエラーになる
  - この場合の number は boolean として判定される。integer 型が入っているとコンパイルエラー
- if 文を使って変数にアサインされる value を切り替えることができるが、型が異なる場合はコンパイルエラーになる

### loop

- loop 文は `'counting_up:` のようにシングルクォートと文字列でラベルを付けることができる
- break で break する loop のラベルを指定することができる
- break に value を渡すことで、loop の結果を変数に渡すことが可能になる

### for in

- Array を `for element in collection {}` で for loop させることができる
- Array には配列の順番を反転させる `.rev()` メソッドが存在する
