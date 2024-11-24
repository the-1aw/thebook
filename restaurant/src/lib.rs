mod back_of_house;
mod front_of_house;

pub use crate::front_of_house::hosting;

use crate::front_of_house::serving;
use back_of_house::Appetizer as HouseAppetizer;
use back_of_house::Breakfast;

fn deliver_order() {}

pub fn take_away() {
    crate::front_of_house::serving::take_order();
    back_of_house::fix_order();
}

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::seat_at_table();
    serving::take_order();
    back_of_house::cook_order();
    serving::serve_order();
    serving::take_payment();
}

pub fn eat_breakfast_at_restaurant() {
    let mut meal = Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!(
        "The meal contains {} toast and {}!",
        meal.toast,
        meal.seasonal_fruit()
    );
}

pub fn just_a_bite_at_restaurant() {
    let _order1 = HouseAppetizer::Soup;
    let _order2 = HouseAppetizer::Salad;
}
