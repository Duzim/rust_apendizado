fn main() {
    let status = String::from("Ativo");

    let r1 = &status;
    let r2 = &status;
    let r3 = &status; // O PROBLEMA ESTÁ AQUI

    println!("{}, {}, e {}", r1, r2, r3);
}
