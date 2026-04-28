# Exercicios de fixação

## Exercicio 1 - O Problema: Despachante de Eventos (Event Dispatcher)

Você precisa modelar um sistema que recebe diferentes tipos de eventos de uma aplicação e os processa.

### Etapa 1: Definir o Enum
Crie um `enum` chamado `EventoApi` que represente três situações distintas que seu framework pode receber:

1. **Ping:** Um evento simples de checagem de saúde (health check) que não carrega nenhum dado.

2. **ProcessamentoBackground:** Um evento que carrega um ID de tarefa (um número inteiro `u32`). Representa um job que deve ser executado em segundo plano.

3. **RequisicaoWeb:** Um evento que carrega dados nomeados (uma estrutura anônima dentro do enum). Deve conter:

    - `rota`: Uma `String`.

    - `metodo`: Uma `String` (ex: "GET", "POST").

    - `token_auth`: Um `Option<String>`, pois a requisição pode ou não vir com um token de autenticação.

### Etapa 2: Implementar a Lógica de Processamento
Crie uma função chamada `processar_evento` que receba um `EventoApi` como parâmetro e não retorne nada. Dentro desta função, você deve usar a expressão `match` para tratar cada variante:

- Se for um **Ping**, imprima no console: `"Status: Servidor rodando (Pong)"`.

- Se for um ProcessamentoBackground, imprima: `"Iniciando worker para o Job ID: [numero_do_id]"`.

- Se for uma RequisicaoWeb, a lógica é dupla:

    - Se o `token_auth` possuir um valor, imprima: `"Processando [metodo] na rota [rota] com usuário autenticado."`

    - Se o `token_auth` for nulo (`None`), imprima: `"Acesso negado para [metodo] na rota [rota]. Token ausente."`
```rust
// Escreva seu enum EventoApi aqui

// Escreva sua função processar_evento aqui

fn main() {
    let evento_saude = EventoApi::Ping;
    
    let evento_job = EventoApi::ProcessamentoBackground(4042);
    
    let evento_web_publico = EventoApi::RequisicaoWeb {
        rota: String::from("/home"),
        metodo: String::from("GET"),
        token_auth: None,
    };
    
    let evento_web_privado = EventoApi::RequisicaoWeb {
        rota: String::from("/dashboard"),
        metodo: String::from("POST"),
        token_auth: Some(String::from("abc-123-token")),
    };

    println!("--- Processando Eventos ---");
    processar_evento(evento_saude);
    processar_evento(evento_job);
    processar_evento(evento_web_publico);
    processar_evento(evento_web_privado);
}
```
Resp.:

<details>
<summary>Resposta do exercicio acima:</summary>

Minha Resposta:

```rust
// Escreva seu enum EventoApi aqui
enum EventoApi {
    Ping,
    ProcessamentoBackground(i32),
    RequisicaoWeb {
        rota: String,
        metodo: String,
        token_auth: Option<String>,
    },
}
// Escreva sua função processar_evento aqui
fn processar_evento(event: EventoApi) {
    match event {
        EventoApi::Ping => {
            println!("Status: Servidor rodando (Pong)");
        }
        EventoApi::ProcessamentoBackground(id) => {
            println!("Iniciando worker para o Job ID: [{}]", id);
        }
        EventoApi::RequisicaoWeb {
            rota,
            metodo,
            token_auth,
        } => {
            if let Some(_) = token_auth {
                println!("Processando [{metodo}] na rota [{rota}] com usuário autenticado.");
            } else {
                println!("Acesso negado para [{metodo}] na rota [{rota}]. Token ausente.");
            }
        }
    }
}
```

</details>


## Exercício 2: Roteamento de um Framework MVC

- **Objetivo:** Definir um `enum` com variantes que armazenam diferentes tipos e quantidades de dados (unitárias, tipo tupla e tipo struct anônima) e usar `match` para extraí-los.

Vamos modelar o sistema de rotas (routes) de um framework web.

1. Defina um enum chamado `Rota`. Ele deve ter três variantes:

    - `Home` (sem dados, representa a página inicial).

    - `Usuario` (armazena um único `u32` representando o ID do usuário).

    - `Busca` (armazena os campos nomeados `termo: String` e `limite: u8`).

2. Crie uma função autônoma (fora de blocos impl) chamada `despachar_rota(rota: Rota)`.

3. Dentro dessa função, use a expressão match para tratar cada variante:

    - Para `Home`, imprima `"Carregando a página inicial."`.

    - Para `Usuario`, imprima `"Buscando perfil do usuário ID: [id_aqui]"`.

    - Para `Busca`, imprima `"Buscando por '[termo_aqui]' com limite de [limite_aqui] resultados."`.
```rust
fn main() {
    let rota_padrao = Rota::Home;
    let rota_perfil = Rota::Usuario(105);
    let rota_pesquisa = Rota::Busca { 
        termo: String::from("Rust MVC"), 
        limite: 10 
    };

    despachar_rota(rota_padrao);
    despachar_rota(rota_perfil);
    despachar_rota(rota_pesquisa);
}
```
Resp.:

<details>
<summary>Resposta do exercicio acima:</summary>

Minha Resposta:

```rust
enum Rota {
    Home,
    Usuario(u32),
    Busca { termo: String, limite: u8 },
}

fn despachar_rota(rota: Rota) {
    match rota {
        Rota::Home => {
            println!("Carregando a página inicial.");
        }
        Rota::Usuario(id) => {
            println!("Buscando perfil do usuário ID: [{id}]");
        }
        Rota::Busca { termo, limite } => {
            println!(
                "Buscando por '[{}]' com limite de [{}] resultados.",
                termo, limite
            );
        }
    }
}
```

</details>

## Exercício 3: Mapeamento de Terrenos e o Enum Option

- **Objetivo:** Combinar `structs` e `enums`, além de utilizar o enum padrão `Option<T>` para representar dados que podem ou não estar presentes de forma segura (sem null pointers).

Em sistemas de Informações Geográficas (GIS), um ponto no mapa pode ter uma descrição detalhada, mas às vezes essa informação simplesmente não existe.

1. Crie um enum `TipoTerreno` com as variantes: `Agua`, `Floresta` e `Urbano`.

2. Crie uma struct `PontoInteresse` contendo:

    - `nome` (`String`)

    - `terreno` (`TipoTerreno`)

    - `descricao` (`Option<String>`)

3. Crie um bloco `impl` para `PontoInteresse` e adicione um método `exibir_info(&self)`.

4. No método `exibir_info`, imprima o nome do ponto. Em seguida, use um `match` na propriedade `descricao` (lembre-se que ela é um `Option` e pode ser `Some(valor)` ou `None`):

    - Se houver uma descrição, imprima `"Descrição: [texto]"`.

    - Se não houver (`None`), imprima `"Descrição: Não disponível."`.

```rust
fn main() {
    let lago = PontoInteresse {
        nome: String::from("Lago Paranoá"),
        terreno: TipoTerreno::Agua,
        descricao: Some(String::from("Lago artificial em Brasília.")),
    };

    let area_desconhecida = PontoInteresse {
        nome: String::from("Setor XYZ"),
        terreno: TipoTerreno::Urbano,
        descricao: None,
    };

    lago.exibir_info();
    println!("---");
    area_desconhecida.exibir_info();
}
```

Resp.:

<details>
<summary>Resposta do exercicio acima:</summary>

Minha Resposta:

```rust
enum TipoTerreno {
    Agua,
    Floresta,
    Urbano,
}
struct PontoInteresse {
    nome: String,
    terreno: TipoTerreno,
    descricao: Option<String>,
}
impl PontoInteresse {
    fn exibir_info(&self) {
        println!("Ponto de Interesse: {}", self.nome);
        match &self.descricao {
            Some(text) => {
                println!("Descrição: {}", text);
            }
            None => {
                println!("Sem descrição");
            }
        }
    }
}

```

</details>

## Exercício 4: Filtragem Limpa com if let

- **Objetivo:** Refatorar um `match` excessivamente verboso usando a estrutura de controle `if let`, focando em capturar apenas uma variante de interesse.

Sistemas de eventos geram muito ruído. Imagine que você está processando interações do usuário. Você só quer executar uma ação quando o usuário digita uma tecla, ignorando cliques do mouse ou fechamentos de janela.

1. Analise o enum `EventoJanela` já fornecido no código base.

2. Em vez de usar um `match` exaustivo com um catch-all (`_ => ()`), use a sintaxe `if let` para verificar se o evento é um `Teclado`.

3. Se for, desestruture o caractere associado à variante e imprima `"Tecla pressionada: [caractere]"`. Se não for, o código simplesmente não deve fazer nada.

```rust
enum EventoJanela {
    CliqueNoMouse(i32, i32), // x, y
    Teclado(char),
    Sair,
}

fn processar_evento_log(evento: EventoJanela) {
    // TODO: Escreva o bloco if let aqui para tratar APENAS a variante Teclado
}

fn main() {
    let evento1 = EventoJanela::CliqueNoMouse(10, 20);
    let evento2 = EventoJanela::Teclado('R');
    let evento3 = EventoJanela::Sair;

    processar_evento_log(evento1);
    processar_evento_log(evento2); // Apenas este deve imprimir algo
    processar_evento_log(evento3);
}
```

Resp.:

<details>
<summary>Resposta do exercicio acima:</summary>

Minha Resposta:

```rust
```

</details>