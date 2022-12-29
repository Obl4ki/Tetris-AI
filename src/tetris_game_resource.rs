use bevy::prelude::Resource;

use crate::tetris::game::Game;
use std::ops::{Deref, DerefMut};

#[derive(Resource)]
pub struct TetrisGameResource(pub Game);

impl Deref for TetrisGameResource {
    type Target = Game;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TetrisGameResource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
