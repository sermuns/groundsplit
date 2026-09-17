use std::fs::File;

use groundsplit::Splits;

fn main() -> color_eyre::Result<()> {
    let splits = Splits {
        title: "Hallway Game".to_string(),
        splits: [("Claw Sequence", 4 * 1000)].map(From::from).to_vec(),
    };

    let file = File::create("examples/hallway-game.json")?;
    serde_json::ser::to_writer_pretty(file, &splits)?;

    Ok(())
}
