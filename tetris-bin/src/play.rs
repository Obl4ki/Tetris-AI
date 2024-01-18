use anyhow::Result;
use macroquad::window::Conf;

fn window_conf() -> Conf {
    Conf {
        window_title: "Tetris Agent".to_string(),
        window_width: 1200,
        window_height: 800,
        window_resizable: false,
        ..Conf::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() -> Result<()> {
    tetris_ui::run().await?;
    Ok(())
}
