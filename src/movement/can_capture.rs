use crate::piece::{Piece, piece_id::PieceId};

pub enum CanCapture<'a, P> {
    None,
    Matching(usize),
    Opposing(usize),
    Specific(&'a dyn Fn(&PieceId<P>, &PieceId<P>, &mut usize) -> bool),
    All,
}

impl<'a, P: Piece> CanCapture<'a, P> {
    pub fn check(&self, id: &PieceId<P>, other: &PieceId<P>, captured: &mut usize) -> bool {
        match self {
            CanCapture::None => other.is_none(),
            CanCapture::Matching(max) => {
                other.is_none() || (*captured < *max && (*captured += 1) == () && id.matches(other))
            }
            CanCapture::Opposing(max) => {
                other.is_none() || (*captured < *max && (*captured += 1) == () && id.opposes(other))
            }
            CanCapture::Specific(s) => s(id, other, captured),
            CanCapture::All => true,
        }
    }
}
