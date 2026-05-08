use std::io;

fn main() {
    let mut v: Vec<i32> = Vec::new();

    println!("com Vec<T>: {:?}", v);

    let v1 = vec![1, 2, 3];

    for i in &v1 {
        //daria erro sem o & pois perderia a propriedade dos valores
        println!("{:?}", i);
    }

    println!("com vec![]: {:?}", v1);

    v.push(1);
    v.push(2);
    v.push(-1);
    v.push(-2);

    for i in &mut v {
        *i += 2;
    }

    println!("com Vec<T>: {:?}", v);

    // let mut comands: Vec<String> = vec![];
    // loop {
    //     let mut input = String::new();
    //     io::stdin().read_line(&mut input).expect("Error to read!");

    //     match input.trim() {
    //         "ls" => {
    //             for cmd in &comands {
    //                 println!("> {}", cmd);
    //             }
    //         }
    //         "remove" => {
    //             let removed_value = comands.pop();
    //             match removed_value {
    //                 Some(r_val) => {
    //                     println!("removendo da lista: {}", r_val);
    //                 }
    //                 None => (),
    //             }
    //         }

    //         "quit" => {
    //             break;
    //         }
    //         _ => {}
    //     }
    //     comands.push(String::from(&input));
    // }
}
