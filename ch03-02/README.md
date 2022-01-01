## MEMO

[3.2 Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html)

Rust には 4 つの型がある。

- integers
- floating point numbers
- boolean
- characters

integers 型にはビット数に応じた i8, i16, i32, i64, i128 の型と、arch 型が存在する。  
ビット数に応じた型には、signed, unsigned な型が存在し、それぞれ `i8`, `u8` のような型宣言を行う。

signed な型はマイナス値を持たない integer として処理される。  
たとえば、 `i8` は 8bit の正の値なので `0` ~ `2^8 - 1` の数値を表現することができる。

unsigned な型はマイナス値を持つ interger として処理される。  
先頭の bit が正負の表現を行うため、`u8` は `-2^7` ~ `2^7 - 1` の数値を表現することができる。

この型表現は[Two's complement](https://en.wikipedia.org/wiki/Two%27s_complement)と呼ばれている。

> 余談
> JS Primer の[ビット演算子](https://jsprimer.net/basic/operator/#bit-operator)の商で書かれていたように、JavaScript の場合は基本的に unsigned な型定義になっている。

arch 型のビット長は Rust が動作しているコンピュータのアーキテクチャに依存する。  
32bit であれば 32bit、64bit であれば 64bit。

integer 型は数値だけではなく、suffix をつけることで様々な表現ができる。

| Number literals | e.g.        |
| :-------------- | :---------- |
| Decimal         | 98_222      |
| Hex             | 0xff        |
| Octal           | 0o77        |
| Binary          | 0b1111_0000 |
| Byte (u8 only)  | b'A'        |
