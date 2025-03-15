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

  pub fn add_starting_pieces(&mut self) {
      for x in 0..8 {
          self.items[Self::get_index(x, 1)] = Pieces::BlackPawn;
          self.items[Self::get_index(x, 6)] = Pieces::WhitePawn;
      }

      self.items[Self::get_index(0, 0)] = Pieces::BlackRook;
      self.items[Self::get_index(1, 0)] = Pieces::BlackKnight;
      self.items[Self::get_index(2, 0)] = Pieces::BlackBishop;
      self.items[Self::get_index(3, 0)] = Pieces::BlackKing;
      self.items[Self::get_index(4, 0)] = Pieces::BlackQueen;
      self.items[Self::get_index(5, 0)] = Pieces::BlackBishop;
      self.items[Self::get_index(6, 0)] = Pieces::BlackKnight;
      self.items[Self::get_index(7, 0)] = Pieces::BlackRook;

      self.items[Self::get_index(0, 7)] = Pieces::WhiteRook;
      self.items[Self::get_index(1, 7)] = Pieces::WhiteKnight;
      self.items[Self::get_index(2, 7)] = Pieces::WhiteBishop;
      self.items[Self::get_index(3, 7)] = Pieces::WhiteKing;
      self.items[Self::get_index(4, 7)] = Pieces::WhiteQueen;
      self.items[Self::get_index(5, 7)] = Pieces::WhiteBishop;
      self.items[Self::get_index(6, 7)] = Pieces::WhiteKnight;
      self.items[Self::get_index(7, 7)] = Pieces::WhiteRook;
  }

  fn get_index(col: usize, row: usize) -> usize {
      ROWS * row + col
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