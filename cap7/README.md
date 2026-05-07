# [Packages, Crates, e Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
Um _crate_(caixa) é a menor quantidade de código que o compilador Rust considera, ou seja, o compilador considera esse arquivo uma caixa. As caixas podem contêm módulos, e os módulos podem ser definidos em outros arquivos que obtêm compilado com a caixa.



**Crates:** Uma crate pode ser apenas um arquivo, mas ela também pode ser expandida para múltiplos arquivos usando módulos. O compilador sempre começa lendo o arquivo raiz e puxa os outros arquivos a partir dele, tratando tudo como uma única crate.

**Package:** Um Package (Pacote) é uma funcionalidade do Cargo (o gerenciador de pacotes e build system do Rust). Pense no package como a pasta principal do seu projeto.

Um package é um conjunto de uma ou mais crates que fornecem uma funcionalidade em conjunto. A característica principal de um package é que ele sempre contém um arquivo `Cargo.toml`, que descreve como construir essas crates.

As regras de um Package são rígidas:

- Um package deve conter pelo menos uma crate (seja biblioteca ou binária).

- Um package pode conter no máximo uma library crate (`src/lib.rs`).

- Um package pode conter várias binary crates (colocando arquivos extras na pasta `src/bin/`).

**Modulo:** Basicamente podemos entender um módulo é um espaço lógico (um namespace), enquanto um arquivo é um espaço físico no seu computador.

Podemos dizer que: 

- Pacotes: Um recurso do Cargo que permite construir, testar e compartilhar caixas
- Caixas: Uma árvore de módulos que produz uma biblioteca ou executável
- Módulos e uso: Permite que você controle a organização, o escopo e a privacidade de caminhos
- Caminhos: Uma maneira de nomear um item, como uma estrutura, função ou módulo

## [Pacotes(Packages) e Caixas(Crates)](https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html)

**Crate (caixa):** Um caixa é a menor quantidade de código que o compilador Rust considera em a tempo. Mesmo que você corra rustc em vez de cargo e passar um único código fonte arquivo, o compilador considera esse arquivo uma caixa. As caixas podem conter módulos, e os módulos podem ser definidos em outros arquivos que obtêm compilado com a caixa.

Uma caixa pode vir em uma de duas formas: 

- **binária:** Caixas binárias são programas que você pode compilar em um executável que você pode executar, como um programa de linha de comando ou um servidor. Cada um deve ter uma função chamada `main` isso define o que acontece quando o executável é executado. Todas as caixas que temos criadas até agora foram caixas binárias.

- **biblioteca:** não tenho um função `main`, e eles não compilam para um executável. Em vez disso, eles definem a funcionalidade destinada a ser compartilhada com vários projetos. Na maioria das vezes quando se diz uma `Crate`, se refere a esse tipo de _Crate Biblioteca_.

Um **Packages** é um pacote de uma ou mais caixas que fornece um conjunto de funcionalidade. Um pacote contém a Carga.toml arquivo que descreve como construa essas caixas. Cargo é na verdade um pacote que contém a caixa binária para a ferramenta de linha de comando que você tem usado para construir seu código.

```
meu_projeto/             <-- Isso é o PACKAGE (a pasta inteira)
├── Cargo.toml           <-- O manifesto que define o package
└── src/
    ├── main.rs          <-- Isso é uma CRATE (Binária)
    └── lib.rs           <-- Isso é outra CRATE (Biblioteca)
```

## [Controle o escopo e a privacidade com módulos](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html)

Teremos de ter em mente as seguintes sintaxes `use`, `pub` e `as`.

### `mod` (Construindo a Árvore)

A palavra `mod` serve para declarar a existência de um módulo. Ela não importa um código de outro lugar; ela avisa ao compilador: "Ei, eu tenho uma seção de código aqui que deve ser tratada como um namespace separado".

### `pub` (Abrindo as Portas)
    
Em Rust, a privacidade é rígida: tudo é privado por padrão.

Se você cria uma função dentro de um módulo, apenas o próprio módulo (e os módulos "filhos" que ele criar) podem usar essa função. O módulo "pai" ou outros módulos externos não conseguem acessá-la. É aqui que entra o `pub` (público).

Você usa pub para expor itens para o mundo exterior. Você pode colocar pub em:

- **Funções:** `pub fn executar() {}`

- **Structs:** `pub struct Usuario {}`

- **Campos de Structs:** Atenção aqui! Mesmo se a struct for pública, seus campos são privados por padrão. Você precisa de `pub id: u32` se quiser que alguém acesse esse campo diretamente.

- **Módulos:** `pub mod banco_de_dados;` (Se o módulo for privado, de nada adianta as funções dentro dele serem públicas, pois ninguém de fora conseguirá entrar no módulo)

### `as` (O Apelidador)

A palavra `as` tem dois usos principais no Rust, mas no contexto de organização de código (com a palavra `use`), ela serve para criar um alias (apelido) para algo que você está trazendo para o escopo.

Isso é extremamente útil em duas situações:

1. **Evitar conflitos de nomes:** Se você usar duas bibliotecas que têm um tipo chamado `Error` (ex: `std::io::Error` e `std::fmt::Error`), você pode importar uma delas com outro nome usando `as`.

2. **Encurtar nomes longos:** Para não ter que digitar nomes muito extensos repetidas vezes.

- **Obs:** O `as` também é usado para "casting" (conversão) de tipos primitivos numéricos, como transformar um `i32` num `f64` fazendo `let x = 10 as f64;` ).

---

```rust
// 1. Usamos 'mod' para DECLARAR a existência de um módulo.
// Como ele está no mesmo arquivo, abrimos as chaves.
mod sistema_de_arquivos {
    
    // O módulo 'arquivos_locais' precisa ser 'pub', senão o 'main' não consegue entrar nele.
    pub mod arquivos_locais {
        
        // A função também precisa ser 'pub' para ser usada de fora.
        pub fn ler_arquivo_texto() {
            println!("Lendo arquivo...");
        }

        // Função privada. O 'main' não consegue ver isso!
        fn checar_permissao() {
            println!("Checando permissões internas...");
        }
    }
}

// 2. Usamos 'use' para trazer o caminho para o escopo atual,
// e usamos 'as' para dar um APELIDO mais curto e amigável.
use sistema_de_arquivos::arquivos_locais::ler_arquivo_texto as ler;

fn main() {
    // 3. Agora podemos chamar a função apenas pelo apelido!
    ler(); 
    
    // Se tentássemos chamar sistema_de_arquivos::arquivos_locais::checar_permissao(); 
    // O compilador daria um erro de privacidade (pois não tem 'pub').
}
```

## [__Paths__ para referência a um item na árvore de módulos](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html)

Para mostrar ao Rust onde encontrar um item em uma árvore de módulos, usamos um caminho no mesmo maneira como usamos um caminho ao navegar em um sistema de arquivos.

Um caminho pode assumir duas formas:

- Um caminho absoluto é o caminho completo começando a partir de uma raiz de caixa; para código de uma caixa externa, o caminho absoluto começa com o nome da caixa e, para código da caixa atual, começa com o literal `crate`.

- Um caminho relativo começa a partir do módulo atual e usa `self`, `super`, ou um identificador no módulo atual.

Os caminhos absolutos e relativos são seguidos por um ou mais identificadores separados por dois pontos duplos (`::`).

### [Expondo Caminhos com a Palavra-chave `pub`](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#exposing-paths-with-the-pub-keyword)

Basicamente o `pub` torna algo em seu modulo publico, naturalmente todas as coisas dentro de um modulo são privadas. 

```rust
pub mod modulo_1 {
    pub fn funcao(text: &str) {
        println!("Função que faz algo [{}]", text);
    }
}
```

Como mostrado acima, a função e modulo estão publicos e podem ser utilizados fora de seu escopo, respeitando oque já foi dito sobre importação de modulos. Podendo sem usado em outro arquivo ou em outro contexto com:

```rust
mod nome_do_arquivo; //em caso se fosse em outro arquivo

crate::nome_do_arquivo::modulo_1::funcao();//pode também ser utilizado o caminho relátiuvo.
```

### [Iniciando caminhos relativos com `super`](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#starting-relative-paths-with-super)

O `super` é basicamente um `../` na navegação de arquivos e pastas, mas aplicado a modulos, permitindo andar para trás em modulos.

```rust
pub fn funcao_no_topo() {
    println!("Chegamos ao topo!");
}

//Sim, isso é possivel (mesmo que não recomendado)
pub mod nivel_1 {
    pub mod nivel_2 {
        pub mod nivel_3 {
            pub mod nivel_4 {
                pub fn tentar_acessar_o_topo() {
                    // Subindo 4 níveis!
                    super::super::super::super::funcao_no_topo();
                }
            }
        }
    }
}
```

O código acima mostra um uso excessivo do `super`, nesse caso seria recomendado utilizar o `crate`. Porém nesse caso seria o mesmo que dizer em uma navegação de diretorios `../../../../funcao_no_topo` subindo 4 níveis de diretorio.

### [Tornando `Structs` e `Enums` Públicos](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#making-structs-and-enums-public)

Quanto a `struct` e `enum`, caso usar `pub` antes da definição dos mesmo, ele será público, mas seus campos ainda serão privados em um `struct`. Deve-se útilizar `pub` em cada um dos campos públicos.

```rust
pub struct Customer {
    pub nome: String,
    idade: i8,
}
```

No caso acima, `idade` é privado.

Em contraste, se tornarmos público um `enum`, todas as suas variantes serão públicas. Nós só precisa do `pub` antes do `enum`

```rust
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```
## [Trazendo Caminhos ao Escopo com a Palavra-chave `use`](https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html)

O `use` é basicamente a criação de um _namespace_, sem a necessidade de todas as vezes ter que expecificar todo o caminho até a oque se deseja utilizar.

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    add_to_waitlist();
}
```

### [Utilizando a palavra-chave `as`](https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#providing-new-names-with-the-as-keyword)

É basicamente a renomeação de algo. Como o seguinte:

```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
```

### [Reexportando nomes com `pub use`](https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#re-exporting-names-with-pub-use)

Quando colocamos um nome no escopo com o use palavra-chave, o nome é privado para o escopo para o qual o importamos. Para permitir que código fora desse escopo faça referência a esse nome como se tivesse sido definido nesse âmbito, podemos combinar `pub` e `use`. Esta técnica é chamada _reexportando_ porque estamos trazendo um item no escopo, mas também disponibilizando esse item para outros trazerem para seus escopo.

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

Permite utilizar o `hosting` como se fosse dominio do arquivo que usa o `pub use`.

### [Usando pacotes externos](https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#using-external-packages)

Usando `crates` baixados como o `rand`, basta utilizar o `use` apenas adicionando aquele _namespace_ a seu escopo.

```rust
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
}
```

### [Usando caminhos aninhados com listas no `use`](https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#using-nested-paths-to-clean-up-use-lists)

Ao invés de ter varia importações com o use de um mesmo modulo, pode-se aninha-las com `{}`.

Sem aninhamento:
```rust
use std::cmp::Ordering;
use std::io;
```

Com aninhamento: 

```rust
use std::{cmp::Ordering, io};
```

Pode-se utilizar qualquer nivel, mesmo de um mesmo módulo:

```rust
use std::io;
use std::io::Write;
```

Com aninhamento: 

```rust
use std::io::{self, Write};
```

### [Importando Itens com o Operador `Glob`(`*`)](https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#importing-items-with-the-glob-operator)

Tráz tudo itens públicos definidos em um caminho para o escopo, podemos especifique esse caminho seguido pelo `*` operador glob:

```rust
use std::collections::*;
```

## [Separando módulos em arquivos diferentes](https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html)

Para utilização de varios arquivos precisa-se chamar o modulo ou oque se deseja utilizar para o seu contexto atual com `mod`. Como se importasse o contexto alheio.

```rust
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

no código acima, permite útilizar o `hosting` como se fosse daquele contexto e também utiliza o proprio `hosting`.