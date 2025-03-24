use std::ops::{Index, IndexMut};
use std::sync::{Arc, Mutex};

struct Piece {
    piece_type: Pieces,
    position: (usize, usize),
}

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
    items: [Pieces; 64],
    turn: bool, //true = white; false = black;
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            items: [
                Pieces::BlackRook,
                Pieces::BlackKnight,
                Pieces::BlackBishop,
                Pieces::BlackQueen,
                Pieces::BlackKing,
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
                Pieces::WhiteQueen,
                Pieces::WhiteKing,
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
        //Don't put yourself in check idiot
        let temp_old_piece = self[old_position].clone();
        let temp_new_piece = self[new_position].clone();
        self[new_position] = self[old_position].clone();
        self[old_position] = Pieces::None;
        if self.in_check(!self.turn) {
            self[old_position] = temp_old_piece;
            self[new_position] = temp_new_piece;
            return;
        }

        //Check for checks
        if self.in_check(self.turn) {
            println!("Holy fuck you're dead");
        }

        self.turn = !self.turn;
    }
    fn get_white_pieces(&self) -> Vec<Piece> {
        let mut pieces: Vec<Piece> = Vec::new();
        let white_pieces: [Pieces; 6] = [
            Pieces::WhitePawn,
            Pieces::WhiteKnight,
            Pieces::WhiteBishop,
            Pieces::WhiteRook,
            Pieces::WhiteQueen,
            Pieces::WhiteKing,
        ];

        for i in 0..self.items.len() {
            let x: usize = i % 8;
            let y: usize = i / 8;
            let piece = self[(x, y)].clone();
            if white_pieces.contains(&piece) {
                pieces.push(Piece {
                    piece_type: piece,
                    position: (x, y),
                });
            }
        }
        pieces
    }
    fn get_black_pieces(&self) -> Vec<Piece> {
        let mut pieces: Vec<Piece> = Vec::new();
        let black_pieces: [Pieces; 6] = [
            Pieces::BlackPawn,
            Pieces::BlackKnight,
            Pieces::BlackBishop,
            Pieces::BlackRook,
            Pieces::BlackQueen,
            Pieces::BlackKing,
        ];

        for i in 0..self.items.len() {
            let x: usize = i % 8;
            let y: usize = i / 8;
            let piece = self[(x, y)].clone();
            if black_pieces.contains(&piece) {
                pieces.push(Piece {
                    piece_type: piece,
                    position: (x, y),
                });
            }
        }
        pieces
    }
    fn get_white_king(&self) -> Piece {
        for i in 0..self.items.len() {
            let x: usize = i % 8;
            let y: usize = i / 8;
            let piece = self[(x, y)].clone();
            if piece == Pieces::WhiteKing {
                return Piece {
                    piece_type: Pieces::WhiteKing,
                    position: (x, y),
                };
            }
        }
        Piece {
            piece_type: Pieces::None,
            position: (0, 0),
        }
    }
    fn get_black_king(&self) -> Piece {
        for i in 0..self.items.len() {
            let x: usize = i % 8;
            let y: usize = i / 8;
            let piece = self[(x, y)].clone();
            if piece == Pieces::BlackKing {
                return Piece {
                    piece_type: Pieces::BlackKing,
                    position: (x, y),
                };
            }
        }
        Piece {
            piece_type: Pieces::None,
            position: (0, 0),
        }
    }
    fn is_move_valid(
        &self,
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
            && (y_move != 1
                || x_move != 0
                || self[(new_position.0, new_position.1)] != Pieces::None)
            && (old_position.1 != 1
                || y_move != 2
                || x_move != 0
                || self[(new_position.0, new_position.1)] != Pieces::None)
        {
            if y_move == 1
                && (x_move == -1 || x_move == 1)
                && (self[(new_position.0, new_position.1)] != Pieces::None)
            {
                return true;
            }
            return false;
        }
        //WHITE PAWN
        if (piece == Pieces::WhitePawn)
            && (y_move != -1
                || x_move != 0
                || self[(new_position.0, new_position.1)] != Pieces::None)
            && (old_position.1 != 6
                || y_move != -2
                || x_move != 0
                || self[(new_position.0, new_position.1)] != Pieces::None)
        {
            if y_move == -1
                && (x_move == -1 || x_move == 1)
                && (self[(new_position.0, new_position.1)] != Pieces::None)
            {
                return true;
            }
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
        &self,
        piece: Pieces,
        old_position: (usize, usize),
        x_move: i8,
        y_move: i8,
    ) -> bool {
        fn check_horizontally(grid: &Grid, old_position: (usize, usize), x_move: i8) -> bool {
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
        fn check_vertically(grid: &Grid, old_position: (usize, usize), y_move: i8) -> bool {
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
        fn check_diagonally_pos(grid: &Grid, old_position: (usize, usize), x_move: i8) -> bool {
            for i in 1..x_move {
                if grid[(
                    (old_position.0 as i8 + i) as usize,
                    (old_position.1 as i8 - i) as usize,
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
        fn check_diagonally_neg(grid: &Grid, old_position: (usize, usize), x_move: i8) -> bool {
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
    fn in_check(&self, attacker: bool) -> bool {
        let attacking_pieces: Vec<Piece>;
        let defending_king: Piece;
        if attacker {
            attacking_pieces = self.get_white_pieces();
            defending_king = self.get_black_king();
        } else {
            attacking_pieces = self.get_black_pieces();
            defending_king = self.get_white_king();
        }

        for piece in attacking_pieces {
            if self.is_move_valid(piece.piece_type, piece.position, defending_king.position) {
                return true;
            }
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
