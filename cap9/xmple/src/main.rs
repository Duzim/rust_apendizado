use std::fs::File;
//use std::io;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => {
            println!("{:?}", file);
            file
        },
        Err(e) => panic!("Problema ao tentar abrir o arquivo: [{:?}]", e),
    };

    // loop {
    //     let mut cmd = String::new();
    //     io::stdin().read_line(&mut cmd).expect("Aconteceu um erro!");
    //     match cmd.trim() {
    //         "panic" => {
    //             panic!("Erro Aconteceu Devido a macro 'panic!'");
    //         }
    //         "quit" => {
    //             println!("Saindo por 'break'.");
    //             break;
    //         }
    //         comand => {
    //             println!("{:?}", &comand);
    //         }
    //     }
    // }
    // println!("Saiu.");
}
