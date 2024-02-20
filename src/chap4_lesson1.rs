#[allow(dead_code)]
pub fn chap4_lesson1() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    // s1 is no longer valid here

    println!("The length of '{}' is {}.", s2, len);
}

#[allow(dead_code)]
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
