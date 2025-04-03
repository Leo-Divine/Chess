use crate::piece::{Piece, PieceType, Position};
use std::ops::{Index, IndexMut};

pub struct Board {
    pieces: [Piece; 64],
    white_turn: bool,
    last_piece_moved: Piece,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            pieces: [
                Piece::new(PieceType::Rook, Position::new(0, 0), false),
                Piece::new(PieceType::Knight, Position::new(1, 0), false),
                Piece::new(PieceType::Bishop, Position::new(2, 0), false),
                Piece::new(PieceType::Queen, Position::new(3, 0), false),
                Piece::new(PieceType::King, Position::new(4, 0), false),
                Piece::new(PieceType::Bishop, Position::new(5, 0), false),
                Piece::new(PieceType::Knight, Position::new(6, 0), false),
                Piece::new(PieceType::Rook, Position::new(7, 0), false),
                Piece::new(PieceType::Pawn, Position::new(0, 1), false),
                Piece::new(PieceType::Pawn, Position::new(1, 1), false),
                Piece::new(PieceType::Pawn, Position::new(2, 1), false),
                Piece::new(PieceType::Pawn, Position::new(3, 1), false),
                Piece::new(PieceType::Pawn, Position::new(4, 1), false),
                Piece::new(PieceType::Pawn, Position::new(5, 1), false),
                Piece::new(PieceType::Pawn, Position::new(6, 1), false),
                Piece::new(PieceType::Pawn, Position::new(7, 1), false),
                Piece::new(PieceType::None, Position::new(0, 2), true),
                Piece::new(PieceType::None, Position::new(1, 2), true),
                Piece::new(PieceType::None, Position::new(2, 2), true),
                Piece::new(PieceType::None, Position::new(3, 2), true),
                Piece::new(PieceType::None, Position::new(4, 2), true),
                Piece::new(PieceType::None, Position::new(5, 2), true),
                Piece::new(PieceType::None, Position::new(6, 2), true),
                Piece::new(PieceType::None, Position::new(7, 2), true),
                Piece::new(PieceType::None, Position::new(0, 3), true),
                Piece::new(PieceType::None, Position::new(1, 3), true),
                Piece::new(PieceType::None, Position::new(2, 3), true),
                Piece::new(PieceType::None, Position::new(3, 3), true),
                Piece::new(PieceType::None, Position::new(4, 3), true),
                Piece::new(PieceType::None, Position::new(5, 3), true),
                Piece::new(PieceType::None, Position::new(6, 3), true),
                Piece::new(PieceType::None, Position::new(7, 3), true),
                Piece::new(PieceType::None, Position::new(0, 4), true),
                Piece::new(PieceType::None, Position::new(1, 4), true),
                Piece::new(PieceType::None, Position::new(2, 4), true),
                Piece::new(PieceType::None, Position::new(3, 4), true),
                Piece::new(PieceType::None, Position::new(4, 4), true),
                Piece::new(PieceType::None, Position::new(5, 4), true),
                Piece::new(PieceType::None, Position::new(6, 4), true),
                Piece::new(PieceType::None, Position::new(7, 4), true),
                Piece::new(PieceType::None, Position::new(0, 5), true),
                Piece::new(PieceType::None, Position::new(1, 5), true),
                Piece::new(PieceType::None, Position::new(2, 5), true),
                Piece::new(PieceType::None, Position::new(3, 5), true),
                Piece::new(PieceType::None, Position::new(4, 5), true),
                Piece::new(PieceType::None, Position::new(5, 5), true),
                Piece::new(PieceType::None, Position::new(6, 5), true),
                Piece::new(PieceType::None, Position::new(7, 5), true),
                Piece::new(PieceType::Pawn, Position::new(0, 6), true),
                Piece::new(PieceType::Pawn, Position::new(1, 6), true),
                Piece::new(PieceType::Pawn, Position::new(2, 6), true),
                Piece::new(PieceType::Pawn, Position::new(3, 6), true),
                Piece::new(PieceType::Pawn, Position::new(4, 6), true),
                Piece::new(PieceType::Pawn, Position::new(5, 6), true),
                Piece::new(PieceType::Pawn, Position::new(6, 6), true),
                Piece::new(PieceType::Pawn, Position::new(7, 6), true),
                Piece::new(PieceType::Rook, Position::new(0, 7), true),
                Piece::new(PieceType::Knight, Position::new(1, 7), true),
                Piece::new(PieceType::Bishop, Position::new(2, 7), true),
                Piece::new(PieceType::Queen, Position::new(3, 7), true),
                Piece::new(PieceType::King, Position::new(4, 7), true),
                Piece::new(PieceType::Bishop, Position::new(5, 7), true),
                Piece::new(PieceType::Knight, Position::new(6, 7), true),
                Piece::new(PieceType::Rook, Position::new(7, 7), true),
            ],
            white_turn: true,
            last_piece_moved: Piece::new(PieceType::None, Position::new(0, 0), false),
        }
    }
}
impl Board {
    pub fn move_piece(&mut self, old_position: Position, new_position: Position) {
        let moved_piece: Piece = self[old_position];
        let attacked_piece: Piece = self[new_position];

        if moved_piece.piece_type == PieceType::None
            || old_position == new_position
            || self.white_turn != moved_piece.is_white
            || !self.is_move_valid(moved_piece, attacked_piece)
            || (self.white_turn == attacked_piece.is_white
                && attacked_piece.piece_type != PieceType::None)
        {
            return;
        }

        self[old_position].position = new_position;
        self[new_position] = self[old_position];
        self[old_position] = Piece::new(PieceType::None, old_position, true);
        if self.in_check(!self.white_turn) {
            self[old_position] = moved_piece;
            self[new_position] = attacked_piece;
            return;
        }
        self[new_position].has_moved = true;
        self.white_turn = !self.white_turn;
        self.last_piece_moved = self[new_position].clone();
    }
    pub fn is_move_valid(&mut self, moved_piece: Piece, attacked_piece: Piece) -> bool {
        let x_move: i8 = (attacked_piece.position.x as i8) - (moved_piece.position.x as i8);
        let y_move: i8 = (attacked_piece.position.y as i8) - (moved_piece.position.y as i8);
        let x_abs: i8 = x_move.abs();
        let y_abs: i8 = y_move.abs();

        //WHITE PAWN
        if moved_piece.piece_type == PieceType::Pawn && moved_piece.is_white {
            println!(
                "{:?}, {}, ({}, {})",
                moved_piece.piece_type,
                moved_piece.is_white,
                moved_piece.position.x,
                moved_piece.position.y
            );
            println!(
                "{:?}, {}, ({}, {})",
                attacked_piece.piece_type,
                attacked_piece.is_white,
                attacked_piece.position.x,
                attacked_piece.position.y
            );
            if (y_move == -1 && x_move == 0 && attacked_piece.piece_type == PieceType::None)
                || (!moved_piece.has_moved
                    && y_move == -2
                    && x_move == 0
                    && attacked_piece.piece_type == PieceType::None)
                || (y_move == -1
                    && (x_move == -1 || x_move == 1)
                    && attacked_piece.piece_type != PieceType::None)
            {
                return !self.is_jumping_vertically(moved_piece, y_move);
            } else if self.last_piece_moved.piece_type == PieceType::Pawn
                && self.last_piece_moved.position.y == 2
                && x_abs == 1
                && y_move == 1
            {
                if self.last_piece_moved
                    == self[Position::new(attacked_piece.position.x, attacked_piece.position.y - 1)]
                {
                    self[Position::new(attacked_piece.position.x, 2)] = Piece::new(
                        PieceType::None,
                        Position::new(attacked_piece.position.x, 2),
                        false,
                    );
                    self[attacked_piece.position] =
                        Piece::new(PieceType::Pawn, attacked_piece.position, true);
                    self[moved_piece.position] =
                        Piece::new(PieceType::None, moved_piece.position, true);

                    if self.in_check(!self.white_turn) {
                        self[Position::new(attacked_piece.position.x, 2)] =
                            self.last_piece_moved.clone();
                        self[attacked_piece.position] =
                            Piece::new(PieceType::None, moved_piece.position, true);
                        self[moved_piece.position] = moved_piece.clone();
                        return false;
                    }

                    self[attacked_piece.position].has_moved = true;
                    self.white_turn = !self.white_turn;
                    self.last_piece_moved = self[attacked_piece.position].clone();

                    return false;
                }
            }
        }
        //BLACK PAWN
        if moved_piece.piece_type == PieceType::Pawn && !moved_piece.is_white {
            if (y_move == 1 && x_move == 0 && attacked_piece.piece_type == PieceType::None)
                || (!moved_piece.has_moved
                    && y_move == 2
                    && x_move == 0
                    && attacked_piece.piece_type == PieceType::None)
                || (y_move == 1
                    && (x_move == -1 || x_move == 1)
                    && attacked_piece.piece_type != PieceType::None)
            {
                return !self.is_jumping_vertically(moved_piece, y_move);
            }
        }
        //ROOK
        if moved_piece.piece_type == PieceType::Rook {
            if moved_piece.position.x == attacked_piece.position.x {
                return !self.is_jumping_vertically(moved_piece, y_move);
            } else if moved_piece.position.y == attacked_piece.position.y {
                return !self.is_jumping_horizontally(moved_piece, x_move);
            }
        }
        //KNIGHT
        if moved_piece.piece_type == PieceType::Knight
            && ((x_abs == 2 && y_abs == 1) || (x_abs == 1 && y_abs == 2))
        {
            return true;
        }
        //BISHOP
        if moved_piece.piece_type == PieceType::Bishop {
            if (x_move + y_move) == 0i8 {
                return !self.is_jumping_diagonally_pos(moved_piece, x_move);
            } else if (x_move - y_move) == 0i8 {
                return !self.is_jumping_diagonally_neg(moved_piece, x_move);
            }
        }
        //QUEEN
        if moved_piece.piece_type == PieceType::Queen {
            if moved_piece.position.x == attacked_piece.position.x {
                return !self.is_jumping_vertically(moved_piece, y_move);
            } else if moved_piece.position.y == attacked_piece.position.y {
                return !self.is_jumping_horizontally(moved_piece, x_move);
            } else if (x_move + y_move) == 0i8 {
                return !self.is_jumping_diagonally_pos(moved_piece, x_move);
            } else if (x_move - y_move) == 0i8 {
                return !self.is_jumping_diagonally_neg(moved_piece, x_move);
            }
        }
        //WHITE KING
        if moved_piece.piece_type == PieceType::King && moved_piece.is_white {
            if (x_abs == 0 && y_abs == 1)
                || (x_abs == 1 && y_abs == 0)
                || (x_abs == 1 && y_abs == 1)
            {
                return true;
            } else if x_move == 2
                && y_move == 0
                && !moved_piece.has_moved
                && self[Position::new(7, 7)].piece_type == PieceType::Rook
                && !self[Position::new(7, 7)].has_moved
                && !self.is_jumping_horizontally(moved_piece, x_move)
            {
                self.short_castling_checks(true);
                return false;
            } else if x_move == -3
                && y_move == 0
                && !moved_piece.has_moved
                && self[Position::new(0, 7)].piece_type == PieceType::Rook
                && !self[Position::new(0, 7)].has_moved
                && !self.is_jumping_horizontally(moved_piece, x_move)
            {
                self.long_castling_checks(true);
                return false;
            }
        }
        //BLACK KING
        if moved_piece.piece_type == PieceType::King && !moved_piece.is_white {
            if (x_abs == 0 && y_abs == 1)
                || (x_abs == 1 && y_abs == 0)
                || (x_abs == 1 && y_abs == 1)
            {
                return true;
            } else if x_move == 2
                && y_move == 0
                && !moved_piece.has_moved
                && self[Position::new(7, 0)].piece_type == PieceType::Rook
                && !self[Position::new(7, 0)].has_moved
                && !self.is_jumping_horizontally(moved_piece, x_move)
            {
                self.short_castling_checks(false);
                return false;
            } else if x_move == -3
                && y_move == 0
                && !moved_piece.has_moved
                && self[Position::new(0, 0)].piece_type == PieceType::Rook
                && !self[Position::new(0, 0)].has_moved
                && !self.is_jumping_horizontally(moved_piece, x_move)
            {
                self.long_castling_checks(false);
                return false;
            }
        }
        false
    }
    fn short_castling_checks(&mut self, is_white: bool) {
        if is_white {
            self[Position::new(5, 7)] = Piece::new(PieceType::King, Position::new(5, 7), true);
            self[Position::new(4, 7)] = Piece::new(PieceType::None, Position::new(4, 7), true);
            if self.in_check(!self.white_turn) {
                self[Position::new(4, 7)] = Piece::new(PieceType::King, Position::new(4, 7), true);
                self[Position::new(5, 7)] = Piece::new(PieceType::None, Position::new(5, 7), true);
                return;
            }
            self[Position::new(6, 7)] = Piece::new(PieceType::King, Position::new(6, 7), true);
            self[Position::new(5, 7)] = Piece::new(PieceType::None, Position::new(5, 7), true);
            if self.in_check(!self.white_turn) {
                self[Position::new(4, 7)] = Piece::new(PieceType::King, Position::new(4, 7), true);
                self[Position::new(6, 7)] = Piece::new(PieceType::None, Position::new(6, 7), true);
                return;
            }
            self[Position::new(5, 7)] = Piece::new(PieceType::Rook, Position::new(5, 7), true);
            self[Position::new(7, 7)] = Piece::new(PieceType::None, Position::new(7, 7), true);
            self[Position::new(6, 7)].has_moved = true;
            self[Position::new(5, 7)].has_moved = true;
            self.last_piece_moved = self[Position::new(6, 7)].clone();
        } else {
            self[Position::new(5, 0)] = Piece::new(PieceType::King, Position::new(5, 0), false);
            self[Position::new(4, 0)] = Piece::new(PieceType::None, Position::new(4, 0), false);
            if self.in_check(self.white_turn) {
                self[Position::new(4, 0)] = Piece::new(PieceType::King, Position::new(4, 0), false);
                self[Position::new(5, 0)] = Piece::new(PieceType::None, Position::new(5, 0), false);
                return;
            }
            self[Position::new(6, 0)] = Piece::new(PieceType::King, Position::new(6, 0), false);
            self[Position::new(5, 0)] = Piece::new(PieceType::None, Position::new(5, 0), false);
            if self.in_check(self.white_turn) {
                self[Position::new(4, 0)] = Piece::new(PieceType::King, Position::new(4, 0), false);
                self[Position::new(6, 0)] = Piece::new(PieceType::None, Position::new(6, 0), false);
                return;
            }
            self[Position::new(5, 0)] = Piece::new(PieceType::Rook, Position::new(5, 0), false);
            self[Position::new(7, 0)] = Piece::new(PieceType::None, Position::new(7, 0), false);
            self[Position::new(6, 0)].has_moved = true;
            self[Position::new(5, 0)].has_moved = true;
            self.last_piece_moved = self[Position::new(6, 0)].clone();
        }
        self.white_turn = !self.white_turn;
    }
    fn long_castling_checks(&mut self, is_white: bool) {
        if is_white {
            self[Position::new(3, 7)] = Piece::new(PieceType::King, Position::new(3, 7), true);
            self[Position::new(4, 7)] = Piece::new(PieceType::None, Position::new(4, 7), true);
            if self.in_check(!self.white_turn) {
                self[Position::new(4, 7)] = Piece::new(PieceType::King, Position::new(4, 7), true);
                self[Position::new(3, 7)] = Piece::new(PieceType::None, Position::new(3, 7), true);
                return;
            }
            self[Position::new(2, 7)] = Piece::new(PieceType::King, Position::new(2, 7), true);
            self[Position::new(3, 7)] = Piece::new(PieceType::None, Position::new(3, 7), true);
            if self.in_check(!self.white_turn) {
                self[Position::new(4, 7)] = Piece::new(PieceType::King, Position::new(4, 7), true);
                self[Position::new(2, 7)] = Piece::new(PieceType::None, Position::new(2, 7), true);
                return;
            }
            self[Position::new(1, 7)] = Piece::new(PieceType::King, Position::new(1, 7), true);
            self[Position::new(2, 7)] = Piece::new(PieceType::None, Position::new(2, 7), true);
            if self.in_check(!self.white_turn) {
                self[Position::new(4, 7)] = Piece::new(PieceType::King, Position::new(4, 7), true);
                self[Position::new(1, 7)] = Piece::new(PieceType::None, Position::new(1, 7), true);
                return;
            }
            self[Position::new(2, 7)] = Piece::new(PieceType::Rook, Position::new(2, 7), true);
            self[Position::new(0, 7)] = Piece::new(PieceType::None, Position::new(0, 7), true);
            self[Position::new(1, 7)].has_moved = true;
            self[Position::new(2, 7)].has_moved = true;
            self.last_piece_moved = self[Position::new(1, 7)].clone();
        } else {
            self[Position::new(3, 0)] = Piece::new(PieceType::King, Position::new(3, 0), false);
            self[Position::new(4, 0)] = Piece::new(PieceType::None, Position::new(4, 0), false);
            if self.in_check(!self.white_turn) {
                self[Position::new(4, 0)] = Piece::new(PieceType::King, Position::new(4, 0), false);
                self[Position::new(3, 0)] = Piece::new(PieceType::None, Position::new(3, 0), false);
                return;
            }
            self[Position::new(2, 0)] = Piece::new(PieceType::King, Position::new(2, 0), false);
            self[Position::new(3, 0)] = Piece::new(PieceType::None, Position::new(3, 0), false);
            if self.in_check(!self.white_turn) {
                self[Position::new(4, 0)] = Piece::new(PieceType::King, Position::new(4, 0), false);
                self[Position::new(2, 0)] = Piece::new(PieceType::None, Position::new(2, 0), false);
                return;
            }
            self[Position::new(1, 0)] = Piece::new(PieceType::King, Position::new(1, 0), false);
            self[Position::new(2, 0)] = Piece::new(PieceType::None, Position::new(2, 0), false);
            if self.in_check(!self.white_turn) {
                self[Position::new(4, 0)] = Piece::new(PieceType::King, Position::new(4, 0), false);
                self[Position::new(1, 0)] = Piece::new(PieceType::None, Position::new(1, 0), false);
                return;
            }
            self[Position::new(2, 0)] = Piece::new(PieceType::Rook, Position::new(2, 0), false);
            self[Position::new(0, 0)] = Piece::new(PieceType::None, Position::new(0, 0), false);
            self[Position::new(1, 0)].has_moved = true;
            self[Position::new(2, 0)].has_moved = true;
            self.last_piece_moved = self[Position::new(1, 0)].clone();
        }
        self.white_turn = !self.white_turn;
    }
    fn is_jumping_horizontally(&self, piece: Piece, x_move: i8) -> bool {
        for i in 1..x_move {
            let mut position: Position = piece.position;
            position.x = (position.x as i8 + i) as u8;
            if self[position].piece_type != PieceType::None {
                return true;
            }
        }
        for i in x_move..0 {
            if i == x_move {
                continue;
            }
            let mut position: Position = piece.position;
            position.x = (position.x as i8 + i) as u8;
            if self[position].piece_type != PieceType::None {
                return true;
            }
        }
        false
    }
    fn is_jumping_vertically(&self, piece: Piece, y_move: i8) -> bool {
        for i in 1..y_move {
            let mut position: Position = piece.position;
            position.y = (position.y as i8 + i) as u8;
            if self[position].piece_type != PieceType::None {
                return true;
            }
        }
        for i in y_move..0 {
            if i == y_move {
                continue;
            }
            let mut position: Position = piece.position;
            position.y = (position.y as i8 + i) as u8;
            if self[position].piece_type != PieceType::None {
                return true;
            }
        }
        false
    }
    fn is_jumping_diagonally_pos(&self, piece: Piece, x_move: i8) -> bool {
        for i in 1..x_move {
            let mut position: Position = piece.position;
            position.x = (position.x as i8 + i) as u8;
            position.y = (position.y as i8 - i) as u8;
            if self[position].piece_type != PieceType::None {
                return true;
            }
        }
        for i in x_move..0 {
            if i == x_move {
                continue;
            }
            let mut position: Position = piece.position;
            position.x = (position.x as i8 + i) as u8;
            position.y = (position.y as i8 - i) as u8;
            if self[position].piece_type != PieceType::None {
                return true;
            }
        }
        false
    }
    fn is_jumping_diagonally_neg(&self, piece: Piece, x_move: i8) -> bool {
        for i in 1..x_move {
            let mut position: Position = piece.position;
            position.x = (position.x as i8 + i) as u8;
            position.y = (position.y as i8 + i) as u8;
            if self[position].piece_type != PieceType::None {
                return true;
            }
        }
        for i in x_move..0 {
            if i == x_move {
                continue;
            }
            let mut position: Position = piece.position;
            position.x = (position.x as i8 + i) as u8;
            position.y = (position.y as i8 + i) as u8;
            if self[position].piece_type != PieceType::None {
                return true;
            }
        }
        false
    }
    fn in_check(&mut self, white_attacking: bool) -> bool {
        let attacking_pieces: Vec<Piece> = self.get_attacking_pieces(white_attacking);
        let defending_king: Piece = self.get_defending_king(white_attacking);
        for piece in attacking_pieces {
            if self.is_move_valid(piece, defending_king) {
                return true;
            }
        }
        false
    }
    fn get_attacking_pieces(&self, white_attacking: bool) -> Vec<Piece> {
        if white_attacking {
            self.pieces
                .iter()
                .filter(|piece| piece.is_white)
                .cloned()
                .collect()
        } else {
            self.pieces
                .iter()
                .filter(|piece| !piece.is_white)
                .cloned()
                .collect()
        }
    }
    fn get_defending_king(&self, white_attacking: bool) -> Piece {
        if white_attacking {
            *self
                .pieces
                .iter()
                .find(|piece| !piece.is_white && piece.piece_type == PieceType::King)
                .unwrap()
        } else {
            *self
                .pieces
                .iter()
                .find(|piece| piece.is_white && piece.piece_type == PieceType::King)
                .unwrap()
        }
    }
}

impl Index<Position> for Board {
    type Output = Piece;

    fn index(&self, position: Position) -> &Self::Output {
        &self.pieces[(8 * position.y + position.x) as usize]
    }
}

impl IndexMut<Position> for Board {
    fn index_mut(&mut self, position: Position) -> &mut Self::Output {
        &mut self.pieces[(8 * position.y + position.x) as usize]
    }
}
