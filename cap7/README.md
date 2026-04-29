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