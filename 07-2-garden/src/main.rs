use crate::garden::vegetables::Asparagus;
// pub mod garden {
//     pub mod vegetables {
//         #[derive(Debug)]
//         pub struct Asparagus {}
//     }
// }

// Or, equivalently, use filesystem:
pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
