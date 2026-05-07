# Exercicios sobre vetores Capitulo 8.1

## Exercício 1: Gerenciamento Simples de Fila

- **Objetivo:** Praticar a criação de um vetor mutável usando a macro vec!, adicionar elementos (`push`), remover elementos (`pop`) e iterar sobre o vetor.

Você está criando um sistema simples de fila de atendimento por nomes.

1. Crie uma função chamada `gerenciar_fila()` que não recebe parâmetros e não retorna nada.

2. Dentro dela, crie um vetor mutável contendo os nomes (Strings) "Ana", "Carlos" e "Bia" usando a macro `vec!`.

3. Adicione um novo nome ao final da fila: "Diego".

4. Remova a última pessoa da fila usando o método `.pop()`. (Lembre-se que o `pop` retorna um `Option`, mas aqui você pode ignorar o retorno e focar apenas na ação de remoção).

5. Crie um laço `for` para iterar sobre referências do vetor (&fila) e imprima cada nome com a mensagem: `"Aguardando: [nome]"`.

```rust
fn main() {
    println!("--- Iniciando Fila ---");
    gerenciar_fila();
}
```

Resp.:

<details>
<summary>Resposta do exercicio acima:</summary>

Minha Resposta:

```rust
fn gerenciar_fila() {
    let mut v: Vec<&str> = vec!["Ana", "Carlos", "Bia"];
    println!("{:?}", v);

    v.push("Diego");
    println!("{:?}", v);

    let value_popped = v.pop();

    match value_popped {
        Some(val) => {
            println!("value no match [{:?}]", val);
        }
        None => {}
    }

    for i in &v {
        println!("Aguardando: [{:?}]", i);
    }
}
```
</details>


## Exercício 2: Leitura Segura de Sensores

- **Objetivo:** Trabalhar com o método .get() e combiná-lo com o que você aprendeu no Capítulo 6 (match ou if let) para evitar panics de "index out of bounds" (índice fora dos limites).

Você tem um vetor com registros de temperaturas, mas o usuário pode pedir para ler uma posição que não existe.

1. Crie uma função chamada `ler_temperatura(historico: &Vec<i32>, indice: usize)`.

2. Dentro da função, use o método `.get(indice)`. Não use colchetes `[]`!

3. O `.get()` retorna um `Option<&i32>`. Use um `match` para lidar com os dois casos:

    - Se houver valor (`Some`), imprima: `"Temperatura na posição [indice]: [valor]°C"`.

    - Se não houver valor (`None`), imprima: `"Erro: Índice [indice] não possui leitura."`.

```rust
fn main() {
    let temperaturas = vec![22, 25, 21, 19, 24];

    ler_temperatura(&temperaturas, 2); // Deve funcionar
    ler_temperatura(&temperaturas, 10); // Deve avisar que não possui leitura e NÃO falhar (panic)
}
```

Resp.:

<details>
<summary>Resposta do exercicio acima:</summary>

Minha Resposta:

```rust
fn ler_temperatura(lista: &Vec<i32>, indice: usize) {
    let valor = lista.get(indice);

    match valor {
        Some(val) => {
            println!("Valor no indice [{}] é [{}].", indice, val);
        }
        None => {
            println!("Nenhum valor nesse indice.");
        }
    }
}
```

</details>

## Exercício 3: Alteração em Massa (Iteração Mutável)

- **Objetivo:** Iterar sobre um vetor de forma mutável (`&mut`) para alterar os valores originais lá dentro, usando o operador de desreferência `*`.

Uma loja de jogos quer aplicar um aumento de R$ 5,00 em todos os produtos da cesta.

1. Crie uma função chamada `aplicar_acrescimo(precos: &mut Vec<f64>)`.

2. Use um laço `for` que itere de forma mutável sobre o vetor.

3. Para cada elemento, adicione `5.0`. (Dica: você precisará usar o operador `*` (asterisco) antes da variável no laço para alterar o valor original para o qual a referência aponta).

```rust
fn main() {
    let mut carrinho = vec![15.50, 40.00, 100.90];
    
    println!("Preços originais: {:?}", carrinho);
    
    aplicar_acrescimo(&mut carrinho);
    
    println!("Preços com acréscimo: {:?}", carrinho);
}
```

Resp.:

<details>
<summary>Resposta do exercicio acima:</summary>

Minha Resposta:

```rust
fn aplicar_acrescimo(card: &mut Vec<f64>) {
    for i in card {
        *i += 5.0;
    }
}
```

</details>

## Exercício 4: Inventário Heterogêneo (Vetores com Enums)

- **Objetivo:** Aplicar uma técnica avançada mas muito comum em Rust: usar um `enum` para permitir que um único vetor armazene tipos de dados estruturalmente diferentes.

Um vetor só pode armazenar dados de um único tipo. Mas em um RPG, o inventário de um personagem tem moedas, poções de vida e armas, cada um requerendo dados diferentes.

1. Defina um enum ItemInventario com as seguintes variantes:

    - `Ouro(u32)` (representa a quantidade de moedas)

    - `Pocao(String)` (representa o nome da poção)

    - `Espada(String, u32)` (representa o nome da espada e o seu dano)

2. No `main`, crie uma variável chamada `bau` e inicialize-a com a macro `vec!`, inserindo os seguintes itens do seu novo enum: 100 de Ouro, uma Poção de "Cura Maior" e uma Espada "Excalibur" de dano 50.

3. Crie um laço `for` iterando sobre referências de `bau`. Dentro dele, use um `match` para imprimir mensagens descritivas diferentes para cada tipo de item encontrado no baú.

```rust
// Defina seu enum aqui

fn main() {
    // 1. Crie o vetor `bau` contendo as variantes do enum aqui

    println!("--- Abrindo o Baú ---");
    // 2. Itere sobre o baú e use o match para imprimir os detalhes de cada item
}
```

Resp.:

<details>
<summary>Resposta do exercicio acima:</summary>

Minha Resposta:

```rust
enum ItemInventario {
    Ouro(u32),
    Pocao(String),
    Espada(String, u32),
}

fn main() {
    // 1. Crie o vetor `bau` contendo as variantes do enum aqui
    let bau = vec![
        ItemInventario::Ouro(100),
        ItemInventario::Pocao(String::from("Cura Maior")),
        ItemInventario::Espada(String::from("Excalibur"), 50)
    ];

    println!("--- Abrindo o Baú ---");
    // 2. Itere sobre o baú e use o match para imprimir os detalhes de cada item

    for item in bau {

        match item {
            ItemInventario::Ouro(coin) => {
                println!("Quantidade de ouro: [{}]", coin);
            },
            ItemInventario::Pocao(potion) => {
                println!("Poções: [{}]", potion);
            },
            ItemInventario::Espada(name, dano) => {
                println!("Nome da espada: {}, Dano da espada: {}", name, dano);
            }
        }
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