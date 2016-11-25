// This is reversi AI with UCT algorythm
use rand;
use rand::Rng;
use std::rc::Rc;
use std::rc::Weak;
use std::cell::{BorrowState, RefCell};
use ai::*;
use reversi::*;

impl Board {
    fn playout(&mut self) -> Judge {
        if self.is_game_over() {
            self.get_judge()
        }
        else {
            if self.get_movable_pos().is_empty() { self.pass(); }
            else {
                let idx = rand::thread_rng().gen_range(0, self.get_movable_pos().len()) as usize;
                // println!("{}", idx);
                let p = self.get_movable_pos()[idx];
                self.put(&p);
            }
            let judge = self.playout();
            self.undo();
            judge
        }
    }
}

struct Node {
    ucb: RefCell<f64>,
    playout_num: RefCell<u64>,
    win_num: RefCell<f64>,
    current_color: Color,
    myself: RefCell<Weak<Node>>,
    childs: RefCell<Vec<(Command, Rc<Node>)>>,
    parent: Option<Weak<Node>>
}

impl Node {
    fn new_rc(current_color: Color, parent: Option<Weak<Node>>) -> Rc<Node> {
        let mut node = Node{ucb: RefCell::new(0.0),
            playout_num: RefCell::new(0),
            win_num: RefCell::new(0.0),
            current_color: current_color,
            myself: RefCell::default(), childs: RefCell::new(vec![]), parent: parent};

        let rc = Rc::new(node);
        *rc.myself.borrow_mut() = Rc::downgrade(&rc);
        rc
    }

    fn extract(&self, board: &mut Board, uct_ai: &UCTAI) -> Vec<(Command, Rc<Node>)> {
        let mut vec = Vec::new();
        if !board.is_game_over() {
            if board.get_movable_pos().is_empty() {
                let node = Node::new_rc(board.get_current_color(), Some(self.myself.borrow().clone()));
                vec.push((Command::Pass, node));
            } else {
                for m in board.get_movable_pos() {
                    let command = Command::Move(*m);
                    let node = Node::new_rc(board.get_current_color(), Some(self.myself.borrow().clone()));
                    vec.push((command, node));
                }
            }
        }

        vec
    }

    fn back_propagation(&self, result: Judge) {
        *self.win_num.borrow_mut() += match result {
            Judge::Even => 0.5,
            Judge::Color(c) if c == self.current_color => 1.0,
            _ => 0.0
        };
        *self.playout_num.borrow_mut() += 1;

        if let Some(ref parent) = self.parent {
            parent.upgrade().unwrap().back_propagation(result);
        }
    }

    fn playout(&self, board: &mut Board, uct_ai: &UCTAI) {
        if self.childs.borrow().is_empty() {
            if *self.playout_num.borrow() < uct_ai.extract_threshold || board.is_game_over() {
                let result = board.playout();
                self.back_propagation(result);
            }
            else {
                {
                    let mut childs = self.extract(board, uct_ai);
                    self.childs.borrow_mut().append(&mut childs);
                }
                for child in self.childs.borrow().iter() {
                    match child.0 {
                        Command::Pass => board.pass(),
                        Command::Move(ref p) => board.put(p)
                    };
                    child.1.playout(board, uct_ai);
                    board.undo();
                }
            }
        }
        else {
            let ref childs = *self.childs.borrow();
            let next_move = childs.iter()
                .fold(childs.first().unwrap(),
                      |max_element, i| if *max_element.1.ucb.borrow() < *i.1.ucb.borrow() {
                          i
                      } else {max_element});

            match next_move.0 {
              Command::Move(ref p) => board.put(p),
              Command::Pass => board.pass()
            };

            next_move.1.playout(board, uct_ai);
            board.undo();
        }
    }

    fn update_ucb(&self, total_playout_num: u64, uct_ai: &UCTAI) {
        if !self.childs.borrow().is_empty() {
            for &(_, ref child_node) in self.childs.borrow().iter() {
                child_node.update_ucb(total_playout_num, uct_ai);
            }
        }

        *self.ucb.borrow_mut() = *self.win_num.borrow() / *self.playout_num.borrow() as f64
            + uct_ai.c * (2.0 * (total_playout_num as f64).ln()
            / *self.playout_num.borrow() as f64).sqrt();
    }
}

pub struct UCTAI {
    c: f64,
    trial_num: u64,
    extract_threshold: u64
}

impl UCTAI {
    pub fn new(c: f64, trial_num: u64, extract_threshold: u64) -> Self {
        let uct_ai = UCTAI {c: c, trial_num: trial_num, extract_threshold: extract_threshold};
        uct_ai
    }
}

impl AI for UCTAI {
    fn consider(&mut self, board: &mut Board) -> Command {
        let root = Node::new_rc(board.get_current_color().inverse(), None);

        root.childs.borrow_mut().append(&mut root.extract(board, self));
        for child in root.childs.borrow().iter() {
            match child.0 {
                Command::Pass => board.pass(),
                Command::Move(ref p) => board.put(p)
            };
            child.1.playout(board, self);
            board.undo();
        }

        while *root.playout_num.borrow() < self.trial_num {
            root.playout(board, self);
            root.update_ucb(*root.playout_num.borrow(), self);
        }
        let ref childs = *root.childs.borrow();
        let next_move = childs.iter()
            .fold(childs.first().unwrap(),
                  |max_element, i| {
                      if *max_element.1.ucb.borrow() < *i.1.ucb.borrow() { i }
                      else { max_element }
                  });
        next_move.0
    }
}
