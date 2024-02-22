#[derive(Debug)]
#[allow(dead_code)]
pub struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
pub struct Color(i32, i32, i32);

#[derive(Debug)]
pub struct AlwaysEqual;

#[allow(dead_code)]
pub fn lesson1() {
    let user1 = User {
        active: true,
        username: String::from("natchi"),
        email: String::from("example@mail.com"),
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("example1@mail.com"),
        ..user1
    };

    // Username is on user2
    // println!("{:#?}", user1.username);
    println!("{:#?}", user2);

    let color1 = Color(0, 0, 0);

    println!("{:?}", color1);

    let subject = AlwaysEqual;

    println!("{:?}", subject);
}

#[allow(dead_code)]
pub fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        email,
        username,
        sign_in_count: 1,
    }
}
