# Exercicios

## Exercício 1: O Mistério do Move

**Objetivo:** Consertar o erro de compilação sem remover o `println!`.

```rust
fn main() {
    let s1 = String::from("Rust é incrível");

    fazer_algo(s1);

    // ERRO: s1 foi movida. Como você alteraria a função
    // ou a chamada para que s1 ainda fosse válida aqui?
    println!("A frase original era: {}", s1);
}

fn fazer_algo(texto: String) {
    println!("Processando: {}", texto);
}
```

Resp.:

<details>
<summary>Resposta do exercicio acima:</summary>

```rust
fn main() {
    let s1 = String::from("Rust é incrível");

    fazer_algo(&s1);


    // ERRO: s1 foi movida. Como você alteraria a função
    // ou a chamada para que s1 ainda fosse válida aqui?
    println!("A frase original era: {}", s1);

}

fn fazer_algo(texto: &String) {
    println!("Processando: {}", texto);
}

//Foi consertado tornando o parametro uma referencia a dona original da String
//Poderia ser consertado também, clonando com .clone()

```

</details>

## Exercício 2: O Empréstimo Mutável

**Objetivo:** Modificar uma string dentro de uma função.
Escreva uma função chamada adicionar_prefixo que recebe uma referência mutável de uma String e adiciona o texto "Cargo: " ao início dela.

```rust
fn main() {
    let mut comando = String::from("build");

    // Chame sua função aqui

    println!("{}", comando); // Deve imprimir: "Cargo: build"
}
```

Resp.:

<details>
<summary>Resposta do exercicio acima:</summary>

Minha Resposta:

```rust
fn main() {
    let mut comando = String::from("build");
    add_prefix(&mut comando);
    println!("{}", comando); // Deve imprimir: "Cargo: build"
}

fn add_prefix(str_value: &mut String) {
    let mut temp = String::from("Cargo: ");
    temp.push_str(str_value);
    *str_value = temp;
}

```

Correção:

```rust
fn add_prefix(str_value: &mut String) {
    // Insere no índice 0 (o início da string)
    str_value.insert_str(0, "Cargo: ");
}
```

</details>

## Exercício 3: O Guardião de Slices

**Objetivo:** Retornar a última palavra de uma frase usando Slices.

Crie uma função `ultima_palavra(s: &str) -> &str`.
**Dica:** Use o método `.as_bytes()` e itere de trás para frente usando `.rev()`.

```rust
fn main() {
    let frase = String::from("Aprendendo Rust agora");
    let palavra = ultima_palavra(&frase);

    println!("A última palavra é: {}", palavra); // Deve imprimir: "agora"
}
```

Resp.:

<details>
<summary>Resposta do exercicio acima:</summary>

Minha Resposta:

```rust
fn ultima_palavra(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().rev().enumerate() {
        if item == b' ' {
            let a = s.len();
            return &s[a - i..];
        }
    }
    &s
}

```

Correção com apenas recursos apresentados até agora:

```rust
fn main() {
    let frase = String::from("Aprendendo Rust agora");
    let palavra = ultima_palavra(&frase);

    println!("A última palavra é: {}", palavra); // Imprime: "agora"
}

fn ultima_palavra(s: &str) -> &str {
    let bytes = s.as_bytes();

    // Assumimos que não há espaço no início.
    // Se não acharmos nenhum espaço, a palavra inteira é a última.
    let mut ultimo_espaco = 0;
    let mut tem_espaco = false;

    // Varremos os bytes do início ao fim, como o livro ensina
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            ultimo_espaco = i;
            tem_espaco = true;
        }
    }

    if tem_espaco {
        // Retornamos um Slice logo após o último espaço encontrado até o fim
        &s[ultimo_espaco + 1..]
    } else {
        // Se não teve espaço, retorna a fatia da string toda
        &s[..]
    }
}
```

</details>

## Exercício 4: O Desafio das Regras de Ouro

**Objetivo:** Entender por que o Rust impede este código.

Tente compilar o código abaixo. Ele vai falhar. **Explique (para si mesmo ou para mim) qual regra de referência ele quebra** e como consertar movendo apenas uma linha de lugar.

```rust
fn main() {
    let mut status = String::from("Ativo");

    let r1 = &status;
    let r2 = &status;
    let r3 = &mut status; // O PROBLEMA ESTÁ AQUI

    println!("{}, {}, e {}", r1, r2, r3);
}
```

<details>
<summary>Resposta do exercicio acima:</summary>

Minha Resposta:

```rust
fn main() {
    let status = String::from("Ativo");

    let r1 = &status;
    let r2 = &status;
    let r3 = &status; // O PROBLEMA ESTÁ AQUI

    println!("{}, {}, e {}", r1, r2, r3);
}

```

Correção:

<p>Aqui sim o compilador grita! Ele pensa: "Espera aí, r1 e r2 estão lendo os dados. Se eu permitir que r3 tenha o poder de modificar (&mut) o status ao mesmo tempo, o chão pode sumir debaixo dos pés de r1 e r2!" Isso previne problemas clássicos de leitura de memória suja (conhecidos como data races).</p>

</details>
