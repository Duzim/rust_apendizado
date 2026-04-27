# [Enums e correspondência de padrões](https://doc.rust-lang.org/book/ch06-00-enums.html)
Os `enums` permitem definir um tipo enumerando suas possíveis variantes de uma determinada coisa.

## [Definindo um `Enum`](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html#defining-an-enum)
Os Enums são definidos com a sintaxe `enum`, e lista tipos possivel que algo pode ter. o mesmo pode ser escrito da seguinte forma.

```rust
enum Ip {
    V4,
    V6
}
```

### [Valores do Enum](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html#enum-values)

Podemos criar instâncias de cada uma das duas variantes de `TypeIp` assim:

```rust
let ipV4 = Ip::V4;
let ipV6 = Ip::V6;
```

Como "fazem parte de um mesmo tipo" podemos entender que cada tipo que o `enum` pode assumir, é um "tipo dentro de outro tipo".
Como um estado, no caso do exemplo, é um `Ip` tipo `V4` ou `V6`.

```rust
fn route(ip_kind: Ip) {}
```

Como ele pode pode ser varias coisas, para usar os valores que cada tipo o `enum` pode assumir. O rust nos obriga a tratar cada possivel estado desse "tipo" veremos mais a frente com `match`.

Cada "estado" pode também carregar valores. Como por exemplo: 

```rust
enum Ip {
    V4(u8, u8, u8, u8), //Diz que o valor carregado será uma String
    V6(String)
}
```
Permitindo que o "tipo do tipo" carregue valores, tornando a codificação mais limpa.
```rust
fn main() {
    let v4 = Ip::V4(String::from(127, 0, 0, 1));
    let loopback = IpAddr::V6(String::from("::1"));
}
```
Aqui, também é mais fácil ver outro detalhe de como funcionam os enums: O nome de cada variante (o que chamamos de estado) enum que definimos também se torna uma função que constrói uma instância do enum.
Isto é, `Ip::V4()` é uma chamada de função isso requer um String argumento e retorna uma instância do `Ip` tipo. Nós obtenha automaticamente esta função construtora definida como resultado da definição do enum.

#### Metodos em `enum`

Também é possível adicionar metodos a um `enum`. Usando a mesma sintaxe `impl`

```rust
enum Ip {
    V4(u8, u8, u8, u8), //Diz que o valor carregado será uma String
    V6(String)
}
impl Ip {
    fn connect(&self) {
        //Faz algo
    }
} 
```
O `&self` vai ter o corpo do que foi chamado, se foi instânciado com `V4` terá as caracteristicas de um `V4`.

### [O `Option` Enum](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html#the-option-enum)

O `Option` é basicamente a possíbilidade de ser algo ou nada. Tendo essas duas opções apenas.

```rust
enum Option<T> {
    None,
    Some(T),
}
```
Em `rust` não existe nulo, portanto é preciso fazer um certo malabarismo.
O conceito que "não valor" está tentando expressar ainda é útil: A null é um valor que atualmente é inválido ou ausente por algum motivo.

O `Option<T>` está como standart da linguagem, ou seja, não precisa ser explicitamente chamado.
Então ao usar o estado `Some` ou `None`, não é preciso usar a sintaxe `Option::`; pode-se apenas chamar diretamente, como uma palavra reservada.

```rust
let x: i8 = 5;
let y: Option<i8> = Some(5); //Pode ser chamado sem o Option::Some(5); 
let z = Some('a'); // será do tipo char
let a: Option<i32>  = None; //Será um None. Precisa de anotçào de <T>, pois não pode-se inferir apenas pelo Mone
```

- **Obs:** Onde `<T>` é um generico que diz que O tipo de pode ser qualquer um.

## [O `match` Construção de fluxo de controle](https://doc.rust-lang.org/book/ch06-02-match.html#the-match-control-flow-construct)

O `match` permite comparar um valor com uma serie de padrões. Perfeito para o uso de enums, onde ele compara todas as possibilidades que aquele valor pode assumir.

```rust
enum HttpResponse {
    OK,
    Redirect(String),
    ErrorClient(u16, String),
    JsonData { status: u16, body: String },
}

fn handle_http_resp(res: HttpResponse) {
    match res {
        HttpResponse::OK => {
            println!("OK");
        }
        HttpResponse::Redirect(url) => {
            println!("{:?}", url);
        }
        HttpResponse::ErrorClient(status, err) => {
            println!("stts: {}, err: {}", status, err);
        }
        HttpResponse::JsonData { status, body } => {
            println!("stts: {:?}, body: {:?}", status, body);
        }
    }
}
```

Ele necessariamente precisa ter ações para todas as possibilidade um valor, funcionando quase como um `switch` só que mais poderoso.

### [Padrões que se ligam a valores](https://doc.rust-lang.org/book/ch06-02-match.html#patterns-that-bind-to-values)
Os valores de um `enum` podem conter outros enums

```rust
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}
```
### [O `Option<T>` `match` Padrão](https://doc.rust-lang.org/book/ch06-02-match.html#the-optiont-match-pattern)

```rust
```