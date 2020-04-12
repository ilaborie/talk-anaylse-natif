use std::fmt::{Display, Formatter, Result};

use wasm_bindgen::prelude::*;

mod utils;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, taquin-rust-wasm!");
}


// xxx
type Tile = u8;

const HOLE: Tile = 0;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Move {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

#[wasm_bindgen]
#[derive(Debug, PartialEq, Eq)]
pub struct Taquin {
    size: u8,
    tiles: Vec<Tile>,
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Position {
    row: u8,
    column: u8,
}

#[wasm_bindgen]
impl Taquin {
    pub fn new(size: u8) -> Self {
        utils::set_panic_hook();
        log!("New taquin, size: {}", size);
        // FIXME start from solution, then randomly move (1.25 * size^4)
        let tiles = vec![
            5, 0, 3,
            8, 1, 2,
            4, 7, 6,
        ];
        Taquin { size, tiles }
    }

    pub fn is_solved(&self) -> bool {
        let last_index = (self.size * self.size - 1) as usize;
        for (i, &tile) in self.tiles.iter().enumerate() {
            if i == last_index {
                return tile == HOLE;
            } else if tile == (i + 1) as u8 {
                continue;
            } else {
                return false;
            };
        }
        true
    }

    pub fn move_hole(&mut self, user_move: Move) -> bool {
        log!("Move {:?}", user_move);
        let Position { row, column } = self.find_hole();

        // Check is valid
        if !self.is_valid(user_move, Position { row, column }) {
            log!("Move not allowed");
            false
        } else {
            let hole_index = self.get_index(row, column);
            let index = match user_move {
                Move::Up => self.get_index(row + 1, column),
                Move::Right => self.get_index(row, column - 1),
                Move::Down => self.get_index(row - 1, column),
                Move::Left => self.get_index(row, column + 1),
            };

            log!("Move hole {} to {}", hole_index, index);
            self.tiles.swap(index, hole_index);
            true
        }
    }

    fn is_valid(&self, user_move: Move, hole_position: Position) -> bool {
        let Position { row, column } = hole_position;
        match user_move {
            Move::Up => row < self.size - 1,
            Move::Right => column > 0,
            Move::Down => row > 0,
            Move::Left => column < self.size - 1,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    fn find_hole(&self) -> Position {
        for (i, &tile) in self.tiles.iter().enumerate() {
            if tile == HOLE {
                return self.get_position(i);
            };
        }
        panic!("Hole not found in {:?}", self);
    }

    fn get_index(&self, row: u8, column: u8) -> usize {
        (row * self.size + column) as usize
    }

    fn get_position(&self, index: usize) -> Position {
        let row = index as u8 / self.size;
        let column = index as u8 % self.size;
        Position { row, column }
    }
}

impl Display for Taquin {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for (i, &v) in self.tiles.iter().enumerate() {
            let Position { row, column } = self.get_position(i);
            if column > 0 { write!(f, " "); }
            let c = if v == HOLE { "·".to_string() } else { format!("{}", v) };
            write!(f, "{}", c);
            if column == (self.size - 1) && row < (self.size - 1) { write!(f, "\n"); }
        };
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_position() {
        let taquin = Taquin::new(3);

        assert_eq!(Position { row: 0, column: 0 }, taquin.get_position(0));
        assert_eq!(Position { row: 0, column: 1 }, taquin.get_position(1));
        assert_eq!(Position { row: 0, column: 2 }, taquin.get_position(2));
        assert_eq!(Position { row: 1, column: 0 }, taquin.get_position(3));
        assert_eq!(Position { row: 1, column: 1 }, taquin.get_position(4));
        assert_eq!(Position { row: 1, column: 2 }, taquin.get_position(5));
        assert_eq!(Position { row: 2, column: 0 }, taquin.get_position(6));
        assert_eq!(Position { row: 2, column: 1 }, taquin.get_position(7));
        assert_eq!(Position { row: 2, column: 2 }, taquin.get_position(8));
    }

    #[test]
    fn get_index() {
        let taquin = Taquin::new(3);

        assert_eq!(0, taquin.get_index(0, 0));
        assert_eq!(1, taquin.get_index(0, 1));
        assert_eq!(2, taquin.get_index(0, 2));
        assert_eq!(3, taquin.get_index(1, 0));
        assert_eq!(4, taquin.get_index(1, 1));
        assert_eq!(5, taquin.get_index(1, 2));
        assert_eq!(6, taquin.get_index(2, 0));
        assert_eq!(7, taquin.get_index(2, 1));
        assert_eq!(8, taquin.get_index(2, 2));
    }

    #[test]
    fn find_hole() {
        let taquin = Taquin {
            size: 3,
            tiles: vec![
                5, 0, 3,
                8, 1, 2,
                4, 7, 6,
            ],
        };

        assert_eq!(Position { row: 0, column: 1 }, taquin.find_hole());
    }

    #[test]
    fn is_solved() {
        let taquin = Taquin::new(3);
        assert!(!taquin.is_solved());

        let taquin = Taquin {
            size: 3,
            tiles: vec![
                1, 2, 3,
                4, 5, 6,
                7, 8, 0,
            ],
        };
        assert!(taquin.is_solved());
    }

    #[test]
    fn is_valid() {
        let taquin = Taquin::new(3);

        let pos = Position { row: 0, column: 0 };
        assert!(taquin.is_valid(Move::Up, pos));
        assert!(!taquin.is_valid(Move::Right, pos));
        assert!(!taquin.is_valid(Move::Down, pos));
        assert!(taquin.is_valid(Move::Left, pos));

        let pos = Position { row: 0, column: 2 };
        assert!(taquin.is_valid(Move::Up, pos));
        assert!(taquin.is_valid(Move::Right, pos));
        assert!(!taquin.is_valid(Move::Down, pos));
        assert!(!taquin.is_valid(Move::Left, pos));

        let pos = Position { row: 2, column: 2 };
        assert!(!taquin.is_valid(Move::Up, pos));
        assert!(taquin.is_valid(Move::Right, pos));
        assert!(taquin.is_valid(Move::Down, pos));
        assert!(!taquin.is_valid(Move::Left, pos));

        let pos = Position { row: 2, column: 0 };
        assert!(!taquin.is_valid(Move::Up, pos));
        assert!(!taquin.is_valid(Move::Right, pos));
        assert!(taquin.is_valid(Move::Down, pos));
        assert!(taquin.is_valid(Move::Left, pos));

        let pos = Position { row: 1, column: 1 };
        assert!(taquin.is_valid(Move::Up, pos));
        assert!(taquin.is_valid(Move::Right, pos));
        assert!(taquin.is_valid(Move::Down, pos));
        assert!(taquin.is_valid(Move::Left, pos));
    }

    #[test]
    fn move_hole() {
        let mut taquin = Taquin {
            size: 3,
            tiles: vec![
                5, 0, 3,
                8, 1, 2,
                4, 7, 6,
            ],
        };

        taquin.move_hole(Move::Up);
        assert_eq!(vec![
            5, 1, 3,
            8, 0, 2,
            4, 7, 6,
        ], taquin.tiles);

        taquin.move_hole(Move::Right);
        assert_eq!(vec![
            5, 1, 3,
            0, 8, 2,
            4, 7, 6,
        ], taquin.tiles);

        taquin.move_hole(Move::Down);
        assert_eq!(vec![
            0, 1, 3,
            5, 8, 2,
            4, 7, 6,
        ], taquin.tiles);

        taquin.move_hole(Move::Left);
        assert_eq!(vec![
            1, 0, 3,
            5, 8, 2,
            4, 7, 6,
        ], taquin.tiles);
    }

    #[test]
    fn display() {
        let taquin = Taquin {
            size: 3,
            tiles: vec![
                5, 0, 3,
                8, 1, 2,
                4, 7, 6,
            ],
        };
        let s = format!("{}", taquin);
        assert_eq!("5 · 3\n8 1 2\n4 7 6", s);
    }
}