#![warn(clippy::pedantic, clippy::nursery)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_precision_loss)]

pub mod heuristics;

pub use heuristics::{highest_block, holes_present};
