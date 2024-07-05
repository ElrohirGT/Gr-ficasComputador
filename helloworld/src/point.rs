#[derive(Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

fn point_from_tuple(&(&x, &y): &(&f32, &f32)) -> Point {
    Point { x, y }
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
