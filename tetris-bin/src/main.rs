#![allow(unused)]

mod game_runner;

use std::{thread::sleep, time::Duration};

use tetris_core::prelude::*;
use tetris_ml::{genetic_algorithm::GA, prelude::*};

use anyhow::Result;
use tetris_heuristics::prelude::*;
fn main() -> Result<()> {
    // populations()?;
    let best_entity = run_model()?;

    play_game_with_entity(best_entity)?;
    Ok(())
}

fn run_model() -> Result<Entity> {
    let mut ga = GA::new(&Config {
        n_entities: 100,
        crossover_rate: 0.98,
        mutation_rate: 0.02,
        max_drops: Some(10_000),
        max_populations: Some(50),
        max_non_progress_populations: Some(3),
        heuristics_used: vec![bumpyness, holes_present],
    })?;

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

fn populations() -> Result<()> {
    let mut population = Population::new(&Config {
        n_entities: 32,
        crossover_rate: 0.98,
        mutation_rate: 0.02,
        max_drops: Some(10_000),
        heuristics_used: vec![highest_block, relative_diff, bumpyness, holes_present],
        max_populations: None,
        max_non_progress_populations: Some(2),
    })?;

    for idx in 1..10 {
        println!("-----------------------------------------------------");
        println!("Gen {}", idx);
        println!("Num of entities before: {}", population.entities.len());
        population = population.advance();
        println!("Num of entities after: {}", population.entities.len());

        let best_entity = population
            .sorted_by_performance()
            .into_iter()
            .next()
            .unwrap();

        println!("Best entity statistics:");
        println!("Weights: {:?}", best_entity.weights);
        println!("Score: {:?}", best_entity.game.score);
        println!("Fitness: {:?}", Population::fitness(best_entity));
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
