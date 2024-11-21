fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> u8 {
    5
}

fn plus_one(x: u8) -> u8 {
    return x + 1;
}

fn main() {
    println!("Hello, world!");
    another_function(12);
    print_labeled_measurement(12, 'h');
    let x = plus_one(five());
    println!("The value of x is: {x}");
}
