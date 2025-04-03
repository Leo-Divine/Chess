#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

impl Position {
    pub fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum PieceType {
    None,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy)]
pub struct Piece {
    pub piece_type: PieceType,
    pub position: Position,
    pub is_white: bool,
    pub has_moved: bool,
}

impl Piece {
    pub fn new(piece_type: PieceType, position: Position, is_white: bool) -> Self {
        Self {
            piece_type,
            position,
            is_white,
            has_moved: false,
        }
    }
}
