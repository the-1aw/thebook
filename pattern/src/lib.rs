pub fn matching_literrals(b: u8) -> &'static str {
    match b {
        b'1' => "one",
        b'2' => "two",
        b'3' => "three",
        _ => "something else",
    }
}

pub fn matching_named_varible(x: Option<u8>) -> (u8, u8) {
    let y = 1 << 4;
    match x {
        Some(50) => (50, y),
        Some(y) => (y, y),
        _ => (0, 0),
    }
}

pub fn matching_multi_pattern(x: u8) -> &'static str {
    match x {
        1 | 2 => "one or two",
        3 => "three",
        _ => "more",
    }
}

#[derive(PartialEq, Debug)]
pub enum CharCase {
    Upper(char),
    Lower(char),
    None,
}

#[derive(PartialEq, Debug)]
pub enum MatchingError {
    NotAlphabetic,
}

pub fn matching_ranges(c: char, x: u8) -> Result<(CharCase, u8), MatchingError> {
    let c = match c {
        'a'..='z' => CharCase::Lower(c),
        'A'..='Z' => CharCase::Upper(c),
        _ => CharCase::None,
    };
    if let CharCase::None = c {
        return Err(MatchingError::NotAlphabetic);
    }
    let x = match x {
        0..8 => x,
        _ => 10,
    };
    Ok((c, x))
}

pub struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq)]
pub enum Axis {
    X,
    Y,
    None,
}

pub fn destructuring_struct(Point { x, y }: Point) -> Axis {
    let p = Point { x, y };
    match p {
        Point { x: _, y: 0 } => Axis::X,
        Point { x: 0, y: _ } => Axis::Y,
        Point { x: _, y: _ } => Axis::None,
    }
}

pub enum Color {
    Rgb(u8, u8, u8),
    Hsv(u8, u8, u8),
}

pub enum Message {
    Quit,
    Move { x: u8, y: u8 },
    MovePoint(Point),
    ChangeColor(Color),
}

pub fn destructuring_enums(msg: Message) -> u8 {
    match msg {
        Message::Quit => 1,
        Message::Move { x: _, y: 0 } => 2,
        Message::Move { x: 0, y: _ } => 3,
        Message::Move { x: _, y: _ } => 4,
        Message::MovePoint(Point { x: _, y: 0 }) => 5,
        Message::MovePoint(Point { x: 0, y: _ }) => 6,
        Message::MovePoint(Point { x: _, y: _ }) => 7,
        Message::ChangeColor(Color::Rgb(_r, 125, _b)) => 8,
        Message::ChangeColor(Color::Rgb(_r, _g, _b)) => 9,
        Message::ChangeColor(Color::Hsv(0, _s, _v)) => 10,
        Message::ChangeColor(Color::Hsv(_h, _s, _v)) => 11,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matching_literrals() {
        assert_eq!(matching_literrals(b'1'), "one");
        assert_eq!(matching_literrals(b'2'), "two");
        assert_eq!(matching_literrals(b'3'), "three");
        assert_eq!(matching_literrals(95), "something else");
    }

    #[test]
    fn test_matching_named_variable() {
        assert_eq!(matching_named_varible(None), (0, 0));
        assert_eq!(matching_named_varible(Some(50)), (50, 16));
        assert_eq!(matching_named_varible(Some(20)), (20, 20));
    }

    #[test]
    fn test_matching_multi_pattern() {
        assert_eq!(matching_multi_pattern(1), "one or two");
        assert_eq!(matching_multi_pattern(2), "one or two");
        assert_eq!(matching_multi_pattern(3), "three");
        assert_eq!(matching_multi_pattern(1 << 2), "more");
    }

    #[test]
    fn test_matching_ranges() {
        assert_eq!(matching_ranges('+', 0), Err(MatchingError::NotAlphabetic));
        assert_eq!(matching_ranges('z', 0), Ok((CharCase::Lower('z'), 0)));
        assert_eq!(matching_ranges('z', 1 << 3), Ok((CharCase::Lower('z'), 10)));
        assert_eq!(matching_ranges('Z', 0), Ok((CharCase::Upper('Z'), 0)));
        assert_eq!(matching_ranges('Z', 1 << 3), Ok((CharCase::Upper('Z'), 10)));
    }

    #[test]
    fn test_destucturing_struct() {
        assert_eq!(destructuring_struct(Point { x: 8, y: 0 }), Axis::X);
        assert_eq!(destructuring_struct(Point { x: 0, y: 0 }), Axis::X);
        assert_eq!(destructuring_struct(Point { x: 0, y: 8 }), Axis::Y);
        assert_eq!(destructuring_struct(Point { x: 2, y: 2 }), Axis::None);
    }

    #[test]
    fn test_destructuring_enums() {
        assert_eq!(destructuring_enums(Message::Quit), 1);
        assert_eq!(destructuring_enums(Message::Move { x: 1, y: 0 }), 2);
        assert_eq!(destructuring_enums(Message::Move { x: 0, y: 1 }), 3);
        assert_eq!(destructuring_enums(Message::Move { x: 1, y: 1 }), 4);
        assert_eq!(
            destructuring_enums(Message::MovePoint(Point { x: 1, y: 0 })),
            5
        );
        assert_eq!(
            destructuring_enums(Message::MovePoint(Point { x: 0, y: 1 })),
            6
        );
        assert_eq!(
            destructuring_enums(Message::MovePoint(Point { x: 1, y: 1 })),
            7
        );
        assert_eq!(
            destructuring_enums(Message::ChangeColor(Color::Rgb(0, 125, 0))),
            8
        );
        assert_eq!(
            destructuring_enums(Message::ChangeColor(Color::Rgb(0, 0, 0))),
            9
        );

        assert_eq!(
            destructuring_enums(Message::ChangeColor(Color::Hsv(0, 0, 0))),
            10
        );
        assert_eq!(
            destructuring_enums(Message::ChangeColor(Color::Hsv(125, 0, 0))),
            11
        );
    }
}
