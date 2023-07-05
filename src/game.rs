use sdl2::{
    video::Window, render::Canvas, Sdl, EventPump, event::Event, rect::Rect
};

use crate::{constants, grid_draw::Grid, textures_manager::TexturesManager, piece_type::PieceType};

pub struct TetrisGame {
    sdl_context: Sdl,
    canvas: Canvas<Window>,
    grid_drawer: Grid,
    running: bool,
    texture_manager: TexturesManager,
    grid: [[Option<PieceType>; 10] ; 20],
    current_piece: (PieceType, (u32, u32))
}


impl TetrisGame {
    
    pub fn new() -> Result<Self, String> {
        let ctx = sdl2::init()?;
        let video_ctx = ctx.video()?;

        let win = video_ctx.window(constants::WINDOW_TITLE, constants::WINDOW_SIZE.0, constants::WINDOW_SIZE.1).position_centered().build().unwrap();
        let canvas = win.into_canvas().build().map_err(|_| "canvas creation failed")?;

        // TODO should be random
        let first_piece = PieceType::T;

        Ok( TetrisGame{ 
            sdl_context: ctx,
            grid_drawer: Grid::new(constants::GRID_RECT, constants::GRID_DIMENSIONS),
            running: false,
            texture_manager: TexturesManager::new(&canvas.texture_creator())?,
            canvas: canvas,
            grid: Default::default(),
            current_piece: (first_piece, (18, 4))
        })
    }

    /* handle events */
    fn event_loop(&mut self, pump: &mut EventPump) {
        for event in pump.poll_iter() {
            match event {
                Event::Quit{..} => {
                    self.running = false;
                }
                _ => {}
            }
        }
    }

    // finds whole rows in the board and fixes it.
    fn board_fix(&mut self) {
        for i in 0..20 {

            if Self::is_full_row(&self.grid[i]) {
                for j in (i+1)..20 {
                    self.grid[j - 1] = self.grid[j];
                }
            }

        }
    }

    fn is_full_row(row: &[Option<PieceType> ; 10]) -> bool {
        row.iter().all(|v| v.is_some() )
    }

    /* update stuff in the game, called every tick */
    fn update(&self) -> Result<(), String> {
        Ok(())
    }

    /* draw the screen */
    fn draw(&mut self) -> Result<(), String> {

        // draw background texture
        self.canvas.copy(&self.texture_manager.background, None, None)?;

        // draw pieces
        let t = PieceType::O;
        self.grid_drawer.fill_square(&mut self.canvas, &self.texture_manager, (0, 1), t)?;

        self.grid_drawer.draw_borders(&mut self.canvas)?;

        Ok(())
    }

    pub fn mainloop(&mut self) -> Result<(), String> {

        self.running = true;
        let mut event_pump = self.sdl_context.event_pump()?;

        let mut timer = self.sdl_context.timer()?;
        let target_delay_ms = 1000u32 / constants::FPS_LIMIT;
        let mut last_frame_time = timer.ticks();

        while self.running {
            self.event_loop(&mut event_pump);

            self.update()?;

            self.draw()?;
            self.canvas.present();

            // limit frame rate
            let elapsed = timer.ticks() - last_frame_time;
            if elapsed < target_delay_ms {
                timer.delay(target_delay_ms - elapsed);
            }
            last_frame_time = timer.ticks();
        }

        Ok(())
    }
}
