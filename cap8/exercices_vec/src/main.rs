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
