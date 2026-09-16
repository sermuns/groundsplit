use std::{fs::File, path::PathBuf, rc::Rc};

use clap::Parser;
use ffmpeg_sidecar::command::FfmpegCommand;
use piet_common::{Device, ImageFormat, RenderContext};
use splits_rs::{
    Splits,
    draw::{BLOCK_HEIGHT_IN, Context, LINE_THICKNESS_IN, TITLE_HEIGHT_IN},
};

#[derive(Parser)]
#[command(version, about)]
struct Args {
    input_path: PathBuf,

    #[arg(short)]
    output_path: PathBuf,

    // In "inches"
    #[arg(short, long, default_value_t = 10.)]
    width: f64,

    /// "Dots per inch" / pixel density. Determines output width/height.
    #[arg(long, default_value_t = 96.)]
    dpi: f64,

    #[arg(short = 'r', long, default_value_t = 30.)]
    fps: f32,
}

fn main() -> color_eyre::Result<()> {
    let Args {
        input_path,
        output_path,
        width,
        dpi,
        fps,
    } = Args::parse();

    color_eyre::install()?;

    let splits_file = File::open(input_path)?;
    let Splits { title, splits } = serde_json::from_reader(splits_file)?;
    let title_rc = Rc::new(title);

    let num_splits = splits.len();
    let num_lines = num_splits - 1;
    let height = (TITLE_HEIGHT_IN
        + num_splits as f64 * BLOCK_HEIGHT_IN
        + num_lines as f64 * LINE_THICKNESS_IN) as f64;

    let width_px = (width * dpi) as usize;
    let height_px = (height * dpi) as usize;

    let mut device = Device::new().unwrap();
    let mut bitmap = device.bitmap_target(width_px, height_px, 1.).unwrap();
    let mut ctx = bitmap.render_context();
    let mut frame_buf = vec![0u8; width_px * height_px * 4];

    let mut context = Context::new(&mut ctx);

    let mut ffmpeg_child = FfmpegCommand::new()
        .format("rawvideo")
        .pix_fmt("rgba")
        .size(width_px as u32, height_px as u32)
        .rate(fps)
        .input("-")
        .overwrite()
        .output(output_path.to_str().unwrap())
        .spawn()?;

    context.draw(title_rc, &splits, width_px, height_px, dpi);

    bitmap
        .copy_raw_pixels(ImageFormat::RgbaPremul, &mut frame_buf)
        .unwrap();
    ffmpeg_child.send_stdin_command(&frame_buf).unwrap();
    ffmpeg_child.quit().unwrap();

    eprintln!("written to '{}'", output_path.display());

    Ok(())
}
