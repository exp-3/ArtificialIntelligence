extern crate rand;
use std::io;

mod reversi;
mod ai;
mod uct_ai;
mod pmc_ai;
use reversi::*;
use ai::*;
use uct_ai::*;
use pmc_ai::*;

fn main() {
    let mut board = Board::new();

    let mut uct_ai = UCTAI::new(1.4, 1000, 700);
    let mut pmc_ai = PMCAI::new(1000);

    let mut blak_win_num = 0;
    let mut white_win_num = 0;
    let mut draw_times = 0;

    board.init();
    // board.print();
    loop {
        // let mut input = String::new();
        // io::stdin().read_line(&mut input)
        //     .ok()
        //     .expect("failed to read input");
        // let input = input.trim();
        // board.input(input);
        if board.is_game_over() {break;}
        let command = uct_ai.consider(&mut board);
        board.input_by_command(&command);
        board.print();

        if board.is_game_over() {break;}
        let command = pmc_ai.consider(&mut board);
        board.input_by_command(&command);
        board.print();
    }
}
