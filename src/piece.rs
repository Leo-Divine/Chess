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
    pub is_white: bool,
    pub has_moved: bool,
}

impl Piece {
    ///Creates a new piece.
    pub fn new(piece_type: PieceType, position: Position, is_white: bool) -> Self {
        Self {
            piece_type,
            position,
            is_white,
            has_moved: false,
        }
    }
}

///Returns the starting positions of the pieces.
pub fn starting_pieces() -> [Piece; 64] {
    [
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
    ]
}
