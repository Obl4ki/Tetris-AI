#![warn(clippy::pedantic, clippy::nursery)]
pub mod heuristics;

pub use heuristics::{highest_block, holes_present};