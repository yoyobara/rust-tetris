use sdl2::{
    video::Window, render::Canvas, Sdl, EventPump, event::Event, rect::Rect, pixels::Color
};
use crate::constants;

pub struct TetrisGame {
    sdl_context: Sdl,
    timer: sdl2::TimerSubsystem,
    canvas: Canvas<Window>,
    running: bool
}

impl TetrisGame {
    
    pub fn new() -> Result<Self, String> {
        let ctx = sdl2::init()?;
        let video_ctx = ctx.video()?;
        let win = video_ctx.window(constants::WINDOW_TITLE, constants::WINDOW_SIZE.0, constants::WINDOW_SIZE.1).position_centered().build().unwrap();
        let canvas = win.into_canvas().build().map_err(|_| "canvas creation failed")?;

        let timer_ctx = ctx.timer()?;

        Ok( TetrisGame{ 
            sdl_context: ctx,
            timer: timer_ctx,
            canvas: canvas,
            running: false
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

    /* update stuff in the game, called every tick */
    fn update(&self) -> Result<(), String> {
        Ok(())
    }

    /* draw the screen */
    fn draw(&mut self) -> Result<(), String> {

        self.canvas.set_draw_color(Color::BLUE);
        self.canvas.clear();

        self.canvas.set_draw_color(Color::YELLOW);
        self.canvas.fill_rect(Rect::new(100, 100, 600, 600))?;
        Ok(())
    }

    pub fn mainloop(&mut self) -> Result<(), String> {

        self.running = true;
        let mut event_pump = self.sdl_context.event_pump()?;

        let target_delay_ms = 1000u32 / constants::FPS_LIMIT;
        let mut last_frame_time = self.timer.ticks();

        while self.running {
            self.event_loop(&mut event_pump);

            self.update()?;

            self.draw()?;
            self.canvas.present();

            // limit frame rate
            let elapsed = self.timer.ticks() - last_frame_time;
            if elapsed < target_delay_ms {
                self.timer.delay(target_delay_ms - elapsed);
            }
            last_frame_time = self.timer.ticks();
        }

        Ok(())
    }
}
