// #![allow(clippy::needless_pass_by_value)]

// use bevy::prelude::*;
// use tetris_bin::used_heuristics;
// use tetris_core::prelude::*;

// use bevy::core_pipeline::prelude::ClearColor;
// use tetris_heuristics::heuristics;

// use std::{
//     ops::{Deref, DerefMut},
//     sync::Arc,
// };

// use tetris_ml::Agent;

// #[derive(Resource)]
// pub struct TetrisGameResource(pub Agent);

// impl Deref for TetrisGameResource {
//     type Target = Agent;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// impl DerefMut for TetrisGameResource {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }

// fn main() {
//     App::new()
//         .add_plugins(DefaultPlugins)
//         .add_systems(Startup, setup)
//         .insert_resource(ClearColor(Color::rgb(
//             173. / 255.,
//             216. / 255.,
//             230. / 255.,
//         )))
//         .insert_resource(Msaa::Sample4)
//         .insert_resource(TetrisGameResource(Agent::new(Arc::new(used_heuristics::get_heuristics()))))
//         .add_systems(PreUpdate, bevy::window::close_on_esc)
//         .add_systems(PreUpdate, draw_background)
//         .add_systems(PreUpdate, despawn_all_blocks)
//         .add_systems(Update, draw_game_state)
//         .run();
// }

// fn setup(mut commands: Commands) {
//     commands.spawn(Camera2dBundle::default());
// }

// fn despawn_all_blocks(mut commands: Commands, query: Query<Entity, With<Sprite>>) {
//     for entity in query.iter() {
//         commands.entity(entity).despawn();
//     }
// }

// fn draw_game_state(mut commands: Commands, agent: ResMut<TetrisGameResource>) {
//     for (pos, block) in agent.game.board.iter_blocks() {
//         let color = get_color_of_block_type(block);

//         draw_rectangle(
//             &mut commands,
//             color,
//             (pos.x as f32).mul_add(BLOCK_SIZE + MARGIN, GLOBAL_OFFSET),
//             (pos.y as f32).mul_add(BLOCK_SIZE + MARGIN, GLOBAL_OFFSET),
//         );
//     }

//     for pos in agent.game.piece.iter_blocks() {
//         let color = get_color_of_block_type(agent.game.piece.block_type);
//         draw_rectangle(
//             &mut commands,
//             color,
//             (pos.x as f32).mul_add(BLOCK_SIZE + MARGIN, GLOBAL_OFFSET),
//             (pos.y as f32).mul_add(BLOCK_SIZE + MARGIN, GLOBAL_OFFSET),
//         );
//     }
// }

use anyhow::Result;
use macroquad::prelude::*;
use tetris_bin::used_heuristics::get_heuristics;
use tetris_core::prelude::*;
use tetris_ml::{Agent, BranchingMode};
const BLOCK_SIZE: f32 = 20.;
const MARGIN: f32 = 5.;
#[macroquad::main("Tetris Agent")]
async fn main() -> Result<()> {
    let mut agent = Agent::from_weights(
        vec![
            0.153_936_06,
            0.664_052_07,
            0.087_044_27,
            0.103_674_956,
            -0.382_218_1,
            0.0,
        ],
        &get_heuristics(),
    )?;

    let game_width = agent.game.width;
    let game_height = agent.game.height;

    loop {
        clear_background(BLACK);
        draw_background(game_width, game_height);
        draw_current_state(&agent);

        if let Some(next_state) = agent.next_best_state(BranchingMode::Current) {
            agent = Agent {
                game: next_state,
                ..agent
            };
        }

        next_frame().await;
    }
}

fn draw_tetrimino(x: f32, y: f32, color: Color) {
    draw_rectangle(
        f32::abs(x - 10.).mul_add(BLOCK_SIZE, MARGIN),
        f32::abs(y - 20.).mul_add(BLOCK_SIZE, MARGIN),
        BLOCK_SIZE - 2.,
        BLOCK_SIZE - 2.,
        color,
    );
}

fn draw_current_state(agent: &Agent) {
    for (pos, block) in agent.game.board.iter_blocks() {
        draw_tetrimino(pos.x as f32, pos.y as f32, get_color_of_block(block));
    }

    for pos in agent.game.piece.iter_blocks() {
        draw_tetrimino(
            pos.x as f32,
            pos.y as f32,
            get_color_of_block(agent.game.piece.block_type),
        );
    }
}

fn draw_background(width: i32, height: i32) {
    for x in 0..width {
        for y in 0..height {
            draw_tetrimino(x as f32, y as f32, Color::from_rgba(127, 127, 127, 255));
        }
    }
}

const fn get_color_of_block(val: PieceType) -> Color {
    match val {
        PieceType::I => Color::new(0., 255., 255., 255.),
        PieceType::O => Color::new(255., 255., 0., 255.),
        PieceType::T => Color::new(127., 0., 127., 255.),
        PieceType::S => Color::new(0., 127., 0., 255.),
        PieceType::Z => Color::new(255., 0., 0., 255.),
        PieceType::J => Color::new(0., 0., 255., 255.),
        PieceType::L => Color::new(255., 127., 0., 255.),
    }
}
