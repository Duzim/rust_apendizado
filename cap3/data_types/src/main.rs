fn main() {
    let val: u32 = "43".parse().expect("Valor deve ser um númerico.");

    println!("{}", val);

    let valf: f32 = 2.0; 
    println!("{}", valf);
}
