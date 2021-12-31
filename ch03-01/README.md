## MEMO

- mutable な変数を宣言するときは `let mut variable = 1`
- compile は通るが、VSCode で `cannot assign twice to immutable variablerustc(E0384)` エラーが出る
- const で定数宣言をできる
  - 変数名は大文字
- `{}`ブロック内でブロック外の変数を参照することは可能
- `{}`ブロック内で宣言した変数はブロック外の同盟変数に影響を与えない
