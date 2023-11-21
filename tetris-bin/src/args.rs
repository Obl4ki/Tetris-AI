use clap::Parser;
use tetris_core::board::Board;
use tetris_heuristics as heuristics;
use tetris_ml::Config;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
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

    #[arg(
        long,
        help = "Indicates that the model should use holes_present heuristic"
    )]
    pub holes_present: bool,

    #[arg(long, help = "Indicates that the model should use bumpyness heuristic")]
    pub bumpyness: bool,

    #[arg(
        long,
        help = "Indicates that the model should use relative difference heuristic"
    )]
    pub relative_diff: bool,

    #[arg(
        long,
        help = "Indicates that the model should use highest block heuristic"
    )]
    pub highest_block: bool,

    #[arg(
        long,
        help = "Indicates that the model should use clear potential heuristic"
    )]
    pub clear_potential: bool,
}

impl From<CliArgs> for Config {
    fn from(args: CliArgs) -> Self {
        let mut heuristics_used: Vec<fn(&Board) -> f32> = vec![];
        if args.bumpyness {
            heuristics_used.push(heuristics::bumpyness)
        }
        if args.holes_present {
            heuristics_used.push(heuristics::holes_present)
        }
        if args.highest_block {
            heuristics_used.push(heuristics::highest_block)
        }
        if args.relative_diff {
            heuristics_used.push(heuristics::relative_diff)
        }
        if args.clear_potential {
            heuristics_used.push(heuristics::clear_potential)
        }

        if heuristics_used.is_empty() {
            panic!("Pick at least one heuristic using the flag arguments.")
        }

        Self {
            n_entities: args.n_entities,
            mutation_rate: args.mutation_rate,
            max_drops: args.max_drops,
            max_populations: args.max_populations,
            max_non_progress_populations: args.max_non_progress_populations,
            heuristics_used,
        }
    }
}
