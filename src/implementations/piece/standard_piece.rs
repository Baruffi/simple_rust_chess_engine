use crate::piece::Piece;

#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub enum StandardPiece {
    None,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl From<StandardPiece> for isize {
    fn from(i: StandardPiece) -> Self {
        match i {
            StandardPiece::None => 0,
            StandardPiece::Pawn => 1,
            StandardPiece::Knight => 2,
            StandardPiece::Bishop => 3,
            StandardPiece::Rook => 4,
            StandardPiece::Queen => 5,
            StandardPiece::King => 6,
        }
    }
}

impl From<isize> for StandardPiece {
    fn from(i: isize) -> Self {
        match i.abs() {
            0 => Self::None,
            1 => Self::Pawn,
            2 => Self::Knight,
            3 => Self::Bishop,
            4 => Self::Rook,
            5 => Self::Queen,
            6 => Self::King,
            _ => panic!("unknown piece {}", i.abs()),
        }
    }
}

impl Piece for StandardPiece {
    fn none() -> Self {
        Self::None
    }
}
