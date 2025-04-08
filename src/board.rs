use crate::piece::{starting_pieces, Piece, PieceType, Position};
use std::ops::{Index, IndexMut};

#[derive(PartialEq)]
enum MoveValidity {
    Valid,
    ShortCastle,
    LongCastle,
    EnPassant,
    Invalid,
}

pub struct Board {
    pieces: [Piece; 64],
    pub white_turn: bool,
    last_piece_moved: Piece,
    previous_pieces: [Piece; 64],
}

impl Default for Board {
    fn default() -> Self {
        Self {
            pieces: starting_pieces(),
            white_turn: true,
            last_piece_moved: Piece::new(PieceType::None, Position::new(0, 0), false),
            previous_pieces: starting_pieces(),
        }
    }
}
impl Board {
    ///Tries to move a piece given a start and end position.
    pub fn move_piece(&mut self, old_position: Position, new_position: Position) -> Option<String> {
        self.previous_pieces = self.pieces;

        let moved_piece: Piece = self[old_position];
        let attacked_piece: Piece = self[new_position];
        let mut piece_captured = false;
        let mut check_checkmate = "";
        let mut en_passant = false;

        if moved_piece.piece_type == PieceType::None
            || old_position == new_position
            || self.white_turn != moved_piece.is_white
            || (self.white_turn == attacked_piece.is_white
                && attacked_piece.piece_type != PieceType::None)
        {
            return None;
        }
        match self.is_move_valid(moved_piece, attacked_piece) {
            MoveValidity::Valid => (),
            MoveValidity::EnPassant => en_passant = true,
            MoveValidity::ShortCastle => return Some("O-O".to_string()),
            MoveValidity::LongCastle => return Some("O-O-O".to_string()),
            MoveValidity::Invalid => return None,
        }

        if attacked_piece.piece_type != PieceType::None {
            piece_captured = true;
        }

        self.do_move(moved_piece, attacked_piece);
        if self.in_check(self.white_turn) {
            self.undo_move();
            return None;
        }
        self.white_turn = !self.white_turn;
        self.last_piece_moved = self[new_position];
        self.previous_pieces = self.pieces;

        if self.in_check(self.white_turn) {
            check_checkmate = "+";
            if self.is_checkmate(self.white_turn) {
                check_checkmate = "#";
            }
        }
        Some(Board::make_piece_notation(
            moved_piece,
            attacked_piece,
            piece_captured,
            en_passant,
            check_checkmate.to_string(),
        ))
    }
    ///Moves a piece to a specified location.
    fn do_move(&mut self, moved_piece: Piece, attacked_piece: Piece) {
        self[attacked_piece.position] = Piece {
            piece_type: moved_piece.piece_type,
            position: attacked_piece.position,
            is_white: moved_piece.is_white,
            has_moved: true,
        };
        self[moved_piece.position] = Piece::new(PieceType::None, moved_piece.position, true);
    }
    ///Sets the board back to the last move.
    fn undo_move(&mut self) {
        self.pieces = self.previous_pieces;
    }
    ///Verifies if a given piece can move to the given position acording to it's movement options.
    fn is_move_valid(&mut self, moved_piece: Piece, attacked_piece: Piece) -> MoveValidity {
        let x_move: i8 = (attacked_piece.position.x as i8) - (moved_piece.position.x as i8);
        let y_move: i8 = (attacked_piece.position.y as i8) - (moved_piece.position.y as i8);
        let x_abs: i8 = x_move.abs();
        let y_abs: i8 = y_move.abs();

        //WHITE PAWN
        if moved_piece.piece_type == PieceType::Pawn && moved_piece.is_white {
            #[allow(clippy::collapsible_if)]
            if (y_move == -1 && x_move == 0 && attacked_piece.piece_type == PieceType::None)
                || (!moved_piece.has_moved
                    && y_move == -2
                    && x_move == 0
                    && attacked_piece.piece_type == PieceType::None)
                || (y_move == -1
                    && (x_move == -1 || x_move == 1)
                    && attacked_piece.piece_type != PieceType::None)
            {
                match self.is_jumping_vertically(moved_piece, y_move) {
                    true => return MoveValidity::Invalid,
                    false => return MoveValidity::Valid,
                }
            } else if self.last_piece_moved.piece_type == PieceType::Pawn
                && self.last_piece_moved.position.y == 3
                && x_abs == 1
                && y_move == -1
            {
                if self.last_piece_moved.position
                    == Position::new(attacked_piece.position.x, attacked_piece.position.y + 1)
                {
                    self[Position::new(attacked_piece.position.x, 3)] = Piece::new(
                        PieceType::None,
                        Position::new(attacked_piece.position.x, 3),
                        false,
                    );
                    self.do_move(moved_piece, attacked_piece);
                    if self.in_check(self.white_turn) {
                        self.undo_move();
                        return MoveValidity::Invalid;
                    }
                    self.last_piece_moved = self[attacked_piece.position];
                    return MoveValidity::EnPassant;
                }
            }
        }
        //BLACK PAWN
        if moved_piece.piece_type == PieceType::Pawn && !moved_piece.is_white {
            #[allow(clippy::collapsible_if)]
            if (y_move == 1 && x_move == 0 && attacked_piece.piece_type == PieceType::None)
                || (!moved_piece.has_moved
                    && y_move == 2
                    && x_move == 0
                    && attacked_piece.piece_type == PieceType::None)
                || (y_move == 1
                    && (x_move == -1 || x_move == 1)
                    && attacked_piece.piece_type != PieceType::None)
            {
                match self.is_jumping_vertically(moved_piece, y_move) {
                    true => return MoveValidity::Invalid,
                    false => return MoveValidity::Valid,
                }
            } else if self.last_piece_moved.piece_type == PieceType::Pawn
                && self.last_piece_moved.position.y == 4
                && x_abs == 1
                && y_move == 1
            {
                if self.last_piece_moved.position
                    == Position::new(attacked_piece.position.x, attacked_piece.position.y - 1)
                {
                    self[Position::new(attacked_piece.position.x, 4)] = Piece::new(
                        PieceType::None,
                        Position::new(attacked_piece.position.x, 4),
                        false,
                    );
                    self.do_move(moved_piece, attacked_piece);

                    if self.in_check(self.white_turn) {
                        self.undo_move();
                        return MoveValidity::Invalid;
                    }
                    self.last_piece_moved = self[attacked_piece.position];
                    return MoveValidity::EnPassant;
                }
            }
        }
        //ROOK
        if moved_piece.piece_type == PieceType::Rook {
            if moved_piece.position.x == attacked_piece.position.x {
                match self.is_jumping_vertically(moved_piece, y_move) {
                    true => return MoveValidity::Invalid,
                    false => return MoveValidity::Valid,
                }
            } else if moved_piece.position.y == attacked_piece.position.y {
                match self.is_jumping_horizontally(moved_piece, x_move) {
                    true => return MoveValidity::Invalid,
                    false => return MoveValidity::Valid,
                }
            }
        }
        //KNIGHT
        if moved_piece.piece_type == PieceType::Knight
            && ((x_abs == 2 && y_abs == 1) || (x_abs == 1 && y_abs == 2))
        {
            return MoveValidity::Valid;
        }
        //BISHOP
        if moved_piece.piece_type == PieceType::Bishop {
            if (x_move + y_move) == 0i8 {
                match self.is_jumping_diagonally_pos(moved_piece, x_move) {
                    true => return MoveValidity::Invalid,
                    false => return MoveValidity::Valid,
                }
            } else if (x_move - y_move) == 0i8 {
                match self.is_jumping_diagonally_neg(moved_piece, x_move) {
                    true => return MoveValidity::Invalid,
                    false => return MoveValidity::Valid,
                }
            }
        }
        //QUEEN
        if moved_piece.piece_type == PieceType::Queen {
            if moved_piece.position.x == attacked_piece.position.x {
                match self.is_jumping_vertically(moved_piece, y_move) {
                    true => return MoveValidity::Invalid,
                    false => return MoveValidity::Valid,
                }
            } else if moved_piece.position.y == attacked_piece.position.y {
                match self.is_jumping_horizontally(moved_piece, x_move) {
                    true => return MoveValidity::Invalid,
                    false => return MoveValidity::Valid,
                }
            } else if (x_move + y_move) == 0i8 {
                match self.is_jumping_diagonally_pos(moved_piece, x_move) {
                    true => return MoveValidity::Invalid,
                    false => return MoveValidity::Valid,
                }
            } else if (x_move - y_move) == 0i8 {
                match self.is_jumping_diagonally_neg(moved_piece, x_move) {
                    true => return MoveValidity::Invalid,
                    false => return MoveValidity::Valid,
                }
            }
        }
        //WHITE KING
        if moved_piece.piece_type == PieceType::King && moved_piece.is_white {
            if (x_abs == 0 && y_abs == 1)
                || (x_abs == 1 && y_abs == 0)
                || (x_abs == 1 && y_abs == 1)
            {
                return MoveValidity::Valid;
            } else if x_move == 2
                && y_move == 0
                && !moved_piece.has_moved
                && self[Position::new(7, 7)].piece_type == PieceType::Rook
                && !self[Position::new(7, 7)].has_moved
                && !self.is_jumping_horizontally(moved_piece, x_move)
                && !self.in_check(true)
            {
                match self.short_castling_checks(true) {
                    true => return MoveValidity::Invalid,
                    false => return MoveValidity::ShortCastle,
                }
            } else if x_move == -3
                && y_move == 0
                && !moved_piece.has_moved
                && self[Position::new(0, 7)].piece_type == PieceType::Rook
                && !self[Position::new(0, 7)].has_moved
                && !self.is_jumping_horizontally(moved_piece, x_move)
                && !self.in_check(true)
            {
                match self.long_castling_checks(true) {
                    true => return MoveValidity::Invalid,
                    false => return MoveValidity::LongCastle,
                }
            }
        }
        //BLACK KING
        if moved_piece.piece_type == PieceType::King && !moved_piece.is_white {
            if (x_abs == 0 && y_abs == 1)
                || (x_abs == 1 && y_abs == 0)
                || (x_abs == 1 && y_abs == 1)
            {
                return MoveValidity::Valid;
            } else if x_move == 2
                && y_move == 0
                && !moved_piece.has_moved
                && self[Position::new(7, 0)].piece_type == PieceType::Rook
                && !self[Position::new(7, 0)].has_moved
                && !self.is_jumping_horizontally(moved_piece, x_move)
                && !self.in_check(false)
            {
                match self.short_castling_checks(false) {
                    true => return MoveValidity::Invalid,
                    false => return MoveValidity::ShortCastle,
                }
            } else if x_move == -3
                && y_move == 0
                && !moved_piece.has_moved
                && self[Position::new(0, 0)].piece_type == PieceType::Rook
                && !self[Position::new(0, 0)].has_moved
                && !self.is_jumping_horizontally(moved_piece, x_move)
                && !self.in_check(false)
            {
                match self.long_castling_checks(false) {
                    true => return MoveValidity::Invalid,
                    false => return MoveValidity::LongCastle,
                }
            }
        }
        MoveValidity::Invalid
    }
    ///Attempts to castle short, and moves the pieces if successful.
    fn short_castling_checks(&mut self, is_white: bool) -> bool {
        if is_white {
            for x in 4..6 {
                self.do_move(
                    Piece::new(PieceType::King, Position::new(x, 7), true),
                    Piece::new(PieceType::None, Position::new(x + 1, 7), true),
                );
                if self.in_check(true) {
                    self.undo_move();
                    return true;
                }
            }
            self.do_move(
                Piece::new(PieceType::Rook, Position::new(7, 7), true),
                Piece::new(PieceType::None, Position::new(5, 7), true),
            );
            self.last_piece_moved = self[Position::new(6, 7)];
        } else {
            for x in 4..6 {
                self.do_move(
                    Piece::new(PieceType::King, Position::new(x, 0), false),
                    Piece::new(PieceType::None, Position::new(x + 1, 0), false),
                );
                if self.in_check(false) {
                    self.undo_move();
                    return true;
                }
            }
            self.do_move(
                Piece::new(PieceType::Rook, Position::new(7, 0), false),
                Piece::new(PieceType::None, Position::new(5, 0), false),
            );
            self.last_piece_moved = self[Position::new(6, 0)];
        }
        self.white_turn = !self.white_turn;
        false
    }
    ///Attempts to castle long, and moves the pieces if successful.
    fn long_castling_checks(&mut self, is_white: bool) -> bool {
        if is_white {
            for x in (2..5).rev() {
                self.do_move(
                    Piece::new(PieceType::King, Position::new(x, 7), true),
                    Piece::new(PieceType::None, Position::new(x - 1, 7), true),
                );
                if self.in_check(true) {
                    self.undo_move();
                    return true;
                }
            }
            self.do_move(
                Piece::new(PieceType::Rook, Position::new(0, 7), true),
                Piece::new(PieceType::None, Position::new(2, 7), true),
            );
            self.last_piece_moved = self[Position::new(1, 7)];
        } else {
            for x in (2..5).rev() {
                self.do_move(
                    Piece::new(PieceType::King, Position::new(x, 0), false),
                    Piece::new(PieceType::None, Position::new(x - 1, 0), false),
                );
                if self.in_check(false) {
                    self.undo_move();
                    return true;
                }
            }
            self.do_move(
                Piece::new(PieceType::Rook, Position::new(0, 0), false),
                Piece::new(PieceType::None, Position::new(2, 0), false),
            );
            self.last_piece_moved = self[Position::new(1, 0)];
        }
        self.white_turn = !self.white_turn;
        false
    }
    ///Checks if a specified move jumps over another piece horizontally.
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
    ///Checks if a specified move jumps over another piece vertically.
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
    ///Checks if a specified move jumps over another piece diagonally with a positive slope.
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
    ///Checks if a specified move jumps over another piece diagonally with a negative slope.
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
    ///Checks if the given side is in check.
    fn in_check(&mut self, king_is_white: bool) -> bool {
        let attacking_pieces: Vec<Piece> = self.get_all_color_pieces(!king_is_white);
        let defending_king: Piece = self.get_king(king_is_white);
        for piece in attacking_pieces {
            if self.is_move_valid(piece, defending_king) == MoveValidity::Valid {
                return true;
            }
        }
        false
    }
    ///Returns all pieces of a specified color/side.
    fn get_all_color_pieces(&self, white: bool) -> Vec<Piece> {
        self.pieces
            .iter()
            .filter(|piece| piece.is_white == white && piece.piece_type != PieceType::None)
            .cloned()
            .collect()
    }
    ///Returns the king of the specified color/side.
    fn get_king(&self, king_is_white: bool) -> Piece {
        *self
            .pieces
            .iter()
            .find(|piece| piece.is_white == king_is_white && piece.piece_type == PieceType::King)
            .unwrap()
    }
    ///Checks if the side in check is in checkmate.
    fn is_checkmate(&mut self, white_in_check: bool) -> bool {
        let defending_pieces: Vec<Piece> = self.get_all_color_pieces(white_in_check);
        for col in 0u8..8u8 {
            for row in 0u8..8u8 {
                let attacked_piece = self[Position::new(row, col)];
                for moved_piece in &defending_pieces {
                    if self.is_move_valid(*moved_piece, attacked_piece) == MoveValidity::Valid
                        && !(self.white_turn == attacked_piece.is_white
                            && attacked_piece.piece_type != PieceType::None)
                    {
                        self.do_move(*moved_piece, attacked_piece);
                        match self.in_check(white_in_check) {
                            true => {
                                self.undo_move();
                                continue;
                            }
                            false => {
                                self.undo_move();
                                return false;
                            }
                        }
                    }
                }
            }
        }
        true
    }
    ///Returns the piece notation for the played move.
    fn make_piece_notation(
        moved_piece: Piece,
        attacked_piece: Piece,
        piece_captured: bool,
        en_passant: bool,
        check_checkmate: String,
    ) -> String {
        let mut piece: String = match moved_piece.piece_type {
            PieceType::None | PieceType::Pawn => "".to_string(),
            PieceType::Knight => "N".to_string(),
            PieceType::Bishop => "B".to_string(),
            PieceType::Rook => "R".to_string(),
            PieceType::Queen => "Q".to_string(),
            PieceType::King => "K".to_string(),
        };
        if moved_piece.piece_type == PieceType::Pawn && piece_captured {
            piece = ((moved_piece.position.x + 97) as char).to_string()
        }
        format!(
            "{}{}{}{}{}{}",
            piece,
            match piece_captured {
                true => "x",
                false => "",
            },
            (attacked_piece.position.x + 97) as char,
            8 - attacked_piece.position.y,
            match en_passant {
                true => " e.p.",
                false => "",
            },
            check_checkmate,
        )
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
