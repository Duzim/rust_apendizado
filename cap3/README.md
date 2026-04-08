# [Conceitos Comuns de Programação](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html)

## [Variavei e mutabilidade](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)

```rust
let a = 5; // Variavel imutável
a = 6; // -> Vai dar erro, por tentar reatribuição

let mut b = 4; //Variavel mutável
b = 8; //Reatribui valor sem problemas
```

### [Declaração Contstantes](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#declaring-constants)

O Rust além da variaveis imutaveis há também constantes, que se assemelham com variaveis definidas por `let`, mas não podem ser atribuidas com `mut`, permanecem as mesmas durante toda a execução do codigo. Além de precisar de uma definição de tipo, como `: i32` e precisam ser declaradas maiusculo.

```rust
const CONSTANTE_RUST: u32 = 20 * 20;
```

### [Sombreamento](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing)

Sombramento é a criação de uma nova variavel utilizando o nome da anterior, basicamente pode permitir mudar o tipo do valor associado aquele nome e também a variavel definina num escopo não foge dele. Premitindo o seguinte codigo:

```rust
let x = 5;
let x = x + 5; // x = 10
{
    let x = x + 5; //x = 15
}
println!(x) // x = 10
```

## Tipos de Dados

## Funções

## Comentários

## Fluxo de controle
