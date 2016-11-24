use reversi::*;

pub trait AI {
    fn consider(&mut self, board: &mut Board) -> Command;
}
