fn main() {
    mutable();
    shadow();
}

fn mutable() {
    // must specify mut, or will get compiler err
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);
}

fn shadow() {
    let y = 5;
    {
        let y = y + 1;
        let y = y * 2;
        println!("The value of y is {}", y);
    }
    println!("The value of y is {}", y);
}

