#![allow(unused)]
pub mod args;

use std::{thread::sleep, time::Duration};

use args::CliArgs;
use tetris_core::prelude::*;
use tetris_ml::{genetic_algorithm::GA, prelude::*};

use anyhow::Result;
use clap::Parser;
use tetris_heuristics::prelude::*;

fn main() -> Result<()> {
    let args = CliArgs::parse();
    let best_entity = run_model(args)?;

    play_game_with_entity(best_entity)?;
    Ok(())
}

fn run_model(args: CliArgs) -> Result<Entity> {
    let mut ga = GA::new(&Config::from(args))?;

    ga.train(true);

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
