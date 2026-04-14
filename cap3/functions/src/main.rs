fn main() {
    println!("funçao: {}", func(5));

    let x = {
        let a = 3;
        a + 1 // Aqui precisa não ter o ; para dezer que a expressão não acabou
    };
    println!("O valor de x é: {x}");
}

fn func(x: i32) -> i32 {
    return x;
}
