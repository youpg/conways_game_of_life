use macroquad::prelude::*;
use miniquad::window::request_quit;

use ::rand::prelude::*;

#[derive(Clone, Copy)]
struct Cell {
    is_alive: bool,
    index: usize,
}

impl Cell {
    fn new(is_alive: bool, index: usize) -> Self {
        Cell {
            is_alive,
            index,
        }
    }

    fn make_alive(&mut self) {
        self.is_alive = true;
    }
    fn make_dead(&mut self) {
        self.is_alive = false;
    }
}

#[macroquad::main("Conways game of life")]
async fn main() {
    let mut rng = thread_rng();
    let (width, height) = (200, 200);

    let mut board: Vec<Cell> = Vec::new();
    for i in 0..width*height {
        board.push(Cell::new(rng.gen_bool(0.3), i));
    }

    let mut frame_cnt = 0;

    loop {
        if is_key_pressed(KeyCode::Escape) {
            request_quit();
        }

        frame_cnt += 1;
        if frame_cnt % 10 == 0 {
            update_board(&mut board, (width, height));
        }
        
        
        // Draw the board
        draw_board(&board, (width, height));

        next_frame().await;
    }


}


fn get_neighbors(cell: &Cell, board_size: (usize, usize)) -> Vec<usize> {
    let (width, height) = board_size;
    let row = cell.index / width;
    let col = cell.index % width;

    let mut neighbors = Vec::new();
    let directions: [(i32, i32); 8] = [
        (-1, -1), (-1, 0), (-1, 1),
        ( 0, -1),          ( 0, 1),
        ( 1, -1), ( 1, 0), ( 1, 1),
    ];

    for (dr, dc) in directions.iter() {
        let new_row = row as i32 + dr;
        let new_col = col as i32 + dc;

        if new_row >= 0 && new_row < height as i32 && new_col >= 0 && new_col < width as i32 {
            let new_index = (new_row as usize * width) + new_col as usize;
            neighbors.push(new_index);
        }
    }

    neighbors
}

fn count_live_neighbors(cell: &Cell, board: &Vec<Cell>, board_size: (usize, usize)) -> u8 {
    get_neighbors(cell, board_size)
        .iter()
        .filter(|&&index| board[index].is_alive)
        .count() as u8
}

fn update_board(board: &mut Vec<Cell>, board_size: (usize, usize)) {
    let old_board = board.clone();
    for cell in board.iter_mut() {
        let live_neighbors = count_live_neighbors(cell, &old_board, board_size);
        if cell.is_alive {
            if live_neighbors < 2 || live_neighbors > 3 {
                cell.make_dead();
            }
        } else {
            if live_neighbors == 3 {
                cell.make_alive();
            }
        }
    }
}
 
fn draw_board(board: &Vec<Cell>, board_size: (usize, usize)) {
    let (width, height) = board_size;
    let cell_width = screen_width() / width as f32;
    let cell_height = screen_height() / height as f32;

    for x in 0..width {
        for y in 0..height {
            let index = get_index_by_xy(x, y, board_size);
            draw_rectangle(
                x as f32 * cell_width,
                y as f32 * cell_height, 
                cell_width, 
                cell_height, 
                if board[index].is_alive { BLUE } else { RED }
            );
        }
    }
}

fn get_index_by_xy(x: usize, y: usize, board_size: (usize, usize)) -> usize {
    let (width, _) = board_size;

    y * width + x
}

fn test() {
    
}