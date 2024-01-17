use tetris_heuristics::Heuristic;

#[must_use]
pub fn get_heuristics() -> Vec<Heuristic> {
    use tetris_heuristics::heuristics as h;

    vec![
        h::bumpyness,
        h::holes_present,
        h::relative_diff,
        h::highest_block,
        h::clear_potential,
        // h::distance_mean_from_4,
    ]
}
