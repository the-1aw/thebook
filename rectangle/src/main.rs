#[derive(Debug)]
struct Rectangle {
    width: usize,
    height: usize,
}

struct UglyRect(usize, usize);

fn area(rect: &Rectangle) -> usize {
    rect.width * rect.height
}

fn uarea(urect: &UglyRect) -> usize {
    urect.0 * urect.1
}

impl Rectangle {
    fn area(&self) -> usize {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: usize) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    println!(
        "The area of the ugly rectangle is {} square pixels.",
        uarea(&UglyRect(6, 7))
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&Rectangle {
            width: 6,
            height: 7
        })
    );
    println!(
        "Debug rectangle {:?}",
        Rectangle {
            width: 6,
            height: 7
        }
    );
    println!(
        "Pretty debug rectangle {:#?}",
        Rectangle {
            width: 6,
            height: 7
        }
    );
    let scale = 3;
    let rect = Rectangle {
        width: 12,
        height: dbg!(12 * scale),
    };
    dbg!(&rect);
    println!("The area from method is {} square pixels.", rect.area());
    if rect.width() {
        println!("Rect as non zero width: {}", rect.width);
    }
    let rect2 = Rectangle {
        width: 10,
        height: 10,
    };
    println!("Can rect hold rect2 ? {}", rect.can_hold(&rect2));
    println!("Can rect2 hold rect ? {}", rect2.can_hold(&rect));
    dbg!(&Rectangle::square(4));
}
