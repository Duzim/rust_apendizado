# [Tratamento de erros](https://doc.rust-lang.org/book/ch09-00-error-handling.html)

Rust agrupa erros em duas categorias principais: 
- **Recuperáveis**,  como um arquivo não encontrado, do qual só queremos relatar o problema ao usuário e tentar novamente a operação.
- **Irrecuperáveis**, são sempre sintomas de bugs, como tentar acessar um localização além do final de uma matriz e, portanto, queremos interromper imediatamente o programa.

## [Erros irrecuperáveis com panic!](https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html#unrecoverable-errors-with-panic)

A macro `panic!` para a execução do código, fazendo o programa parar de rodar.

> [Desenrolando a pilha ou abortando em resposta a um pânico](https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html#unwinding-the-stack-or-aborting-in-response-to-a-panic): Ao causar um pânico o rust limpa a `stack`, mas permite que não faça isso. Com a seguinte configuração no `Cargo.toml`. 
```toml
[profile.release]
panic = 'abort'
```

## [Erros recuperáveis com `Result`](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)

Os erros recuperáveis são aquele que se quer tomar uma medida protetiva em seu programa, onde não tem a necessidade de parar totalmente o programa.

O `Result` é um `enum` com 2 estados diferentes, `Ok` e `Err`, representando sucesso e erro, respectivamente. Do qual tem o `T` e `E` são parâmetros de tipo genéricos. `T` representa o tipo do valor que será retornado em um caso de sucesso dentro do `Ok`, e `E` representa o tipo de erro que será retornado em a caso de falha dentro do `Err`.

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### [Correspondência em diferentes erros](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#matching-on-different-errors)

No seguinte código há um tratamento de erro um pouco mais avançado, pois ao não encontrar o arquivo, criará o mesmo. Só realizará um `panic!` se não tiver permissão ou outro erro.

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };
}
```
#### [Alternativas ao uso `match` com `Result<T, E>`](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#alternatives-to-using-match-with-resultt-e)

O seguinte código basicamente _desmpacota_ ou faz outra coisa com o método `unwrap_or_else`. Esse _desmpacotamento_ será mais bem explorado em sessões seguintes. 

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
}
```

#### [Atalhos para pânico em caso de erro](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#shortcuts-for-panic-on-error)

Os métodos de _desempacotamento_ como `unwrap` serve como um atralho de `match`, onde diz que: "Eu tenho certeza absoluta de que há um valor válido dentro desta caixa. Me dê esse valor agora. Se eu estiver errado, pode explodir (crashar) o programa". Ao quebrar o programa queremos dizer que, ocorrerá um `panic!`.

```rust
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap();
}
```

- **Ex.:** com `Option` e com `Result`, ambos podem ser _desempacotados_.

> Com `Option`
```rust
let meu_numero: Option<i32> = Some(10);
let numero_ausente: Option<i32> = None;

// Sucesso: Retorna o número 10
let valor = meu_numero.unwrap(); 

// PÂNICO (Crash!): O programa é encerrado imediatamente com um erro, 
// pois tentamos fazer unwrap em um None.
let valor_erro = numero_ausente.unwrap();
```

> Com `Result`

```rust
// Imaginemos uma função que tenta converter um texto em número
let sucesso: Result<i32, _> = "42".parse();
let falha: Result<i32, _> = "texto_invalido".parse();

// Sucesso: Retorna o número 42
let numero = sucesso.unwrap();

// PÂNICO (Crash!): O programa é encerrado porque tentamos
// fazer unwrap em um Err (houve falha na conversão).
let erro = falha.unwrap();
```

O `expect` tem uma perspectiva parecida, mas permite uma mensagem de debug

```rust
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}
```

Outra maneira de tratar esse erro é com `unwrap_or`

```rust
let numero = falha.unwrap_or(0); // Se falhar, usa 0
```

### [Propagando Erros](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#propagating-errors)

Quando a implementação de uma função chama algo que pode falhar, em vez de lidando com o erro dentro da própria função, você pode retornar o erro para que dopis código para que ele retorna possa decidir o que fazer. Isto é conhecido como propagação de erro e dá mais controle ao código de chamada, onde pode haver mais informação ou lógica que determina como o erro deve ser tratado do que o que você tem disponível no contexto do seu código.

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
```

#### [O `?` Atalho do operador](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#the--operator-shortcut)

O código abaixo faz exatamente a mesma coisa que o código anterior com o atalho `?`, onde no seguinte código ou retorna o caso positivo, ou o próprio erro como o `return Err(e)`

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
```

Valores de erro que têm o operador `?` chamado eles passam pela função `from`, que é usada para converter valores de um tipo para outro.

Quando o `?` o operador chama o from função, o tipo de erro recebido é convertido no tipo de erro definido no tipo de retorno da corrente função. Isso é útil quando uma função retorna um tipo de erro para representar todas as maneiras pelas quais uma função pode falhar, mesmo que as partes possam falhar em muitas situações diferentes razões.

Utilizando o `?` descaradamente:

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}
```

O código acima ainda faz a mesma coisa, mas se torna mais legivel.
Onde cada um dos `?`s sairá da função com um erro ou continuará a _pipeline_, no final retornando o `OK` com uma `String`.

Encurtando ainda mais

```rust
use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
```

Como ler um arquivo é um processo bem comum, a biblioteca padrão faz isso automanticamente para nós.

#### [Onde usar o operador `?`](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#where-to-use-the--operator)

função que retorna um tipo compatível.

- Se você usar `?` em um `Result`, a sua função precisa retornar `Result`.

- Se você usar `?` em um `Option`, a sua função precisa retornar `Option`.

Você não pode usar `?` em um Option dentro de uma função que retorna `Result` de forma direta, porque o compilador não sabe como transformar o `None` em um Erro específico (`Err`).

Se você está em uma função que retorna `Result`, mas precisa usar o `?` em um `Option`, você usa o método `ok_or()` ou `ok_or_else()` para transformar o `Option` em `Result` antes de aplicar o `?`:

```rust
fn buscar_configuracao() -> Result<String, String> {
    let variavel_ambiente: Option<String> = std::env::var("MINHA_CONFIG").ok();

    // Transformamos o Option em Result.
    // Se for None, vira um Err("Variável não encontrada").
    // Aí sim podemos usar o `?`
    let valor = variavel_ambiente.ok_or("Variável não encontrada")?;

    Ok(format!("Configuração carregada: {}", valor))
}
```