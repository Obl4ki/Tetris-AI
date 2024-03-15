use std::fs::File;
use std::io::Write as _;

use tetris_bin::meshgrid;
use tetris_bin::meshgrid::FitnessAtPoint;

use tetris_heuristics::heuristics::{bumpyness, holes_present};

use anyhow::Result;

fn main() -> Result<()> {
    let grid = meshgrid::generate_grid(-1.0, 1.0, 100, bumpyness, holes_present, 50_000)?;

    let mut file = File::create("data/output2d.csv")?;
    file.write_all(b"x, y, fitness\n")?;

    for FitnessAtPoint { x, y, fitness } in grid {
        file.write_all(format!("{x},{y},{fitness}\n").as_bytes())?;
    }

    Ok(())
}
