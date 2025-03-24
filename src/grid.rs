use std::ops::{Index, IndexMut};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Default, PartialEq, Copy)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
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

const WHITE_PIECES: [Pieces; 6] = [
    Pieces::WhitePawn,
    Pieces::WhiteKnight,
    Pieces::WhiteBishop,
    Pieces::WhiteRook,
    Pieces::WhiteQueen,
    Pieces::WhiteKing,
];

const BLACK_PIECES: [Pieces; 6] = [
    Pieces::BlackPawn,
    Pieces::BlackKnight,
    Pieces::BlackBishop,
    Pieces::BlackRook,
    Pieces::BlackQueen,
    Pieces::BlackKing,
];

#[derive(Debug, Clone, Default)]
pub struct Piece {
    pub piece_type: Pieces,
    pub position: Position,
}

pub struct Grid {
    items: [Piece; 64],
    turn: bool, //true = white; false = black;
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            items: [
                Piece {
                    piece_type: Pieces::BlackRook,
                    position: Position::new(0, 0),
                },
                Piece {
                    piece_type: Pieces::BlackKnight,
                    position: Position::new(1, 0),
                },
                Piece {
                    piece_type: Pieces::BlackBishop,
                    position: Position::new(2, 0),
                },
                Piece {
                    piece_type: Pieces::BlackQueen,
                    position: Position::new(3, 0),
                },
                Piece {
                    piece_type: Pieces::BlackKing,
                    position: Position::new(4, 0),
                },
                Piece {
                    piece_type: Pieces::BlackBishop,
                    position: Position::new(5, 0),
                },
                Piece {
                    piece_type: Pieces::BlackKnight,
                    position: Position::new(6, 0),
                },
                Piece {
                    piece_type: Pieces::BlackRook,
                    position: Position::new(7, 0),
                },
                Piece {
                    piece_type: Pieces::BlackPawn,
                    position: Position::new(0, 1),
                },
                Piece {
                    piece_type: Pieces::BlackPawn,
                    position: Position::new(1, 1),
                },
                Piece {
                    piece_type: Pieces::BlackPawn,
                    position: Position::new(2, 1),
                },
                Piece {
                    piece_type: Pieces::BlackPawn,
                    position: Position::new(3, 1),
                },
                Piece {
                    piece_type: Pieces::BlackPawn,
                    position: Position::new(4, 1),
                },
                Piece {
                    piece_type: Pieces::BlackPawn,
                    position: Position::new(5, 1),
                },
                Piece {
                    piece_type: Pieces::BlackPawn,
                    position: Position::new(6, 1),
                },
                Piece {
                    piece_type: Pieces::BlackPawn,
                    position: Position::new(7, 1),
                },
                Piece {
                    piece_type: Pieces::None,
                    position: Position::new(0, 2),
                },
                Piece {
                    piece_type: Pieces::None,
                    position: Position::new(1, 2),
                },
                Piece {
                    piece_type: Pieces::None,
                    position: Position::new(2, 2),
                },
                Piece {
                    piece_type: Pieces::None,
                    position: Position::new(3, 2),
                },
                Piece {
                    piece_type: Pieces::None,
                    position: Position::new(4, 2),
                },
                Piece {
                    piece_type: Pieces::None,
                    position: Position::new(5, 2),
                },
                Piece {
                    piece_type: Pieces::None,
                    position: Position::new(6, 2),
                },
                Piece {
                    piece_type: Pieces::None,
                    position: Position::new(7, 2),
                },
                Piece {
                    piece_type: Pieces::None,
                    position: Position::new(0, 3),
                },
                Piece {
                    piece_type: Pieces::None,
                    position: Position::new(1, 3),
                },
                Piece {
                    piece_type: Pieces::None,
                    position: Position::new(2, 3),
                },
                Piece {
                    piece_type: Pieces::None,
                    position: Position::new(3, 3),
                },
                Piece {
                    piece_type: Pieces::None,
                    position: Position::new(4, 3),
                },
                Piece {
                    piece_type: Pieces::None,
                    position: Position::new(5, 3),
                },
                Piece {
                    piece_type: Pieces::None,
                    position: Position::new(6, 3),
                },
                Piece {
                    piece_type: Pieces::None,
                    position: Position::new(7, 3),
                },
                Piece {
                    piece_type: Pieces::None,
                    position: Position::new(0, 4),
                },
                Piece {
                    piece_type: Pieces::None,
                    position: Position::new(1, 4),
                },
                Piece {
                    piece_type: Pieces::None,
                    position: Position::new(2, 4),
                },
                Piece {
                    piece_type: Pieces::None,
                    position: Position::new(3, 4),
                },
                Piece {
                    piece_type: Pieces::None,
                    position: Position::new(4, 4),
                },
                Piece {
                    piece_type: Pieces::None,
                    position: Position::new(5, 4),
                },
                Piece {
                    piece_type: Pieces::None,
                    position: Position::new(6, 4),
                },
                Piece {
                    piece_type: Pieces::None,
                    position: Position::new(7, 4),
                },
                Piece {
                    piece_type: Pieces::None,
                    position: Position::new(0, 5),
                },
                Piece {
                    piece_type: Pieces::None,
                    position: Position::new(1, 5),
                },
                Piece {
                    piece_type: Pieces::None,
                    position: Position::new(2, 5),
                },
                Piece {
                    piece_type: Pieces::None,
                    position: Position::new(3, 5),
                },
                Piece {
                    piece_type: Pieces::None,
                    position: Position::new(4, 5),
                },
                Piece {
                    piece_type: Pieces::None,
                    position: Position::new(5, 5),
                },
                Piece {
                    piece_type: Pieces::None,
                    position: Position::new(6, 5),
                },
                Piece {
                    piece_type: Pieces::None,
                    position: Position::new(7, 5),
                },
                Piece {
                    piece_type: Pieces::WhitePawn,
                    position: Position::new(0, 6),
                },
                Piece {
                    piece_type: Pieces::WhitePawn,
                    position: Position::new(1, 6),
                },
                Piece {
                    piece_type: Pieces::WhitePawn,
                    position: Position::new(2, 6),
                },
                Piece {
                    piece_type: Pieces::WhitePawn,
                    position: Position::new(3, 6),
                },
                Piece {
                    piece_type: Pieces::WhitePawn,
                    position: Position::new(4, 6),
                },
                Piece {
                    piece_type: Pieces::WhitePawn,
                    position: Position::new(5, 6),
                },
                Piece {
                    piece_type: Pieces::WhitePawn,
                    position: Position::new(6, 6),
                },
                Piece {
                    piece_type: Pieces::WhitePawn,
                    position: Position::new(7, 6),
                },
                Piece {
                    piece_type: Pieces::WhiteRook,
                    position: Position::new(0, 7),
                },
                Piece {
                    piece_type: Pieces::WhiteKnight,
                    position: Position::new(1, 7),
                },
                Piece {
                    piece_type: Pieces::WhiteBishop,
                    position: Position::new(2, 7),
                },
                Piece {
                    piece_type: Pieces::WhiteQueen,
                    position: Position::new(3, 7),
                },
                Piece {
                    piece_type: Pieces::WhiteKing,
                    position: Position::new(4, 7),
                },
                Piece {
                    piece_type: Pieces::WhiteBishop,
                    position: Position::new(5, 7),
                },
                Piece {
                    piece_type: Pieces::WhiteKnight,
                    position: Position::new(6, 7),
                },
                Piece {
                    piece_type: Pieces::WhiteRook,
                    position: Position::new(7, 7),
                },
            ],
            turn: true,
        }
    }
}

impl Grid {
    pub fn move_piece(&mut self, piece: Piece, new_position: Position) {
        //Don't move nothing
        if piece.piece_type == Pieces::None {
            return;
        }
        //Make sure a piece moved
        if piece.position == new_position {
            return;
        }
        //It better be your turn
        if (self.turn && BLACK_PIECES.contains(&piece.piece_type))
            || (!self.turn && WHITE_PIECES.contains(&piece.piece_type))
        {
            return;
        }
        //Make sure the piece moved correctly
        if !self.is_move_valid(piece.clone(), new_position) {
            return;
        }
        //Don't capture your own pieces
        if (self.turn && WHITE_PIECES.contains(&self[new_position].piece_type))
            || (!self.turn && BLACK_PIECES.contains(&self[new_position].piece_type))
        {
            return;
        }
        //Don't put yourself in check idiot
        let temp_old_piece = piece.piece_type.clone();
        let temp_new_piece = self[new_position].piece_type.clone();
        self[new_position].piece_type = piece.piece_type.clone();
        self[piece.position].piece_type = Pieces::None;
        if self.in_check(!self.turn) {
            self[piece.position].piece_type = temp_old_piece;
            self[new_position].piece_type = temp_new_piece;
            return;
        }
        //Check for checks
        if self.in_check(self.turn) {
            println!("Holy fuck you're dead");
        }
        self.turn = !self.turn;
    }
    fn get_white_pieces(&self) -> Vec<Piece> {
        self.items
            .iter()
            .filter(|piece| WHITE_PIECES.contains(&piece.piece_type))
            .cloned()
            .collect()
    }
    fn get_black_pieces(&self) -> Vec<Piece> {
        self.items
            .iter()
            .filter(|piece| BLACK_PIECES.contains(&piece.piece_type))
            .cloned()
            .collect()
    }
    fn get_white_king(&self) -> Piece {
        self.items
            .iter()
            .find(|piece| piece.piece_type == Pieces::WhiteKing)
            .unwrap()
            .clone()
    }
    fn get_black_king(&self) -> Piece {
        self.items
            .iter()
            .find(|piece| piece.piece_type == Pieces::BlackKing)
            .unwrap()
            .clone()
    }
    fn is_move_valid(&self, piece: Piece, new_position: Position) -> bool {
        let x_move: i8 = (new_position.x as i8) - (piece.position.x as i8);
        let y_move: i8 = (new_position.y as i8) - (piece.position.y as i8);
        let x_abs: i8 = x_move.abs();
        let y_abs: i8 = y_move.abs();

        //BLACK PAWN
        if (piece.piece_type == Pieces::BlackPawn)
            && (y_move != 1 || x_move != 0 || self[new_position].piece_type != Pieces::None)
            && (piece.position.y != 1
                || y_move != 2
                || x_move != 0
                || self[new_position].piece_type != Pieces::None)
        {
            if y_move == 1
                && (x_move == -1 || x_move == 1)
                && self[new_position].piece_type != Pieces::None
            {
                return true;
            }
            return false;
        }
        //WHITE PAWN
        if (piece.piece_type == Pieces::WhitePawn)
            && (y_move != -1 || x_move != 0 || self[new_position].piece_type != Pieces::None)
            && (piece.position.y != 6
                || y_move != -2
                || x_move != 0
                || self[new_position].piece_type != Pieces::None)
        {
            if y_move == -1
                && (x_move == -1 || x_move == 1)
                && self[new_position].piece_type != Pieces::None
            {
                return true;
            }
            return false;
        }
        //ROOK
        if (piece.piece_type == Pieces::BlackRook || piece.piece_type == Pieces::WhiteRook)
            && (piece.position.x != new_position.x && piece.position.y != new_position.y)
        {
            return false;
        }
        //KNIGHT
        if (piece.piece_type == Pieces::BlackKnight || piece.piece_type == Pieces::WhiteKnight)
            && ((x_abs != 2 || y_abs != 1) && (x_abs != 1 || y_abs != 2))
        {
            return false;
        }
        //BISHOP
        if (piece.piece_type == Pieces::BlackBishop || piece.piece_type == Pieces::WhiteBishop)
            && ((x_move + y_move) != 0i8 && (x_move - y_move) != 0i8)
        {
            return false;
        }
        //QUEEN
        if (piece.piece_type == Pieces::BlackQueen || piece.piece_type == Pieces::WhiteQueen)
            && (((x_move + y_move) != 0i8 && (x_move - y_move) != 0i8)
                && (piece.position.x != new_position.x && piece.position.y != new_position.y))
        {
            return false;
        }
        //KING
        if (piece.piece_type == Pieces::BlackKing || piece.piece_type == Pieces::WhiteKing)
            && ((x_abs != 0 || y_abs != 1)
                && (x_abs != 1 || y_abs != 0)
                && (x_abs != 1 || y_abs != 1))
        {
            return false;
        }
        if self.is_jumping_piece(piece, x_move, y_move) {
            return false;
        }
        true
    }

    fn is_jumping_piece(&self, piece: Piece, x_move: i8, y_move: i8) -> bool {
        fn check_horizontally(grid: &Grid, mut position: Position, x_move: i8) -> bool {
            for i in 1..x_move {
                position.x = (position.x as i8 + i) as usize;
                if grid[position].piece_type != Pieces::None {
                    return true;
                }
            }
            for i in x_move..0 {
                if i == x_move {
                    continue;
                }
                position.x = (position.x as i8 + i) as usize;
                if grid[position].piece_type != Pieces::None {
                    return true;
                }
            }
            false
        }
        fn check_vertically(grid: &Grid, mut position: Position, y_move: i8) -> bool {
            for i in 1..y_move {
                position.y = (position.y as i8 + i) as usize;
                if grid[position].piece_type != Pieces::None {
                    return true;
                }
            }
            for i in y_move..0 {
                if i == y_move {
                    continue;
                }
                position.y = (position.y as i8 + i) as usize;
                if grid[position].piece_type != Pieces::None {
                    return true;
                }
            }
            false
        }
        fn check_diagonally_pos(grid: &Grid, mut position: Position, x_move: i8) -> bool {
            for i in 1..x_move {
                position.x = (position.x as i8 + i) as usize;
                position.y = (position.y as i8 - i) as usize;
                if grid[position].piece_type != Pieces::None {
                    return true;
                }
            }
            for i in x_move..0 {
                if i == x_move {
                    continue;
                }
                position.x = (position.x as i8 + i) as usize;
                position.y = (position.y as i8 - i) as usize;
                if grid[position].piece_type != Pieces::None {
                    return true;
                }
            }
            false
        }
        fn check_diagonally_neg(grid: &Grid, mut position: Position, x_move: i8) -> bool {
            for i in 1..x_move {
                position.x = (position.x as i8 + i) as usize;
                position.y = (position.y as i8 + i) as usize;
                if grid[position].piece_type != Pieces::None {
                    return true;
                }
            }
            for i in x_move..0 {
                if i == x_move {
                    continue;
                }
                position.x = (position.x as i8 + i) as usize;
                position.y = (position.y as i8 + i) as usize;
                if grid[position].piece_type != Pieces::None {
                    return true;
                }
            }
            false
        }
        //PAWN
        if piece.piece_type == Pieces::BlackPawn || piece.piece_type == Pieces::WhitePawn {
            return check_vertically(self, piece.position, y_move);
        }
        //ROOK
        if piece.piece_type == Pieces::BlackRook || piece.piece_type == Pieces::WhiteRook {
            if x_move == 0 {
                return check_vertically(self, piece.position, y_move);
            }
            return check_horizontally(self, piece.position, x_move);
        }
        //BISHOP
        if piece.piece_type == Pieces::BlackBishop || piece.piece_type == Pieces::WhiteBishop {
            if (x_move + y_move) == 0i8 {
                return check_diagonally_pos(self, piece.position, x_move);
            }
            return check_diagonally_neg(self, piece.position, x_move);
        }
        //QUEEN
        if piece.piece_type == Pieces::BlackQueen || piece.piece_type == Pieces::WhiteQueen {
            if (x_move + y_move) == 0i8 {
                return check_diagonally_pos(self, piece.position, x_move);
            } else if (x_move - y_move) == 0i8 {
                return check_diagonally_neg(self, piece.position, x_move);
            } else if x_move == 0 {
                return check_vertically(self, piece.position, y_move);
            }
            return check_horizontally(self, piece.position, x_move);
        }
        //KING
        if piece.piece_type == Pieces::BlackKing || piece.piece_type == Pieces::WhiteKing {
            return check_horizontally(self, piece.position, y_move);
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
            if self.is_move_valid(piece, defending_king.position) {
                return true;
            }
        }
        false
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = Piece;

    fn index(&self, (col, row): (usize, usize)) -> &Self::Output {
        &self.items[8 * row + col]
    }
}

impl Index<Position> for Grid {
    type Output = Piece;

    fn index(&self, position: Position) -> &Self::Output {
        &self.items[8 * position.y + position.x]
    }
}

impl IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, (col, row): (usize, usize)) -> &mut Self::Output {
        &mut self.items[8 * row + col]
    }
}

impl IndexMut<Position> for Grid {
    fn index_mut(&mut self, position: Position) -> &mut Self::Output {
        &mut self.items[8 * position.y + position.x]
    }
}

pub type Board = Arc<Mutex<Grid>>;
