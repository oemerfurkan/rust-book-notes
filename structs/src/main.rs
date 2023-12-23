/*
STRUCTS
    - Some blueprint to store related data

    - struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    };

    - let mut user = User {
        active: true,
        username: String::from("oemerfurkan"),
        email: String::from("omerfurkanyuruk1@gmail.com"),
        sign_in_count: 0,
    };

    - Tuple Structs
        * struct Color(i32, i32, i32);

    - Unit Like Structs
        * struct AlwaysEqual;
*/
use std::fmt::{self, Debug, Display, Formatter, Result};

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(u8, u8, u8);
struct Point(i32, i32, i32);

impl Display for User {
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(
            f,
            "Activity: {}",
            if self.active { "Active" } else { "Inactive" }
        )
        .expect("Cannot reached to the activity value!");
        writeln!(f, "Username: {}", self.username).expect("Cannot reached to the username value!");
        writeln!(f, "E-mail: {}", self.email).expect("Cannot reached to the E-mail value!");
        writeln!(f, "Sign-in Count: {}", self.sign_in_count)
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "Red: {}", self.0).expect("Cannot get the Red value");
        writeln!(f, "Green: {}", self.1).expect("Cannot get the Green value");
        writeln!(f, "Blue: {}", self.2)
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "X: {} ", self.0).expect("Cannot get the X value");
        write!(f, "Y: {} ", self.0).expect("Cannot get the Y value");
        write!(f, "Z: {}", self.0)
    }
}

fn main() {
    let mut user = User {
        active: true,
        username: String::from("oemerfurkan"),
        email: String::from("omerfurkanyuruk1@gmail.com"),
        sign_in_count: 0,
    };

    let user2 = User {
        email: String::from("sesdeneme@gmail.com"),
        username: String::from("omerfurkan"),
        
        // With this ..user, used user's String type variables
        // and these String values moved to user2, so the user cannot be used after this.
        // However if we get only the active and sign_in_count
        // values from the user then we can use the user variable.
        // Because sign_in_count and active values use Copy trait.  
        ..user
    };

    user.email = String::from("omerfy57omer12@gmail.com");

    println!("User1: {}", user);
    println!("User1 E-mail: {}", user.email);
    println!("User2: {user2}");

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{black}");
    println!("{origin}");
}

fn build_user(email: String, username: String) -> User {
    // Returns a user
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

fn build_user_conventional(email: String, username: String) -> User {
    // If there is value with the same name
    // They can be passed to the struct
    // like this as a shorthand.
    User {
        active: true,
        //
        username,
        email,
        //
        sign_in_count: 1,
    }
}
