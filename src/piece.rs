use crate::tetris_grid::Grid;

type SpotType = (i32, i32);

/* a generic piece trait */
pub trait Piece {
    
    // get the current tuple spot in the grid
    fn get_spot(&self) -> SpotType;

    // get its index
    fn get_index() -> usize;

    // move the piece one square down
    fn move_down(&mut self);

    // check if piece should stop
    fn should_stop(&self, game_grid: &Grid);
}

pub struct IPiece {
    spot: SpotType
}

impl Piece for IPiece {
    fn get_spot(&self) -> SpotType {
        return self.spot;
    }

    fn get_index() -> usize {
        return 0;
    }

    fn move_down(&mut self) {
        self.spot.1 -= 1;
    }

    fn should_stop(&self, game_grid: &Grid) {
        const LOWER_SQUARE: SpotType = (0, -3);
    }
}

pub struct TPiece {
    spot: SpotType
}

pub struct LPiece {
    spot: SpotType
}

pub struct JPiece {
    spot: SpotType
}

pub struct ZPiece {
    spot: SpotType
}

pub struct SPiece {
    spot: SpotType
}

pub struct OPiece {
    spot: SpotType
}
