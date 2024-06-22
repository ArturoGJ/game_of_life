use rand::{thread_rng, Rng};

use crate::{GRID_X_SIZE, GRID_Y_SIZE};

#[derive(Clone, Copy, Debug)]
pub enum CellState {
    Dead,
    Alive,
}

pub struct GameContext {
    pub board: [[CellState; GRID_X_SIZE as usize]; GRID_Y_SIZE as usize],
}

impl GameContext {
    pub fn new() -> GameContext {
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

    pub fn update(&mut self) {
        let mut new_board = GameContext::get_empty_board();

        for (i, row) in self.board.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                new_board[i][j] = *cell;
                let neighbors = GameContext::get_alive_neighbors_count(i, j, &self.board);
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

    fn get_alive_neighbors_count(
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
