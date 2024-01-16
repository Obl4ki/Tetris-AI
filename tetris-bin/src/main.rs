pub mod args;
pub mod meshgrid;
pub mod persistance;

use std::fs::File;
use std::io::Write as _;
use std::thread::sleep;
use std::time::Duration;

use args::CliArgs;

use tetris_core::prelude::*;
use tetris_core::scoring::Score;
use tetris_heuristics::Heuristic;
use tetris_ml::prelude::*;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let config: Config = CliArgs::parse().try_into()?;

    // let best_entity = run_model(config)?;
    // let weights = &best_entity.weights;
    // write weight to file

    // let mut file = File::create("best_weight.txt")?;
    // file.write_all(format!("{:?}", weights).as_bytes())?;

    // play_game_with_entity(best_entity)?;

    run_with_weights(
        vec![0.15393606, 0.66405207, 0.08704427, 0.103674956, -0.3822181, 0.0], // najlepszy do n=1
        // vec![0.8096802, 0.8273554, 0.35399604, -0.56748724, 0.30727196, 0.1149559], // najlepszy do n=2
        &config.heuristics_used,
    )?;

    //[0.8096802, 0.8273554, 0.35399604, -0.56748724, 0.30727196, 0.1149559]

    Ok(())
}

#[allow(unused)]
fn run_model(mut config: Config) -> Result<Agent> {
    let mut ga = GA::new(&mut config, |population| {
        let best_entity = population.get_best_entity();
        println!("Best entity this population:");
        println!("Weights:\t{:?}", best_entity.weights);

        println!("Max fitness:\t{:.2}", population.biggest_fitness());
        println!("Worst fitness:\t{:.2}", population.lowest_fitness());

        println!("Mean fitness:\t{:.2}", population.mean_fitness());
        println!("Median fitness:\t{:.2}", population.median_fitness());

        let Score {
            cleared_rows,
            score,
            dropped_pieces,
            fours,
            threes,
            twos,
            ones,
        } = best_entity.game.score;

        println!("/--------------------------------------------");
        println!("| Cleared rows |   Score   | Dropped pieces |");
        println!("|-------------------------------------------|");
        println!(
            "|{:^14.2}|{:^11.2}|{:^16.2}|",
            cleared_rows, score, dropped_pieces
        );

        println!("Fours: {fours}");
        println!("Threes: {threes}");
        println!("Twos: {twos}");
        println!("Ones: {ones}");

        println!("\\-------------------------------------------/");

        println!("-----------------------------------------------------------");
    })?;

    ga.train();

    Ok(ga.get_best_entity())
}

// fn play_game_with_entity(mut entity: Agent) -> Result<()> {
//     entity.game = Game::new();
//     while let Some(agent) = entity.next_best_state(Piece::random()) {
//         entity = agent;
//         clearscreen::clear()?;

//         println!("Metaheuristic: {}", entity.forward());
//         println!("Score: {:?}", entity.game.score);

//         println!("{}", entity.game.board);

//         sleep(Duration::from_millis(200));
//     }

//     Ok(())
// }

fn run_with_weights(weights: Vec<f32>, heuristics: &[Heuristic]) -> Result<()> {
    let mut current_entity = Agent::from_weights(weights, heuristics)?;

    while let Some(next_state) = current_entity.next_best_state(BranchingMode::CurrentAndNext) {
        println!("Metaheuristic: {}", current_entity.forward());
        println!("Score: {:?}", current_entity.game.score);
        println!("Current piece: {:?}", current_entity.game.piece.block_type);
        println!("Next piece: {:?}", current_entity.game.next_piece.block_type);
        println!("Board:");
        println!("{}", current_entity.game.board);
        println!("--------------------------");
        current_entity = Agent {
            game: next_state,
            ..current_entity
        };
        // clearscreen::clear().unwrap();


        sleep(Duration::from_millis(200));
    }

    Ok(())
}
