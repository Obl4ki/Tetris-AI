use crate::{population::Population, Agent, Config};
use anyhow::Result;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub struct GA {
    pub populations: Vec<Population>,
    pub max_populations: Option<usize>,
    pub max_non_progress: Option<usize>,
}

impl GA {
    /// Create new Genetic algorithm instance, containing all the populations.
    /// # Errors
    ///
    /// This function will return an error if [`Config::validate`] fails.
    pub fn new(config: &mut Config, population_evaluator: fn(&Population)) -> Result<Self> {
        let start_population = Population::new(config, population_evaluator)?;
        Ok(Self {
            max_populations: config.max_populations,
            max_non_progress: config.max_non_progress_populations,
            populations: vec![start_population],
        })
    }

    #[must_use]
    fn get_current_population(&self) -> &Population {
        self.populations
            .last()
            .expect("New constructs GA with at least 1 starting population, so last population will always exist.")
    }

    pub fn train(&mut self) {
        for _ in 0..self.max_populations.unwrap_or(usize::MAX) {
            if matches!(self.max_non_progress, Some(n) if n == 0) {
                break;
            }

            let current = self.get_current_population();
            let next = current.advance_population();

            let best_before = current.get_best_entity();
            let best_after = next.get_best_entity();

            if best_before.fitness() >= best_after.fitness() {
                self.max_non_progress = self.max_non_progress.map(|x| x.saturating_sub(1));
            }

            self.populations.push(next);
        }
    }

    #[must_use]
    pub fn get_best_entity(&self) -> Agent {
        let population_with_best_agent = self
            .populations
            .par_iter()
            .max_by(|p1, p2| {
                p1.get_best_entity()
                    .fitness()
                    .total_cmp(&p2.get_best_entity().fitness())
            })
            .unwrap();

        population_with_best_agent
            .entities
            .par_iter()
            .max_by(|x, y| x.fitness().total_cmp(&y.fitness()))
            .unwrap()
            .clone()
    }
}
