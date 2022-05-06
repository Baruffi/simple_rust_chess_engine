pub mod piece_id;
pub mod piece_pos;
pub mod piece_set;
pub mod sign;

pub trait Piece: Copy + std::convert::From<isize> + std::convert::Into<isize> + PartialEq {
    fn none() -> Self;
}
