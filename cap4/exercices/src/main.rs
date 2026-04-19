fn main() {
    let mut comando = String::from("build");
    add_prefix(&mut comando);
    println!("{}", comando); // Deve imprimir: "Cargo: build"
}

fn add_prefix(str_value: &mut String) {
    let mut temp = String::from("Cargo: ");
    temp.push_str(str_value);
    *str_value = temp;
}
