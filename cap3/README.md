# [Conceitos Comuns de Programação](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html)

## [Variavei e mutabilidade](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)

```rust
let a = 5; // Variavel imutável
a = 6; // -> Vai dar erro, por tentar reatribuição

let mut b = 4; //Variavel mutável
b = 8; //Reatribui valor sem problemas
```

### [Declaração Contstantes](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#declaring-constants)

O Rust além da variaveis imutaveis há também constantes, que se assemelham com variaveis definidas por `let`, mas não podem ser atribuidas com `mut`, permanecem as mesmas durante toda a execução do codigo. Além de precisar de uma definição de tipo, como `: i32` e precisam ser declaradas maiusculo.

```rust
const CONSTANTE_RUST: u32 = 20 * 20;
```

### [Sombreamento](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing)

Sombramento é a criação de uma nova variavel utilizando o nome da anterior, basicamente pode permitir mudar o tipo do valor associado aquele nome e também a variavel definina num escopo não foge dele. Premitindo o seguinte codigo:

```rust
let x = 5;
let x = x + 5; // x = 10
{
    let x = x + 5; //x = 15
}
println!(x) // x = 10
```

## [Tipos de Dados](https://doc.rust-lang.org/book/ch03-02-data-types.html)

Rust Necessariamente exige a tipagem de variaveis

### [Tipos escalares](https://doc.rust-lang.org/book/ch03-02-data-types.html#scalar-types)

Tipos escalares representam um único valor, dos quais são quatro primários: `inteiros`, `números de ponto flutuantes`, `booleanos` e `caracteres`.

#### [Tipos Inteiros](https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-types)

Um inteiro é um número sem componente fracionário

| Tamanho                       | Assinado | Não Assinado |
| :---------------------------- | :------: | -----------: |
| **8 bits**                    |   `i8`   |         `u8` |
| **16 bits**                   |  `i16`   |        `u16` |
| **32 bits**                   |  `i32`   |        `u32` |
| **64 bits**                   |  `i64`   |        `u64` |
| **128 bits**                  |  `i128`  |       `u128` |
| **Dependente da arquitetura** | `isize`  |      `usize` |

Cada variante pode ser assinada ou não assinada e tem um tamanho explícito. Assinado e não assinado consulte se é possível que o número seja negativo.

- **Assinado:** É basicamente o número que pode ser negativo, portanto pode levar sinal +/-
- **Não Assinado:** É o númerio inteiro que não pode ser negativo, logo, não pode levar +/-

Os números assinados são armazenados usando [complemento de dois](https://pt.wikipedia.org/wiki/Complemento_para_dois).

- Os valores declarados com `i` precisam abranger números negativos, portanto terão uma quantidade menor de números possiveis que `u`, por exemplo: `i8` consegue abrange (−(2^7) até 2^7 − 1), ou seja, -128 até 127. Sendo que, `u8` vai de 0 até 255, pois ele é representado pelo seguinte 2^8 − 1.

- Além disso, o `isize` e `usize` os tipos dependem da arquitetura do computador em que seu programa está sendo executado: 64 bits se você estiver em uma arquitetura de 64 bits e 32 bits se você estiver em uma arquitetura de 32 bits.

| Tipos de Inteiros |      Exemplos |
| :---------------- | ------------: |
| Decimal           |      `98_123` |
| Hexadecimal       |      `0xffa4` |
| Octal             |        `0o77` |
| Binário           | `0b1111_0000` |
| Byte(`u8` apenas) |        `b'A'` |

A `_` pode ser usada como um separador, apenas para ajudar na leitura.

#### [Tipos de pontos flutuantes](https://doc.rust-lang.org/book/ch03-02-data-types.html#floating-point-types)

São para números com virgula, basicamente. O padrão deles é ser `f64`, ou seja, com `64 bits` de tamanho. Todos os _floats_ são assinados (podem ser negativos).

```rust
let x = 2.0; // f64

let y: f32 = 3.0; // f32
```

- É possivel fazer diferentes tipos de [operações matematicas](https://doc.rust-lang.org/book/appendix-02-operators.html) no Rust, tanto com **inteiros** quanto com **_Floats_**

#### [O Tipo Booleano](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-boolean-type)

Pode ser apenas `true` ou `false`, tem um byte (8 bits) de tamanho e é definido com a sintaxe `bool`.

```rust
    let t = true;

    let f: bool = false; // Com anotação explicita
```

#### [O tipo caractere](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-character-type)

É um único caractere [**Unicode**](https://home.unicode.org/), tem 4 bytes de tamanho, pode ser definido como `char` e precisa está entre aspas simples `''`.

```rust
let c = 'z';
let z: char = 'ℤ'; // Com anotação explicita
let heart_eyed_cat = '😻';
```

### [Tipos Compostos](https://doc.rust-lang.org/book/ch03-02-data-types.html#compound-types)

Tipos compostos pode agrupar vários valores em um tipo. A ferrugem tem dois tipos de compostos primitivos: tuplas e Arrays.

#### [O tipo tupla](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-tuple-type)

Tuplas tem o tamanho fixo, uma vez definido um tamanho ele será imutável, podem conter varios tipos de dados em uma única tupla. Definida com `()`.

```rust
let tup: (u32, bool, i8) = (222, false, -1);
let tup2 = (-44, 'A', 33);
```

Para obter os valores da tupla, podemos desestrutura-la.
As variaveis serão preenchidas respectivamente.

```rust
let tup = (-44, 'A', 33);
let (x, y, z) = tup; // x -> -44 ; y -> 'A'; z -> 33;
```

É possível acessar via indice também

```rust
let tup: (u32, bool, i8) = (214, true, -1);
let tup_val = tup.0; // tup_val -> 214
```

- **OBS.:**A tupla sem valores tem um nome especial, unidade. Este valor e seu o tipo correspondente é escrito `()` e representam um valor vazio ou um tipo de retorno vazio. As expressões retornam implicitamente o valor unitário se não o fizerem retornar qualquer outro valor.

#### [O tipo array](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-array-type)

Arrays tem um comprimento fixo e todos os seus valores dever obedecer a apenas um tipos. São definidos com `[]`.

```rust
let a = [1, 2, 3, 4, 5];
```

As matrizes são mais úteis quando você sabe que o número de elementos não o será preciso mudar

- Você escreve o tipo de uma matriz usando colchetes com o tipo de cada elemento, um ponto e vírgula e, em seguida, o número de elementos na matriz, assim:

```rust
let array: [i32; 5] = [1, 2, 3, 4, 5];
```

Também pode-se definir um valo inicial com:

```rust
let array = [3; 5]
//É o mesmo que dizer
let array = [3, 3, 3, 3, 3];
```

sendo assim `[tipo/valor_inicial; tamanho_do_array]`.

Pode-se acessar o valor em alguma posição do vetor com `nome_do_vetor[indice]`

```rust
let a = [1, 2, 3, 4, 5];

let first = a[0];
let second = a[1];
```

- Caso tente acessar um index fora to tamanho do vetor, ocorrerá uma erro de "index out of bounds"

## [Funções](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)

Funções podem executar um bloco de código no momento em que for preciso, são declaradas com `fn`. É usado por padrão _snake case_ para o nome das funções.

```rust
fn main() {
    println!("Hello, world!");

    another_function(); // <- "()" chama a função
}

fn another_function() {
    println!("Outra função.");
}
```

Rust não se importa onde você define suas funções, apenas que elas estejam definido em algum lugar em um escopo que pode ser visto pelo chamador. Ou seja, pode ser antes ou depois do `main`.

### [Parâmetros](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html#parameters)

É algum valor que é passado de fora para dentro do escopo da função. Valor esse, definico entre os parenteses `funcao(5)`.
Rust exige tipagem do parâmetro, pode ser qualquer tipo anteriormente citado.

```rust
fn main() {
    funcao(5) //chamada de função com parâmetro
}

fn funcao(x: i32) {
    println!("O valor de x é: {x}");
}
```

### [Declarações e Expressões](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html#statements-and-expressions)

Os corpos funcionais são compostos por uma série de declarações que terminam opcionalmente em um expressão.

- Declarações são instruções que realizam alguma ação e não retornam um valor. (faz algo mas não retorna)
- Expressões avaliar até um valor resultante. (retorna alguma coisa)

Ex.: `let` é uma sintaxe que faz algo, mas não retorna nada. Uma declaração basicamente.

```rust
let a = 6; // <- let é uma declaração
```

É possivel que qualquer bloco de código seja uma expressão.

```rust
let x = {
        let a = 3;
        a + 1 // Aqui precisa não ter o ; para dezer que a expressão não acabou
    };
println!("O valor de x é: {x}");
```

onde na expressão,

```rust
{
    let a = 3;
    a + 1
}
```

é um bloco que é avaliado como **4**. Observe que `a + 1` não tem o `;`, expressões não incluem ponto e vírgula(`;`) final. Se você adicionar um ponto e vírgula ao final de um expressão, você a transforma em uma instrução e ela não retornará um valor.

### [Funções com valores de retorno](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html#functions-with-return-values)

Os retornos de função em rust devem ser definidos na declaração da função se houver um retorno com `->`. Como já mostrado o retorno de uma função pode ser uma expressão final sem o `;`.

```rust
fn five() -> i32 {
    5
}
```

Outro exemplo com parâmetros

```rust
fn main() {
    let y = mais_um(5); // "y" agora tem o valor de 6, poís será o retono de função
}

fn mais_um(x: i32) -> i32 {
    x + 1
}
```

Essa função retorna o valor `5` mesmo sem a sintaxe `return`. O `return` permite sair de uma função antecipadamente da função.

## [Comentários](https://doc.rust-lang.org/book/ch03-04-comments.html)

Podem ser escritos com `//` e inutilizam tudo oque vem depois até a proxima linha

```rust
// Comentário
let x = 1; //Comentário na linha
//Outro comentário
```

## [Fluxo de controle](https://doc.rust-lang.org/book/ch03-05-control-flow.html)

### Sintaxe `if`

Basicamente `se` uma condição é atendida executa algo, `se não` faz outra coisa ou não faz nada.

```rust
if 1 <= 1 {
    //Faz algo
}
//Não faz nada

//if else
if 5 > 0 {
    //Faz algo
} else {
    //Faz outra coisa
}
```

Podem também ser aninhados como o seguinte

```rust
let a = 3;
if a == 1 {
    //Faz algor
} else if a == 2 {
    //Faz outra coisa
} else if a == 3 {
    // ...
} else {
    // Caso não seja nenhum dos outros
}
```

Necessariamente a condição precisa ser `bool`.

#### [Usando `if` com `let`](https://doc.rust-lang.org/book/ch03-05-control-flow.html#using-if-in-a-let-statement)

Como `if` é uma expressão, podemos usá-lo em conjunto com um `let` e atribuir o resultado a uma variavel

```rust
let condicao = true;
let numero = if condicao {5} else {6};
```

funcionando como um operador ternario, os valores a serem retornados precisam ser do mesmo tipo. Como no exemplo acima onde ambos são inteiros.

### [Repetição com Loops](https://doc.rust-lang.org/book/ch03-05-control-flow.html#repetition-with-loops)

Rust tem três tipos de loops: `loop`, `while`, e `for`.

#### [Loop](https://doc.rust-lang.org/book/ch03-05-control-flow.html#repeating-code-with-loop)

O `loop` é utilizado para executar um bloco de código repetidamente ou para sempre ou até que você diga explicitamente para parar.

```rust
loop {
    println!("again!");
}
```

Para sair de um `loop` ou qualquer outro laço de repetição basta usar o `break` e o `continue` para pular uma iterção do laço.

##### [Retornando valores de loops](https://doc.rust-lang.org/book/ch03-05-control-flow.html#returning-values-from-loops)

Um dos usos de a `loop` é tentar novamente uma operação que você sabe que pode falhar, como como verificar se um thread concluiu seu trabalho. Você também pode precisar passar o resultado dessa operação fora do `loop` para o resto do seu código. Pendência isso, você pode adicionar o valor que deseja retornar após o `break` expressão você use para parar o `loop`; esse valor será retornado para fora do `loop` para que você pode usá-lo

```rust
let mut counter = 0;

let result = loop {
    counter += 1;

    if counter == 10 {
        break counter * 2;
    }
};
```

##### [Desambiguando com rótulos de loop](https://doc.rust-lang.org/book/ch03-05-control-flow.html#disambiguating-with-loop-labels)

A desanbiguidade de loops, é basicamente adicionar um label (uma etiqueta) a um `loop`, `for`ou `while`. É utilizado normalmente quando se tem aninhamentos de loops e é preciso para um loops exterior. O `break` e o `continue` aplicam suas funções apenas nos loops mais internos, caso precise parar algum loops exterior, basta utilizar a _tag desse loop_, como o seguinte exemplo.

Se define um tag de loop com `'` antes do nome da _tag_ (`'tag_do_loop`).

```rust
fn main() {
    let mut contador_externo = 0;

    // Nomeamos o loop de fora como 'externo
    'externo: loop {
        println!("Contagem externa: {}", contador_externo);
        let mut contador_interno = 5;

        // Nomeamos o loop de dentro como 'interno (opcional, mas bom para clareza)
        'interno: loop {
            println!("  Contagem interna: {}", contador_interno);

            if contador_interno == 3 {
                println!("  -> Parando apenas o loop interno.");
                break; // Isso quebra apenas o loop 'interno
            }

            if contador_externo == 2 {
                println!("  -> Quebrando TUDO a partir do loop interno!");
                break 'externo; // Isso resolve a ambiguidade e quebra o loop de fora!
            }

            contador_interno -= 1;
        }

        contador_externo += 1;
    }

    println!("Fim do programa.");
}
```

O mesmo vale para `for` e `while`.

```rust
fn main() {
    let matriz = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9],
    ];

    let alvo = 5;

    'busca: for linha in matriz {
        for numero in linha {
            if numero == alvo {
                println!("Encontramos o {}! Parando a busca inteira.", alvo);
                // Se usássemos apenas "break", ele iria para a próxima linha.
                // Com "break 'busca", ele sai dos dois loops imediatamente.
                break 'busca;
            }
        }
    }
}
```

#### [while](http://doc.rust-lang.org/book/ch03-05-control-flow.html#streamlining-conditional-loops-with-while)

O `while` executa o bloco de código **_enquanto_** sua condição for verdadeira.
Caso não seja, não executará mais.

Uma forma interssante de se ler o `while` é: "`enquanto` isso (condição) acontece, execute isso (bloco de código)".

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```

#### [for](https://doc.rust-lang.org/book/ch03-05-control-flow.html#looping-through-a-collection-with-for)

O `for` vai iterar (percorrer índice por índice) sobre algo. Como uma lista ou tupla.

Uma forma interessante de se ler o `for` é: "`para` cada elemento em _algo_ (lista iterada) execute isso (bloco entre `{}`)".

```rust
let a = [10, 20, 30, 40, 50];

for element in a {
    println!("the value is: {element}");
}
```
