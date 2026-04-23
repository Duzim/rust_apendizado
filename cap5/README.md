# [Capitulo 5 - Usando Structs para estruturar dados](https://doc.rust-lang.org/book/ch05-00-structs.html#using-structs-to-structure-related-data)

Nos `Structs` você nomeará cada dado para que fique claro o que os valores significam. Adicionando estes nomes significa que as estruturas são mais flexíveis que as tuplas: você não precisa confiar na ordem dos dados para especificar ou acessar os valores de uma instância. Funcionando como os atributos de um objeto.

## Definindo e instânciando Structs

Para definir um `struct` precisamos utilizar a sintaxe `struct`, em seguida o nome em _PascalCase_ e um `{}` para definir seus atributos.

```rust
struct User {
    name: String;
    email: String;
    sign_in_account: u64;
    active: bool;
}
```

E para instância-lo:

```rust
let user = User {
    name: String::new("Duzim"),
    email: String::new("Email@email.com"),
    sign_in_account: u64;
    active: true,
}
```

E para acessar valores especificos do struct, sendo possivel altera-los.
Observe que toda a instância deve ser mutável; Rust não nos permite marcar apenas certos campos como mutáveis. Como acontece com qualquer expressão, podemos construir uma nova instância da estrutura como a última expressão no corpo da função para retorne implicitamente essa nova instância.

```rust
fn main() {
    let mut user1 = User {
        active: true,
        name: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
}
```

Uma boa prática é a utilização de funções de criação

```rust
fn create_user(name, email) -> User {
    User {
        active: true,
        sign_in_count: 1,
        name: name,
        email: email
    }
}
```

### [Usando Abreviações](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#using-the-field-init-shorthand)

Existem algumas Abreviações uteis no `struct` que podemos utilizar como:

- se houver uma outra variavel com o memos nome, não é necessario repetir

  ```rust
  fn build_user(email: String, name: String) -> User {
      User {
          active: true,
          name,  //Não foi necessario a repetição
          email, //Não foi necessario a repetição
          sign_in_count: 1,
      }
  }
  ```

- Há também algo parecido com o `...spred` do javascript, que copia os dados sem a necessidade de copiar um a um.
  Sem o uso do atalho:

  ```rust
  fn main() {
      // --snip--

      let user2 = User {
          active: user1.active,
          name: user1.name,
          email: String::from("another@example.com"),
          sign_in_count: user1.sign_in_count,
      };
  }
  ```

  Com o atalho:

  ```rust
  fn main() {
      // --snip--

      let user2 = User {
          email: String::from("another@example.com"),
          ..user1
      };
  }
  ```

  Basicamente, ele copia todos os valores que estão sob o efeito da sintaxe `..outro_struct`, mas troca os valores que foram explicitamente descritos como o `email` no exemplo acima.
  - **Obs.:** O uso do `..` deve vir depois do valores trocados.

> **Atenção**: Quando se utiliza o `=` como no ultimo exemplo, os valores de `user1` mudam de dono, sendo agora de `user2`, portanto `user1` perde os valores que estão na memoria `heap` como o `name`, porém os outros ainda podem ser utilizados. Dependendo da necessidade, o certo seria fazer com `deep copy`(usando `.clone()`) para o `name` que é o único que não implementa o `Copy`.

### [Definindo estruturas semelhantes a unidades (Unit-Like Structs)](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#defining-unit-like-structs)

Também é possível definir `structs` sem nenhum atributo, se comportando como um `()` uma tupla vazia.
São úteis para implementar uma caracteristica, pois servem para implementar métodos em um tipo que não precisa guardar estado.

```rust
struct QualquerTipo;

fn main() {
    let coisa = QualquerTipo;
}
```

## Exemplo de uso de `structs`

```rust
fn main() {
    let ret = Retangulo {
        width: 50,
        heigth: 60,
        unit: String::from("cm"),
    };

    let area_ret = calc_area(ret);

    println!("Area do retangulo é: {}", area_ret);
}

fn calc_area(ret: Retangulo) -> String {
    let a = ret.width * ret.heigth;
    let mut str_a = a.to_string();
    str_a.push_str(&    .unit);
    str_a
}

struct Retangulo {
    width: u32,
    heigth: u32,
    unit: String,
}
```
