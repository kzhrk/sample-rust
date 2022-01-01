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

### floating point numbers

浮動小数点の型は `f32` と `f64` が存在する。  
integer の arch 型と同じく、コンピュータのアーキテクチャによって型が決定される。  
デフォルトの浮動小数点の型は `f64` になっている。
浮動小数点は IEEE-754 に準拠している。

#### 演算子

二項演算子の四則演算は JavaScript と同じ。

### boolean

boolean の型宣言は `bool` を使う。

### charcter

文字列の型宣言は存在しない？

### tuple type

JavaScript の配列のような Tuple 型宣言ができる。  
複数型が含まれた変数から、丸括弧で各変数を取得することができる。

```
let tup = (500, 6.4, 1);
let (x, y, z) = tup;
```

tuple 内の値は JavaScript の配列の参照のように、index 番号をドットでつなげることで参照可能になる。

```
let tmp2: (i32, f64, u8) = (500, 6.4, 1);
let tmp2_0 = tmp2.0;
```

### Array

複数の型を指定できるものとして、Tuple の他に Array が存在する。  
Tuple との違いは、Array はすべて同じ型の複数型になることだ。  
TypeScript の Array 型とほぼ同じ扱いになる。

Rust では Array 内の存在しない index 番号を参照しようとするとランタイムエラーが発生する。  
これは Rust がメモリセーフな設計になっている証左でもある。
