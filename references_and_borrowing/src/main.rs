/*
REFERENCES AND BORROWING
*/

fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s2 = String::from("hello");

    change(&mut s2);

    let r1 = &mut s2;
    // Cannot borrow s2 as mutable more than once
    // let r2 = &mut s2;

    println!("{}", r1); //r2);

    println!("{s2}");

    let r1 = &s2; // no problem
    let r2 = &s2; // no problem
    // let r3 = &mut s2; // BIG PROBLEM

    println!("{}{}", r1, r2);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world!");
}