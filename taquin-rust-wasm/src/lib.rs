use wasm_bindgen::prelude::*;
use js_sys::Math::random;
use wasm_bindgen::__rt::std::collections::HashSet;

mod utils;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Taquin
type Tile = u8;

const HOLE: Tile = 0;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Hash, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Move {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl Move {
    fn all() -> Vec<Move> {
        vec![Move::Up, Move::Right, Move::Down, Move::Left]
    }

    fn apply(self, position: Position) -> Position {
        let Position { row, column } = position;
        match self {
            Move::Up => Position { row: row + 1, column },
            Move::Right => Position { row, column: column - 1 },
            Move::Down => Position { row: row - 1, column },
            Move::Left => Position { row, column: column + 1 },
        }
    }

    fn reverse(self) -> Move {
        match self {
            Move::Up => Move::Down,
            Move::Right => Move::Left,
            Move::Down => Move::Up,
            Move::Left => Move::Right,
        }
    }
}

#[wasm_bindgen]
pub struct Moves(Vec<Move>);

#[wasm_bindgen]
impl Moves {
    pub fn moves(&self) -> *const Move {
        self.0.as_ptr()
    }
    pub fn size(&self) -> usize {
        self.0.len()
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Position {
    pub row: u8,
    pub column: u8,
}

#[wasm_bindgen]
#[derive(Hash, Clone, Debug, PartialEq, Eq)]
pub struct Taquin {
    size: u8,
    tiles: Vec<Tile>,
}

type StateWithHistory = (Taquin, Vec<Move>);

#[wasm_bindgen]
impl Taquin {
    pub fn new(size: u8) -> Self {
        utils::set_panic_hook(); // FIXME how to enable only in Browser ?

        if size != 3 && size != 4 { panic!("Expected size of 3 or 4, got {}", size); }

        // Start with solution
        let last_index = (size * size - 1) as usize;
        let mut tiles: Vec<u8> = vec![];
        for i in 0..last_index {
            tiles.push((i + 1) as u8);
        }
        tiles.push(HOLE);
        Taquin { size, tiles }
    }

    pub fn size(&self) -> u8 {
        self.size
    }

    pub fn tiles(&self) -> *const Tile {
        self.tiles.as_ptr()
    }

    pub fn is_solved(&self) -> bool {
        let last_index = (self.size * self.size - 1) as usize;
        for (i, &tile) in self.tiles.iter().enumerate() {
            if i == last_index {
                return tile == HOLE;
            } else if tile != (i + 1) as u8 {
                return false;
            };
        }
        false
    }

    pub fn move_hole(&mut self, user_move: Move) -> bool {
        let hole_position = self.find_hole();

        if self.is_valid(user_move, hole_position) {
            let position = user_move.apply(hole_position);
            let hole_index = self.get_index(hole_position);
            let index = self.get_index(position);
            self.tiles.swap(index, hole_index);

            true
        } else {
            false
        }
    }

    pub fn move_from_position(&self, position: Position) -> Option<Move> {
        let hole_position = self.find_hole();
        Move::all().iter()
            .find(|&m| m.apply(hole_position) == position)
            .copied()
    }

    pub fn is_valid(&self, user_move: Move, hole_position: Position) -> bool {
        let Position { row, column } = hole_position;
        match user_move {
            Move::Up => row < self.size - 1,
            Move::Right => column > 0,
            Move::Down => row > 0,
            Move::Left => column < self.size - 1,
        }
    }

    pub fn get_index(&self, position: Position) -> usize {
        let Position { row, column } = position;
        (row * self.size + column) as usize
    }

    pub fn get_position(&self, index: usize) -> Position {
        let row = index as u8 / self.size;
        let column = index as u8 % self.size;
        Position { row, column }
    }

    pub fn find_hole(&self) -> Position {
        for (i, &tile) in self.tiles.iter().enumerate() {
            if tile == HOLE {
                return self.get_position(i);
            };
        }
        panic!("Hole not found in {:?}", self);
    }

    pub fn shuffle(&mut self, count: u32) {
        let mut hole_position = self.find_hole();
        let mut last_move = None::<Move>;

        for _i in 0..count {
            // Valid moves
            let valid_moves = self.valid_moves(hole_position, last_move);

            // Shuffle
            let index = (random() * valid_moves.len() as f64).floor() as usize;
            let current_move = valid_moves[index];

            // update
            hole_position = self.find_hole();
            last_move = Some(current_move);
            self.move_hole(current_move);
        }
    }

    pub fn solve(&self) -> Moves {
        let mut states = HashSet::new();
        states.insert(self.clone());
        let initial = vec![(self.clone(), vec![])];
        let moves = Taquin::solve_aux(initial, &mut states);
        Moves(moves)
    }

    fn valid_moves(&self, hole_position: Position, last_move: Option<Move>) -> Vec<Move> {
        let mut valid_moves = vec![];
        for m in Move::all() {
            let not_back = last_move.map_or(true, |last| last != m.reverse());
            if not_back && self.is_valid(m, hole_position) {
                valid_moves.push(m);
            }
        }
        valid_moves
    }

    // Solving aux
    fn solve_aux(states_with_history: Vec<StateWithHistory>, visited_states: &mut HashSet<Taquin>) -> Vec<Move> {
        let mut next = vec![];
        println!("Visited: {}", visited_states.len());
        for (taquin, history) in states_with_history {
            // Try found solution
            if taquin.is_solved() {
                return history;
            }

            // Find next states
            let hole = taquin.find_hole();
            let last_move = history.last().map(|m| m.reverse());
            let available_moves = taquin.valid_moves(hole, last_move);
            for m in available_moves {
                // Apply move
                let mut new_taquin = taquin.clone();
                new_taquin.move_hole(m);

                if !visited_states.contains(&new_taquin) {
                    // Found a new state
                    visited_states.insert(new_taquin.clone());
                    let mut next_history = history.clone();
                    next_history.push(m);
                    next.push((new_taquin, next_history));
                }
            }
        }

        // Deeper
        Taquin::solve_aux(next, visited_states)
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

        assert_eq!(0, taquin.get_index(Position { row: 0, column: 0 }));
        assert_eq!(1, taquin.get_index(Position { row: 0, column: 1 }));
        assert_eq!(2, taquin.get_index(Position { row: 0, column: 2 }));
        assert_eq!(3, taquin.get_index(Position { row: 1, column: 0 }));
        assert_eq!(4, taquin.get_index(Position { row: 1, column: 1 }));
        assert_eq!(5, taquin.get_index(Position { row: 1, column: 2 }));
        assert_eq!(6, taquin.get_index(Position { row: 2, column: 0 }));
        assert_eq!(7, taquin.get_index(Position { row: 2, column: 1 }));
        assert_eq!(8, taquin.get_index(Position { row: 2, column: 2 }));
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
        assert!(taquin.is_solved());

        let taquin = Taquin {
            size: 3,
            tiles: vec![
                5, 0, 3,
                8, 1, 2,
                4, 7, 6,
            ],
        };
        assert!(!taquin.is_solved());
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
    fn solve_3x3() {
        let taquin = Taquin {
            size: 3,
            tiles: vec![
                5, 0, 3,
                8, 1, 2,
                4, 7, 6,
            ],
        };

        let result = taquin.solve();

        assert_eq!(31, result.size());
    }

    // #[test]
    // fn solve_4x4() {
    //     let taquin = Taquin {
    //         size: 4,
    //         tiles: vec![
    //             15, 13, 1, 14,
    //             11, 0, 6, 7,
    //             10, 3, 4, 2,
    //             9, 8, 4, 12,
    //         ],
    //     };
    //
    //     let result = taquin.solve();
    //
    //     assert_eq!(31, result.size());
    // }
}