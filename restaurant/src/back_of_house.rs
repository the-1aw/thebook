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
