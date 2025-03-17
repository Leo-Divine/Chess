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

#[derive(Default)]
pub struct Grid {
    rows: usize,
    cols: usize,
    items: Vec<Pieces>,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            rows: ROWS,
            cols: COLS,
            items: vec![Pieces::None; ROWS * COLS],
        }
    }

    fn add_starting_pieces(&mut self) {
        for x in 0..8 {
            self.items[(x, 1)] = Pieces::BlackPawn;
            self.items[(x, 6)] = Pieces::WhitePawn;
        }

        self.items[(0, 0)] = Pieces::BlackRook;
        self.items[(1, 0)] = Pieces::BlackKnight;
        self.items[(2, 0)] = Pieces::BlackBishop;
        self.items[(3, 0)] = Pieces::BlackKing;
        self.items[(4, 0)] = Pieces::BlackQueen;
        self.items[(5, 0)] = Pieces::BlackBishop;
        self.items[(6, 0)] = Pieces::BlackKnight;
        self.items[(7, 0)] = Pieces::BlackRook;

        self.items[(0, 7)] = Pieces::WhiteRook;
        self.items[(1, 7)] = Pieces::WhiteKnight;
        self.items[(2, 7)] = Pieces::WhiteBishop;
        self.items[(3, 7)] = Pieces::WhiteKing;
        self.items[(4, 7)] = Pieces::WhiteQueen;
        self.items[(5, 7)] = Pieces::WhiteBishop;
        self.items[(6, 7)] = Pieces::WhiteKnight;
        self.items[(7, 7)] = Pieces::WhiteRook;
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = Pieces;

    fn index(&self, (col, row): (usize, usize)) -> &Self::Output {
        &self.items[self.rows * row + col]
    }
}

impl IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, (col, row): (usize, usize)) -> &mut Self::Output {
        &mut self.items[self.rows * row + col]
    }
}

pub type Board = Arc<Mutex<Grid>>;
