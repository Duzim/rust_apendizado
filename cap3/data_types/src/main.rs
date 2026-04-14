fn main() {
    let val: u32 = "43".parse().expect("Valor deve ser um númerico.");

    println!("{}", val);

    let valf: f32 = 2.0;
    println!("{}", valf);

    let tup: (u32, bool, i8) = (214, true, -1);
    let (_x, y, _z) = tup;

    println!("Desestruturado: {}", y);
    println!("Por indice: {}", tup.0);

    let array: [i32; 5] = [2, 5, -2, -5, 5];
    println!("{}", array[2]);
}
