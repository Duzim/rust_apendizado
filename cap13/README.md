# [Recursos de Linguagem Funcional: Iteradores e Closures](https://doc.rust-lang.org/book/ch13-00-functional-features.html#functional-language-features-iterators-and-closures)

## [Closures](https://doc.rust-lang.org/book/ch13-01-closures.html)

As *`closures`* do Rust são funções anônimas que você pode armazenar em uma variável ou passar como argumentos para outras funções. Você pode criar a *`closure`* em um ponto e, posteriormente, invocá-la em outro local para executá-la em um contexto diferente. Ao contrário das funções, as *`closures`* podem capturar valores do escopo em que foram definidas. Demonstraremos como essas características das *`closures`* possibilitam a reutilização de código e a personalização de comportamento.

### [Capturando o ambiente](https://doc.rust-lang.org/book/ch13-01-closures.html#capturing-the-environment)

```rust
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}
impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_prefix1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_prefix1);

    println!(
        "The user with preference {:?} gets {:?}",
        user_prefix1, giveaway1
    );

    let user_prefix2 = None;

    let giveaway2 = store.giveaway(user_prefix2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_prefix2, giveaway2
    );
}
```

No trecho de código um exemplo de utilização do `closures` no `unwrap_or_else` do qual ao resultado ser um `None` (uma das possibilidades de `Option`) ele executa o trecho `|| {}` assim como uma callback do javascript sendo práticamente equivalentes o trecho `|| {...}` com o seguinte `() => {...}`.

### [Inferência e Anotação de Tipos de Fechamento](https://doc.rust-lang.org/book/ch13-01-closures.html#inferring-and-annotating-closure-types)

Os `closures` permitem não anotar o tipos de retorno, permitindo uma sintaxe mais simple, inferindo com base no retorno da função.
Como podemos ver no seguinte

```rust
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

podemos executar um `closures` como uma função comum

```rust
let example_closure = |x| x;

let n = example_closure(5);
```

### [Captura de referências ou mover a propriedade](https://doc.rust-lang.org/book/ch13-01-closures.html#capturing-references-or-moving-ownership)

`Closures` podem capturar valores de seu ambiente de três maneiras, que correspondem diretamente às três formas de uma função receber um parâmetro: empréstimo imutável, empréstimo mutável e transferência de propriedade. A `closure` decidirá qual dessas formas utilizar com base no que o corpo da função faz com os valores capturados.


- Podemos ter múltiplas referências imutáveis, com o seguinte uso de uma variavel.
```rust
fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");
}
```

- O fechamento agora captura uma referência mutável.
```rust
fn main() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {list:?}");
}
```

Quando `borrows_mutably` é definida, ela captura uma referência mutável para `list`. Não utilizamos a *closure* novamente após ela ser chamada, portanto, o empréstimo mutável é encerrado. Entre a definição e a chamada da *closure*, não é permitido um empréstimo imutável para impressão, pois nenhum outro empréstimo é permitido quando existe um empréstimo mutável. Tente adicionar um `println!` nesse ponto para ver qual mensagem de erro você recebe!



- Transferência de propriedade, Podemos forçar a transferencia de propriedade com a sintaxe `move` antes da *closure*.

```rust
let list = vec![1, 2, 3];
println!("Antes da definição do closure: {list:?}");

thread::spawn(move || println!("Vem do thread: {list:?}"))
    .join()
    .unwrap();
```

### [Movendo valores capturados fora de fechamentos](https://doc.rust-lang.org/book/ch13-01-closures.html#moving-captured-values-out-of-closures)