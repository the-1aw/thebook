pub trait Demonize {
    type EvilType;

    fn make_evil(&self) -> Self::EvilType;
}

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn _make_evil_point() {
        let p = Point { x: 4, y: 20 };
        assert_eq!(p.make_evil(), EvilPoint { x: 4, y: 20, z: 69 })
    }
}
