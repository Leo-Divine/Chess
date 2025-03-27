use std::ops::{Index, IndexMut};

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

#[derive(Debug, Default, Clone, PartialEq, Copy)]
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
    pub fn move_piece(&mut self, old_position: Position, new_position: Position) -> Option<String> {
        let moved_piece: Pieces = self[old_position];
        let attacked_piece: Pieces = self[new_position];
        let mut piece_captured = false;
        let mut in_check = false;
        //Don't move a blank tile
        if moved_piece == Pieces::None {
            return None;
        }
        //Make sure a piece moved
        if old_position == new_position {
            return None;
        }
        //It better be your turn
        if (self.turn && BLACK_PIECES.contains(&moved_piece))
            || (!self.turn && WHITE_PIECES.contains(&moved_piece))
        {
            return None;
        }
        //Make sure the piece moved correctly
        if !self.is_move_valid(old_position, new_position) {
            return None;
        }
        //Don't capture your own pieces
        if (self.turn && WHITE_PIECES.contains(&attacked_piece))
            || (!self.turn && BLACK_PIECES.contains(&attacked_piece))
        {
            return None;
        }
        //Check if piece is captured
        if attacked_piece != Pieces::None {
            piece_captured = true;
        }
        //Don't put yourself in check idiot
        self[new_position] = moved_piece;
        self[old_position] = Pieces::None;
        if self.in_check(!self.turn) {
            self[old_position] = moved_piece;
            self[new_position] = attacked_piece;
            return None;
        }
        //Check for checks
        if self.in_check(self.turn) {
            in_check = true;
        }
        self.turn = !self.turn;

        let mut piece_notation: String = match moved_piece {
            Pieces::None | Pieces::WhitePawn | Pieces::BlackPawn => "".to_string(),
            Pieces::WhiteKnight | Pieces::BlackKnight => "N".to_string(),
            Pieces::WhiteBishop | Pieces::BlackBishop => "B".to_string(),
            Pieces::WhiteRook | Pieces::BlackRook => "R".to_string(),
            Pieces::WhiteQueen | Pieces::BlackQueen => "Q".to_string(),
            Pieces::WhiteKing | Pieces::BlackKing => "K".to_string(),
        };
        if (moved_piece == Pieces::WhitePawn || moved_piece == Pieces::BlackPawn) && piece_captured
        {
            piece_notation = ((old_position.y + 97) as u8 as char).to_string()
        }

        let mut check_checkmate = "";
        if in_check {
            check_checkmate = "+";
        }

        Some(format!(
            "{}{}{}{}{}",
            piece_notation,
            match piece_captured {
                true => "x",
                false => "",
            },
            (new_position.x + 97) as u8 as char,
            8 - new_position.y,
            check_checkmate
        ))
    }
    fn get_white_pieces_pos(&self) -> Vec<Position> {
        let mut positions: Vec<Position> = Vec::new();
        for (i, &piece) in self.items.iter().enumerate() {
            if WHITE_PIECES.contains(&piece) {
                positions.push(Position { x: i % 8, y: i / 8 });
            }
        }
        positions
    }
    fn get_black_pieces_pos(&self) -> Vec<Position> {
        let mut positions: Vec<Position> = Vec::new();
        for (i, &piece) in self.items.iter().enumerate() {
            if BLACK_PIECES.contains(&piece) {
                positions.push(Position { x: i % 8, y: i / 8 });
            }
        }
        positions
    }
    fn get_white_king_pos(&self) -> Position {
        let index = self
            .items
            .iter()
            .enumerate()
            .find(|x| *x.1 == Pieces::WhiteKing)
            .unwrap()
            .0;
        Position {
            x: index % 8,
            y: index / 8,
        }
    }
    fn get_black_king_pos(&self) -> Position {
        let index = self
            .items
            .iter()
            .enumerate()
            .find(|x| *x.1 == Pieces::BlackKing)
            .unwrap()
            .0;
        Position {
            x: index % 8,
            y: index / 8,
        }
    }
    fn is_move_valid(&self, old_position: Position, new_position: Position) -> bool {
        let piece: Pieces = self[old_position];
        let x_move: i8 = (new_position.x as i8) - (old_position.x as i8);
        let y_move: i8 = (new_position.y as i8) - (old_position.y as i8);
        let x_abs: i8 = x_move.abs();
        let y_abs: i8 = y_move.abs();

        //BLACK PAWN
        if (piece == Pieces::BlackPawn)
            && (y_move != 1 || x_move != 0 || self[new_position] != Pieces::None)
            && (old_position.y != 1
                || y_move != 2
                || x_move != 0
                || self[new_position] != Pieces::None)
        {
            if y_move == 1 && (x_move == -1 || x_move == 1) && self[new_position] != Pieces::None {
                return true;
            }
            return false;
        }
        //WHITE PAWN
        if (piece == Pieces::WhitePawn)
            && (y_move != -1 || x_move != 0 || self[new_position] != Pieces::None)
            && (old_position.y != 6
                || y_move != -2
                || x_move != 0
                || self[new_position] != Pieces::None)
        {
            if y_move == -1 && (x_move == -1 || x_move == 1) && self[new_position] != Pieces::None {
                return true;
            }
            return false;
        }
        //ROOK
        if (piece == Pieces::BlackRook || piece == Pieces::WhiteRook)
            && (old_position.x != new_position.x && old_position.y != new_position.y)
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
                && (old_position.x != new_position.x && old_position.y != new_position.y))
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
        if self.is_jumping_piece(old_position, x_move, y_move) {
            return false;
        }
        true
    }
    fn is_jumping_piece(&self, start_position: Position, x_move: i8, y_move: i8) -> bool {
        fn check_horizontally(grid: &Grid, piece_position: Position, x_move: i8) -> bool {
            for i in 1..x_move {
                let mut position: Position = piece_position;
                position.x = (position.x as i8 + i) as usize;
                if grid[position] != Pieces::None {
                    return true;
                }
            }
            for i in x_move..0 {
                if i == x_move {
                    continue;
                }
                let mut position: Position = piece_position;
                position.x = (position.x as i8 + i) as usize;
                if grid[position] != Pieces::None {
                    return true;
                }
            }
            false
        }
        fn check_vertically(grid: &Grid, piece_position: Position, y_move: i8) -> bool {
            for i in 1..y_move {
                let mut position: Position = piece_position;
                position.y = (position.y as i8 + i) as usize;
                if grid[position] != Pieces::None {
                    return true;
                }
            }
            for i in y_move..0 {
                if i == y_move {
                    continue;
                }
                let mut position: Position = piece_position;
                position.y = (position.y as i8 + i) as usize;
                if grid[position] != Pieces::None {
                    return true;
                }
            }
            false
        }
        fn check_diagonally_pos(grid: &Grid, piece_position: Position, x_move: i8) -> bool {
            for i in 1..x_move {
                let mut position: Position = piece_position;
                position.x = (position.x as i8 + i) as usize;
                position.y = (position.y as i8 - i) as usize;
                if grid[position] != Pieces::None {
                    return true;
                }
            }
            for i in x_move..0 {
                if i == x_move {
                    continue;
                }
                let mut position: Position = piece_position;
                position.x = (position.x as i8 + i) as usize;
                position.y = (position.y as i8 - i) as usize;
                if grid[position] != Pieces::None {
                    return true;
                }
            }
            false
        }
        fn check_diagonally_neg(grid: &Grid, piece_position: Position, x_move: i8) -> bool {
            for i in 1..x_move {
                let mut position: Position = piece_position;
                position.x = (position.x as i8 + i) as usize;
                position.y = (position.y as i8 + i) as usize;
                if grid[position] != Pieces::None {
                    return true;
                }
            }
            for i in x_move..0 {
                if i == x_move {
                    continue;
                }
                let mut position: Position = piece_position;
                position.x = (position.x as i8 + i) as usize;
                position.y = (position.y as i8 + i) as usize;
                if grid[position] != Pieces::None {
                    return true;
                }
            }
            false
        }
        let piece = self[start_position];
        //PAWN
        if piece == Pieces::BlackPawn || piece == Pieces::WhitePawn {
            return check_vertically(self, start_position, y_move);
        }
        //ROOK
        if piece == Pieces::BlackRook || piece == Pieces::WhiteRook {
            if x_move == 0 {
                return check_vertically(self, start_position, y_move);
            }
            return check_horizontally(self, start_position, x_move);
        }
        //BISHOP
        if piece == Pieces::BlackBishop || piece == Pieces::WhiteBishop {
            if (x_move + y_move) == 0i8 {
                return check_diagonally_pos(self, start_position, x_move);
            }
            return check_diagonally_neg(self, start_position, x_move);
        }
        //QUEEN
        if piece == Pieces::BlackQueen || piece == Pieces::WhiteQueen {
            if (x_move + y_move) == 0i8 {
                return check_diagonally_pos(self, start_position, x_move);
            } else if (x_move - y_move) == 0i8 {
                return check_diagonally_neg(self, start_position, x_move);
            } else if x_move == 0 {
                return check_vertically(self, start_position, y_move);
            }
            return check_horizontally(self, start_position, x_move);
        }
        //KING
        if piece == Pieces::BlackKing || piece == Pieces::WhiteKing {
            return check_horizontally(self, start_position, y_move);
        }
        false
    }
    fn in_check(&self, attacker: bool) -> bool {
        let attacking_pieces_positions: Vec<Position>;
        let defending_king_position: Position;
        if attacker {
            attacking_pieces_positions = self.get_white_pieces_pos();
            defending_king_position = self.get_black_king_pos();
        } else {
            attacking_pieces_positions = self.get_black_pieces_pos();
            defending_king_position = self.get_white_king_pos();
        }

        for piece_position in attacking_pieces_positions {
            if self.is_move_valid(piece_position, defending_king_position) {
                return true;
            }
        }
        false
    }
    pub fn get_turn(&self) -> String {
        if self.turn {
            "It's White's Turn!".to_string()
        } else {
            "It's Black's Turn!".to_string()
        }
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = Pieces;

    fn index(&self, (col, row): (usize, usize)) -> &Self::Output {
        &self.items[8 * row + col]
    }
}

impl Index<Position> for Grid {
    type Output = Pieces;

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
