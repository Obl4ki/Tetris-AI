use anyhow::Result;
use macroquad::prelude::*;
use tetris_core::prelude::*;
use tetris_heuristics::used_heuristics::get_heuristics;
use tetris_ml::{Agent, BranchingMode};
const BLOCK_SIZE: f32 = 30.;
const BORDER: f32 = 2.;
const GRID_SPACING: f32 = 2.;
const BOARD_MARGIN: f32 = 5.;

pub async fn run() -> Result<()> {
    let mut agent = Agent::from_weights(
        vec![
            0.153_936_06,
            0.664_052_07,
            0.087_044_27,
            0.103_674_956,
            -0.382_218_1,
        ],
        &get_heuristics(),
    )?;

    let game_width = agent.game.width;
    let game_height = agent.game.height;

    loop {
        clear_background(BLACK);
        draw_background(game_width, game_height);
        draw_current_state(&agent);

        agent.make_a_move(BranchingMode::Current);

        next_frame().await;
    }
}

fn draw_tetrimino(x: f32, y: f32, color: Color) {
    let x = f32::abs(x - 10.).mul_add(BLOCK_SIZE + GRID_SPACING, BOARD_MARGIN);
    let y = f32::abs(y - 20.).mul_add(BLOCK_SIZE + GRID_SPACING, BOARD_MARGIN);

    draw_rectangle(x, y, BLOCK_SIZE, BLOCK_SIZE, color);
    // top
    draw_line(
        x + BORDER / 2.,
        y - BORDER / 2.,
        x - BORDER / 2. + BLOCK_SIZE,
        y - BORDER / 2.,
        BORDER,
        Color::from_rgba(255, 255, 255, 55),
    );

    // left
    draw_line(
        x + BORDER / 2.,
        y + BORDER / 2.,
        x - BORDER / 2.,
        y - BORDER / 2. + BLOCK_SIZE,
        BORDER,
        Color::from_rgba(255, 255, 255, 55),
    );

    // right
    draw_line(
        x - BORDER / 2. + BLOCK_SIZE,
        y + BORDER / 2.,
        x - BORDER / 2. + BLOCK_SIZE,
        y - BORDER / 2. + BLOCK_SIZE,
        BORDER,
        Color::from_rgba(255, 255, 255, 55),
    );

    // bottom
    draw_line(
        x + BORDER / 2.,
        y - BORDER / 2. + BLOCK_SIZE,
        x - BORDER / 2. + BLOCK_SIZE,
        y - BORDER / 2. + BLOCK_SIZE,
        BORDER,
        Color::from_rgba(255, 255, 255, 55),
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
