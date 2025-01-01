use std::ops::Add;

pub trait Demonize: std::fmt::Display {
    type EvilType;

    fn make_evil(&self) -> Self::EvilType;
    fn scream(&self) {
        println!("{} screams: MAKE ME EVIL!", self);
    }
}

#[derive(Debug, PartialEq)]
pub struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug, PartialEq)]
pub struct EvilPoint {
    x: u32,
    y: u32,
    z: u32,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point({}, {})", self.x, self.y)
    }
}

impl Demonize for Point {
    type EvilType = EvilPoint;
    fn make_evil(&self) -> Self::EvilType {
        EvilPoint {
            x: self.x,
            y: self.y,
            z: 69,
        }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<EvilPoint> for Point {
    type Output = Point;

    fn add(self, rhs: EvilPoint) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

pub trait Ambiguous {
    fn ambiguous_method(&self) -> usize;
    fn ambiguous_static() -> usize;
}

impl Ambiguous for Point {
    fn ambiguous_method(&self) -> usize {
        6
    }

    fn ambiguous_static() -> usize {
        9
    }
}

impl Point {
    pub fn ambiguous_method(&self) -> usize {
        4
    }

    pub fn ambiguous_static() -> usize {
        20
    }
}

pub struct PrintableVec(Vec<String>);

impl std::fmt::Display for PrintableVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

// We implement deref on PrintableVec so every method availble on Vec<String> is available on
// PrintableVec
impl std::ops::Deref for PrintableVec {
    type Target = Vec<String>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn make_evil_point() {
        let p = Point { x: 4, y: 20 };
        assert_eq!(p.make_evil(), EvilPoint { x: 4, y: 20, z: 69 })
    }

    #[test]
    fn add_point_to_point() {
        assert_eq!(
            Point { x: 2, y: 10 } + Point { x: 2, y: 10 },
            Point { x: 4, y: 20 }
        );
    }

    #[test]
    fn add_evil_point_to_point() {
        assert_eq!(
            Point { x: 2, y: 10 } + EvilPoint { x: 2, y: 10, z: 69 },
            Point { x: 4, y: 20 }
        );
    }

    #[test]
    fn call_ambiguous_methods() {
        let p = Point { x: 0, y: 0 };
        assert_eq!(p.ambiguous_method(), 4);
        assert_eq!(Ambiguous::ambiguous_method(&p), 6);
    }

    #[test]
    fn call_ambiguous_statics() {
        assert_eq!(Point::ambiguous_static(), 20);
        assert_eq!(<Point as Ambiguous>::ambiguous_static(), 9);
    }

    #[test]
    fn newtype_pattern() {
        let printable_vec = PrintableVec(vec!["a".to_string(), "b".to_string()]);
        let content = format!("{}", printable_vec);
        assert_eq!(content, "[a, b]");
        assert_eq!(printable_vec.len(), 2);
    }
}
