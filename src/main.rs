use std::{fs::File, path::PathBuf, rc::Rc};

use clap::Parser;
use ffmpeg_sidecar::command::FfmpegCommand;
use groundsplit::{
    Splits,
    draw::{Context, LINE_THICKNESS_IN, SPLIT_HEIGHT_IN, TIME_HEIGHT_IN, TITLE_HEIGHT_IN},
};
use piet_common::Device;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    input_path: PathBuf,

    #[arg(short)]
    output_path: PathBuf,

    // In "inches"
    #[arg(short, long, default_value_t = 4.)]
    width: f64,

    // In "inches"
    #[arg(short, long, default_value_t = 0.1)]
    padding: f64,

    /// "Dots per inch" / pixel density. Determines output width/height.
    #[arg(long, default_value_t = 200.)]
    dpi: f64,

    #[arg(short = 'r', long, default_value_t = 30.)]
    fps: f32,

    /// If given, only render until this many seconds
    #[arg(short, long)]
    max_duration_secs: Option<f32>,
}

fn main() -> color_eyre::Result<()> {
    let Args {
        input_path,
        output_path,
        width,
        padding,
        dpi,
        fps,
        max_duration_secs,
    } = Args::parse();
    dbg!(max_duration_secs);

    color_eyre::install()?;

    let splits_file = File::open(input_path)?;
    let Splits { title, splits } = serde_json::from_reader(splits_file)?;
    let title_rc = Rc::new(title);

    let num_splits = splits.len();
    let num_lines = num_splits - 1;
    let height = TITLE_HEIGHT_IN
        + num_splits as f64 * SPLIT_HEIGHT_IN
        + num_lines as f64 * LINE_THICKNESS_IN
        + TIME_HEIGHT_IN;

    let width_px = (width * dpi) as usize;
    let height_px = (height * dpi) as usize;

    let mut device = Device::new().unwrap();
    let bitmap = device.bitmap_target(width_px, height_px, 1.).unwrap();
    let mut context = Context::new(bitmap, width_px, height_px, dpi, padding);

    let mut ffmpeg_child = FfmpegCommand::new()
        .format("rawvideo")
        .pix_fmt("rgba")
        .size(width_px as u32, height_px as u32)
        .rate(fps)
        .input("-")
        .overwrite()
        .output(output_path.to_str().unwrap())
        .spawn()?;

    let video_duration_ms = splits.last().unwrap().ms_since_start;
    let ms_per_frame = 1000. / fps;
    // WARNING: truncating
    let num_frames = (video_duration_ms as f32 / ms_per_frame) as usize;

    for frame_number in 0..num_frames {
        let milliseconds_since_start = (ms_per_frame * frame_number as f32) as u128;
        eprintln!(
            "doing frame {} ({} ms)",
            frame_number, milliseconds_since_start
        );

        context.draw_to_frame_buf(title_rc.clone(), &splits, milliseconds_since_start);

        ffmpeg_child.send_stdin_command(&context.frame_buf).unwrap();

        if max_duration_secs.is_some_and(|s| milliseconds_since_start as f32 / 1000. >= s) {
            eprintln!("breaking early due to `max_duration_secs` being given");
            break;
        }
    }

    ffmpeg_child.quit().unwrap();
    eprintln!("written to '{}'", output_path.display());

    Ok(())
}
