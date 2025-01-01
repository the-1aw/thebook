use std::ops::Add;

pub trait Demonize {
    type EvilType;

    fn make_evil(&self) -> Self::EvilType;
}

#[derive(Debug, PartialEq)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug, PartialEq)]
struct EvilPoint {
    x: u32,
    y: u32,
    z: u32,
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
}
