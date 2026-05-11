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

fn buscar_saldo(bd: &HashMap<String, f64>, cliente: &str) -> Result<f64, String> {
    match bd.get(cliente) {
        // Desestruturamos a referência com &saldo,
        // assim a variável 'saldo' é um f64, não um &f64
        Some(&saldo) => Ok(saldo),
        None => Err(format!("Cliente [{}] não cadastrado.", cliente)),
    }
}
