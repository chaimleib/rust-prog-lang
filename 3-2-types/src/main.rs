fn main() {
    println!("Hello, world!");
   parse_str_num();
   floats();
}

fn parse_str_num() {
    // Must specify type, or compiler won't know what to parse string into
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("Should be 21: {}", guess/2);
}

fn floats() {
    let x = 2.0; // f64
    let y: f32 = 3.0;
    
    println!("x = {}, y = {}", x, y);
    println!("x/6. = {}, y/6. = {}", x/6., y/6.);
}

