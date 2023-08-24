#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(r: &Rectangle) -> u32 {
    r.width * r.height
}

fn main() {
    let scale = 2;
    let r = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    println!("r: {:#?}", r);
    println!("area: {}", area(&r));
}
