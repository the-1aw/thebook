#[derive(PartialEq, Debug)]
pub struct Shoe {
    size: usize,
    style: String,
}

pub fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: usize) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

pub struct Fibonacci {
    curr: usize,
    next: usize,
}

impl Iterator for Fibonacci {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;

        self.curr = self.next;
        self.next += current;

        Some(current)
    }
}

impl Fibonacci {
    pub fn new() -> Fibonacci {
        Fibonacci { next: 1, curr: 0 }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn iterator_demo() {
        let v = vec![1, 2, 3];
        let mut it = v.iter();

        assert_eq!(it.next(), Some(&1));
        assert_eq!(it.next(), Some(&2));
        assert_eq!(it.next(), Some(&3));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn iterator_sum() {
        let v = vec![1, 2, 3];
        let sum: i32 = v.iter().sum();
        assert_eq!(sum, 6);
    }

    #[test]
    fn iterator_map_inc() {
        let v: Vec<i32> = vec![1, 2, 3];
        let inc_v: Vec<i32> = v.iter().map(|num| num + 1).collect();
        assert_eq!(inc_v, [2, 3, 4]);
    }

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("Sandal"),
            },
            Shoe {
                size: 13,
                style: String::from("Sneaker"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];
        assert_eq!(
            shoes_in_size(shoes, 13),
            vec![Shoe {
                size: 13,
                style: String::from("Sneaker")
            }]
        );
    }

    #[test]
    fn fibonacci_iterator() {
        let mut it = Fibonacci::new();
        assert_eq!(it.next(), Some(0));
        assert_eq!(it.next(), Some(1));
        assert_eq!(it.next(), Some(1));
        assert_eq!(it.next(), Some(2));
        assert_eq!(it.next(), Some(3));
        assert_eq!(it.next(), Some(5));
        assert_eq!(it.next(), Some(8));
    }
}
