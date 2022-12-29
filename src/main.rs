#[macro_use]
extern crate lazy_static;

use bevy::prelude::*;
use itertools::Itertools;
mod tetris;
use tetris::blocks::BlockType;
use tetris::game::Game;

const BLOCK_SIZE: f32 = 20.;
const GLOBAL_OFFSET: f32 = -200.;

mod tetris_game_resource;
use tetris_game_resource::TetrisGameResource;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_system)
        .insert_resource(ClearColor(Color::rgb(
            173. / 255.,
            216. / 255.,
            230. / 255.,
        )))
        .insert_resource(Msaa { samples: 4 })
        .add_system(bevy::window::close_on_esc)
        .insert_resource(TetrisGameResource(Game::new(10, 20)))
        .add_system(draw_game_state)
        .add_system(keyboard_handling)
        .run();
}

fn setup_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn draw_game_state(mut commands: Commands, game: ResMut<TetrisGameResource>) {
    for (x, y, block) in game.iter_board() {
        let color = get_color_of_block_type(block);

        draw_rectangle(
            &mut commands,
            color,
            x as f32 * BLOCK_SIZE + GLOBAL_OFFSET,
            y as f32 * BLOCK_SIZE + GLOBAL_OFFSET,
        )
    }

    //draw falling piece
    for (x, y) in game.falling_piece.iter_blocks() {
        let color = get_color_of_block_type(game.falling_piece.block_type);
        draw_rectangle(
            &mut commands,
            color,
            x as f32 * BLOCK_SIZE + GLOBAL_OFFSET,
            y as f32 * BLOCK_SIZE + GLOBAL_OFFSET,
        )
    }
}

fn get_color_of_block_type(val: BlockType) -> Color {
    match val {
        BlockType::IShape => Color::rgb(0., 0., 1.),
        BlockType::OShape => Color::rgb(1., 1., 0.),
        BlockType::TShape => Color::rgb(0.5, 0., 0.5),
        BlockType::SShape => Color::rgb(0., 0.5, 0.),
        BlockType::ZShape => Color::rgb(1., 0., 0.),
        BlockType::JShape => Color::rgb(0., 0., 1.),
        BlockType::LShape => Color::rgb(1., 0.5, 0.),
        BlockType::None => Color::rgb(0.5, 0.5, 0.5),
    }
}

fn draw_rectangle(commands: &mut Commands, color: Color, x: f32, y: f32) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color,
            custom_size: Some(Vec2::new(20.0, 20.0)),
            ..default()
        },
        transform: Transform::from_xyz(x, y, 0.),
        ..default()
    });
}

fn keyboard_handling(keyboard: Res<Input<KeyCode>>, mut game: ResMut<TetrisGameResource>) {
    if keyboard.just_pressed(KeyCode::Space) {
        game.drop();
    }
    if keyboard.just_pressed(KeyCode::Left) {
        game.go_left();
    }
    if keyboard.just_pressed(KeyCode::Right) {
        game.go_right();
    }
    if keyboard.just_pressed(KeyCode::Down) {
        game.fall_by_one();
    }
    if keyboard.just_pressed(KeyCode::Z) {
        game.rotate_cw();
    }
    if keyboard.just_pressed(KeyCode::X) {
        game.rotate_ccw();
    }
}
