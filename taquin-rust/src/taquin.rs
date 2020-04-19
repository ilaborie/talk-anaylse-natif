use std::fmt::{Display, Formatter, Result};

use rand::prelude::*;

use crate::grid::{Grid, Position, Size};
use crate::taquin::Tile::{Hole, Value};
use crate::solver::Problem;


/// Tile
#[derive(Hash, Copy, Clone, Debug, PartialEq, Eq)]
pub enum Tile {
    Hole,
    Value(u8),
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Hole => write!(f, "·"),
            Value(i) => write!(f, "{}", i)
        }
    }
}

impl Into<Tile> for u8 {
    fn into(self) -> Tile {
        if self == 0 {
            Hole
        } else {
            Value(self)
        }
    }
}

/// Move
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Move {
    Up,
    Right,
    Down,
    Left,
}

impl Move {
    pub fn all() -> Vec<Move> {
        vec![Move::Up, Move::Right, Move::Down, Move::Left]
    }

    pub fn reverse(self) -> Move {
        match self {
            Move::Up => Move::Down,
            Move::Right => Move::Left,
            Move::Down => Move::Up,
            Move::Left => Move::Right,
        }
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let c = match self {
            Move::Up => "⬆️",
            Move::Right => "➡️",
            Move::Down => "⬇️",
            Move::Left => "⬅️",
        };
        write!(f, "{}", c)
    }
}

// Taquin

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
pub struct Taquin {
    grid: Grid<Tile>,
}

impl Taquin {
    pub fn new(size: Size) -> Self {
        if size < 2 { panic!("Size should be greater than 2"); }

        let last_index = (size * size - 1) as usize;
        let mut tiles = vec![];
        for i in 0..last_index {
            tiles.push(Value((i + 1) as u8));
        }
        tiles.push(Hole);
        let grid = Grid::new(size, tiles);

        Taquin { grid }
    }

    pub fn from_str(size: Size, str: &str) -> Self {
        if size < 2 { panic!("Size should be greater than 2"); }

        let tiles: Vec<Tile> = str.split(',')
            .map(str::trim)
            .map(|s| s.parse::<u8>().unwrap())
            .map(|s| s.into())
            .collect();
        let last_index = (size * size) as usize;
        if tiles.len() != last_index { panic!("") }

        let grid = Grid::new(size, tiles);

        Taquin { grid }
    }

    pub fn shuffle(&mut self, count: u32) {
        let mut hole_position = self.find_hole();
        let mut last_move = None::<Move>;
        let mut rng = thread_rng();

        for _i in 0..count {
            // Valid moves
            let valid_moves = self.valid_moves(last_move);

            // Shuffle
            let index = rng.gen_range(0, valid_moves.len() - 1);
            let current_move = valid_moves[index];

            // update
            let position = Taquin::apply_move(&hole_position, current_move);
            self.grid.swap(&position, &hole_position);

            hole_position = self.find_hole();
            last_move = Some(current_move);
        }
    }

    pub fn size(&self) -> Size {
        self.grid.size()
    }

    pub fn is_solved(&self) -> bool {
        let last_index = (self.size() * self.size() - 1) as usize;
        for (i, &tile) in self.grid.iter().enumerate() {
            if i == last_index {
                return tile == Hole;
            } else if tile != Value((i + 1) as u8) {
                return false;
            };
        }
        false
    }

    fn find_hole(&self) -> Position {
        self.grid.find(|&tile| tile == Hole)
            .expect("No Hole found !")
    }

    pub fn move_hole(&self, user_move: Move) -> Self {
        let hole_position = self.find_hole();

        if self.is_valid(user_move, &hole_position) {
            let position = Taquin::apply_move(&hole_position, user_move);
            let mut new_grid = self.grid.clone();
            new_grid.swap(&position, &hole_position);
            Taquin { grid: new_grid }
        } else {
            self.clone()
        }
    }

    fn apply_move(hole_position: &Position, cmd: Move) -> Position {
        let row = hole_position.row();
        let column = hole_position.column();

        match cmd {
            Move::Up => Position::new(row + 1, column),
            Move::Right => Position::new(row, column - 1),
            Move::Down => Position::new(row - 1, column),
            Move::Left => Position::new(row, column + 1),
        }
    }

    fn is_valid(&self, user_move: Move, hole_position: &Position) -> bool {
        let row = hole_position.row();
        let column = hole_position.column();
        match user_move {
            Move::Up => row < self.size() - 1,
            Move::Right => column > 0,
            Move::Down => row > 0,
            Move::Left => column < self.size() - 1,
        }
    }

    fn valid_moves(&self, last_move: Option<Move>) -> Vec<Move> {
        let hole_position = self.find_hole();

        let mut valid_moves = vec![];
        for m in Move::all() {
            let not_back = last_move.map_or(true, |last| last != m.reverse());
            if not_back && self.is_valid(m, &hole_position) {
                valid_moves.push(m);
            }
        }
        valid_moves
    }
}

impl Display for Taquin {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let size = self.size();
        let width = ((size * size) as f64).log10().floor() as usize + 1;

        for (i, value) in self.grid.iter().enumerate() {
            let pos = self.grid.index_to_position(i).unwrap();
            if pos.column() > 0 { write!(f, " ")?; }
            write!(f, "{:width$}", value, width = width)?;
            if pos.column() == (size - 1) && pos.row() < (size - 1) { write!(f, "\n")?; }
        }
        Ok(())
    }
}

// Solve
impl Problem<Move> for Taquin {
    fn is_solved(&self) -> bool {
        self.is_solved()
    }

    fn available_steps(&self) -> Vec<Move> {
        self.valid_moves(None)
    }

    fn next(&self, step: Move) -> Self {
        self.move_hole(step)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod given {
        use super::*;

        pub fn a_taquin() -> Taquin {
            let grid = Grid::new(3, vec![
                Value(5), Hole, Value(3),
                Value(8), Value(1), Value(2),
                Value(4), Value(7), Value(6), ]);

            Taquin { grid }
        }
    }

    mod tile {
        use super::*;

        #[test]
        fn u8_to_tile() {
            assert_eq!(3.into(), Value(3));
            assert_eq!(0.into(), Hole);
            assert_eq!(0.into(), Hole);
        }
    }

    mod moves {
        use super::*;

        #[test]
        fn all() {
            assert_eq!(Move::all().len(), 4);
        }

        #[test]
        fn reverse() {
            assert_eq!(Move::Up.reverse(), Move::Down);
            assert_eq!(Move::Right.reverse(), Move::Left);
            assert_eq!(Move::Down.reverse(), Move::Up);
            assert_eq!(Move::Left.reverse(), Move::Right);
        }
    }

    mod taquin {
        use super::*;

        mod from_str {
            use super::*;

            #[test]
            fn ok() {
                let taquin = Taquin::from_str(3, "5,0,3,  8,1,2,  4,7,6");
                assert_eq!(taquin, given::a_taquin());
            }

            #[test]
            #[should_panic]
            fn fail_size() {
                Taquin::from_str(1, "5,0,3,  8,1,2,  4,7,6");
            }

            #[test]
            #[should_panic]
            fn fail_content() {
                Taquin::from_str(3, "5,0,3,  8,1,2,  4,7");
            }

            #[test]
            #[should_panic]
            fn fail_no_hole() {
                Taquin::from_str(3, "5,10,3,  8,1,2,  4,7,6");
            }
        }

        mod is_solved {
            use super::*;
            use test::assert_test_result;

            #[test]
            fn solved() {
                let taquin = Taquin::new(3);
                assert!(taquin.is_solved())
            }

            #[test]
            fn unsolved() {
                let taquin = given::a_taquin();
                assert!(!taquin.is_solved())
            }
        }

        mod find_hole {
            use super::*;
            use core::num::dec2flt::parse::Sign::Positive;

            #[test]
            fn ok() {
                let taquin = given::a_taquin();
                let pos = taquin.find_hole();
                assert_eq!(pos, Position::new(0, 1));
            }
        }

        mod move_hole {
            use super::*;

            #[test]
            fn ok() {
                let taquin = given::a_taquin();

                taquin.move_hole(Move::Up);
                assert_eq!(taquin, Taquin::from_str(3, "5,1,3,  8,0,2,  4,7,6"));

                taquin.move_hole(Move::Right);
                assert_eq!(taquin, Taquin::from_str(3, "5,1,3,  0,8,2,  4,7,6"));

                taquin.move_hole(Move::Down);
                assert_eq!(taquin, Taquin::from_str(3, "0,1,3,  5,8,2,  4,7,6"));

                taquin.move_hole(Move::Left);
                assert_eq!(taquin, given::a_taquin());
            }
        }

        mod is_valid {
            use super::*;

            #[test]
            fn ok() {
                let taquin = given::a_taquin();

                let pos = Position::new(0, 0);
                assert!(taquin.is_valid(Move::Up, &pos));
                assert!(!taquin.is_valid(Move::Right, &pos));
                assert!(!taquin.is_valid(Move::Down, &pos));
                assert!(taquin.is_valid(Move::Left, &pos));

                let pos = Position::new(0, 2);
                assert!(taquin.is_valid(Move::Up, &pos));
                assert!(taquin.is_valid(Move::Right, &pos));
                assert!(!taquin.is_valid(Move::Down, &pos));
                assert!(!taquin.is_valid(Move::Left, &pos));

                let pos = Position::new(2, 2);
                assert!(!taquin.is_valid(Move::Up, &pos));
                assert!(taquin.is_valid(Move::Right, &pos));
                assert!(taquin.is_valid(Move::Down, &pos));
                assert!(!taquin.is_valid(Move::Left, &pos));

                let pos = Position::new(2, 0);
                assert!(!taquin.is_valid(Move::Up, &pos));
                assert!(!taquin.is_valid(Move::Right, &pos));
                assert!(taquin.is_valid(Move::Down, &pos));
                assert!(taquin.is_valid(Move::Left, &pos));

                let pos = Position::new(1, 1);
                assert!(taquin.is_valid(Move::Up, &pos));
                assert!(taquin.is_valid(Move::Right, &pos));
                assert!(taquin.is_valid(Move::Down, &pos));
                assert!(taquin.is_valid(Move::Left, &pos));
            }
        }

        mod valid_moves {
            use super::*;

            #[test]
            fn ok() {
                let taquin = given::a_taquin();
                let moves = taquin.valid_moves(None);
                assert_eq!(moves, vec![Move::Up, Move::Right, Move::Down])
            }

            #[test]
            fn ok_with_previous() {
                let taquin = given::a_taquin();
                let moves = taquin.valid_moves(Some(Move::Down));
                assert_eq!(moves, vec![Move::Right, Move::Down])
            }
        }
    }
}