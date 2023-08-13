use sdl2::{pixels::Color};

pub const WINDOW_SIZE: (u32, u32) = (800, 800);
pub const WINDOW_TITLE: &str = "rust-tetris";

pub const FPS_LIMIT: u32 = 30;

pub const GRID_RECT: (i32, i32, u32, u32) = (250, 100, 300, 600); // centered 300x600 sized grid
pub const GRID_DIMENSIONS: (u32, u32) = (10, 20); // standard tetris dimensions

pub const STARTING_PIECE_POSITION: (i32, i32) = (18,4);
