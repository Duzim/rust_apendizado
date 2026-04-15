fn main() {
    let number_of_men_you_kissed = 6;

    if number_of_men_you_kissed > 5 {
        println!("Você é gay");
    } else if number_of_men_you_kissed > 2 {
        println!("Tendências gay");
    } else {
        println!("Tá podendo beijar mais");
    }

    let a = if true { 5 } else { 6 };
    println!("{}", a)
}
