struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    // NOTE: We could also create use mixed generics here
    // fn mixed_generics<U>(&self, other: &Point<U>) {}
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {result}");
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {result}");

    Point { x: 1, y: 2 };
    let point = Point { x: 1.0, y: 2.0 };
    println!(
        "point({},{}), dist from origin {}",
        point.x(),
        point.y,
        point.distance_from_origin()
    );
}
