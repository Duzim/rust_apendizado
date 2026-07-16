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