fn main() {
    let ret = Retangulo {
        width: 50,
        heigth: 60,
    };

    let other_ret_1 = Retangulo {
        width: 30,
        heigth: 30,
    };
    let other_ret_2 = Retangulo {
        width: 80,
        heigth: 30,
    };

    let other_ret_3 = Retangulo::square(32);

    println!("ret 1 cabe em ret: {:?}", ret.can_hold(&other_ret_1));
    println!("ret 2 cabe em ret: {:?}", ret.can_hold_area(&other_ret_2));
    println!("ret 2 cabe em ret: {:?}", other_ret_3);
    //let area_ret = calc_area(&ret);
    //println!("Area do retangulo é: {}", area_ret)

    println!("Area do retangulo é: {}", ret.calc_area());
    println!("{:?}", ret.width);
    println!("{:?}", ret.width());
    //dbg!(&ret); // Pra debugar e mostrar o formato do struct
}

// fn calc_area(ret: &Retangulo) -> String {
//     let a = ret.width * ret.heigth; //não precisam de & pois são u32
//     let mut str_a = a.to_string();
//     str_a.push_str(&ret.unit); // Precisa de & pois é String
//     str_a
// }

#[derive(Debug)] // Diretiva que permite mostrar o debug
struct Retangulo {
    width: u32,
    heigth: u32,
}
impl Retangulo {
    fn calc_area(&self) -> u32 {
        self.width * self.heigth
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold_area(&self, other: &Retangulo) -> bool {
        let my_size = self.calc_area();
        let other_size = other.calc_area();
        my_size > other_size
    }

    fn can_hold(&self, other: &Retangulo) -> bool {
        self.width > other.width && self.heigth > other.heigth
    }

    //Static
    fn square(size: u32) -> Self {
        Self {
            width: size,
            heigth: size,
        }
    }

    fn new(size_h: u32, size_w: u32) -> Self {
        Self {
            heigth: size_h,
            width: size_w,
        }
    }
}
