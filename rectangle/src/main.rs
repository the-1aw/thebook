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
}
