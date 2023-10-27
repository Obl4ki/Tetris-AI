#![allow(unused)]
use bevy::prelude::*;
use tetris_core::prelude::*;

const BLOCK_SIZE: f32 = 20.;
const GLOBAL_OFFSET: f32 = -200.;
const MARGIN: f32 = 2.0;

mod tetris_game_resource;
use bevy::core_pipeline::prelude::ClearColor;
use tetris_game_resource::TetrisGameResource;
use tetris_ml::population::Population;

use std::process;

fn main() {
    run_game();
}

fn run_game() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Msaa::Sample4)
        .insert_resource(TetrisGameResource(Game::new()))
        .add_systems(PreUpdate, bevy::window::close_on_esc)
        .add_systems(PreUpdate, draw_background)
        .add_systems(PreUpdate, despawn_all_blocks)
        .add_systems(Update, draw_game_state)
        .add_systems(Update, keyboard_handling)
        .add_systems(PostUpdate, if_lost_then_exit)
        .run();
}
fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn despawn_all_blocks(mut commands: Commands, query: Query<Entity, With<Sprite>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn draw_game_state(mut commands: Commands, game: Res<TetrisGameResource>) {
    for (pos, block) in game.board.iter_blocks() {
        let color = get_color_of_block_type(block);

        draw_rectangle(
            &mut commands,
            color,
            pos.x as f32 * (BLOCK_SIZE + MARGIN) + GLOBAL_OFFSET,
            pos.y as f32 * (BLOCK_SIZE + MARGIN) + GLOBAL_OFFSET,
        )
    }

    for pos in game.piece.iter_blocks() {
        let color = get_color_of_block_type(game.piece.block_type);
        draw_rectangle(
            &mut commands,
            color,
            pos.x as f32 * (BLOCK_SIZE + MARGIN) + GLOBAL_OFFSET,
            pos.y as f32 * (BLOCK_SIZE + MARGIN) + GLOBAL_OFFSET,
        );
    }
}

fn get_color_of_block_type(val: PieceType) -> Color {
    match val {
        PieceType::I => Color::rgb(0., 1., 1.),
        PieceType::O => Color::rgb(1., 1., 0.),
        PieceType::T => Color::rgb(0.5, 0., 0.5),
        PieceType::S => Color::rgb(0., 1., 0.),
        PieceType::Z => Color::rgb(1., 0., 0.),
        PieceType::J => Color::rgb(0., 0., 1.),
        PieceType::L => Color::rgb(1., 0.5, 0.),
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
    if game.is_lost() {
        return;
    }

    if keyboard.just_pressed(KeyCode::Space) {
        game.hard_drop();
        dbg!(&game.score);
    }
    if keyboard.just_pressed(KeyCode::Left) {
        game.go_left();
    }
    if keyboard.just_pressed(KeyCode::Right) {
        game.go_right();
    }
    if keyboard.just_pressed(KeyCode::Down) {
        game.go_down();
    }
    if keyboard.just_pressed(KeyCode::Z) {
        game.rotate(Rotation::Counterclockwise);
    }
    if keyboard.just_pressed(KeyCode::X) {
        game.rotate(Rotation::Clockwise);
    }
}

fn draw_background(mut commands: Commands, game: Res<TetrisGameResource>) {
    for x in 0..game.width {
        for y in 0..game.height {
            draw_rectangle(
                &mut commands,
                Color::rgb(0.5, 0.5, 0.5),
                x as f32 * (BLOCK_SIZE + MARGIN) + GLOBAL_OFFSET,
                y as f32 * (BLOCK_SIZE + MARGIN) + GLOBAL_OFFSET,
            )
        }
    }
}

fn if_lost_then_exit(game: Res<TetrisGameResource>) {
    if game.is_lost() {
        process::exit(0);
    }
}
