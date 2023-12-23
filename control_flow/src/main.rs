/*
CONTROL FLOW
    -If Expressions
        * Expects bool (No JavaScript type if)
        * let number = if condition {1} else {0}
*/

fn main() {
    let number1 = 5;

    if number1 < 7 {
        println!("Number is lower than 7.")
    } else {
        println!("Number is equal or higher than 7.")
    }


    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number2 = if condition { 5 } else { 6 };

    println!("The value of number is: {number2}");
    
    // TYPES IN THE CONDITION MUST MATCH
    // let condition = true;

    // let number = if condition { 5 } else { "six" };

    // println!("The value of number is: {number}");
}
