use std::time::Duration;

use sdl2::{pixels::Color, rect::Rect};

extern crate sdl2;

const GRID_X_SIZE: i32 = 20;
const GRID_Y_SIZE: i32 = 20;
const DOT_SIZE_IN_PXS: i32 = 20;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Game of life", (GRID_X_SIZE * DOT_SIZE_IN_PXS) as u32, (GRID_Y_SIZE * DOT_SIZE_IN_PXS) as u32)
        .position_centered()
        .opengl()
        .build()
        .map_err(|err| err.to_string())?;

    let mut canvas = window
        .into_canvas()
        .build()
        .map_err(|err| err.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    let game_context = GameContext::new();
    // let mut frame_count = 0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        canvas.set_draw_color(Color::WHITE);
        canvas.clear();

        for row in game_context.board.iter() {
            for cell in row.iter() {
                if let CellState::Alive = cell.state {
                    canvas.set_draw_color(Color::BLACK);
                    canvas.draw_rect(Rect::new(
                        cell.position.0 * DOT_SIZE_IN_PXS, 
                        cell.position.1 * DOT_SIZE_IN_PXS, 
                        DOT_SIZE_IN_PXS as u32, 
                        DOT_SIZE_IN_PXS as u32)
                    )?;
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

#[derive(Clone, Copy)]
enum CellState {
    Dead,
    Alive,
}

#[derive(Clone, Copy)]
struct Cell {
    state: CellState,
    position: (i32, i32),
}

impl Cell {
    fn new(state: CellState, position: (i32, i32)) -> Cell {
        Cell {
            state,
            position
        }
    }
}

struct GameContext {
    pub board: [[Cell; GRID_X_SIZE as usize]; GRID_Y_SIZE as usize],
}

impl GameContext {
    fn new() -> GameContext {
        let mut board = [
            [Cell::new(CellState::Dead, (0, 0)); GRID_X_SIZE as usize]; 
            GRID_Y_SIZE as usize
        ];
        for i in 0..GRID_X_SIZE as usize {
            for j in 0..GRID_Y_SIZE as usize {
                if (i + j) % 7 == 0 {
                    let cell = Cell::new(CellState::Alive, (i as i32, j as i32));
                    board[i][j] = cell;
                }
            }
        }

        GameContext {
            board
        }
    }
}
