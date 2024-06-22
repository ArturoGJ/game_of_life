pub mod game_context;
pub mod renderer;

use std::time::Duration;

use game_context::GameContext;
use renderer::Renderer;

extern crate rand;
extern crate sdl2;

const GRID_X_SIZE: i32 = 300;
const GRID_Y_SIZE: i32 = 200;
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

    let mut event_pump = sdl_context.event_pump()?;

    let mut game_context = GameContext::new();
    let mut renderer = Renderer::new(window)?;
    let mut frame_count = 0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        frame_count += 1;
        if frame_count % 5 == 0 {
            game_context.update();
            frame_count = 0;
        }

        renderer.draw(&game_context)?;

        // We wait until the next loop, in this case it means we wait one
        // thirtieth of a second for each frame, which means that we process
        // at most, 30 frames per second.
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}
