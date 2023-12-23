// Variables are immutable by default.
// You can shadow a variable by assigning a variable with same name.
// With shadowing you can change variable type
// But with mutable variable you cannot change variable type.
// Constants cannot be shadowed in the same scope.

fn main() {
    const MY_VAR: i32 = 32;
    println!("The value of MY_VAR: {MY_VAR}");
    let x = 5;
    println!("The value of x: {x}");
    //notTODO: const MY_VAR: u32 = 24;
    {
        let x = 32;
        println!("The value of x: {x}");
    }
    println!("The value of x: {x}");
}
