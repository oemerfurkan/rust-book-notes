/*
DATA TYPES
- Scalar
    * Integers
        i8, i16, i32, i64, i128
        u8, u16, u32, u64, u128
    * Floating-Point Numbers
        f32, f64
    * Booleans
        true, false
    * Characters
- Compound
    * Tuples
        - Fixed Length
        - let tup: (i32, u32, bool) = (31, 32, true)
    * Arrays
        - Fixed Length
        - let arr: [u32;  5   ] = [1,2,3,4,5]
                   [type, size]
        - let arr = [3    ; 5   ] // [3,3,3,3,3]
                    [value; size]
 */
use std::io;
fn main() {
    let x: f32 = -5.0 / 3.0;
    println!("Value: {x}");
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Extract Tuple
    let (a, b, c) = tup;

    // Indexing Tuple
    let five_hundred = tup.0;
    println!("{a} {b} {c} {five_hundred}");

    let arr: [i32; 5] = [1,2,3,4,5];
    println!("First Element: {}", arr[0]);

    println!("Please enter an index!");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Enter valid index!");

    let index: usize = index
        .trim()
        .parse()
        .expect("Not a valid input!");

    // If entered an index that out of the array's bounds there
    // will be a runtime error: 'index out of bounds'
    println!("Value at the index: {}", arr[index]);
}
