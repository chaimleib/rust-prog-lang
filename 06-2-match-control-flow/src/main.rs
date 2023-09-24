enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Coin {
    fn value(self) -> u8 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
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

fn main() {
    dbg!(Coin::Dime.value());

    dbg!(plus_one(Some(5)));
    dbg!(plus_one(None));
    dbg!(plus_one_match(Some(5)));
    dbg!(plus_one_match(None));
}
