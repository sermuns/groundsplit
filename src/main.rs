use std::io::{BufWriter, Write};

use clap::Parser;
use piet_common::{
    Brush, Device, ImageFormat, RenderContext,
    kurbo::{Point, Rect, Size},
};

// TODO: make not constant
const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    /// "Dots per inch" / pixel density. Determines output width/height.
    #[arg(long, default_value_t = 96.)]
    dpi: f64,
}

fn main() -> color_eyre::Result<()> {
    let mut frame_buf = vec![0u8; WIDTH * HEIGHT * 4];
    let mut device = Device::new().unwrap();
    let mut bitmap = device.bitmap_target(WIDTH, HEIGHT, 1.0).unwrap();
    let mut stdout = BufWriter::new(std::io::stdout().lock());
    {
        let mut ctx = bitmap.render_context();

        ctx.fill(
            Rect::from_origin_size(Point::new(0., 0.), Size::new(200., 100.)),
            &Brush::Solid(0xffff00ff),
        );

        ctx.finish().unwrap();
        bitmap
            .copy_raw_pixels(ImageFormat::RgbaPremul, &mut frame_buf)
            .unwrap();
        stdout.write_all(&frame_buf).unwrap();
    }

    Ok(())
}
