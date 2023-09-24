#[allow(dead_code)]
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // etc
}

#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin {
    fn value(self) -> u8 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }
}

fn plus_one_match(x: Option<i32>) -> Option<i32> {
    match x {
        Some(x) => Some(x+1),
        None => None,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> { x.map(|y| y + 1) }

fn plus_one_tests() {
    // dbg! and println! statements seem to be running concurrently, and so output ordering on
    // stdout and stderr are individually stable, but may interleave nondeterministically.
    println!("## Plus one tests");
    dbg!(plus_one(Some(5)));
    dbg!(plus_one(None));
    dbg!(plus_one_match(Some(5)));
    dbg!(plus_one_match(None));
}

fn main() {
    plus_one_tests();

    println!("## Coin tests");
    dbg!(Coin::Dime.value());
    dbg!(Coin::Quarter(UsState::Alabama).value());
}
