# Exercicios de tratamento de Erro

## Exercício 1: O Básico do `Result` e `match`

- **Objetivo:** Lidar manualmente com o enum `Result` retornado por funções da biblioteca padrão, extraindo o valor de sucesso ou tratando a falha graciosamente.

Quando tentamos converter uma `String` para um número usando o método `.parse::<T>()`, a operação pode falhar (por exemplo, se a string tiver letras).

1. Crie uma função chamada `imprimir_idade(entrada: &str)`. Ela não retorna nada.

2. Dentro dela, tente converter `entrada` para um `u8` usando `.parse::<u8>()`.
 
3. Use um `match` no retorno do `parse`:

    - Se der certo (`Ok`), imprima: `"Idade válida: [valor] anos."`

    - Se der errado (`Err`), imprima: `"Erro: '[entrada]' não é uma idade válida."`

```rust
fn main() {
    imprimir_idade("25");
    imprimir_idade("quarenta");
    imprimir_idade("-5"); // u8 não aceita negativos
}
```

Resp.:

<details>
<summary>Resposta do exercicio acima:</summary>

Minha Resposta:

```rust
fn imprimir_idade(idade: &str) {
    //let num_idade = idade.parse::<u8>();
    //parse retorna um Result<u8, ParseIntError>
    match idade.parse::<u8>() {
        Ok(i) => {
            println!("Idade válida: [{}] anos.", i);
        }
        Err(e) => {
            println!("Erro: '[{}]' não é uma idade válida.", e);
        }
    }
}
```

</details>

## Exercício 2: Propagação de Erros com o Operador `?`

- **Objetivo:** Criar uma função que retorna um `Result` e usar o operador `?` para repassar (propagar) erros para quem chamou a função, em vez de tratá-los internamente.

Você precisa escrever uma função que recebe duas strings, tenta converter ambas para inteiros (`i32`) e retorna a multiplicação delas. Se qualquer uma das conversões falhar, a função inteira deve falhar imediatamente.

1. Importe o tipo de erro de parsing numérico: `use std::num::ParseIntError;`

2. Crie uma função `multiplicar_textos(a: &str, b: &str) -> Result<i32, ParseIntError>`.

3. Dentro da função, use o método `.parse::<i32>()` na string a seguido do operador `?` para extrair o valor ou propagar o erro. Faça o mesmo para b.

4. Retorne a multiplicação dos dois valores embrulhada em um `Ok()`.

```rust
use std::num::ParseIntError;

// Escreva sua função multiplicar_textos aqui

fn main() {
    let sucesso = multiplicar_textos("10", "5");
    let falha = multiplicar_textos("10", "cinco");

    println!("Sucesso: {:?}", sucesso); // Deve imprimir: Ok(50)
    println!("Falha: {:?}", falha);     // Deve imprimir: Err(ParseIntError { ... })
}
```

Resp.:

<details>
<summary>Resposta do exercicio acima:</summary>

Minha Resposta:

```rust
use std::num::ParseIntError;

fn main() {
    let sucesso = multiplicar_textos("10", "5");
    let falha = multiplicar_textos("10", "cinco");

    println!("Sucesso: {:?}", sucesso); // Deve imprimir: Ok(50)
    println!("Falha: {:?}", falha); // Deve imprimir: Err(ParseIntError { ... })
}

fn multiplicar_textos(a: &str, b: &str) -> Result<i32, ParseIntError> {
    // O operador '?' avalia o Result.
    // Se for Ok(valor), ele desempacota e atribui à variável.
    // Se for Err(erro), ele dá return prematuro repassando o erro.
    let num_a = a.parse::<i32>()?;
    let num_b = b.parse::<i32>()?;

    Ok(num_a * num_b)
}
```

</details>

## Exercício 3: Validação em Construtores (Tipos Seguros)

- **Objetivo:** Garantir que uma `struct` nunca exista em um estado inválido. Em vez de usar um `panic!`, o construtor retorna um `Result`.

Vamos criar um sistema bancário simples onde uma transferência não pode ser negativa nem zero.

1. Defina uma `struct Transferencia` contendo um campo `valor: f64`.

2. Crie um bloco `impl` com uma função associada `nova(valor: f64) -> Result<Transferencia, String>`. (Nota: usar `String` como tipo de erro é comum e simples para mensagens de texto).

3. Na função `nova`:

    - Se o valor for menor ou igual a `0.0`, retorne `Err` contendo a mensagem `"O valor da transferência deve ser maior que zero."`.

    - Caso contrário, retorne um `Ok` contendo a nova instância de `Transferencia`.


```rust
// Defina a struct e o impl aqui

fn main() {
    let t1 = Transferencia::nova(150.50);
    let t2 = Transferencia::nova(-20.0);

    // O unwrap_or_else permite tratar o erro e extrair um valor padrão (ou dar panic se quisermos)
    match t1 {
        Ok(t) => println!("Transferência 1 criada com sucesso: R${}", t.valor),
        Err(e) => println!("Falha na Transferência 1: {}", e),
    }

    match t2 {
        Ok(t) => println!("Transferência 2 criada com sucesso: R${}", t.valor),
        Err(e) => println!("Falha na Transferência 2: {}", e),
    }
}
```

Resp.:

<details>
<summary>Resposta do exercicio acima:</summary>

Minha Resposta:

```rust
struct Transferencia {
    valor: f64,
}
impl Transferencia {
    fn nova(val: f64) -> Result<Self, String> {
        if val <= 0.0 {
            return Err(String::from(
                "O valor da transferência deve ser maior que zero.",
            ));
        }

        Ok(Transferencia { valor: val })
    }
}
```

</details>

## Exercício 4: Integrando `HashMap` e `Result`

- Objetivo: Unir o conceito de `HashMap` (Capítulo 8) com tratamento de erros (Capítulo 9). Buscar itens em mapas retorna `Option`, que muitas vezes precisamos converter logicamente para `Result` em regras de negócio.

Você tem um banco de dados em memória contendo o saldo de clientes. Sua função precisa buscar o saldo, mas se o cliente não existir, deve retornar um erro claro em vez de apenas `None`.

1. Importe `std::collections::HashMap`.

2. Crie a função `buscar_saldo(banco: &HashMap<String, f64>, cliente: &str) -> Result<f64, String>`.

3. Use o método `.get(cliente)` no HashMap.

4. Lide com o retorno usando `match`:

```rust
use std::collections::HashMap;

// Escreva sua função aqui

fn main() {
    let mut banco_dados = HashMap::new();
    banco_dados.insert(String::from("Alice"), 1200.50);
    banco_dados.insert(String::from("Bob"), 50.0);

    let saldo_alice = buscar_saldo(&banco_dados, "Alice");
    let saldo_carlos = buscar_saldo(&banco_dados, "Carlos");

    // O método unwrap() tenta extrair o valor do Ok. 
    // Se for um Err, o programa sofre um panic! (útil para testes rápidos)
    println!("Alice tem: {}", saldo_alice.unwrap());
    
    // Para imprimir o erro com segurança, vamos dar um match:
    match saldo_carlos {
        Ok(v) => println!("Carlos tem: {}", v),
        Err(e) => println!("Aviso do sistema: {}", e),
    }
}
```

Resp.:

<details>
<summary>Resposta do exercicio acima:</summary>

Minha Resposta:

```rust
fn buscar_saldo(bd: &HashMap<String, f64>, cliente: &str) -> Result<f64, String> {
    match bd.get(cliente) {
        // Desestruturamos a referência com &saldo,
        // assim a variável 'saldo' é um f64, não um &f64
        Some(&saldo) => Ok(saldo),
        None => Err(format!("Cliente [{}] não cadastrado.", cliente)),
    }
}

```

</details>

---

Resp.:

<details>
<summary>Resposta do exercicio acima:</summary>

Minha Resposta:

```rust
```

</details>