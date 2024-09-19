fn main() {
    println!("Hello, world!");
    
// Numbers
    let x: i32 = -32;
    let y: u32 = 32;
    let z: f32 = 32.001;

    println!("x: {}, y: {}, z: {}", x,y,z);

// Boolean
    let is_male = false;
    let is_above_18 = true;
    
    if is_male {
        println!("You are a male");

    } else {
        println!("You are not a male");
    }

    if is_male && is_above_18 {
        print!("You are a legal male");
    }


    // String
    // let greeting: &str = "hello world"; // other way of doing the same
    let greeting = String::from("hello world");
    println!("{}", greeting);

    // print!("{}", greeting.chars().nth(1000))
}

// Basic intitalisation command for end user app setup-> "cargo init"
// Basic intitalisation command for library creation will be  -> "cargo init --lib"