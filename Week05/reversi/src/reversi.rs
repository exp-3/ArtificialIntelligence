use std::ops::Index;

const BOARD_SIZE: usize = 8;
const MAX_TURNS: usize = 60;

#[derive(Copy, Clone)]
pub struct Point {
    i: usize,
    j: usize
}

#[derive(PartialEq, Copy, Clone)]
pub enum Color {
    Black,
    White
}

#[derive(PartialEq, Clone, Copy)]
pub enum Attr {
    Empty,
    Wall,
    Color(Color)
}

#[derive(Copy, Clone)]
pub struct Disc {
    color: Color,
    point: Point
}

#[derive(Copy, Clone)]
pub enum Command {
    Pass,
    Move(Point)
}

pub enum Judge {
    Color(Color),
    Even
}

#[derive(PartialEq, Copy, Clone)]
enum Directions {
    Upper,
    UpperLeft,
    Left,
    LowerLeft,
    Lower,
    LowerRight,
    Right,
    UpperRight
}

pub struct Board {
    raw_board: Vec<Vec<Attr>>,
    turns: usize,
    current_color: Color,

    update_log: Vec<Vec<Disc>>,
    movable_pos: Vec<Vec<Point>>,
    movable_dir: Vec<Vec<Vec<Option<Vec<Directions>>>>>,
    discs_black: u32,
    discs_white: u32,
    discs_empty: u32
}

impl Board {
    // public functions
    pub fn new() -> Board {
        let mut board = Board {raw_board: vec![vec![Attr::Empty; BOARD_SIZE + 2]; BOARD_SIZE + 2],
                               turns: 0, current_color: Color::Black,
                               update_log: vec![],
                               movable_pos: vec![vec![]; MAX_TURNS + 1],
                               movable_dir: vec![vec![vec![Some(vec![]); BOARD_SIZE + 2]; BOARD_SIZE + 2]; MAX_TURNS + 1],
                               discs_black: 2,
                               discs_white: 2,
                               discs_empty: (BOARD_SIZE * BOARD_SIZE - 4) as u32};
        board.init();
        board
    }

    pub fn init(&mut self) {
        for i in 1 .. BOARD_SIZE + 1 {
            for j in 1 .. BOARD_SIZE + 1 {
                self.raw_board[i][j] = Attr::Empty;
            }
        }
        for i in 0 .. BOARD_SIZE + 2 {
            self.raw_board[0][i] = Attr::Wall;
            self.raw_board[BOARD_SIZE + 1][i] = Attr::Wall;
            self.raw_board[i][0] = Attr::Wall;
            self.raw_board[i][BOARD_SIZE + 1] = Attr::Wall;
        }

        self.raw_board[4][4] = Attr::Color(Color::White);
        self.raw_board[4][5] = Attr::Color(Color::Black);
        self.raw_board[5][4] = Attr::Color(Color::Black);
        self.raw_board[5][5] = Attr::Color(Color::White);

        self.discs_black = 2;
        self.discs_white = 2;
        self.discs_empty = (BOARD_SIZE * BOARD_SIZE - 4) as u32;

        self.turns = 0;
        self.current_color = Color::Black;

        self.update_log.clear();

        self.update_movable();
    }

    pub fn put(&mut self, point: &Point) -> bool {
        if point.i < 1 || point.i > BOARD_SIZE || point.j < 1 || point.j > BOARD_SIZE {
            false
        } else {
            match self.movable_dir[self.turns][point.i][point.j] {
                None => false,
                Some(_) => {
                    self.flip_discs(point);

                    self.turns += 1;
                    self.current_color = match self.current_color {
                        Color::Black => Color::White,
                        Color::White => Color::Black
                    };
                    self.update_movable();
                    true
                }
            }
        }
    }

    pub fn pass(&mut self) -> bool {
        if !self.movable_pos[self.turns].is_empty() {false}
        else if self.is_game_over() {false}
        else {
            self.current_color = match self.current_color {
                Color::Black => Color::White,
                Color::White => Color::Black
            };
            self.update_log.push(Vec::new());
            self.update_movable();
            true
        }
    }

    pub fn input(&mut self, input_string: &str) {
        println!("入力: {}", input_string);
        let cs = input_string.bytes().collect::<Vec<u8>>();
        if cs[0] == 'p' as u8 {self.pass();}
        else if cs[0] == 'u' as u8 {self.undo();}
        else {
            let collumn = cs[0] - 'a' as u8 + 1;
            let row = cs[1] - '0' as u8;
            self.put(&Point{i: row as usize, j: collumn as usize});
        }
    }

    pub fn input_by_command(&mut self, input_command: &Command) {
        let _ = match *input_command {
            Command::Pass => self.pass(),
            Command::Move(ref p) => self.put(p)
        };
    }

    pub fn print(&self) {
        println!("-------------------------");
        println!("現在の手番: {}", match self.current_color {
            Color::Black => "● 黒番",
            Color::White => "○ 白番"
        });
        println!("黒: {}石\n白: {}石", self.discs_black, self.discs_white);
        println!("");
        println!("   a b c d e f g h");
        for i in 1 .. BOARD_SIZE + 1 {
            print!("{} ", i);
            for j in 1 .. BOARD_SIZE + 1 {
                print!("{} ", match self.raw_board[i][j] {
                    Attr::Empty => if let Some(_) = self.movable_dir[self.turns][i][j] {
                        " "
                    } else {
                        "×"
                    },
                    Attr::Color(c) => match c {
                        Color::Black => "●",
                        Color::White => "○"
                    },
                    _ => " "
                })
            }
            print!("\n");
        }
        print!("\n");
    }

    pub fn undo(&mut self) -> bool {
        if self.turns == 0 {false}
        else {
            let opposite_color = self.current_color;
            self.current_color = match self.current_color {
                Color::Black => Color::White,
                Color::White => Color::Black
            };
            let update = self.update_log.pop().unwrap();

            if update.is_empty() {
                self.movable_pos[self.turns].clear();
                for i in 1 .. BOARD_SIZE + 1 {
                    for j in 1 .. BOARD_SIZE + 1 {
                        self.movable_dir[self.turns][i][j] = None;
                    }
                }
            } else {
                self.turns -= 1;

                self.raw_board[update[0].point.i][update[0].point.j] = Attr::Empty;
                if let Some((_, rest)) = update.split_first() {
                    for disc in rest {
                        self.raw_board[disc.point.i][disc.point.j] = Attr::Color(opposite_color);
                    }
                }

                let discdiff = update.len();
                match self.current_color {
                    Color::Black => {
                        self.discs_black -= discdiff as u32;
                        self.discs_white += (discdiff - 1) as u32;
                    }
                    Color::White => {
                        self.discs_white -= discdiff as u32;
                        self.discs_black += (discdiff - 1) as u32;
                    }
                }
                self.discs_empty += 1;
            }

            true
        }
    }

    pub fn is_game_over(&self) -> bool {
        if self.turns >= MAX_TURNS {true}
        else if !self.movable_pos[self.turns].is_empty() {false}
        else {
            let opposite_color = match self.current_color {
                Color::Black => Color::White,
                Color::White => Color::Black
            };
            let mut game_over_flag = true;
            'outer: for i in 1 .. BOARD_SIZE + 1 {
                for j in 1 .. BOARD_SIZE + 1 {
                    let disc = Disc{point: Point{i: i, j: j}, color: opposite_color};
                    if let Some(_) = self.check_mobility(&disc) {
                        game_over_flag = false;
                        break 'outer;
                    }
                }
            }
            game_over_flag
        }
    }

    pub fn get_judge(&self) -> Judge {
        if self.discs_black == self.discs_white { Judge::Even }
        else if self.discs_black > self.discs_white { Judge::Color(Color::Black) }
        else { Judge::Color(Color::White) }
    }

    pub fn get_color(&self, point: &Point) -> &Attr {
        &self.raw_board[point.i][point.j]
    }

    pub fn count_disc(&self, attr: Attr) -> u32 {
        match attr {
            Attr::Color(c) => match c {
                Color::Black => self.discs_black,
                Color::White => self.discs_white
            },
            Attr::Empty => self.discs_empty,
            _ => 0
        }
    }

    pub fn get_current_color(&self) -> &Color {
        &self.current_color
    }

    pub fn get_diff(&self) -> Option<&Vec<Disc>> {
        self.update_log.last()
    }

    pub fn get_movable_pos(&self) -> &Vec<Point> {
        &self.movable_pos[self.turns]
    }

    pub fn get_turns(&self) -> usize {
        self.turns
    }

    // private functions
    fn check_mobility(&self, disc: &Disc) -> Option<Vec<Directions>> {
        let mut dirs: Vec<Directions> = Vec::new();

        if self.raw_board[disc.point.i][disc.point.j] == Attr::Empty {
            let dir_vec = vec![Directions::Upper, Directions::UpperLeft,
                               Directions::Left, Directions::LowerLeft,
                               Directions::Lower, Directions::LowerRight,
                               Directions::Right, Directions::UpperRight];
            for dir in dir_vec {
                if let Some(points) = self.list_turnable_discs(disc, &dir) {
                    if points.len() > 0 {
                        dirs.push(dir);
                    }
                }
            }
        }

        if dirs.is_empty() {
            None
        } else {
            Some(dirs)
        }
    }

    fn list_turnable_discs(&self, disc: &Disc, dir: &Directions) -> Option<Vec<Point>> {
        self.recursive_search_discs(disc, dir)
    }

    fn recursive_search_discs(&self, disc: &Disc, dir: &Directions) -> Option<Vec<Point>> {
        let (di, dj) = match *dir {
            Directions::Upper => (-1, 0),
            Directions::UpperLeft => (-1, -1),
            Directions::Left => (0, -1),
            Directions::LowerLeft => (1, -1),
            Directions::Lower => (1, 0),
            Directions::LowerRight => (1, 1),
            Directions::Right => (0, 1),
            Directions::UpperRight => (-1, 1),
        };

        let opposite_color = match disc.color {
            Color::Black => Color::White,
            Color::White => Color::Black,
        };

        let i = (disc.point.i as isize + di) as usize;
        let j = (disc.point.j as isize + dj) as usize;
        let next_disc = Disc {point: Point {i: i, j: j}, .. *disc};

        match self.raw_board[i][j] {
            Attr::Color(ref c) if *c == disc.color => Some(Vec::new()),
            Attr::Color(ref c) if *c == opposite_color => {
                if let Some(mut points) = self.list_turnable_discs(&next_disc, dir) {
                    points.push(next_disc.point);
                    Some(points)
                } else {
                    None
                }
            },
            _ => None
        }
    }

    fn flip_discs(&mut self, point: &Point) {
        let mut update = Vec::new();

        self.raw_board[point.i][point.j] = Attr::Color(self.current_color);
        let disc = Disc{point: *point, color: self.current_color};
        update.push(disc);

        if let Some(ref dirs) = self.movable_dir[self.turns][point.i][point.j] {
            for dir in dirs {
                if let Some(points) = self.list_turnable_discs(&disc, dir) {
                    for point in points {
                        self.raw_board[point.i][point.j] = Attr::Color(self.current_color);
                        let disc = Disc{point: point, .. disc};
                        update.push(disc);
                    }
                }
            }
        }

        let discdiff = update.len();

        match self.current_color {
            Color::Black => {
                self.discs_black += discdiff as u32;
                self.discs_white -= (discdiff - 1) as u32;
            }
            Color::White => {
                self.discs_white += discdiff as u32;
                self.discs_black -= (discdiff - 1) as u32;
            }
        }
        self.discs_empty -= 1;

        self.update_log.push(update);
    }

    fn update_movable(&mut self) {
        self.movable_pos[self.turns].clear();
        for i in 1 .. BOARD_SIZE + 1 {
            for j in 1 .. BOARD_SIZE + 1 {
                let disc = Disc{point: Point{i: i, j: j}, color: self.current_color};
                let dirs = self.check_mobility(&disc);
                if let Some(_) = dirs {
                    self.movable_pos[self.turns].push(disc.point);
                }
                self.movable_dir[self.turns][i][j] = dirs;
            }
        }
    }
}
