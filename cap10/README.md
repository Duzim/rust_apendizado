# [Tipos genéricos(`Generic Types`), características(`Traits`) e tempos de vida(`Lifetimes`)](https://doc.rust-lang.org/book/ch10-00-generics.html)

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

Também podemos definir structs para usar um parâmetro de tipo genérico em um ou mais campos usando o `<>`.

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
```

Para cada tipo declarado é preciso associar um genérico, se quissemos usar um tipo diferente em `x` e `y` precisariamos de mais um genérico como `U`.

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}
```

### [Em Definições Enum](https://doc.rust-lang.org/book/ch10-01-syntax.html#in-enum-definitions)

Como fizemos com structs, podemos definir enums para armazenar tipos de dados genéricos em seus variantes. Vamos dar outra olhada no enum `Option<T>` que o padrão a biblioteca fornece

```rust
enum Option<T> {
    Some(T),
    None,
}
```

Da qual podemos ver agora que o `T` como visto empiricamente anteriormente, pode assumir qualquer valor de qualquer tipo.

O mesmo se dá a `Result<T, E>`:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Do quak tem dois tipos genéricos `T` e `E` um reprensentando o valor de caso de sucesso e outro de erro, respectivamente.

### [Em Definições de Métodos](https://doc.rust-lang.org/book/ch10-01-syntax.html#in-method-definitions)

O mesmo que aplicamos a `structs` e funções também vale para metódos

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
```

Também pode ser feito algo assim:

```rust
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
```

onde ele cria uma nova instância de `Point` com os determinados tipos de cada um dos pontos.

### [Desempenho do código usando genéricos](https://doc.rust-lang.org/book/ch10-01-syntax.html#performance-of-code-using-generics)

A boa notícia é que usar tipos genéricos não fará seu programa corra mais devagar do que faria com tipos concretos.

Rust consegue isso realizando a _monomorfização_ do código usando genéricos em tempo de compilação. _Monomorfização_ é o processo de tornar genérico código em código específico preenchendo os tipos concretos que são usados quando compilado.

Algo análogo ao seguinte:
```rust
let integer = Some(5);
let float = Some(5.0);
```
Onde ele cria codígos que atendem a cada tipo que `Option<T>` deveria responder, no caso `i32` e `f64`.

O código _monomorfização_ seria parecido com o seguinte:

```rust
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```

## [Definindo comportamento compartilhado com características (`traits`)](https://doc.rust-lang.org/book/ch10-02-traits.html)

Um característica (`traits`) define a funcionalidade que um determinado tipo possui e com a qual pode compartilhar outros tipos. Podemos usar características para definir o comportamento compartilhado de forma abstrata. Nós podemos usar limites de características para especificar que um tipo genérico pode ser qualquer tipo que tenha certo comportamento.

Pense na **Trait do Rust** como uma habilidade que você pode "ensinar" a qualquer tipo de dado (mesmo os que já existem), e que o compilador usa para gerar um código otimizado como se aquela habilidade fosse nativa.

> **Nota:** As características são semelhantes a uma característica frequentemente chamada `interfaces` em outro línguas, embora com algumas diferenças.

### [Definindo uma `trait`](https://doc.rust-lang.org/book/ch10-02-traits.html#defining-a-trait)

As características são uma maneira de agrupar assinaturas de métodos para definir um conjunto de comportamentos necessários para atingir algum propósito.

Para criar um características usamos a palávra `trait`

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

 Cada tipo que implementa esta característica deve fornecer seu próprio comportamento personalizado para o corpo do método. O compilador aplicará que qualquer tipo que tenha o `trait` `Summary` terá o método `summarize` definido exatamente com esta assinatura.

### [Implementando uma característica em um tipo](https://doc.rust-lang.org/book/ch10-02-traits.html#implementing-a-trait-on-a-type)

Implementar uma característica em um tipo é semelhante a implementar métodos regulares. O a diferença é que depois `impl`, colocamos o nome da característica que queremos implementar, então usamos o `for` e, em seguida, especifique o nome do tipo que queremos implementar a característica para. Dentro do `impl` bloco, colocamos as assinaturas do método que a definição da característica definiu. Em vez de adicionar um ponto e vírgula após cada um assinatura, usamos colchetes encaracolados e preenchemos o corpo do método com o específico comportamento que queremos que os métodos da característica tenham para o tipo específico.

```rust
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

Também podemos implementar características em coisas comuns do rust como o seguinte:

```rust
// Definimos nosso contrato (Trait)
trait Gritar {
    fn gritar(&self) -> String;
}

// Implementamos para o tipo i32 (número inteiro padrão do Rust)
impl Gritar for i32 {
    fn gritar(&self) -> String {
        format!("EU SOU O NÚMERO {}!!!", self)
    }
}

fn main() {
    let numero = 42;
    println!("{}", numero.gritar()); // Imprime: EU SOU O NÚMERO 42!!!
}
```

### [Usando implementações padrão](https://doc.rust-lang.org/book/ch10-02-traits.html#using-default-implementations)

