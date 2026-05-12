# Tipos genéricos(`Generic Types`), características(`Traits`) e tempos de vida(`Lifetimes`)

Genéricos (`Generic Types`) são úteis para ter "tipos comuns", basicamente como o "x" ou "y" funciona na matematica, o genérico funciona para qualquer tipo.

Genéricos são úteis para evitar duplicatas de funções ou outros códigos que precisariam ser reescritos na ausencia de genéricos.

Existem também as características (`Traits`) que é um comportamento padrão de algo ou um genérico. Como dizer ao compilador: "Use esse qualquer tipo e valor como um número", também pode ser extrapolado para outras coisas, como a `trait Copy` que copia algo para a memória `stack` mesmo que naturalmente ficasse na memória `heap`.

Há também os tempos de vida (`Lifetimes`), basicamente é o tempo de emprestimo. Com isso podemos ajudar o compilador a decidir que vai manter um valor emprestado por mais tempo ou não.

## [Tipos de dados genéricos](https://doc.rust-lang.org/book/ch10-01-syntax.html)

### [Em Definições de Funções](https://doc.rust-lang.org/book/ch10-01-syntax.html#in-function-definitions)
Um bom exemplo para a utilização de Genéricos é com o seguintem, temos duas funçõe que faze essencialmente a mesma coisa, mas com tipos diferentes:

```rust
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {result}");
}
```

Com os Genéricos podemos reduzir a um única função que ainda abrage uma variedade maior de tipos: 

```rust
use std::cmp::PartialOrd;

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest_val = &list[0];

    for item in list {
        if item > largest_val {
            largest_val = item;
        }
    }
    largest_val
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {result}");
}
```

Precisamos do `PartialOrd` para dezer ao compilador que será um valor comparável, permitindo a comparação com `>`, é basicamente adicionar uma _característica_ ao seu tipo.

Mas caso não houvesse essa comparação não seria necessário adicionar essa _característica_, ou talvez fosse necessário adicionar outra.

### [Em Definições de Estruturas](https://doc.rust-lang.org/book/ch10-01-syntax.html#in-struct-definitions)

### [Em Definições Enum](https://doc.rust-lang.org/book/ch10-01-syntax.html#in-enum-definitions)

### [Em Definições de Métodos](https://doc.rust-lang.org/book/ch10-01-syntax.html#in-method-definitions)

### [Desempenho do código usando genéricos](https://doc.rust-lang.org/book/ch10-01-syntax.html#performance-of-code-using-generics)
