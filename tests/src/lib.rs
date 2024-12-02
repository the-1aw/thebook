#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got: {value}.");
        }
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

pub fn add_two(left: u64) -> u64 {
    left + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {name}!")
}

pub fn fact(n: i128) -> i128 {
    let mut res = 1i128;
    for i in 1..=n {
        res *= i
    }
    res
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    #[ignore = "Test is heavy to compute, run it only if needed"]
    fn compute_heavy() {
        for _ in 0..10_000_000 {
            fact(20);
        }
    }

    #[test]
    fn add_two_and_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }

    #[test]
    fn add_two_and_three() {
        let result = add_two(3);
        assert_eq!(result, 5);
    }

    #[test]
    fn add_two_and_four() {
        let result = add_two(4);
        assert_eq!(result, 6);
    }

    #[test]
    fn result_style() -> Result<(), String> {
        let result = add_two(2);
        match result {
            4 => Ok(()),
            _ => Err("Some error occured".to_string()),
        }
    }

    #[test]
    #[should_panic(expected = "must be between 1 and 100, got:")]
    fn guess_too_big() {
        Guess::new(101);
    }

    #[test]
    #[should_panic(expected = "must be between 1 and 100, got:")]
    fn guess_too_small() {
        Guess::new(-1);
    }

    #[test]
    fn greeting_contains_name() {
        let name: &'static str = "Sophie";
        let output = greeting(name);
        assert!(
            output.contains(name),
            "Greeting did not contain {name}, value was {output}"
        );
    }

    #[test]
    fn bigger_can_hold_smaller() {
        let bigger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(bigger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cant_hold_bigger() {
        let bigger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(!smaller.can_hold(&bigger));
    }
}
