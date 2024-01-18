use anyhow::Result;

#[macroquad::main("Tetris Agent")]
async fn main() -> Result<()> {
    tetris_ui::run().await?;
    Ok(())
}
