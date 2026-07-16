// use std::cmp::PartialOrd;

// fn largest<T: PartialOrd>(list: &[T]) -> &T {
//     let mut largest_val = &list[0];

//     for item in list {
//         if item > largest_val {
//             largest_val = item;
//         }
//     }
//     largest_val
// }

fn main() {
    //let number_list = vec![34, 50, 25, 100, 65];

    //let result = largest(&number_list);
    //println!("The largest number is {result}");

    // let char_list = vec!['y', 'm', 'a', 'q'];

    // let result = largest(&char_list);
    // println!("The largest char is {result}");

    let name = "aaaaaaaaaa da silva";

    let res = first_word(name);

    println!("{}", res);

    let m = maior("aaaa", "bbbbb");

    println!("{m}");
}

fn maior<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            println!("i = {}", i);
            return &s[0..i];
        }
    }

    &s[..]
}
