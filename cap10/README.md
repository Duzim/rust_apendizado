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

Podemos também adidionar um comportamento à um característica mesmo sem sempre precisar refazer o corpo do metódo.

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

impl Summary for NewsArticle {}

fn main() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
}

```

Basicamente adicionamos um comportamento sem a necessidade de sobrescrever o metodo.

E ambas as formas podem existir simultaneamente em uma única `trait`.

```rust
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

impl Summary for SocialPost {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
```

### [Usando `Traits` como Parâmetros](https://doc.rust-lang.org/book/ch10-02-traits.html#using-traits-as-parameters)

Como pode-se ver no seguinte bloco de código, podemos usar `Traits` como parâmetros.

Qualquer tipo que implemente essa `Trait` pode ser utilizada, sem precisar ser especificada no parâmetro. Podemos utilizar os métodos padrões a essa `Trait`, como no caso o `summarize`.


```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

#### [`Trait` Ligado Sintaxe](https://doc.rust-lang.org/book/ch10-02-traits.html#trait-bound-syntax)

O caso mais comum de ser utilizado é da seguinte forma

```rust
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```
Em sintese é tem a mesma função de fazer o mesmo da sessão [anterior](#usando-traits-como-parâmetros).

Utilizar essa sitaxe é mais simples em alguns casos, como quando temos mais de um parâmetro, do mesmo tipo.

Ex.: usando `impl Trait`
```rust
pub fn notify(item1: &impl Summary, item2: &impl Summary) {
```
De outra forma,
```rust
pub fn notify<T: Summary>(item1: &T, item2: &T) {
```


#### [Limites de Múltiplos Traits com a Sintaxe `+`](https://doc.rust-lang.org/book/ch10-02-traits.html#multiple-trait-bounds-with-the--syntax)

Usando mais de uma `Trait` com `+`, nesse caso, o `item` deve implementar ambas as `Traits`.

```rust
pub fn notify(item: &(impl Summary + Display)) {
```
ou
```rust
pub fn notify<T: Summary + Display>(item: &T) {
```

#### [Restrições de Trait mais claras com cláusulas `where`](https://doc.rust-lang.org/book/ch10-02-traits.html#clearer-trait-bounds-with-where-clauses)

Exite uma desvantagem de usar múltiplas `Traits` como patâmetro, a difícil leitura da sintaxe. Como pode ser visto no seguinte:

```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
```

podemos usar o `where` para deixar a sintaxe mais tragavel, como da seguinte forma

```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
```

### [Retornando tipos que implementam traits](https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits)

Também podemos usar a sintaxe `impl Trait` na posição de retorno para retornar a valor de algum tipo que implementa um traço, como mostrado aqui:

```rust
fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    }
}
```

No caso acima o a função `returns_summarizable` retorna o `SocialPost`.

> Nota: Não é possível retornar mais de um tipo apenas por exemplo retornar `SocialPost` ou `NewsArticle`

### [Usando Restrições de Trait para Implementar Métodos Condicionalmente](https://doc.rust-lang.org/book/ch10-02-traits.html#using-trait-bounds-to-conditionally-implement-methods)

Traços e limites de traço nos permitem escrever código que usa parâmetros de tipo genérico para reduzir a duplicação mas também especificar ao compilador que queremos o genérico tipo para ter um comportamento particular. O compilador pode então usar o limite de traço informações para verificar se todos os tipos de concreto utilizados com o nosso código fornecem o comportamento correto. Em linguagens digitadas dinamicamente, obteríamos um erro em runtime se chamássemos um método em um tipo que não definisse o método. Mas Ferrugem move esses erros para o tempo de compilação para que sejamos forçados a corrigir os problemas antes mesmo do nosso código ser capaz de ser executado. Além disso, não precisamos escrever código que verifica o comportamento em tempo de execução, porque já verificamos em tempo de compilação. Fazer isso melhora o desempenho sem ter que abrir mão da flexibilidade de genéricos.

> Basicamente permite usar `Traits` de forma condicional, com uma especie de polimorfismo (_É apenas uma analogia_).

```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```

## [Validando referências com tempos de vida (`Lifetimes`)](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#validating-references-with-lifetimes)

Toda referência em Rust tem uma vida inteira, que é o escopo para o qual essa referência é válida. Na maioria das vezes, as vidas são implícitas e inferidas, assim como na maioria das vezes, os tipos são inferidos.