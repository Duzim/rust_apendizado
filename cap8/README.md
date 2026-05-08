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

Discutimos strings no contexto de coleções porque strings são implementado como uma coleção de bytes, além de alguns métodos para fornecer informações úteis funcionalidade quando esses bytes são interpretados como texto.

### [Definindo Strings](https://doc.rust-lang.org/book/ch08-02-strings.html#defining-strings)

### [Criando uma nova `String`](https://doc.rust-lang.org/book/ch08-02-strings.html#creating-a-new-string)

Muitas das mesmas operações disponíveis com `Vec<T>` estão disponíveis com `String` também porque `String` na verdade é implementado como um invólucro em torno de um vetor de bytes com algumas garantias, restrições e recursos extras.

```rust
let mut s = String::new(); //Cria uma String vazia
```

Da seguinte forma podemos criar `strings` com conteúdo inicial
```rust
let data = "initial contents";

let s = data.to_string();

// The method also works on a literal directly:
let s = "initial contents".to_string();
```
O mesmo pode ser feito com o método `from` do `String`

```rust
let s = String::from("initial contents");
```
`String::from` e `to_string` fazem a mesma coisa, então qual você escolher é uma questão de estilo e legibilidade.

### [Atualizando uma String](https://doc.rust-lang.org/book/ch08-02-strings.html#updating-a-string)

#### [Anexando com `push_str` ou `push`](https://doc.rust-lang.org/book/ch08-02-strings.html#appending-with-push_str-or-push)

O `push_str` adiciona qualquer fatia de string a uma `String`

```rust
let mut s = String::from("foo");
s.push_str("bar");
```

Já o `push` no contexto de `Strings` apenas adiciona um único caractere

```rust
let mut s = String::from("lo");
s.push('l');
```

#### [Concatenando com `+` ou `format!`](https://doc.rust-lang.org/book/ch08-02-strings.html#concatenating-with--or-format)

Muitas vezes, você vai querer combinar duas strings existentes. Uma maneira de fazer isso é usar o operador `+`.

Com o operador `+` podemos fazer da seguinte forma

```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
```

O operador `+` funciona como a seguinte função 

```rust
fn add(self, s: &str) -> String {
```

onde nosso `s1` será movido para o `self` assim não sendo mais valido após essa operação

para formatações mais complexas podemos usar a macro `format!`

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{s1}-{s2}-{s3}");
```

O macro `format!` funciona como `println!`, mas em vez de imprimir a saída na tela, ele retorna um `String` com o conteúdo.

### [Indexação em Strings](https://doc.rust-lang.org/book/ch08-02-strings.html#indexing-into-strings)

#### [Representação Interna](https://doc.rust-lang.org/book/ch08-02-strings.html#internal-representation)

#### [Bytes, valores escalares e clusters de grafemas](https://doc.rust-lang.org/book/ch08-02-strings.html#bytes-scalar-values-and-grapheme-clusters)

### [Cortando `Strings`](https://doc.rust-lang.org/book/ch08-02-strings.html#slicing-strings)

Em vez de indexar usando [] com um único número, você pode usar [] com a intervalo para criar uma fatia de string contendo bytes específicos:

```rust
let hello = "Здравствуйте";

let s = &hello[0..4];
```
Aqui, s será um &str que contém os primeiros 4 bytes da string. Significa s será _Зд_ e não _Здра_
Portanto, evite fazer dessa forma, pois é não é preciso depende de quantos bytes o caractere gasta no UTF-8

### [Iterando sobre strings](https://doc.rust-lang.org/book/ch08-02-strings.html#iterating-over-strings)

A melhor maneira de operar em pedaços de cordas é ser explícito sobre se você quer caracteres ou bytes.
dessa forma cada valor será um `char`

```rust
for c in "Зд".chars() {
    println!("{c}");
}
```

Alternativamente, o bytes o método retorna cada byte bruto, que pode ser apropriado para o seu domínio:

```rust
for b in "Зд".bytes() {
    println!("{b}");
}
```

## [Armazenando chaves com valores associados em mapas Hash](https://doc.rust-lang.org/book/ch08-03-hash-maps.html)


O tipo `HashMap<K, V>` armazena um mapeamento de chaves de tipo `K` para valores do tipo `V`usando um hashing função, que determina como coloca essas chaves e valores na memória.

Mapas hash são úteis quando você deseja procurar dados não usando um índice, como você pode com vetores, mas usando uma chave que pode ser de qualquer tipo.

Todas as chaves devem ter o mesmo tipo e todos os valores deve ter o mesmo tipo.

### [Criando um novo hash map](https://doc.rust-lang.org/book/ch08-03-hash-maps.html#creating-a-new-hash-map)

Uma maneira de se criar um `HashMap` é cria um vazio usando o `new` e adicionar valores com o metodo `insert`.

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

### [Acessando valores em um Hash map](https://doc.rust-lang.org/book/ch08-03-hash-maps.html#accessing-values-in-a-hash-map)

Podemos obter um valor do mapa hash fornecendo sua chave para o método `get`.

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
let score = scores.get(&team_name).copied().unwrap_or(0);
```
O `get` método retorna um `Option<&V>`;se não houver valor para essa chave no mapa hash, get retornará None. Este programa lida com o `Option` ligando copied para obter um `Option<i32>` em vez de um `Option<&i32>`, então unwrap_or definir score para zero se scores não tenha uma entrada para a chave.

Podemos iterar sobre cada par chave-valor em um mapa hash de maneira semelhante à nossa faça com vetores, usando um loop `for`:

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

for (key, value) in &scores {
    println!("{key}: {value}");
}
```

### [Gerenciando propriedade em mapas Hash](https://doc.rust-lang.org/book/ch08-03-hash-maps.html#managing-ownership-in-hash-maps)

Para tipos que implementam a característica `Copy`, como `i32`, os valores são copiados no hash map. Para valores próprios como `String`, os valores serão movidos e o mapa hash será o proprietário desses valores.

Se inserirmos referências a valores no mapa hash, os valores não serão movidos no mapa hash. Os valores para os quais as referências apontam devem ser válidos em pelo menos enquanto o mapa hash for válido.

### [Atualizando um mapa de hash](https://doc.rust-lang.org/book/ch08-03-hash-maps.html#updating-a-hash-map)

#### [Sobrescrevendo um valor](https://doc.rust-lang.org/book/ch08-03-hash-maps.html#overwriting-a-value)

Basta inserir um valor a uma chave já existente

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);

println!("{scores:?}");
```

O codigo acima terá o print `{"Blue": 25}` pois o 10 sobrescrito.

#### [Adicionar uma chave e um valor somente se uma chave não estiver presente](https://doc.rust-lang.org/book/ch08-03-hash-maps.html#adding-a-key-and-value-only-if-a-key-isnt-present)

É comum verificar se uma chave específica já existe no mapa hash com um valor e então tomar as seguintes ações: Se a chave existir em o mapa hash, o valor existente deve permanecer como está; se a chave não existe, insira-o e um valor para ele.

Os mapas de hash têm uma API especial para isso chamada entry isso pega a chave você quer verificar como parâmetro. O valor de retorno do entry método é um enum chamado Entry isso representa um valor que pode ou não existir. Digamos queremos verificar se a chave para a equipe Amarela tem um valor associado com ele. Caso contrário, queremos inserir o valor 50, e o mesmo para o Equipe azul. 

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);

println!("{scores:?}");
```

O or_insert método em Entry é definido para retornar uma referência mutável a o valor para o correspondente Entry chave se essa chave existir e, caso contrário, ela insere o parâmetro como o novo valor para esta chave e retorna um mutável referência ao novo valor. Esta técnica é muito mais limpa do que escrever o lógica nós mesmos e, além disso, joga mais bem com o verificador de empréstimo.

#### [Atualizando um valor com base no valor antigo](https://doc.rust-lang.org/book/ch08-03-hash-maps.html#updating-a-value-based-on-the-old-value)

Outro caso de uso comum para mapas hash é procurar o valor de uma chave e então atualize-o com base no valor antigo.

```rust
use std::collections::HashMap;

let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

println!("{map:?}");
```