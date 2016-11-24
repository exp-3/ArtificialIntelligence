// This is reversi AI with UCT algorythm
use rand;
use rand::Rng;
use ai::*;
use reversi::*;

enum Judge {
    Color(Color),
    Even
}

struct Node {
    ucb: f64,
    playout_num: u64,
    win_num: f64,
    childs: Vec<(Command, Node)>
}

impl Board {
    fn playout(&mut self) -> Judge {
        if self.is_game_over() {
            let black = self.count_disc(Attr::Color(Color::Black));
            let white = self.count_disc(Attr::Color(Color::White));
            if black == white { Judge::Even }
            else if black > white { Judge::Color(Color::Black) }
            else { Judge::Color(Color::White) }
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

impl Node {
    fn new() -> Node {
        let node = Node{ucb: 0.0, playout_num: 0,
                        win_num: 0.0, childs: vec![]};
        node
    }

    fn extract(board: &mut Board, uct_ai: &UCTAI) -> Vec<(Command, Node)> {
        let mut vec = Vec::new();
        if !board.is_game_over() {
            {
                let movable = board.get_movable_pos();
                if movable.is_empty() {
                    let node = Node::new();
                    vec.push((Command::Pass, node));
                } else {
                    for m in movable {
                        let command = Command::Move(*m);
                        let node = Node::new();
                        vec.push((command, node));
                    }
                }
            }

            for element in &mut vec {
                let _ = match element.0 {
                    Command::Pass => board.pass(),
                    Command::Move(ref p) => board.put(p)
                };
                element.1.playout(board, uct_ai);
                board.undo();
            }
        }

        vec
    }

    fn playout(&mut self, board: &mut Board, uct_ai: &UCTAI) {
        if self.childs.is_empty() && self.playout_num > uct_ai.extract_threshold && !board.is_game_over() {
            self.childs.append(&mut Node::extract(board, uct_ai));
        }
        if !self.childs.is_empty() {
            self.childs.sort_by(|a, b| a.1.ucb.partial_cmp(&b.1.ucb).unwrap());
            let next_move = self.childs.last_mut().unwrap();

            match next_move.0 {
                Command::Move(ref p) => board.put(p),
                Command::Pass => board.pass()
            };

            next_move.1.playout(board, uct_ai);
            board.undo();
        } else {
            let result = board.playout();
            self.win_num += match result {
                Judge::Even => 0.5,
                Judge::Color(c) if c == *board.get_current_color() => 1.0,
                _ => 0.0
            };
            self.playout_num += 1;
        }
    }

    fn update_ucb(&mut self, uct_ai: &UCTAI) {
        if !self.childs.is_empty() {
            self.playout_num = 0;
            self.win_num = 0.0;
            for &mut (_, ref mut child_node) in &mut self.childs {
                child_node.update_ucb(uct_ai);
                self.playout_num += child_node.playout_num;
                self.win_num += child_node.playout_num as f64 - child_node.win_num;
            }
        }

        self.ucb = self.win_num / self.playout_num as f64
                    + uct_ai.c * (2.0 * (uct_ai.total_playout_num as f64).ln() / self.playout_num as f64).sqrt();
    }
}

pub struct UCTAI {
    c: f64,
    total_playout_num: u64,
    trial_num: u64,
    extract_threshold: u64
}

impl UCTAI {
    pub fn new(c: f64, trial_num: u64, extract_threshold: u64) -> Self {
        let uct_ai = UCTAI {c: c, total_playout_num: 0,
                            trial_num: trial_num, extract_threshold: extract_threshold};
        uct_ai
    }
}

impl AI for UCTAI {
    fn consider(&mut self, board: &mut Board) -> Command {
        let mut root = Node::new();

        root.childs.append(&mut Node::extract(board, self));
        for i in 0 .. self.trial_num {
            let _ = i;
            root.playout(board, self);
            self.total_playout_num += 1;
            root.update_ucb(self);
        }

        let next_move = root.childs.iter()
            .fold(root.childs.first().unwrap(),
                  |max_element, i| if max_element.1.ucb < i.1.ucb {
                      i
                  } else {max_element});

        next_move.0
    }
}
