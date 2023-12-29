pub mod args;
pub mod meshgrid;
pub mod persistance;

use std::thread::sleep;
use std::time::Duration;

use args::CliArgs;

use tetris_core::prelude::*;
use tetris_core::scoring::Score;
use tetris_ml::prelude::*;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let args = CliArgs::parse();

    let best_entity = run_model(args)?;

    // play_game_with_entity(best_entity)?;
    Ok(())
}

fn run_model(args: CliArgs) -> Result<Entity> {
    let mut ga = GA::new(&mut Config::try_from(args)?, |population| {
        let best_entity = population.get_best_entity();
        println!("Best entity so far:");
        println!("Weights:\t{:?}", best_entity.weights);

        println!("Max fitness:\t{:.2}", population.biggest_fitness());
        println!("Worst fitness:\t{:.2}", population.lowest_fitness());

        println!("Mean fitness:\t{:.2}", population.mean_fitness());
        println!("Median fitness:\t{:.2}", population.median_fitness());

        let Score {
            cleared_rows,
            score,
            dropped_pieces,
        } = best_entity.game.score;
        println!("/-------------------------------------------\\");
        println!("| Cleared rows |   Score   | Dropped pieces |");
        println!("|-------------------------------------------|");
        println!(
            "|{:^14.2}|{:^11.2}|{:^16.2}|",
            cleared_rows, score, dropped_pieces
        );
        println!("\\-------------------------------------------/");

        println!("-----------------------------------------------------------");
    })?;

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
