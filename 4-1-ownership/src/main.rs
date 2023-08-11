fn main() {
    string_push();
    move_string();
    clone_string();
    arg_pass();
}

fn label(s: &str) {
    println!("");
    println!("## {s}");
}

fn string_push() {
    label("string_push");
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{s}");
}

fn move_string() {
    label("move_string");
    let s1 = String::from("hello");
    let s2 = s1; // move
    println!("{}, world!", s2);
}

fn clone_string() {
    label("clone_string");
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}");
}

fn arg_pass() {
    label("arg_pass");
    let s = String::from("hello");
    takes_ownership(s.clone());
    takes_ownership(s);
    let x = 5;
    makes_copy(x);
}

fn takes_ownership(s: String) {
    println!("{}", s);
}

fn makes_copy(i: i32) {
    println!("{}", i);
}
