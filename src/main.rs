fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let first = v[0];
    println!("The first element is {first}");

    let third = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }

    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 37, 50];
    for i in &mut v {
        *i += 50;
    }

    println!("{:?}", v);
}
