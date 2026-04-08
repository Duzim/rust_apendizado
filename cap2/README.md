# [Jogo de Adivinhação](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)

Em rust as variaveis são imutaveis por natureza, a não ser que seja definido que serão mutaveim com o termo `mut`.

```rust
let apples = 5; // immutable
let mut bananas = 5; // mutable
```

O termo `&` diz que alguma coisa é uma referencia, e `&mut` que essa referencia é mutável.

```rust
&bananas //Referencial imutável

&mut bananas //Referencia mutável
```
