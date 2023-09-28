#![warn(clippy::pedantic, clippy::nursery)]
#![allow(clippy::match_bool)]
// tetris board is only 10x20 and proper checks are made, so no numerical errors
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
pub mod board;
pub mod entities;
pub mod game;
pub mod game_builder;
pub mod piece;
pub mod prelude;
pub mod scoring;
pub mod srs;
