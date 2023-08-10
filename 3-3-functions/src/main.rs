fn main() {
    println!("Hello, world!");
    another_function(5, 'm');
}

fn another_function(x: i32, unit: char) {
    println!("x = {x}{unit}");
}
