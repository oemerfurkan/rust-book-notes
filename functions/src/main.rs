/*
FUNCTIONS
    - Keyword: fn
    - Entry Point: fn main() {}
    - Convention: snake_case
*/

fn main() {
    println!("Hello, world!");
    another_function(5);
    print_labeled_measurement(5, 'm');

    let y = {
        let x = 5;
        x + 1
    };

    println!("y: {y}");
    println!("five(): {}", five());
    println!("plus_one(5): {}", plus_one(5));
}

fn another_function(x: i32) {
    println!("Number is: {x}")
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    // Expression
    x + 1
    // Statement
    // x + 1 returns i32 but x + 1; returns nothing
    // () unit type != i32
    // x + 1;
}