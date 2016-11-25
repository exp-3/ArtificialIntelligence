#![feature(borrow_state)]

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

    // let mut uct_ai = UCTAI::new(1.4, 10, 10000);
    // let mut pmc_ai = PMCAI::new(10000);
    let mut ai1 = PMCAI::new(1000);
    let mut ai2 = UCTAI::new(0.1, 1000, 30);

    let mut blak_win_num = 0;
    let mut white_win_num = 0;
    let mut draw_times = 0;

    for i in 0.. 100 {
        board.init();
        // board.print();
        'inner: loop {
            // let mut input = String::new();
            // io::stdin().read_line(&mut input)
            //     .ok()
            //     .expect("failed to read input");
            // let input = input.trim();
            // board.input(input);
            if board.is_game_over() {break 'inner;}
            let command = ai1.consider(&mut board);
            board.input_by_command(&command);
            // board.print();

            if board.is_game_over() {break 'inner;}
            let command = ai2.consider(&mut board);
            board.input_by_command(&command);
            // board.print();
        }

        match board.get_judge() {
            Judge::Even => draw_times += 1,
            Judge::Color(c) if c == Color::Black => blak_win_num += 1,
            _ => white_win_num += 1
        };
        println!("{}th loop finished.", i + 1);
        println!("  black wins {} times.", blak_win_num);
        println!("  white wins {} times.", white_win_num);
        println!("  drawed {} times.", draw_times);
    }
}
