# [Capitulo 8 - Coleções comuns](https://doc.rust-lang.org/book/ch08-00-common-collections.html)

Cada um dos tipos de coleções a serem apresentadas, são armazenadas na memoria `heap`, ou seja, podem ter seus valores auterados em tempo de execução. 

- Um _vetor_ permite armazenar um número variável de valores próximos uns dos outros.
- Um `String` é uma coleção de `chars`. Mencionamos o tipo `String` anteriormente, mas neste capítulo, falaremos sobre isso em profundidade.
- Um _hash map_ permite que você associe um valor a uma chave específica. É um implementação particular da estrutura de dados mais geral chamada a `map`.

## [Armazenando listas de valores com vetores](https://doc.rust-lang.org/book/ch08-01-vectors.html)

Os vetores permitem armazenar mais de um valor em uma única estrutura de dados que coloca todos os valores um ao lado do outro na memória. Os vetores só podem armazenar valores do mesmo tipo.


### [Criando um novo vetor](https://doc.rust-lang.org/book/ch08-01-vectors.html#creating-a-new-vector)

Criando com anotações e por meio da função construtora.
```rust
let v: Vec<i32> = Vec::new(); //Precisa inserir notação em <T>
```

Criando via macro com `vec![]`, onde o rust vai inferir o tipo com base nos valores iniciais.

```rust
let v = vec![1, 2, 3]; // Infere o tipo automaticamente
```

### [Atualizando um vetor](https://doc.rust-lang.org/book/ch08-01-vectors.html#updating-a-vector)

Como em qualquer coisa em rust para atualizar valores é necessario que seja mutável com `mut`.
- [Existem diversas operações possíveis para um vetor](https://doc.rust-lang.org/std/vec/index.html)

```rust
let mut v = Vec::new();

v.push(5);
v.push(6);
v.push(7);
v.push(8);
```

### [Lendo elementos de vetores](https://doc.rust-lang.org/book/ch08-01-vectors.html#reading-elements-of-vectors)

Existem duas maneiras de referenciar um valor armazenado em um vetor: via indexação com `[]` ou por usando o método `get`.

```rust
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("The third element is {third}");

let third: Option<&i32> = v.get(2);
match third {
    Some(third) => println!("The third element is {third}"),
    None => println!("There is no third element."),
}
```

Basicamente com os `[]` ao tentar acessar um elemento numa posição inexistente o programa entrará em erro. Mas com o `get` ele tem um tratamento de erro implicito.

Os vetores colocam os valores um ao lado do outro na memória, adicionar um novo elemento ao final do vetor pode exigir alocando nova memória e copiando os elementos antigos para o novo espaço, se houver não há espaço suficiente para colocar todos os elementos próximos uns dos outros onde o vetor está armazenado no momento. Nesse caso, a referência ao primeiro elemento seria apontando para memória desalocada. As regras de empréstimo impedem que os programas acabando nessa situação.

### [Iterando sobre os valores em um vetor](https://doc.rust-lang.org/book/ch08-01-vectors.html#iterating-over-the-values-in-a-vector)

É preciso utilizar o operador `&`, para que o vetor continue sendo o dono de seus valores

```rust
let v = vec![100, 32, 57];
for i in &v {
    println!("{i}");
}
```

Também é possível fazer alterações durante a iteração, mas é necessario derreferenciar com o operador `*` para que ele chegue até o valor de fato e altere seu valor.

```rust
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;
}

```

### [Usando um Enum para armazenar vários tipos](https://doc.rust-lang.org/book/ch08-01-vectors.html#using-an-enum-to-store-multiple-types)

É possível também armazenar diversos estados de um `enum` em um vetor.

```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```

Rust precisa saber quais tipos estarão no vetor em tempo de compilação para que ele sabe exatamente quanta memória no heap será necessária para armazenar cada elemento. Também devemos ser explícitos sobre quais tipos são permitidos neste vetor.

Se você não conhece o conjunto exaustivo de tipos que um programa obterá em tempo de execução armazenar em um vetor, a técnica enum não funcionará. 

### [Descartando seus elementos](https://doc.rust-lang.org/book/ch08-01-vectors.html#dropping-a-vector-drops-its-elements)

Quando o vetor é descartado, todo o seu conteúdo também é descartado, o que significa que os números inteiros que ele contém serão limpos. O verificador de empréstimos garante que qualquer referências ao conteúdo de um vetor são usadas apenas enquanto o vetor em si é válido.

```rust
{
    let v = vec![1, 2, 3, 4];

    // do stuff with v
} // <- v goes out of scope and is freed here
```

## [Armazenando texto codificado em UTF-8 com strings](https://doc.rust-lang.org/book/ch08-02-strings.html)

## [Armazenando chaves com valores associados em mapas Hash](https://doc.rust-lang.org/book/ch08-03-hash-maps.html)