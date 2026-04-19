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
