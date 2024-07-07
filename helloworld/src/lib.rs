pub mod bmp;
pub mod color;
pub mod framebuffer;
pub mod point;

pub fn are_equal(first: f32, second: f32, eps: f32) -> bool {
    println!("Comparing: {} == {}", first, second);
    (first - second).abs() <= eps
}
