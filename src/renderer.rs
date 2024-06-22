use sdl2::{pixels::Color, rect::Rect, render::WindowCanvas, video::Window};

use crate::{game_context::{CellState, GameContext}, DOT_SIZE_IN_PXS};

pub struct Renderer {
    canvas: WindowCanvas,
}

impl Renderer {
    pub fn new(window: Window) -> Result<Renderer, String> {
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        Ok(Renderer { canvas })
    }

    pub fn draw(&mut self, game_context: &GameContext) -> Result<(), String> {
        self.canvas.set_draw_color(Color::WHITE);
        self.canvas.clear();

        for (i, row) in game_context.board.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if let CellState::Alive = cell {
                    self.canvas.set_draw_color(Color::BLACK);
                    self.canvas.fill_rect(Rect::new(
                        j as i32 * DOT_SIZE_IN_PXS,
                        i as i32 * DOT_SIZE_IN_PXS,
                        DOT_SIZE_IN_PXS as u32,
                        DOT_SIZE_IN_PXS as u32,
                    ))?;
                }
            }
        }

        self.canvas.present();
        Ok(())
    }
}
