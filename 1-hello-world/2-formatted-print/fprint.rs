//https://rustbyexample.com/hello/print.html
fn main() {
    let month_days = 30;
    println!("{} days in the month", month_days);
    
    introduce("Alice", "Bob");
    
    println!("{s} {v} {o}",
             o="the lazy dog",
             s="the quick brown fox",
             v="pushes");
    
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);
    println!("{number:>width$}", number=1, width=6);
    println!("{number:>0width$}", number=1, width=6);
}

fn introduce(a: &str, b: &str) -> () {
    println!("{0}, this is {1}. {1}, this is {0}.", a, b);
}

