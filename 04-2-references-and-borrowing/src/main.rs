fn main() {
    change_print();
    multiple_borrow();
}

fn label(s: &str) {
    println!("");
    println!("## {s}");
}

fn change_print() {
    label("change_print");
    let mut s1 = String::from("hello");
    change(&mut s1);
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn multiple_borrow() {
    label("multiple_borrow");
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2); // not used again
    let r3 = &mut s;
    println!("{}", r3);
}
