fn plus_one_match(x: Option<i32>) -> Option<i32> {
    match x {
        Some(x) => Some(x+1),
        None => None,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> { x.map(|y| y + 1) }

fn main() {
    dbg!(plus_one(Some(5)));
    dbg!(plus_one(None));
    dbg!(plus_one_match(Some(5)));
    dbg!(plus_one_match(None));
}
