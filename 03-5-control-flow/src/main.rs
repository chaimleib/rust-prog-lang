fn main() {
    else_if(6);
    if_assign(false);
    loop_value(10);
    double_loop();
    while_loop();
    for_indexer();
    for_in();
    countdown_for();
    println!("");
    println!("fib(5) = {}", fib(5));
}

fn else_if(number: i32) {
    println!("");
    println!("## else_if");
    if number % 4 == 0 {
        println!("div 4");
    } else if number % 3 == 0 {
        println!("div 3");
    } else if number % 2 == 0 {
        println!("div 2");
    } else {
        println!("nodiv");
    }
}

fn if_assign(condition: bool) {
    println!("");
    println!("## if_assign");
    let number = if condition { 5 } else  { 6 };

    println!("number = {number}");
}

fn loop_value(limit: i32) {
    println!("");
    println!("## loop_value");
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == limit {
            break counter * 2;
        }
    };

    println!("result = {result}");
}

fn double_loop() {
    println!("");
    println!("## double_loop");
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");

        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn while_loop() {
    println!("");
    println!("## while_loop");
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!");
}

fn for_indexer() {
    println!("");
    println!("## for_indexer");
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5  {
        println!("the value is: {}", a[index]);
        index += 1;
    }
}

fn for_in() {
    println!("");
    println!("## for_in");
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }
}

fn countdown_for() {
    println!("");
    println!("## countdown_for");
    for number in (1..=3).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!");
}

fn fib(n: i32) -> i32 {
    if n < 1 {
        return 1;
    }
    let mut a = 0;
    let mut b = 1;
    for i in (1..=n) {
        let last = b;
        b += a;
        a = last;
        println!("fib({i}): {b}");
    }
    b
}
