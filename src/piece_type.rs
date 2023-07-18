
// enum order is important here cuz texture index
#[derive(Clone, Copy)]
pub enum PieceType {
    I,
    T,
    L,
    J,
    Z,
    S,
    O
}

trait Piece {
    
}

impl PieceType {
    pub fn index(&self) -> i32 {
        return *self as i32
    }
}
