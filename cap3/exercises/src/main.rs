fn main() {
    //let value = fahrenheit_and_celsius(32.0);
    let value = fibonacci(15);
    println!("{}", value);

    let value2 = fibonacci_2(15);
    println!("{}", value2);
}

// fn fahrenheit_and_celsius(value: f32, is_fahrenheit: bool) -> f32 {
//     return if is_fahrenheit {
//         (value / 32.0) / 1.8
//     } else {
//         (value * 1.8) / 32.0
//     };
// }

fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return 1;
    };
    fibonacci(n - 1) + fibonacci(n - 2)
}

fn fibonacci_2(mut n: u32) -> u32 {
    let mut temp;
    let mut a = 1;
    let mut b = 0;
    loop {
        temp = a;
        a += b;
        b = temp;
        if n <= 0 {
            break b;
        }
        n -= 1;
    }
}
