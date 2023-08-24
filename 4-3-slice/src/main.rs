fn main() {
    println!("first_word: [{}]", first_word("Hello, world!"));

    match find("ab b c", 'd') {
        Some(i) => println!("find: i={}", i),
        None => println!("find: no match"),
    }
}

fn first_word(s: &str) -> &str {
    for (i, c) in s.char_indices() {
        if c == ' ' {
            return &s[..i]
        }
    }
    s
}

fn find(s: &str, to_find: char) -> Option<usize> {
    for (i, c) in s.char_indices() {
        if c == to_find {
            return Some(i)
        }
    }
    None
}
