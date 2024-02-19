use std::io;

#[allow(dead_code)]
pub fn data_type() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{guess}")
}

#[allow(dead_code)]
pub fn tuple() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // let (x, y, z) = tup;
    let x = tup.0;
    let y = tup.1;
    let z = tup.2;

    println!("{:?}", tup);

    println!("X value: {x}");
    println!("Y value: {y}");
    println!("Z value: {z}");
}

#[allow(dead_code)]
pub fn array() {
    let arr = [1, 2, 3, 4, 5];

    println!("{:?}", arr);

    let arr2: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", arr2);

    // Declare array with duplicate value of 3 and length of 5
    let arr3 = [3; 5];

    println!("{:?}", arr3);

    let first = arr[0];
    let second = arr[1];

    println!("First value: {first}");
    println!("Second value: {second}");
}

#[allow(dead_code)]
pub fn element_array() {
    let arr = [1, 2, 3, 4, 5];

    println!("Enter an array index.");

    let mut key = String::new();

    io::stdin()
        .read_line(&mut key)
        .expect("Please enter an array index.");

    let index: usize = key.trim().parse().expect("Please enter a valid number.");

    // Warning: This will panic if the index is out of bounds.
    let element = arr[index];

    println!("The value of the element at {index} is: {element}");
}
