use crate::{population::Population, Config, Entity};
use anyhow::Result;

pub struct GA {
    pub populations: Vec<Population>,
    pub max_populations: Option<usize>,
    pub max_non_progress: Option<usize>,
    best_entity: Entity,
}

impl GA {
    /// Create new Genetic algorithm instance, containing all the populations.
    /// # Errors
    ///
    /// This function will return an error if [`Config::validate`] fails.
    pub fn new(config: &mut Config, evaluator: fn(&Population)) -> Result<Self> {
        let start_population = Population::new(config, evaluator)?;
        Ok(Self {
            max_populations: config.max_populations,
            max_non_progress: config.max_non_progress_populations,
            best_entity: start_population.get_best_entity().clone(),
            populations: vec![start_population],
        })
    }

    #[must_use]
    pub fn get_current_population(&self) -> &Population {
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
            let next = current.advance();

            let best_before = current.sorted_by_performance()[0];
            let best_after = next.sorted_by_performance()[0];

            if Population::fitness(best_before) >= Population::fitness(best_after) {
                self.max_non_progress = self.max_non_progress.map(|x| x.saturating_sub(1));
            }

            let next_best = next.get_best_entity();
            if Population::fitness(next_best) > Population::fitness(&self.best_entity) {
                self.best_entity = next_best.clone();
            }

            self.populations.push(next);
        }
    }

    pub fn advance(&mut self) -> Option<&Population> {
        if matches!(self.max_non_progress, Some(n) if n == 0) {
            return None;
        }

        if matches!(self.max_populations, Some(max) if max >= self.populations.len()) {
            return None;
        }

        Some(self.get_current_population())
    }

    #[must_use]
    pub fn get_best_entity(&self) -> Entity {
        self.best_entity.clone()
    }
}
