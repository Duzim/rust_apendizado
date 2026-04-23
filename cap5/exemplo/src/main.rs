fn main() {
    let ret = Retangulo {
        width: 50,
        heigth: 60,
        unit: String::from("cm"),
    };

    //let area_ret = calc_area(ret);

    println!("Area do retangulo é: {}", ret.calc_area());
}

// fn calc_area(ret: Retangulo) -> String {
//     let a = ret.width * ret.heigth;
//     let mut str_a = a.to_string();
//     str_a.push_str(&ret.unit);
//     str_a
// }

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
