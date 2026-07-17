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
