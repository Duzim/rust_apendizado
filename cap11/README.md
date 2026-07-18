# [Escrevendo Testes Automatizados](https://doc.rust-lang.org/book/ch11-00-testing.html#writing-automated-tests)
Testes são funções em Rust que verificam se o código que não é de teste está funcionando da maneira esperada. O corpo das funções de teste normalmente realiza estas três ações:

- Configure todos os dados ou estados necessários.
- Execute o código que deseja testar.
- Afirme que os resultados são o que você espera.

Vejamos os recursos que Rust fornece especificamente para escrever testes que executam essas ações, que incluem o atributo test, algumas macros e o atributo should_panic.

## [Estruturando funções de teste]()

Em sua forma mais simples, um teste em Rust é uma função anotada com o atributo `test`. Atributos são metadados sobre partes do código Rust; um exemplo é o atributo `derive`, que utilizamos com `structs` no Capítulo 5. Para transformar uma função em uma função de teste, adicione `#[test]` na linha anterior a `fn`. Ao executar seus testes com o comando `cargo test`, o Rust compila um binário executor de testes que executa as funções anotadas e informa se cada função de teste passou ou falhou.

```rust
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn not_works() {
        panic!("Teste que falha!");
    }
}
```

### [Verificando resultados com `assert!`](https://doc.rust-lang.org/book/ch11-01-writing-tests.html#checking-results-with-assert)

A macro `assert!` ornecido pela biblioteca padrão, é útil quando você deseja para garantir que alguma condição em um teste avalie como `true`.
Ele basicamente verifica se é `true`.

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }
}
```

### [Testando a igualdade com `assert_eq!` e `assert_ne!`](https://doc.rust-lang.org/book/ch11-01-writing-tests.html#testing-equality-with-assert_eq-and-assert_ne)

Uma maneira comum de verificar se algum resultado é ou não é igual a um valor.
`assert_eq!` para valores que devem ser iguais e `assert_ne!` para quando não devem ser iguais.

```rust
pub fn add_two(a: u64) -> u64 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }
}
```

### [Adicionando mensagens de falha personalizadas](https://doc.rust-lang.org/book/ch11-01-writing-tests.html#adding-custom-failure-messages)
Pode-se também adicionar uma mensagem  personalizada, adicionando mais um parametro nas macros já descritas.

como no seguite:

```rust
#[test]
fn greeting_contains_name() {
    let result = greeting("Carol");
    assert!(
        result.contains("Carol"),
        "Greeting did not contain name, value was `{result}`"
    );
}
```

### [Verificando se há pânico com `should_panic`](https://doc.rust-lang.org/book/ch11-01-writing-tests.html#checking-for-panics-with-should_panic)
Além de verificar os valores de retorno, é importante verificar se o nosso código lida com condições de erro como esperamos.

Testamos isso com a diretiva `#[should_panic]` como mostrado no seguinte código
```rust
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

Alguns testes com o `should_panic` podem não funcionar perfeitamente, portando especificamos cada vez mais o tipo de `panic` esperado com o `expected`. Como Mostrado no seguinte código:

```rust
// --snip--

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {value}."
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {value}."
            );
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

### [Usando `Result<T, E>` em testes](https://doc.rust-lang.org/book/ch11-01-writing-tests.html#using-resultt-e-in-tests)
Também podemos escrever testes que usam `Result<T, E>`, como podemos ver no seguinte trecho

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), String> {
        let result = add(2, 2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
```
A função `it_works`  agora tem o tipo de retorno `Result<(), String>`. No corpo da função, em vez de chamar a macro `assert_eq!`, voltamos `Ok(())` quando o teste passa e um `Err` com a `String` no interior quando o teste falha.

Você não pode usar a anotação `#[should_panic]` em testes que utilizam `Result<T, E>`. Para verificar se uma operação retorna uma variante `Err`, não utilize o operador de interrogação no valor `Result<T, E>`. Em vez disso, utilize `assert!(value.is_err())`.

> Nota: Precisa ser tipado se não o compilador vai reclamar.

## [Controlando Como Os Testes São Executados](https://doc.rust-lang.org/book/ch11-02-running-tests.html#controlling-how-tests-are-run)

### [Execução de testes em paralelo ou consecutivamente](https://doc.rust-lang.org/book/ch11-02-running-tests.html#running-tests-in-parallel-or-consecutively)
Quando você executa vários testes, por padrão, eles são executados em paralelo usando threads, significando que eles terminam de correr mais rapidamente e você recebe feedback mais cedo. Como os testes estão sendo executados ao mesmo tempo, você deve se certificar de que seus testes não dependem uns dos outros ou de qualquer estado compartilhado, incluindo um ambiente compartilhado, como o diretório de trabalho atual ou variáveis de ambiente.

Se você não quiser executar os testes em paralelo ou simplesmente um olhar mais fino pode-se usar o comando `--test-threads`

```bash
$ cargo test -- --test-threads=1
```

Definindo no número de threads para `1`, não utilizan nenhum paralelismo, assim executando um teste atrás do outro.

### [Exibindo a saída da função](https://doc.rust-lang.org/book/ch11-02-running-tests.html#showing-function-output)

utiliza-se o `print!` ou o `println!` normal e o comando `--show-output` como no seguinte:

```bash
$ cargo test -- --show-output
```

código com o `println!`:
```rust
fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {a}");
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(value, 10);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(value, 5);
    }
}
```

### [Executando um subconjunto de testes por nome](https://doc.rust-lang.org/book/ch11-02-running-tests.html#running-a-subset-of-tests-by-name)

Ao executar o comando `cargo test` rodamos todos os testes, para rodar um único teste, basta adicionar o nome do teste

```bash
$ cargo test nome_do_teste
```

O parametro de nome de testes pega nomes parciais também, ou seja, apenas um pedaço do nome.

```bash
$ cargo test nome # <- pedaço do nome do teste
```

### [Ignorar testes, a menos que especificamente solicitado.](https://doc.rust-lang.org/book/ch11-02-running-tests.html#ignoring-tests-unless-specifically-requested)
Testes podem ser ignorar um teste com a diretiva `#[ignore]`, como no seguinte:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }
}
```

caso queira rodar apenas os ignorados podemos usar o seguinte comando:

```bash
$ cargo test -- --ignored
```

## [Organização de Teste](https://doc.rust-lang.org/book/ch11-03-test-organization.html#test-organization)

A comunidade Rust pensa em testes em termos de duas categorias principais: _testes unitários_ e _testes de integração_.

- Testes unitários são pequenos e mais focados, testando um módulo de cada vez de forma isolada, e podem testar interfaces privadas.

- Os testes de integração são totalmente externos à sua biblioteca e utilizam o seu código da mesma forma que qualquer outro código externo faria, utilizando apenas a interface pública e, potencialmente, exercitando múltiplos módulos por teste.

### [Testes unitários](https://doc.rust-lang.org/book/ch11-03-test-organization.html#unit-tests)

Testes não vão para o build final o `cfg` diz ao rust que é uma configuração, no caso, de testes.

### [Testes de funções privadas](https://doc.rust-lang.org/book/ch11-03-test-organization.html#private-function-tests)

Rust permite testar funções privadas, como no seguinte:

```rust
pub fn add_two(a: u64) -> u64 {
    internal_adder(a, 2)
}

fn internal_adder(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        let result = internal_adder(2, 2);
        assert_eq!(result, 4);
    }
}
```

Itens em módulos filhos podem utilizar itens de seus módulos ancestrais. Neste teste, trazemos para o escopo todos os itens pertencentes ao módulo pai do módulo de testes usando `use super::*`; assim, o teste pode chamar `internal_adder`. Se você acha que funções privadas não devem ser testadas, não há nada no Rust que o obrigue a fazê-lo.

### [Testes de Integração](https://doc.rust-lang.org/book/ch11-03-test-organization.html#integration-tests)

No Rust, os testes de integração são totalmente externos à sua biblioteca. Eles usam o seu biblioteca da mesma forma que qualquer outro código faria, o que significa que eles só podem chamar funções que fazem parte da API pública da sua biblioteca.

#### [O Diretório de testes](https://doc.rust-lang.org/book/ch11-03-test-organization.html#the-tests-directory)
Para esse tipo de teste, é criado um diretorio fora do projeto como no seguinte exemplo.

```
adder
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── integration_test.rs
```

Neste caso não é necessario usar a diretiva `#[cfg(test)]` pois o cargo trata o diretorio de teste já como teste.

Para esse caso ainda podemos usar o nome do teste para especifica-lo, mas também podemos especificar o arquivo.

```bash
$ cargo test --test integration_test
```

#### [Submódulos em Testes de Integração](https://doc.rust-lang.org/book/ch11-03-test-organization.html#submodules-in-integration-tests)

Quando se quer utilizar funções utilitárias que não são teste, por noma utilizamos o `common.rs` ou caso não queira que saia no terminal, podemos usar o diretorio `common` como no seguinte.

```rust
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    ├── common
    │   └── mod.rs
    └── integration_test.rs
```

Nomear o arquivo dessa forma instrui o Rust a não tratar o módulo comum como um arquivo de teste de integração. 