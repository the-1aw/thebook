use gui::{Button, Draw, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        let mut display = String::new();
        display.push_str(&format!(
            "Select box: w - {}, h - {} \n",
            self.width, self.height
        ));
        display.push_str("Options: \n");
        for opt in self.options.iter() {
            display.push_str(&format!("- {opt}\n"));
        }
        print!("{display}");
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("Ok"),
            }),
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("No"),
                    String::from("Maybe"),
                    String::from("I don't know"),
                    String::from("Can you repeat the question"),
                ],
            }),
        ],
    };
    screen.display();
}
