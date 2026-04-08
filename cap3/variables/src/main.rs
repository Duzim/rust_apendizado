fn main() {
    const A: i32 = 20 * 20;
    println!("Variavel A = {A}");
    let mut x = 5;
    println!("Variavel x = {x}");
    x = 6;
    println!("Variavel x = {x}");

    println!("================");

    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("O valore de y dentro do escopo é: {y}")
    }
    println!("O valore de y fora do escopo é: {y}")
}
