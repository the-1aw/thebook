use macros::MacroDemo;
use macros_derive::MacroDemo;

#[derive(MacroDemo)]
struct Pancake {}

fn main() {
    Pancake::demo_macro();
}
