use taquin_rust::taquin::*;
use taquin_rust::grid::Grid;


mod given {
    use super::*;

    pub fn a_taquin() -> Taquin {
        let grid = Grid::new(3, vec![
            5, 0, 3,
            8, 1, 2,
            4, 7, 6, ]);

        Taquin { grid }
    }
}

mod tile {
    use super::*;

    #[test]
    fn u8_to_tile() {
        let i = 3;
        let tile: Tile =  i
    }
}