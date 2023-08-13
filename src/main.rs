mod constants;
mod game;
mod grid_draw;
mod textures_manager;
mod piece;
mod tetris_grid;

use game::TetrisGame;

fn main() {
    let mut tetris_game = TetrisGame::new().unwrap();

    tetris_game.mainloop().unwrap();
}
