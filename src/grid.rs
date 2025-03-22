use std::ops::{Index, IndexMut};
use std::sync::{Arc, Mutex};

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
    turn: bool, //true = white; false = black;
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            items: vec![
                Pieces::BlackRook,
                Pieces::BlackKnight,
                Pieces::BlackBishop,
                Pieces::BlackKing,
                Pieces::BlackQueen,
                Pieces::BlackBishop,
                Pieces::BlackKnight,
                Pieces::BlackRook,
                Pieces::BlackPawn,
                Pieces::BlackPawn,
                Pieces::BlackPawn,
                Pieces::BlackPawn,
                Pieces::BlackPawn,
                Pieces::BlackPawn,
                Pieces::BlackPawn,
                Pieces::BlackPawn,
                Pieces::None,
                Pieces::None,
                Pieces::None,
                Pieces::None,
                Pieces::None,
                Pieces::None,
                Pieces::None,
                Pieces::None,
                Pieces::None,
                Pieces::None,
                Pieces::None,
                Pieces::None,
                Pieces::None,
                Pieces::None,
                Pieces::None,
                Pieces::None,
                Pieces::None,
                Pieces::None,
                Pieces::None,
                Pieces::None,
                Pieces::None,
                Pieces::None,
                Pieces::None,
                Pieces::None,
                Pieces::None,
                Pieces::None,
                Pieces::None,
                Pieces::None,
                Pieces::None,
                Pieces::None,
                Pieces::None,
                Pieces::None,
                Pieces::WhitePawn,
                Pieces::WhitePawn,
                Pieces::WhitePawn,
                Pieces::WhitePawn,
                Pieces::WhitePawn,
                Pieces::WhitePawn,
                Pieces::WhitePawn,
                Pieces::WhitePawn,
                Pieces::WhiteRook,
                Pieces::WhiteKnight,
                Pieces::WhiteBishop,
                Pieces::WhiteKing,
                Pieces::WhiteQueen,
                Pieces::WhiteBishop,
                Pieces::WhiteKnight,
                Pieces::WhiteRook,
            ],
            turn: true,
        }
    }
}

impl Grid {
    pub fn move_piece(&mut self, old_position: (usize, usize), new_position: (usize, usize)) {
        let white_pieces: [Pieces; 6] = [
            Pieces::WhitePawn,
            Pieces::WhiteKnight,
            Pieces::WhiteBishop,
            Pieces::WhiteRook,
            Pieces::WhiteQueen,
            Pieces::WhiteKing,
        ];
        let black_pieces: [Pieces; 6] = [
            Pieces::BlackPawn,
            Pieces::BlackKnight,
            Pieces::BlackBishop,
            Pieces::BlackRook,
            Pieces::BlackQueen,
            Pieces::BlackKing,
        ];

        //Don't move nothing
        if self[old_position] == Pieces::None {
            return;
        }
        //Make sure a piece moved
        if old_position == new_position {
            return;
        }
        //It better be your turn
        if (self.turn && black_pieces.contains(&self[old_position]))
            || (!self.turn && white_pieces.contains(&self[old_position]))
        {
            return;
        }
        //Make sure the piece moved correctly
        if !self.is_move_valid(self[old_position].clone(), old_position, new_position) {
            return;
        }
        //Don't capture your own pieces
        if (self.turn && white_pieces.contains(&self[new_position]))
            || (!self.turn && black_pieces.contains(&self[new_position]))
        {
            return;
        }

        self[new_position] = self[old_position].clone();
        self[old_position] = Pieces::None;
        self.turn = !self.turn;
    }
    fn is_move_valid(
        &mut self,
        piece: Pieces,
        old_position: (usize, usize),
        new_position: (usize, usize),
    ) -> bool {
        let x_move: i8 = (new_position.0 as i8) - (old_position.0 as i8);
        let y_move: i8 = (new_position.1 as i8) - (old_position.1 as i8);
        let x_abs: i8 = x_move.abs();
        let y_abs: i8 = y_move.abs();

        //BLACK PAWN
        if (piece == Pieces::BlackPawn)
            && (y_move != 1 || x_move != 0)
            && (old_position.1 != 1 || y_move != 2)
        {
            return false;
        }
        //WHITE PAWN
        if (piece == Pieces::WhitePawn)
            && (y_move != -1 || x_move != 0)
            && (old_position.1 != 6 || y_move != -2)
        {
            return false;
        }
        //ROOK
        if (piece == Pieces::BlackRook || piece == Pieces::WhiteRook)
            && (old_position.0 != new_position.0 && old_position.1 != new_position.1)
        {
            return false;
        }
        //KNIGHT
        if (piece == Pieces::BlackKnight || piece == Pieces::WhiteKnight)
            && ((x_abs != 2 || y_abs != 1) && (x_abs != 1 || y_abs != 2))
        {
            return false;
        }
        //BISHOP
        if (piece == Pieces::BlackBishop || piece == Pieces::WhiteBishop)
            && ((x_move + y_move) != 0i8 && (x_move - y_move) != 0i8)
        {
            return false;
        }
        //QUEEN
        if (piece == Pieces::BlackQueen || piece == Pieces::WhiteQueen)
            && (((x_move + y_move) != 0i8 && (x_move - y_move) != 0i8)
                && (old_position.0 != new_position.0 && old_position.1 != new_position.1))
        {
            return false;
        }
        //KING
        if (piece == Pieces::BlackKing || piece == Pieces::WhiteKing)
            && ((x_abs != 0 || y_abs != 1)
                && (x_abs != 1 || y_abs != 0)
                && (x_abs != 1 || y_abs != 1))
        {
            return false;
        }
        if self.is_jumping_piece(piece.clone(), old_position, x_move, y_move) {
            return false;
        }
        true
    }

    fn is_jumping_piece(
        &mut self,
        piece: Pieces,
        old_position: (usize, usize),
        x_move: i8,
        y_move: i8,
    ) -> bool {
        fn check_horizontally(grid: &mut Grid, old_position: (usize, usize), x_move: i8) -> bool {
            for i in 1..x_move {
                if grid[((old_position.0 as i8 + i) as usize, old_position.1)] != Pieces::None {
                    return true;
                }
            }
            for i in x_move..0 {
                if i == x_move {
                    continue;
                }
                if grid[((old_position.0 as i8 + i) as usize, old_position.1)] != Pieces::None {
                    return true;
                }
            }
            false
        }
        fn check_vertically(grid: &mut Grid, old_position: (usize, usize), y_move: i8) -> bool {
            for i in 1..y_move {
                if grid[(old_position.0, (old_position.1 as i8 + i) as usize)] != Pieces::None {
                    return true;
                }
            }
            for i in y_move..0 {
                if i == y_move {
                    continue;
                }
                if grid[(old_position.0, (old_position.1 as i8 + i) as usize)] != Pieces::None {
                    return true;
                }
            }
            false
        }
        fn check_diagonally_pos(grid: &mut Grid, old_position: (usize, usize), x_move: i8) -> bool {
            for i in 1..x_move {
                if grid[(
                    (old_position.0 as i8 - i) as usize,
                    (old_position.1 as i8 + i) as usize,
                )] != Pieces::None
                {
                    return true;
                }
            }
            for i in x_move..0 {
                if i == x_move {
                    continue;
                }
                if grid[(
                    (old_position.0 as i8 + i) as usize,
                    (old_position.1 as i8 - i) as usize,
                )] != Pieces::None
                {
                    return true;
                }
            }
            false
        }
        fn check_diagonally_neg(grid: &mut Grid, old_position: (usize, usize), x_move: i8) -> bool {
            for i in 1..x_move {
                if grid[(
                    (old_position.0 as i8 + i) as usize,
                    (old_position.1 as i8 + i) as usize,
                )] != Pieces::None
                {
                    return true;
                }
            }
            for i in x_move..0 {
                if i == x_move {
                    continue;
                }
                if grid[(
                    (old_position.0 as i8 + i) as usize,
                    (old_position.1 as i8 + i) as usize,
                )] != Pieces::None
                {
                    return true;
                }
            }
            false
        }
        //PAWN
        if piece == Pieces::BlackPawn || piece == Pieces::WhitePawn {
            return check_vertically(self, old_position, y_move);
        }
        //ROOK
        if piece == Pieces::BlackRook || piece == Pieces::WhiteRook {
            if x_move == 0 {
                return check_vertically(self, old_position, y_move);
            }
            return check_horizontally(self, old_position, x_move);
        }
        //BISHOP
        if piece == Pieces::BlackBishop || piece == Pieces::WhiteBishop {
            if (x_move + y_move) == 0i8 {
                return check_diagonally_pos(self, old_position, x_move);
            }
            return check_diagonally_neg(self, old_position, x_move);
        }
        //QUEEN
        if piece == Pieces::BlackQueen || piece == Pieces::WhiteQueen {
            if (x_move + y_move) == 0i8 {
                return check_diagonally_pos(self, old_position, x_move);
            } else if (x_move - y_move) == 0i8 {
                return check_diagonally_neg(self, old_position, x_move);
            } else if x_move == 0 {
                return check_vertically(self, old_position, y_move);
            }
            return check_horizontally(self, old_position, x_move);
        }
        //KING
        if piece == Pieces::BlackKing || piece == Pieces::WhiteKing {
            return check_horizontally(self, old_position, y_move);
        }
        false
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = Pieces;

    fn index(&self, (col, row): (usize, usize)) -> &Self::Output {
        &self.items[8 * row + col]
    }
}

impl IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, (col, row): (usize, usize)) -> &mut Self::Output {
        &mut self.items[8 * row + col]
    }
}

pub type Board = Arc<Mutex<Grid>>;
