use grid::Grid;
use std::sync::{Arc, Mutex};

#[derive(Debug, Default, Clone)]
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
pub struct GameBoard {
    pub grid: Arc<Mutex<Grid<Pieces>>>,
}

impl GameBoard {
    pub fn add_starting_pieces() -> Grid<Pieces> {
        let mut board: Grid<Pieces> = Grid::new(8, 8);

        for x in 0..8 {
            board[(x, 1)] = Pieces::BlackPawn;
            board[(x, 6)] = Pieces::WhitePawn;
        }

        board[(0, 0)] = Pieces::BlackRook;
        board[(1, 0)] = Pieces::BlackKnight;
        board[(2, 0)] = Pieces::BlackBishop;
        board[(3, 0)] = Pieces::BlackKing;
        board[(4, 0)] = Pieces::BlackQueen;
        board[(5, 0)] = Pieces::BlackBishop;
        board[(6, 0)] = Pieces::BlackKnight;
        board[(7, 0)] = Pieces::BlackRook;

        board[(0, 6)] = Pieces::WhiteRook;
        board[(1, 6)] = Pieces::WhiteKnight;
        board[(2, 6)] = Pieces::WhiteBishop;
        board[(3, 6)] = Pieces::WhiteKing;
        board[(4, 6)] = Pieces::WhiteQueen;
        board[(5, 6)] = Pieces::WhiteBishop;
        board[(6, 6)] = Pieces::WhiteKnight;
        board[(7, 6)] = Pieces::WhiteRook;

        board
    }

    pub fn get_piece_at(&self, x: i32, y: i32) -> Pieces {
        let grid = self.grid.lock().unwrap();
        grid.get(x, y).unwrap().clone()
    }

    pub fn test_piece_move(&mut self) {
        self.grid.lock().unwrap()[(0, 3)] = Pieces::BlackRook;
    }
}
