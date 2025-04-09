use std::ops::Not;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Color {
    White,
    Black,
}

impl Not for Color {
    type Output = Self;
    fn not(self) -> Self::Output {
        if self == Self::White {
            return Self::Black;
        }
        Self::White
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

impl Position {
    ///Returns a position with the given x and y values.
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub position: Position,
    pub color: Color,
    pub has_moved: bool,
}

impl Piece {
    ///Creates a new piece.
    pub fn new(piece_type: PieceType, position: Position, color: Color) -> Self {
        Self {
            piece_type,
            position,
            color,
            has_moved: false,
        }
    }
}

///Returns the starting positions of the pieces.
pub fn starting_pieces() -> [Piece; 64] {
    [
        Piece::new(PieceType::Rook, Position::new(0, 0), Color::Black),
        Piece::new(PieceType::Knight, Position::new(1, 0), Color::Black),
        Piece::new(PieceType::Bishop, Position::new(2, 0), Color::Black),
        Piece::new(PieceType::Queen, Position::new(3, 0), Color::Black),
        Piece::new(PieceType::King, Position::new(4, 0), Color::Black),
        Piece::new(PieceType::Bishop, Position::new(5, 0), Color::Black),
        Piece::new(PieceType::Knight, Position::new(6, 0), Color::Black),
        Piece::new(PieceType::Rook, Position::new(7, 0), Color::Black),
        Piece::new(PieceType::Pawn, Position::new(0, 1), Color::Black),
        Piece::new(PieceType::Pawn, Position::new(1, 1), Color::Black),
        Piece::new(PieceType::Pawn, Position::new(2, 1), Color::Black),
        Piece::new(PieceType::Pawn, Position::new(3, 1), Color::Black),
        Piece::new(PieceType::Pawn, Position::new(4, 1), Color::Black),
        Piece::new(PieceType::Pawn, Position::new(5, 1), Color::Black),
        Piece::new(PieceType::Pawn, Position::new(6, 1), Color::Black),
        Piece::new(PieceType::Pawn, Position::new(7, 1), Color::Black),
        Piece::new(PieceType::None, Position::new(0, 2), Color::White),
        Piece::new(PieceType::None, Position::new(1, 2), Color::White),
        Piece::new(PieceType::None, Position::new(2, 2), Color::White),
        Piece::new(PieceType::None, Position::new(3, 2), Color::White),
        Piece::new(PieceType::None, Position::new(4, 2), Color::White),
        Piece::new(PieceType::None, Position::new(5, 2), Color::White),
        Piece::new(PieceType::None, Position::new(6, 2), Color::White),
        Piece::new(PieceType::None, Position::new(7, 2), Color::White),
        Piece::new(PieceType::None, Position::new(0, 3), Color::White),
        Piece::new(PieceType::None, Position::new(1, 3), Color::White),
        Piece::new(PieceType::None, Position::new(2, 3), Color::White),
        Piece::new(PieceType::None, Position::new(3, 3), Color::White),
        Piece::new(PieceType::None, Position::new(4, 3), Color::White),
        Piece::new(PieceType::None, Position::new(5, 3), Color::White),
        Piece::new(PieceType::None, Position::new(6, 3), Color::White),
        Piece::new(PieceType::None, Position::new(7, 3), Color::White),
        Piece::new(PieceType::None, Position::new(0, 4), Color::White),
        Piece::new(PieceType::None, Position::new(1, 4), Color::White),
        Piece::new(PieceType::None, Position::new(2, 4), Color::White),
        Piece::new(PieceType::None, Position::new(3, 4), Color::White),
        Piece::new(PieceType::None, Position::new(4, 4), Color::White),
        Piece::new(PieceType::None, Position::new(5, 4), Color::White),
        Piece::new(PieceType::None, Position::new(6, 4), Color::White),
        Piece::new(PieceType::None, Position::new(7, 4), Color::White),
        Piece::new(PieceType::None, Position::new(0, 5), Color::White),
        Piece::new(PieceType::None, Position::new(1, 5), Color::White),
        Piece::new(PieceType::None, Position::new(2, 5), Color::White),
        Piece::new(PieceType::None, Position::new(3, 5), Color::White),
        Piece::new(PieceType::None, Position::new(4, 5), Color::White),
        Piece::new(PieceType::None, Position::new(5, 5), Color::White),
        Piece::new(PieceType::None, Position::new(6, 5), Color::White),
        Piece::new(PieceType::None, Position::new(7, 5), Color::White),
        Piece::new(PieceType::Pawn, Position::new(0, 6), Color::White),
        Piece::new(PieceType::Pawn, Position::new(1, 6), Color::White),
        Piece::new(PieceType::Pawn, Position::new(2, 6), Color::White),
        Piece::new(PieceType::Pawn, Position::new(3, 6), Color::White),
        Piece::new(PieceType::Pawn, Position::new(4, 6), Color::White),
        Piece::new(PieceType::Pawn, Position::new(5, 6), Color::White),
        Piece::new(PieceType::Pawn, Position::new(6, 6), Color::White),
        Piece::new(PieceType::Pawn, Position::new(7, 6), Color::White),
        Piece::new(PieceType::Rook, Position::new(0, 7), Color::White),
        Piece::new(PieceType::Knight, Position::new(1, 7), Color::White),
        Piece::new(PieceType::Bishop, Position::new(2, 7), Color::White),
        Piece::new(PieceType::Queen, Position::new(3, 7), Color::White),
        Piece::new(PieceType::King, Position::new(4, 7), Color::White),
        Piece::new(PieceType::Bishop, Position::new(5, 7), Color::White),
        Piece::new(PieceType::Knight, Position::new(6, 7), Color::White),
        Piece::new(PieceType::Rook, Position::new(7, 7), Color::White),
    ]
}
