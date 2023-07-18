use crate::piece_type::PieceType;

#[derive(Default)]
pub struct Grid {
    state: [[Option<PieceType>; 10] ; 20],
}

impl Grid {
    // finds whole rows in the board and fixes it.
    pub fn board_fix(&mut self) {
        for i in 0..20 {

            if Self::is_full_row(&self.state[i]) {
                for j in (i+1)..20 {
                    self.state[j - 1] = self.state[j];
                }
            }

        }
    }

    fn is_full_row(row: &[Option<PieceType> ; 10]) -> bool {
        row.iter().all(|v| v.is_some() )
    }
}
