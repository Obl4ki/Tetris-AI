use anyhow::{bail, Result};
use clap::Parser;
use tetris_core::board::Board;
use tetris_heuristics as heuristics;
use tetris_ml::Config;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[allow(clippy::struct_excessive_bools)]
pub struct CliArgs {
    #[arg(short, default_value_t = 100)]
    pub n_entities: usize,
    #[arg(short, default_value_t = 0.02)]
    pub mutation_rate: f64,
    #[arg(long, help = "Maximum drops that a simulated game can last.")]
    pub max_drops: Option<usize>,
    #[arg(long, help = "Hard limit for number of populations to simulate.")]
    pub max_populations: Option<usize>,
    #[arg(
        long,
        help = "How many populations that aren't improving the best entity are allowed"
    )]
    pub max_non_progress_populations: Option<usize>,
}

impl TryFrom<CliArgs> for Config {
    type Error = anyhow::Error;

    fn try_from(args: CliArgs) -> Result<Self> {
        let heuristics_used: Vec<fn(&Board) -> f32> = vec![
            heuristics::bumpyness,
            heuristics::holes_present,
            heuristics::relative_diff,
            heuristics::highest_block,
            heuristics::clear_potential,
            heuristics::distance_mean_from_4,
        ];

        Ok(Self {
            n_entities: args.n_entities,
            mutation_rate: args.mutation_rate,
            max_drops: args.max_drops,
            max_populations: args.max_populations,
            max_non_progress_populations: args.max_non_progress_populations,
            heuristics_used,
        })
    }
}
