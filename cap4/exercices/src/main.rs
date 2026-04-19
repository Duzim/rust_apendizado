fn main() {
    let mut comando = String::from("build");

    // Chame sua função aqui

    println!("{}", comando); // Deve imprimir: "Cargo: build"
}

fn add_prefix(&str) -> &str {
    "Cargo: {str}"
}