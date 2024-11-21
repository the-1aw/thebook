fn so_this_is_if_as_ternary(condition: bool) {
    let number: u8 = if condition { 5 } else { 21 };
    println!("number is: {number}");
}

fn break_from_loop_returns(num: i32) -> i32 {
    let mut count = 0;
    return loop {
        if count == 5 {
            break num * 3;
        }
        count += 1;
    };
}

fn print_reverse_range(min: u8, max: u8) {
    for num in (min..max).rev() {
        println!("{num}!");
    }
}

fn fahrenheit_to_celsius(temp: f64) -> f64 {
    (temp - 32.0) * 5.0 / 9.0
}

fn fib(num: usize) -> usize {
    if num == 0 || num == 1 {
        return 1;
    }
    return num + fib(num - 1);
}

const GIFTS_FROM_LOVER: [&str; 12] = [
    "partridge in a pear tree",
    "Two turtle doves",
    "Three French hens",
    "Four calling birds",
    "Five gold rings",
    "Six geese a-laying",
    "Seven swans a-swimming",
    "Eight maids a-milking",
    "Nine ladies dancing",
    "Ten lords a-leaping",
    "Eleven pipers piping",
    "Twelve drummers drumming",
];

const DAYS_OF_CHRISTMAS: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    "eleventh", "twelfth",
];

fn fine_ill_do_the_caroll() {
    for day in 0..DAYS_OF_CHRISTMAS.len() {
        println!(
            "On the {} day of Christmas,\nmy true love sent to me",
            DAYS_OF_CHRISTMAS[day]
        );
        for gift_idx in (0..(day + 1)).rev() {
            if gift_idx == 0 {
                println!(
                    "{} {}{}",
                    if day == 0 { "A" } else { "And a" },
                    GIFTS_FROM_LOVER[0],
                    if day == DAYS_OF_CHRISTMAS.len() - 1 {
                        "!"
                    } else {
                        "."
                    }
                )
            } else {
                println!("{},", GIFTS_FROM_LOVER[gift_idx]);
            }
        }
        println!("");
    }
}

fn main() {
    so_this_is_if_as_ternary(false);
    let from_loop = break_from_loop_returns(12);
    println!("value from loop is {from_loop}");
    print_reverse_range(1, 12);
    let celsius = fahrenheit_to_celsius(87f64);
    println!("87 fahrenheit is {celsius} celsius");
    let fib8 = fib(8);
    println!("8 fib is {fib8}");
    fine_ill_do_the_caroll();
}
