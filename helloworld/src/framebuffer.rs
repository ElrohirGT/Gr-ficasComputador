use crate::{bmp::write_bmp_file, color::Color};

#[derive(Debug)]
pub struct Framebuffer {
    width: usize,
    height: usize,
    buffer: Buffer,
    background_color: Color,
    current_color: Color,
    empty_buffer: Vec<u32>,
}

type Buffer = Vec<u32>;

fn create_filled_buffer(width: &usize, height: &usize, color: &Color) -> Buffer {
    let color_hex: u32 = color.into();

    (0..(width * height)).map(|_| color_hex).collect()
}

#[derive(Debug)]
pub enum PaintPointErrors {
    XTooLarge,
    XTooSmall,
    YTooLarge,
    YTooSmall,
}
impl std::fmt::Display for PaintPointErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}
impl std::error::Error for PaintPointErrors {}

pub enum GetColorErrors {
    XTooLarge,
    YTooLarge,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        let background_color = Color::black();
        let current_color = Color::white();

        Framebuffer {
            width,
            height,
            buffer: vec![],
            background_color,
            current_color,
            empty_buffer: create_filled_buffer(&width, &height, &Color::black()),
        }
    }

    /// Creates an empty buffer according to the corresponding `background_color`.
    ///
    /// The implementation of this method assumes the background color will not change that much.
    pub fn clear(&mut self) {
        self.buffer = self.empty_buffer.clone();
    }

    /// Colors a point in the given location. Rounds x and y.
    /// If either x or y are exactly half between integers then the value is rounded up.
    /// `x` and `y` are 0 indexed and represent the pixels to go up and right from the lower left
    /// corner.
    ///
    /// The color used is the one provided by `current_color`.
    pub fn paint_point(&mut self, x: f32, y: f32) -> Result<(), PaintPointErrors> {
        let Framebuffer {
            width,
            height,
            buffer,
            current_color,
            ..
        } = self;

        if x < 0.0 {
            Err(PaintPointErrors::XTooSmall)?
        }

        if y < 0.0 {
            Err(PaintPointErrors::YTooSmall)?
        }

        let x = x.round() as usize;
        let y = y.round() as usize;

        match (x <= *width, y <= *height) {
            (false, _) => Err(PaintPointErrors::XTooLarge),
            (_, false) => Err(PaintPointErrors::YTooLarge),
            _ => {
                buffer[y * *width + x] = current_color.into();
                Ok(())
            }
        }
    }

    /// Gets the color of a point in the buffer.
    pub fn get_color(&self, x: usize, y: usize) -> Result<Color, GetColorErrors> {
        let Framebuffer {
            width,
            height,
            buffer,
            ..
        } = self;

        match (x <= *width, y <= *height) {
            (_, false) => Err(GetColorErrors::YTooLarge),
            (false, _) => Err(GetColorErrors::XTooLarge),
            _ => Ok(buffer[y * *width + x].into()),
        }
    }

    /// Sets the `background_color` property.
    /// This method should also regenerate the `empty_buffer`.
    ///
    /// * `new_color`: The color to apply.
    pub fn set_background_color(&mut self, new_color: Color) {
        let Framebuffer {
            width,
            height,
            background_color,
            empty_buffer,
            ..
        } = self;

        *background_color = new_color;
        *empty_buffer = create_filled_buffer(width, height, background_color);
    }

    /// Sets the `current_color` property.
    ///
    /// * `new_color`: The color to apply.
    pub fn set_current_color(&mut self, new_color: Color) {
        self.current_color = new_color;
    }

    /// Saves the pixel data into a .bmp located in the given `file_path`.
    pub fn save(&self, file_path: &str) -> std::io::Result<()> {
        let Framebuffer {
            width,
            height,
            buffer,
            ..
        } = self;

        write_bmp_file(file_path, buffer, *width, *height)
    }
}
