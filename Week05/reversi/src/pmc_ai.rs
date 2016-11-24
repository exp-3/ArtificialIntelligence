// This is reversi AI with primitive monte-carlo algorythm
use rand;
use rand::Rng;
use ai::*;
use reversi::*;

struct Node {
    playout_num: u64,
    win_num: f64
}

impl Node {
    fn new() -> Self {
        Node {playout_num: 0, win_num: 0.0}
    }
}

pub struct PMCAI {
    total_playout_num: u64,
    trial_num: u64
}

impl Board {
    fn playout(&mut self) -> Judge {
        if self.is_game_over() {
            self.get_judge()
        }
        else {
            if self.get_movable_pos().is_empty() { self.pass(); }
            else {
                let idx = rand::thread_rng().gen_range(0, self.get_movable_pos().len()) as usize;
                let p = self.get_movable_pos()[idx];
                self.put(&p);
            }
            let judge = self.playout();
            self.undo();
            judge
        }
    }
}

impl PMCAI {
    pub fn new(trial_num: u64) -> Self {
        PMCAI{total_playout_num: 0, trial_num: trial_num}
    }
}

impl AI for PMCAI {
    fn consider(&mut self, board: &mut Board) -> Command {
        if board.get_movable_pos().is_empty() {
            Command::Pass
        }
        else {
            let mut sets: Vec<(Point, Node)> = board.get_movable_pos()
                .iter().map(|m| (*m, Node::new())).collect();

            'outer: loop {
                for &mut (ref point, ref mut node) in &mut sets {
                    if self.total_playout_num >= self.trial_num { break 'outer; }
                    let current_color = *board.get_current_color();
                    board.put(point);
                    match board.playout() {
                        Judge::Even => node.win_num += 0.5,
                        Judge::Color(c) if c == current_color => node.win_num += 1.0,
                        _ => node.win_num += 0.0
                    };
                    node.playout_num += 1;
                    board.undo();

                    self.total_playout_num += 1;
                }
            }

            let next_move = sets.iter()
                .fold(sets.first().unwrap(),
                      |max_element, i| {
                          let max_element_rate = max_element.1.win_num / max_element.1.playout_num as f64;
                          let i_rate = i.1.win_num / i.1.playout_num as f64;
                          if max_element_rate < i_rate {
                              i
                          } else {max_element}
                      });

            Command::Move(next_move.0)
        }
    }
}
