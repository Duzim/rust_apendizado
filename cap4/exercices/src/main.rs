fn main() {
    let frase = String::from("Aprendendo Rust agora");
    let palavra = ultima_palavra(&frase);

    println!("A última palavra é: [{}]", palavra); // Deve imprimir: "agora"
}

fn ultima_palavra(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().rev().enumerate() {
        if item == b' ' {
            let a = s.len();
            return &s[a - i..];
        }
    }
    &s
}
