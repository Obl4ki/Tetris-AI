use tetris_core::scoring::Score;
use tetris_heuristics::used_heuristics::get_heuristics;

use anyhow::Result;
use tetris_ml::{Config, GA};

fn main() -> Result<()> {
    let configs = [
        Config {
            n_entities: 500,
            mutation_rate: 0.02,
            max_drops: Some(20_000),
            max_populations: Some(30),
            max_non_progress_populations: None,
            heuristics_used: get_heuristics(),
        },
        Config {
            n_entities: 1000,
            mutation_rate: 0.02,
            max_drops: Some(10_000),
            max_populations: Some(30),
            max_non_progress_populations: None,
            heuristics_used: get_heuristics(),
        },
    ];

    for (idx, mut config) in configs.into_iter().enumerate() {
        println!("Training number {idx}");
        println!("best_weights, best_fitness, mean_fitness, dropped_pieces, fours");

        let mut ga = GA::new(&mut config, |population| {
            let best_entity = population.get_best_entity();

            let Score {
                dropped_pieces,
                fours,
                ..
            } = best_entity.game.score;

            println!(
                "{:?}, {:?}, {:?}, {}, {}",
                best_entity.weights,
                population.biggest_fitness(),
                population.mean_fitness(),
                dropped_pieces,
                fours
            );
        })?;

        ga.train();
    }

    Ok(())
}
