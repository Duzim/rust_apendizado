fn main() {
    let ret = Retangulo {
        width: 50,
        heigth: 60,
        unit: String::from("cm"),
    };

    //let area_ret = calc_area(&ret);
    //println!("Area do retangulo é: {}", area_ret)

    println!("Area do retangulo é: {}", ret.calc_area());
    //dbg!(&ret); // Pra debugar e mostrar o formato do struct
}

// fn calc_area(ret: &Retangulo) -> String {
//     let a = ret.width * ret.heigth; //não precisam de & pois são u32
//     let mut str_a = a.to_string();
//     str_a.push_str(&ret.unit); // Precisa de & pois é String
//     str_a
// }

//#[derive(Debug)] // Diretiva que permite mostrar o debug
struct Retangulo {
    width: u32,
    heigth: u32,
    unit: String,
}
impl Retangulo {
    fn calc_area(&self) -> String {
        let a = self.width * self.heigth;
        let mut str_a = a.to_string();
        str_a.push_str(&self.unit);
        str_a
    }
}
