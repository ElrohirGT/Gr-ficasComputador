#[derive(Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

fn point_from_tuple(&(&x, &y): &(&f32, &f32)) -> Point {
    Point { x, y }
}

pub fn slope(p1: &Point, p2: &Point) -> f32 {
    let Point { x, y } = p1;
    let Point { x: x2, y: y2 } = p2;

    (y2 - y) / (x2 - x)
}

impl From<(f32, f32)> for Point {
    fn from(value: (f32, f32)) -> Self {
        point_from_tuple(&(&value.0, &value.1))
    }
}

impl From<&(f32, f32)> for Point {
    fn from(value: &(f32, f32)) -> Self {
        point_from_tuple(&(&value.0, &value.1))
    }
}

impl From<&(&f32, &f32)> for Point {
    fn from(value: &(&f32, &f32)) -> Self {
        point_from_tuple(value)
    }
}
