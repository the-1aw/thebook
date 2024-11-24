mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        pub fn seat_at_table() {}
    }
    pub mod serving {
        pub fn take_order() {}
        pub fn serve_order() {}
        pub fn take_payment() {}
    }
}

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }

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

        pub fn seasonal_fruit(&self) -> &str {
            &self.seasonal_fruit[..]
        }
    }

    pub fn fix_order() {
        cook_order();
        super::deliver_order();
    }

    pub fn cook_order() {}
}

fn deliver_order() {}

pub fn take_away() {
    crate::front_of_house::serving::take_order();
    back_of_house::fix_order();
}

pub use crate::front_of_house::hosting;
use crate::front_of_house::serving;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::seat_at_table();
    serving::take_order();
    back_of_house::cook_order();
    serving::serve_order();
    serving::take_payment();
}

pub fn eat_brakfast_at_restaurant() {
    use back_of_house::Breakfast;

    let mut meal = Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!(
        "The meal contains {} toast and {}!",
        meal.toast,
        meal.seasonal_fruit()
    );
}

pub fn just_a_bite_at_restaurant() {
    use back_of_house::Appetizer as HouseAppetizer;

    let _order1 = HouseAppetizer::Soup;
    let _order2 = HouseAppetizer::Salad;
}
