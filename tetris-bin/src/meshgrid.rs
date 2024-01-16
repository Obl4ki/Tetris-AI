use anyhow::Result;
use indicatif::{ParallelProgressIterator, ProgressStyle};
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use tetris_core::game::Game;
use tetris_heuristics::Heuristic;
use tetris_ml::{Agent, Population};

fn linspace(start: f32, stop: f32, num: usize) -> Vec<f32> {
    if num == 0 {
        return Vec::new();
    }

    let step_by = (stop - start) / ((num - 1) as f32);
    let mut result = Vec::with_capacity(num);

    for i in 0..num {
        result.push((i as f32).mul_add(step_by, start));
    }

    result
}

const N_TRIES: usize = 20;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct FitnessAtPoint {
    pub x: f32,
    pub y: f32,
    pub fitness: f64,
}

pub fn generate_grid(
    from: f32,
    to: f32,
    n_samples: usize,
    x_heuristic: Heuristic,
    y_heuristic: Heuristic,
) -> Result<Vec<FitnessAtPoint>> {
    let mut locations: Vec<(f32, f32)> = Vec::with_capacity(n_samples.pow(2));

    for (x, y) in linspace(from, to, n_samples)
        .into_iter()
        .cartesian_product(linspace(from, to, n_samples).into_iter())
    {
        locations.push((x, y));
    }

    locations
        .into_par_iter()
        .map(|(x, y)| {
            let mut entity =
            Agent::from_weights(vec![x, y], &[x_heuristic, y_heuristic])?;
            let mut mean_fitness = 0.0;
            for _ in 0..N_TRIES {
                entity.game = Game::new();
                entity = entity.play_until_lost(tetris_ml::BranchingMode::Current);
                mean_fitness += Population::fitness(&entity);
            }

            Ok(FitnessAtPoint { x, y, fitness: mean_fitness / N_TRIES as f64 })
        })
        .progress_with_style(
            ProgressStyle::with_template(
                "Generating meshgrid: [{elapsed_precise}] eta: {eta} {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
            )
            ?
        )
        .collect()
}

#[cfg(test)]
mod tests {
    use super::linspace;

    #[must_use]
    fn approx_eq(a: f32, b: f32) -> bool {
        (a - b).abs() <= f32::EPSILON
    }

    #[test]
    fn test_linspace_simple() {
        let expected = 45.;
        let res = linspace(0.0, 9.0, 10);
        assert!(approx_eq(expected, res.iter().sum::<f32>()));

        assert!(approx_eq(*res.first().unwrap(), 0.0));
        assert!(approx_eq(*res.last().unwrap(), 9.0));
        assert_eq!(res.len(), 10);
    }
}
