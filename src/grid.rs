use std::ops::{Index, IndexMut};
use std::sync::{Arc, Mutex};

const ROWS: usize = 8;
const COLS: usize = 8;

#[derive(Debug, Default, Clone, PartialEq)]
pub enum Pieces {
    #[default]
    None,
    WhitePawn,
    WhiteKnight,
    WhiteBishop,
    WhiteRook,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackKnight,
    BlackBishop,
    BlackRook,
    BlackQueen,
    BlackKing,
}

pub struct Grid {
    items: Vec<Pieces>,
}

impl Default for Grid {
    fn default() -> Self {
        let mut items: Vec<Pieces> = vec![Pieces::None; ROWS * COLS];
        for x in 0..8 {
            items[Grid::get_index(x, 1)] = Pieces::BlackPawn;
            items[Grid::get_index(x, 6)] = Pieces::WhitePawn;
        }

        items[Grid::get_index(0, 0)] = Pieces::BlackRook;
        items[Grid::get_index(1, 0)] = Pieces::BlackKnight;
        items[Grid::get_index(2, 0)] = Pieces::BlackBishop;
        items[Grid::get_index(3, 0)] = Pieces::BlackKing;
        items[Grid::get_index(4, 0)] = Pieces::BlackQueen;
        items[Grid::get_index(5, 0)] = Pieces::BlackBishop;
        items[Grid::get_index(6, 0)] = Pieces::BlackKnight;
        items[Grid::get_index(7, 0)] = Pieces::BlackRook;

        items[Grid::get_index(0, 7)] = Pieces::WhiteRook;
        items[Grid::get_index(1, 7)] = Pieces::WhiteKnight;
        items[Grid::get_index(2, 7)] = Pieces::WhiteBishop;
        items[Grid::get_index(3, 7)] = Pieces::WhiteKing;
        items[Grid::get_index(4, 7)] = Pieces::WhiteQueen;
        items[Grid::get_index(5, 7)] = Pieces::WhiteBishop;
        items[Grid::get_index(6, 7)] = Pieces::WhiteKnight;
        items[Grid::get_index(7, 7)] = Pieces::WhiteRook;

        Self { items }
    }
}

impl Grid {
    pub fn new() -> Self {
        Self {
            items: Default::default(),
        }
    }

    fn get_index(col: usize, row: usize) -> usize {
        ROWS * row + col
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = Pieces;

    fn index(&self, (col, row): (usize, usize)) -> &Self::Output {
        &self.items[ROWS * row + col]
    }
}

impl IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, (col, row): (usize, usize)) -> &mut Self::Output {
        &mut self.items[ROWS * row + col]
    }
}

pub type Board = Arc<Mutex<Grid>>;
