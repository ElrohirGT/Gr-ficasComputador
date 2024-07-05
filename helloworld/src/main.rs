use std::error::Error;

use helloworld::{color::Color, framebuffer::Framebuffer};

fn main() -> Result<(), Box<dyn Error>> {
    let mut framebuffer = Framebuffer::new(2, 2);

    framebuffer.set_background_color(Color::black());
    framebuffer.clear();

    framebuffer.set_current_color(0xff0000.into());
    framebuffer.paint_point(0.0, 0.0)?;

    framebuffer.set_current_color(0xffffff.into());
    framebuffer.paint_point(0.0, 1.0)?;

    println!("{:?}", framebuffer);

    // framebuffer.set_background_color(0xADD8E6.into());
    // framebuffer.clear();
    //
    // framebuffer.set_current_color(0xFF0000.into());
    // framebuffer.paint_point(400.0, 300.0)?;
    // framebuffer.paint_point(401.0, 300.0)?;
    // framebuffer.paint_point(400.0, 301.0)?;
    // framebuffer.paint_point(401.0, 301.0)?;
    //
    // framebuffer.set_current_color(0x00FF00.into());
    // framebuffer.paint_point(200.0, 150.0)?;
    // framebuffer.paint_point(201.0, 150.0)?;
    // framebuffer.paint_point(200.0, 151.0)?;
    // framebuffer.paint_point(201.0, 151.0)?;
    //
    // framebuffer.set_current_color(0x0000FF.into());
    // framebuffer.paint_point(600.0, 450.0)?;
    // framebuffer.paint_point(601.0, 450.0)?;
    // framebuffer.paint_point(600.0, 451.0)?;
    // framebuffer.paint_point(601.0, 451.0)?;

    framebuffer.save("output.bmp")?;

    println!("Framebuffer rendered to output.bmp");

    Ok(())
}
