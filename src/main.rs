mod constants;
mod game;

use game::TetrisGame;

fn main() {
    let mut tetris_game = TetrisGame::new().unwrap();

    tetris_game.mainloop().unwrap();
}
