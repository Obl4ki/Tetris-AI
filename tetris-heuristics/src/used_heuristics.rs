use crate::Heuristic;

#[must_use]
pub fn get_heuristics() -> Vec<Heuristic> {
    use crate::heuristics as h;

    vec![
        h::bumpyness,
        h::holes_present,
        h::relative_diff,
        h::highest_block,
        h::i_clear_potential,
        h::distance_mean_from_4,
    ]
}
