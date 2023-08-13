/* a generic piece trait */
pub trait Piece {
    
    // get the current tuple spot in the grid
    fn get_spot(&self) -> (i32, i32);

    // get its index
    fn get_index() -> usize;

    // move the piece one square down
    fn tick(&mut self);

}
