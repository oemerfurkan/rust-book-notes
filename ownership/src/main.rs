/*
OWNERSHIP
    - Stack
        * Last in, First out
    - Heap
        * Uncertain sized elements
    - Ownership rules
        * Each value in Rust has an owner.
        * There can only be ONE owner at a time.
        * When the owner goes out of scope, the value will be dropped.
*/

fn main() {
    //SCOPE
    {
        // s is not valid here, it’s not yet declared
        let s: &str = "hello"; // s is valid from this point forward

        // do stuff with s
    } // this scope is now over, and s is no longer valid

    let s = String::from("Deneme");
    // If the .clone() didn't used
    // the s value will be moved to s5 and the s cannot be used.
    let mut s5 = s.clone();

    s5.push_str("Deneme");
    println!("s: {}, s5: {}", s, s5);

    let mut s2 = "Deneme2";
    let s3 = s2;

    s2 = "Deneme3";

    println!("s2: {} s3: {}", s2, s3);

    let s10 = String::from("hello"); // s comes into scope

    takes_ownership(s10); // s's value moves into the function...
                          // ... and so is no longer valid here

    // borrow of moved value: `s10`
    // println!("s10: {}", s10);
    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward

    let s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {             // gives_ownership will move its
    // return value into the function
    // that calls it

let some_string = String::from("yours"); // some_string comes into scope

some_string                              // some_string is returned and
    // moves out to the calling
    // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
             // scope

a_string  // a_string is returned and moves out to the calling function
}