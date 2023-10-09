mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        // fn seat_at_table() {}
    }
    // mod serving {
    //     fn take_order() {}
    //     fn serve_order() {}
    //     fn take_payment() {}
    // }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

pub fn order_breakfast() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // won't compile: private field
    // meal.seasonal_fruit = String::from("blueberries");
    println!("The fruit this season is {}", meal.fruit());
}

fn deliver_order() {}

mod back_of_house {
    pub fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }

        pub fn fruit(self) -> String {
            String::from(self.seasonal_fruit)
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn order_appetizer() {
    let _order1 = back_of_house::Appetizer::Soup;
    let _order2 = back_of_house::Appetizer::Salad;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_eat() {
        eat_at_restaurant();
    }

    #[test]
    fn can_fix() {
        back_of_house::fix_incorrect_order();
    }

    #[test]
    fn can_breakfast() {
        order_breakfast();
    }

    #[test]
    fn can_appetizer() {
        order_appetizer();
    }
}
