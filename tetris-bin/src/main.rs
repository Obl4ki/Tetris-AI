pub mod args;
pub mod meshgrid;

use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

use args::CliArgs;

use clearscreen::clear;
use meshgrid::FitnessAtPoint;
use tetris_core::prelude::*;
use tetris_heuristics::{bumpyness, clear_potential};
use tetris_ml::prelude::*;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let args = CliArgs::parse();

    let data = meshgrid::generate_grid(-1.0, 1.0, 20, bumpyness, clear_potential)?;
    write_points_to_file(
        "data/output.csv",
        &data,
        "Bumpyness",
        "Clear potential",
        "Fitness",
    )?;

    show_graph()?;
    // let best_entity = run_model(args)?;

    // play_game_with_entity(best_entity)?;
    Ok(())
}

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

fn show_graph() -> Result<()> {
    Command::new("python")
        .arg("scripts/plot_heuristics.py")
        .spawn()?;
    Ok(())
}

fn run_model(args: CliArgs) -> Result<Entity> {
    let mut ga = GA::new(&Config::try_from(args)?)?;

    ga.train();

    Ok(ga.get_best_entity())
}

fn play_game_with_entity(mut entity: Entity) -> Result<()> {
    entity.game = Game::new();
    while let Some(agent) = entity.next_best_state(Piece::random()) {
        entity = agent;
        clearscreen::clear()?;

        println!("Metaheuristic: {}", entity.forward());
        println!("Score: {:?}", entity.game.score);

        println!("{}", entity.game.board);

        sleep(Duration::from_millis(200));
    }

    Ok(())
}

#[allow(unused)]
fn run_with_weights(weights: Vec<f32>) -> Result<()> {
    let mut current_entity = Entity::from_weights(weights, &[])?;

    while let Some(entity) = current_entity.next_best_state(Piece::random()) {
        current_entity = entity;
        clearscreen::clear().unwrap();

        println!("Metaheuristic: {}", current_entity.forward());
        println!("Score: {:?}", current_entity.game.score);

        println!("{}", current_entity.game.board);

        sleep(Duration::from_millis(200));
    }
    Ok(())
}
