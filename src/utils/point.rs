use std::ops::Add;
#[derive(Copy, Clone, Debug, Default, PartialOrd, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn neighbors(&self) -> Vec<Self> {
        let modifiers = [
            (-1, 0).into(),
            (1, 0).into(),
            (0, -1).into(),
            (0, 1).into(),
            (-1, -1).into(),
            (-1, 1).into(),
            (1, -1).into(),
            (1, 1).into(),
        ];

        modifiers.iter().map(|shift| *self + *shift).collect()
    }

    pub fn constrain(&self, min: &Point, max: &Point) -> Self {
        Self {
            x: std::cmp::min(std::cmp::max(min.x, self.x), max.x),
            y: std::cmp::min(std::cmp::max(min.y, self.y), max.y),
        }
    }

    pub fn is_valid(&self, min: &Point, max: &Point) -> bool {
        self.x >= min.x && self.y >= min.y && self.x <= max.x && self.y <= max.y
    }

    pub fn manhattan_distance(&self, other: &Point) -> u64 {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as u64
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl From<(i64, i64)> for Point {
    fn from(value: (i64, i64)) -> Self {
        Point {
            x: value.0,
            y: value.1,
        }
    }
}
