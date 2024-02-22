#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[allow(dead_code)]
pub fn lesson2() {
    let rect = Rectangle {
        width: 50,
        height: 50,
    };

    println!("The area of rectangle is {}", area(&rect));

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(scale * 2),
        height: 50,
    };

    dbg!(&rect2);
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
