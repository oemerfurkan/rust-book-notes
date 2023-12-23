fn main() {
    let mut v: Vec<i32> = Vec::new();

    v.push(5);

    // Getting values with indexing
    let first = &v[0];

    // Getting values with .get(index: i32) method
    let first = v.get(0);

    match first {
        Some(val) => println!("{val}"),
        None => println!("None"),
    }

    let mut v2 = vec![1,2,3];

    for i in &v2 {
        println!("{i}");
    }

    for i in &mut v2 {
        *i += 5;
    }

    println!("{v2:?}");

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{row:?}");
}
