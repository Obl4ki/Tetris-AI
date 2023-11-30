use std::io::Write;
use std::{fs::File, path::Path};

use anyhow::Result;

use crate::meshgrid::FitnessAtPoint;

fn write_points_to_file(
    filename: impl AsRef<Path>,
    data_points: &[FitnessAtPoint],
    x_label: &str,
    y_label: &str,
    z_label: &str,
) -> Result<()> {
    let mut file = File::create(filename)?;
    writeln!(file, "{x_label},{y_label},{z_label}")?;

    for FitnessAtPoint { x, y, fitness } in data_points {
        writeln!(file, "{x},{y},{fitness}")?;
    }

    Ok(())
}
