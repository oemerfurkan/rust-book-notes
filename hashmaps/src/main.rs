use std::collections::{HashMap, hash_map};

fn main() {
    println!("Hello, world!");

    let mut scores: HashMap<String, u32> = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 20);

    let red = String::from("Red");
    let blue = String::from("Blue");

    let red_point = scores.get(&red).copied().unwrap_or(0);
    let blue_point = scores.get(&blue).copied().unwrap_or(0);

    println!("Red Team: {red_point}, Blue Team: {blue_point}");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    let mut scores2 = HashMap::new();

    scores2.insert(String::from("Blue"), 10);
    // This insert overrides the value.
    scores2.insert(String::from("Blue"), 25);

    println!("{:?}", scores2);
}
