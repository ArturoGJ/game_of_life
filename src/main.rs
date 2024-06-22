use std::time::Duration;

use rand::{thread_rng, Rng};
use sdl2::{pixels::Color, rect::Rect};

extern crate rand;
extern crate sdl2;

const GRID_X_SIZE: i32 = 80;
const GRID_Y_SIZE: i32 = 80;
const DOT_SIZE_IN_PXS: i32 = 5;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window(
            "Game of life",
            (GRID_X_SIZE * DOT_SIZE_IN_PXS) as u32,
            (GRID_Y_SIZE * DOT_SIZE_IN_PXS) as u32,
        )
        .position_centered()
        .opengl()
        .build()
        .map_err(|err| err.to_string())?;

    let mut canvas = window
        .into_canvas()
        .build()
        .map_err(|err| err.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    let mut game_context = GameContext::new();
    let mut frame_count = 0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        // Update
        frame_count += 1;
        if frame_count % 10 == 0 {
            game_context.update();
        }

        // Draw
        canvas.set_draw_color(Color::WHITE);
        canvas.clear();

        for (i, row) in game_context.board.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if let CellState::Alive = cell {
                    canvas.set_draw_color(Color::BLACK);
                    canvas.fill_rect(Rect::new(
                        i as i32 * DOT_SIZE_IN_PXS,
                        j as i32 * DOT_SIZE_IN_PXS,
                        DOT_SIZE_IN_PXS as u32,
                        DOT_SIZE_IN_PXS as u32,
                    ))?;
                }
            }
        }

        canvas.present();

        // We wait until the next loop, in this case it means we wait one
        // thirtieth of a second for each frame, which means that we process
        // at most, 30 frames per second.
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}

#[derive(Clone, Copy, Debug)]
enum CellState {
    Dead,
    Alive,
}

struct GameContext {
    pub board: [[CellState; GRID_X_SIZE as usize]; GRID_Y_SIZE as usize],
}

impl GameContext {
    fn new() -> GameContext {
        let mut board = GameContext::get_empty_board();
        let mut rng = thread_rng();
        for i in 0..GRID_Y_SIZE as usize {
            for j in 0..GRID_X_SIZE as usize {
                if (i + j) % 3 == 0 {
                    let cell = if rng.gen_bool(0.5) {
                        CellState::Alive
                    } else {
                        CellState::Dead
                    };
                    board[i][j] = cell;
                }
            }
        }

        GameContext { board }
    }

    fn update(&mut self) {
        let mut new_board = GameContext::get_empty_board();

        for (i, row) in self.board.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                new_board[i][j] = *cell;
                let neighbors = GameContext::get_alive_neighbors(i, j, &self.board);
                match cell {
                    CellState::Dead => {
                        if neighbors == 3 {
                            new_board[i][j] = CellState::Alive;
                        }
                    }
                    CellState::Alive => {
                        if neighbors < 2 || neighbors > 3 {
                            new_board[i][j] = CellState::Dead;
                        }
                    }
                }
            }
        }

        self.board = new_board;
    }

    fn get_empty_board() -> [[CellState; GRID_X_SIZE as usize]; GRID_Y_SIZE as usize] {
        [[CellState::Dead; GRID_X_SIZE as usize]; GRID_Y_SIZE as usize]
    }

    fn get_alive_neighbors(
        i: usize,
        j: usize,
        board: &[[CellState; GRID_X_SIZE as usize]; GRID_Y_SIZE as usize],
    ) -> i32 {
        let mut neighbors = 0;
        let row_start = if i > 0 { i - 1 } else { i };
        let row_end = if i < board.len() - 1 { i + 2 } else { i };
        let col_start = if j > 0 { j - 1 } else { j };
        let col_end = if j < board[i].len() - 1 { j + 2 } else { j };
        let is_alive = if let CellState::Alive = board[i][j] {
            true
        } else {
            false
        };

        for i in row_start..row_end {
            for j in col_start..col_end {
                if let CellState::Alive = board[i][j] {
                    neighbors += 1;
                }
            }
        }

        if is_alive {
            neighbors -= 1;
        }

        neighbors
    }
}
