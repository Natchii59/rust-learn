#[allow(dead_code)]
pub fn function() {
    let x = five();

    println!("The value of x is: {x}");
}

#[allow(dead_code)]
fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

#[allow(dead_code)]
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}")
}

#[allow(dead_code)]
pub fn statement() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

fn five() -> i32 {
    5
}

#[allow(dead_code)]
fn plus_one(x: i32) -> i32 {
    x + 1
}
