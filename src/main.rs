use clap::Parser;
use piet_common::{Device, RenderContext};

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
    let mut device = Device::new().unwrap();
    let mut bitmap = device.bitmap_target(WIDTH, HEIGHT, 1.0).unwrap();
    {
        let mut ctx = bitmap.render_context();

        ctx.finish().unwrap();
    }

    bitmap.save_to_file("temp-image.png").unwrap();

    Ok(())
}
