# Exercicios de fixação 

## Exercício 1: Fundamentos de Geoinformática

- **Objetivo**: Praticar a criação de Structs clássicas, funções associadas (construtores) e métodos com referência imutável (`&self`).

Crie um sistema básico para representar coordenadas geográficas num mapa.

1. Defina uma struct chamada PontoGeografico com dois campos: latitude e longitude (ambos do tipo f64).

2. Crie um bloco impl para PontoGeografico.

3. Implemente uma função associada chamada novo que recebe a latitude e longitude e retorna uma nova instância (use o atalho de inicialização de campos, se possível).

4. Implemente um método chamado hemisferio que não altera o ponto (recebe &self) e imprime se o ponto está no Hemisfério Norte (latitude > 0) ou Sul (latitude < 0), e Leste (longitude > 0) ou Oeste (longitude < 0).

```rust
fn main() {
    let brasilia = PontoGeografico::novo(-15.793889, -47.882778);
    brasilia.hemisferio(); // Deve imprimir algo como: "Hemisfério Sul e Oeste"
}
```
Resp.:

<details>
<summary>Resposta do exercicio acima:</summary>

Minha Resposta:

```rust
fn main() {
    let brasilia = PontoGeografico::novo(-15.793889, -47.882778);
    brasilia.hemisferio(); // Deve imprimir algo como: "Hemisfério Sul e Oeste"
}

struct PontoGeografico {
    latitude: f64,
    longitude: f64,
}
impl PontoGeografico {
    fn novo(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
        }
    }

    fn hemisferio(&self) -> String {
        let mut hemis = String::new();

        hemis.push_str(if self.latitude > 0.0 { "Hemisfério Norte" } else { "Hemisfério Sul" });

        hemis.push_str(if self.longitude > 0.0 { " e Leste" } else { " e Oeste" });
        println!("{:?}", hemis);
        hemis
    }
}
```
</details>

## Exercício 2: O Motor do Sistema de Gestão
- **Objetivo:** Trabalhar com mutabilidade controlada (`&mut self`) e alteração de estado interno.

Vamos modelar o núcleo de um sistema de controle de faturamento e estoque.

1. Defina uma `struct` chamada Produto contendo: `nome` (`String`), `preco` (`f64`) e `quantidade_estoque` (`u32`).

2. No `impl`, crie a função associada `novo(nome: String, preco: f64) -> Self`. O estoque deve começar sempre em 0.

3. Implemente um método `receber_lote(&mut self, quantidade: u32)` que adiciona itens ao estoque.

4. Implemente um método `processar_venda(&mut self, quantidade: u32) -> bool`. Este método deve:

    - Verificar se há estoque suficiente.

    - Se houver, subtrair a quantidade do estoque e retornar `true`.

    - Se não houver, imprimir uma mensagem de "Estoque insuficiente" e retornar `false`.

```rust
fn main() {
    let mut item = Produto::novo(String::from("Servidor Rack 1U"), 4500.00);
    
    item.processar_venda(2); // Deve falhar (estoque 0)
    item.receber_lote(5);
    
    if item.processar_venda(2) {
        println!("Venda aprovada! Estoque restante: {}", item.quantidade_estoque);
    }
}
```

Resp.:
<details>
<summary>Resposta do exercicio acima:</summary>

Minha Resposta:

```rust
fn main() {
    let mut item = Produto::novo(String::from("Servidor Rack 1U"), 4500.00);

    item.processar_venda(2); // Deve falhar (estoque 0)
    item.receber_lote(5);

    if item.processar_venda(2) {
        println!(
            "Venda aprovada! Estoque restante: {}",
            item.quantidade_estoque
        );
    }
}

#[derive(Debug)]
struct Produto {
    nome: String,
    preco: f64,
    quantidade_estoque: u32,
}
impl Produto {
    fn novo(nome: String, preco: f64) -> Self {
        Self {
            nome,
            preco,
            quantidade_estoque: 0,
        }
    }

    fn receber_lote(&mut self, quantidade: u32) {
        self.quantidade_estoque += quantidade;
    }

    fn processar_venda(&mut self, qtn_venda: u32) -> bool {
        if self.quantidade_estoque < qtn_venda {
            println!("Falha na venda {:?}", self);
            return false;
        }
        self.quantidade_estoque -= qtn_venda;
        println!("Venda processada {:?}", self);
        true
    }
}
```

</details>

## Exercício 3: Composição de Structs e Ownership

- **Objetivo:** Usar uma `struct` dentro de outra `struct` e manipular propriedades aninhadas, simulando componentes de uma arquitetura complexa.

No processamento de dados espaciais, é comum calcularmos uma "Caixa Delimitadora" (Bounding Box) para saber a área que abrange determinados pontos.

1. Aproveite a struct `PontoGeografico` do Exercício 1.

2. Crie uma nova struct chamada CaixaDelimitadora que contém dois campos: `canto_inferior_esquerdo` e `canto_superior_direito`. Ambos devem ser do tipo `PontoGeografico`.

3. Crie um `impl` para a `CaixaDelimitadora` com um método `largura(&self) -> f64`. A largura é a diferença absoluta entre a longitude dos dois cantos. (Dica: em Rust, você pode usar o método `.abs()` em um `f64` para obter o valor absoluto, ex: `(a - b).abs()`).

4. Crie um método `altura(&self) -> f64` (diferença absoluta entre as latitudes).

```rust
fn main() {
    let p1 = PontoGeografico::novo(-23.5505, -46.6333); // SP
    let p2 = PontoGeografico::novo(-22.9068, -43.1729); // RJ
    
    let bounding_box = CaixaDelimitadora {
        canto_inferior_esquerdo: p1,
        canto_superior_direito: p2,
    };
    
    println!("Largura da região: {:.4} graus", bounding_box.largura());
    println!("Altura da região: {:.4} graus", bounding_box.altura());
}
```

Resp.:

<details>
<summary>Resposta do exercicio acima:</summary>

Minha Resposta:

```rust
```

</details>

## 

- **Objetivo:** aaa

```rust
```

Resp.:

<details>
<summary>Resposta do exercicio acima:</summary>

Minha Resposta:

```rust
```

</details>