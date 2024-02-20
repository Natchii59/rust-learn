#[allow(dead_code)]
pub fn chap4_lesson2() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut string_change = String::from("Hello how are you?");

    change(&mut string_change);

    println!("{string_change}");

    let s2 = String::from("ahah");

    let r2 = &s2;
    let r3 = &s2;

    println!("old {r2} {r3}");
}

#[allow(dead_code)]
fn calculate_length(s: &String) -> usize {
    s.len()
}

#[allow(dead_code)]
fn change(some_string: &mut String) {
    some_string.push_str(" is changed.");
}
