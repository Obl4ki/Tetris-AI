pub mod args;
pub mod meshgrid;
pub mod persistance;

use std::thread::sleep;
use std::time::Duration;

use args::CliArgs;

use tetris_core::prelude::*;
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
        println!("Weights: {:?}", best_entity.weights);

        println!("Mean fitness: {}", population.mean_fitness());
        println!("Max fitness: {}", population.biggest_fitness());
        println!("Worst fitness: {}", population.lowest_fitness());

        println!("Score: {:?}", best_entity.game.score);
        println!("Fitness: {:?}", Population::fitness(best_entity));
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
