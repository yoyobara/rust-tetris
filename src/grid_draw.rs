use sdl2::{
    rect::Rect, render::Canvas, video::Window, pixels::Color
};

use crate::{piece_type::PieceType, textures_manager::TexturesManager};

pub struct Grid {
    grid_rect: Rect,
    current_square_rect: Rect,
    grid_dims: (u32, u32)
}

impl Grid {

    /*
     * create a new grid.
     * rect - the rect the grid is in.
     * grid_dims - the dimensions of the grid
     */
    pub fn new(rect: (i32, i32, u32, u32), grid_dims: (u32, u32)) -> Self {
        
        let rect = Rect::new(rect.0, rect.1, rect.2, rect.3);
        let square_rect_dims : (u32, u32) = (rect.width() / grid_dims.0, rect.height() / grid_dims.1);

        Self { grid_rect: rect, current_square_rect: Rect::new(0, 0, square_rect_dims.0, square_rect_dims.1), grid_dims: grid_dims}
    }

    fn calculate_position(&self, sq: (i32, i32)) -> (i32, i32){
        (
            self.grid_rect.x() + (self.current_square_rect.width() as i32 * sq.0),
            self.grid_rect.y() + (self.current_square_rect.height() as i32 * sq.1)
        )
    }

    pub fn fill_square(&mut self, canvas: &mut Canvas<Window>, texture_mng: &TexturesManager, sq: (i32, i32), piece_type: PieceType) -> Result<(), String> {
        let (x, y) = self.calculate_position(sq);

        self.current_square_rect.set_x(x);
        self.current_square_rect.set_y(y);

        canvas.copy(&texture_mng.pieces, Rect::new(piece_type.index() * 30, 0, 30, 30), self.current_square_rect)?;

        Ok(())
    }
    
    pub fn draw_borders(&mut self, canvas: &mut Canvas<Window>) -> Result<(), String> {

        canvas.set_draw_color(Color::GREY);

        // vertical
        for i in 0..=self.grid_dims.0{
            let line_x = self.grid_rect.x + (i * self.current_square_rect.width()) as i32;
            canvas.draw_line((line_x, self.grid_rect.top()), (line_x, self.grid_rect.bottom()))?;
        }

        // horizontal
        
        for i in 0..=self.grid_dims.1 {
            let line_y = self.grid_rect.y + (i * self.current_square_rect.height()) as i32;
            canvas.draw_line((self.grid_rect.left(), line_y), (self.grid_rect.right(), line_y))?;
        }
        Ok(())


    }
}
