use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);

    let team_name = String::from("blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("The score is {score}");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    scores.entry(String::from("blue")).or_insert(100);
    scores.entry(String::from("green")).or_insert(100);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
