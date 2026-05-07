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
}
