extern crate rand;
use std::io;

mod reversi;
mod ai;
mod uct_ai;
use reversi::*;
use ai::*;
use uct_ai::*;

fn main() {
    let mut board = Board::new();
    board.init();
    let mut uct_ai = UCTAI::new(5.0, 1000, 50);

    board.print();
    while !board.is_game_over() {
        // let mut input = String::new();
        // io::stdin().read_line(&mut input)
        //     .ok()
        //     .expect("failed to read input");
        // let input = input.trim();
        // board.input(input);
        let command = uct_ai.consider(&mut board);
        board.input_by_command(&command);
        board.print();
    }
}
