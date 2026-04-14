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

## [Tipos de Dados](https://doc.rust-lang.org/book/ch03-02-data-types.html)

Rust Necessariamente exige a tipagem de variaveis

### [Tipos escalares](https://doc.rust-lang.org/book/ch03-02-data-types.html#scalar-types)

Tipos escalares representam um único valor, dos quais são quatro primários: `inteiros`, `números de ponto flutuantes`, `booleanos` e `caracteres`.

#### [Tipos Inteiros](https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-types)

Um inteiro é um número sem componente fracionário

| Tamanho                       | Assinado | Não Assinado |
| :---------------------------- | :------: | -----------: |
| **8 bits**                    |   `i8`   |         `u8` |
| **16 bits**                   |  `i16`   |        `u16` |
| **32 bits**                   |  `i32`   |        `u32` |
| **64 bits**                   |  `i64`   |        `u64` |
| **128 bits**                  |  `i128`  |       `u128` |
| **Dependente da arquitetura** | `isize`  |      `usize` |

Cada variante pode ser assinada ou não assinada e tem um tamanho explícito. Assinado e não assinado consulte se é possível que o número seja negativo.

- **Assinado:** É basicamente o número que pode ser negativo, portanto pode levar sinal +/-
- **Não Assinado:** É o númerio inteiro que não pode ser negativo, logo, não pode levar +/-

Os números assinados são armazenados usando [complemento de dois](https://pt.wikipedia.org/wiki/Complemento_para_dois).

- Os valores declarados com `i` precisam abranger números negativos, portanto terão uma quantidade menor de números possiveis que `u`, por exemplo: `i8` consegue abrange (−(2^7) até 2^7 − 1), ou seja, -128 até 127. Sendo que, `u8` vai de 0 até 255, pois ele é representado pelo seguinte 2^8 − 1.

- Além disso, o `isize` e `usize` os tipos dependem da arquitetura do computador em que seu programa está sendo executado: 64 bits se você estiver em uma arquitetura de 64 bits e 32 bits se você estiver em uma arquitetura de 32 bits.

| Tipos de Inteiros |      Exemplos |
| :---------------- | ------------: |
| Decimal           |      `98_123` |
| Hexadecimal       |      `0xffa4` |
| Octal             |        `0o77` |
| Binário           | `0b1111_0000` |
| Byte(`u8` apenas) |        `b'A'` |

A `_` pode ser usada como um separador, apenas para ajudar na leitura.

#### [Tipos de pontos flutuantes](https://doc.rust-lang.org/book/ch03-02-data-types.html#floating-point-types)

São para números com virgula, basicamente. O padrão deles é ser `f64`, ou seja, com `64 bits` de tamanho. Todos os _floats_ são assinados (podem ser negativos).

```rust
let x = 2.0; // f64

let y: f32 = 3.0; // f32
```

- É possivel fazer diferentes tipos de [operações matematicas](https://doc.rust-lang.org/book/appendix-02-operators.html) no Rust, tanto com **inteiros** quanto com **_Floats_**

#### [O Tipo Booleano](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-boolean-type)

Pode ser apenas `true` ou `false`, tem um byte (8 bits) de tamanho e é definido com a sintaxe `bool`.

```rust
    let t = true;

    let f: bool = false; // Com anotação explicita
```

#### [O tipo caractere](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-character-type)

É um único caractere [**Unicode**](https://home.unicode.org/), tem 4 bytes de tamanho, pode ser definido como `char` e precisa está entre aspas simples `''`.

```rust
let c = 'z';
let z: char = 'ℤ'; // Com anotação explicita
let heart_eyed_cat = '😻';
```

#### [Tipos Compostos](https://doc.rust-lang.org/book/ch03-02-data-types.html#compound-types)

## Funções

## Comentários

## Fluxo de controle
