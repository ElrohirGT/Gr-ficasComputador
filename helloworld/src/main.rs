use std::error::Error;

use helloworld::{color::Color, framebuffer::Framebuffer};

fn main() -> Result<(), Box<dyn Error>> {
    let mut framebuffer = Framebuffer::new(2, 2);

    framebuffer.set_background_color(Color::black());
    framebuffer.clear();

    framebuffer.set_current_color(0xff0000);
    framebuffer.paint_point((0.0, 0.0))?;

    framebuffer.set_current_color(0xffffff);
    framebuffer.paint_point((1.0, 0.0))?;

    framebuffer.set_current_color(0x0000ff);
    framebuffer.paint_point((0.0, 1.0))?;

    framebuffer.set_current_color(0x00ff00);
    framebuffer.paint_point((1.0, 1.0))?;
    framebuffer.save("test_grid.bmp")?;

    let mut framebuffer = Framebuffer::new(3, 3);

    framebuffer.set_background_color(Color::black());
    framebuffer.clear();

    framebuffer.set_current_color(0xff0000);
    framebuffer.paint_point((0.0, 0.0))?;

    framebuffer.set_current_color(0xffffff);
    framebuffer.paint_point((1.0, 0.0))?;

    framebuffer.set_current_color(0x0000ff);
    framebuffer.paint_point((0.0, 1.0))?;

    framebuffer.set_current_color(0x00ff00);
    framebuffer.paint_point((1.0, 1.0))?;
    framebuffer.save("bigger_test_grid.bmp")?;

    let mut framebuffer = Framebuffer::new(800, 600);
    framebuffer.set_background_color(0xADD8E6);
    framebuffer.clear();

    framebuffer.set_current_color(0xFF0000);
    framebuffer.paint_point((400.0, 300.0))?;
    framebuffer.paint_point((401.0, 300.0))?;
    framebuffer.paint_point((400.0, 301.0))?;
    framebuffer.paint_point((401.0, 301.0))?;

    framebuffer.set_current_color(0x00FF00);
    framebuffer.paint_point((200.0, 150.0))?;
    framebuffer.paint_point((201.0, 150.0))?;
    framebuffer.paint_point((200.0, 151.0))?;
    framebuffer.paint_point((201.0, 151.0))?;

    framebuffer.set_current_color(0x0000FF);
    framebuffer.paint_point((600.0, 450.0))?;
    framebuffer.paint_point((601.0, 450.0))?;
    framebuffer.paint_point((600.0, 451.0))?;
    framebuffer.paint_point((601.0, 451.0))?;

    framebuffer.save("canva_test.bmp")?;

    let mut framebuffer = Framebuffer::new(8, 4);
    framebuffer.clear();

    framebuffer.set_background_color(Color::black());

    framebuffer.paint_line((0.0, 0.0), (7.0, 2.0))?;

    // println!("{:?}", framebuffer);

    framebuffer.save("line_test.bmp")?;

    let mut framebuffer = Framebuffer::new(20, 20);
    framebuffer.clear();

    // framebuffer.set_current_color(0);
    framebuffer.paint_line((5.0, 5.0), (15.0, 5.0))?;
    framebuffer.paint_line((15.0, 5.0), (15.0, 15.0))?;
    framebuffer.paint_line((15.0, 15.0), (5.0, 15.0))?;
    framebuffer.paint_line((5.0, 15.0), (5.0, 5.0))?;

    framebuffer.save("square_test.bmp")?;

    Ok(())
}
