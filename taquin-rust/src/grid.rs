use std::slice::Iter;
use crate::taquin::Move;

pub type Row = u8;
pub type Column = u8;
pub type Size = u8;


#[derive(Debug, PartialEq, Eq)]
pub struct Position {
    row: u8,
    column: u8,
}

impl Position {
    pub(crate) fn new(row: Row, column: Column) -> Self {
        Position { row, column }
    }

    pub fn row(&self) -> Row {
        self.row
    }

    pub fn column(&self) -> Column {
        self.column
    }
}


#[derive(Hash, Clone, Debug, PartialEq, Eq)]
pub struct Grid<T> where T: PartialEq + Clone {
    size: Size,
    content: Vec<T>,
}

impl<T> Grid<T> where T: PartialEq + Clone {
    pub fn new(size: Size, content: Vec<T>) -> Self {
        if content.len() != (size * size) as usize {
            panic!("Content should have a size of {}, got {}", (size * size), content.len())
        }

        Grid { size, content }
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.content.iter()
    }

    pub fn index_to_position(&self, index: usize) -> Option<Position> {
        let row = index as Size / self.size;
        let column = index as Size % self.size;

        if row < self.size {
            Some(Position::new(row, column))
        } else {
            None
        }
    }

    fn position_to_index(&self, position: &Position) -> Option<usize> {
        let row = position.row();
        let column = position.column();
        if row < self.size && column < self.size {
            Some((row * self.size + column) as usize)
        } else {
            None
        }
    }

    pub fn get(&self, position: &Position) -> Option<&T> {
        self.position_to_index(position)
            .and_then(|index| self.content.get(index))
    }

    pub fn find<P>(&self, predicate: P) -> Option<Position>
        where P: Fn(&T) -> bool {
        self.content.iter()
            .enumerate()
            .find(|(_i, v)| predicate(v))
            .and_then(|(i, _v)| self.index_to_position(i))
    }

    pub fn swap(&mut self, position1: &Position, position2: &Position) {
        if let Some(index1) = self.position_to_index(position1) {
            if let Some(index2) = self.position_to_index(position2) {
                self.content.swap(index1, index2)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod given {
        use super::*;

        pub(crate) fn a_grid() -> Grid<i32> {
            Grid {
                size: 2,
                content: vec![1, 2, 3, 4],
            }
        }
    }

    mod index_to_position {
        use super::*;

        #[test]
        fn ok() {
            let pos = given::a_grid().index_to_position(2);

            assert_eq!(pos, Some(Position::new(1, 0)))
        }

        #[test]
        fn out_of_bound() {
            let pos = given::a_grid().index_to_position(4);

            assert_eq!(pos, None)
        }
    }

    mod position_to_index {
        use super::*;

        #[test]
        fn ok() {
            let pos = Position::new(1, 0);
            let index = given::a_grid().position_to_index(&pos);

            assert_eq!(index, Some(2))
        }

        #[test]
        fn out_of_bound() {
            let pos = Position::new(2, 0);
            let index = given::a_grid().position_to_index(&pos);

            assert_eq!(index, None)
        }
    }

    mod find {
        use super::*;

        #[test]
        fn find_success() {
            let found = given::a_grid().find(|&i| i == 2);

            assert_eq!(found, Some(Position::new(0, 1)))
        }

        #[test]
        fn find_failure() {
            let found = given::a_grid().find(|&i| i == 5);

            assert_eq!(found, None)
        }
    }

    mod swap {
        use super::*;

        #[test]
        fn swap_iso() {
            let mut grid = given::a_grid();
            let pos = Position::new(0, 0);

            grid.swap(&pos, &pos);

            assert_eq!(grid, given::a_grid())
        }

        #[test]
        fn swap() {
            let mut grid = given::a_grid();
            let pos1 = Position::new(0, 0);
            let pos2 = Position::new(1, 1);

            grid.swap(&pos1, &pos2);

            let expected = vec![4, 2, 3, 1];
            assert_eq!(grid.content, expected)
        }
    }
}

