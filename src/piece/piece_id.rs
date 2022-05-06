use super::{Piece, sign::Sign};

pub struct PieceId<T>(pub T, pub Sign, pub usize);

impl<P: Piece> From<(isize, usize)> for PieceId<P> {
    fn from((i, version): (isize, usize)) -> Self {
        PieceId(i.into(), i.signum().into(), version)
    }
}

impl<P: Piece> From<PieceId<P>> for (isize, usize) {
    fn from(piece_id: PieceId<P>) -> Self {
        (&piece_id).into()
    }
}

impl<P: Piece> From<&PieceId<P>> for (isize, usize) {
    fn from(piece_id: &PieceId<P>) -> Self {
        (piece_id.i(), piece_id.version())
    }
}

impl<T> PieceId<T>
where
    T: Piece,
{
    pub fn is_none(&self) -> bool {
        self.0 == T::none()
    }

    pub fn piece(&self) -> T {
        self.0
    }

    pub fn sign(&self) -> Sign {
        self.1
    }

    pub fn version(&self) -> usize {
        self.2
    }

    pub fn i(&self) -> isize {
        self.0.into() * self.1
    }

    pub fn matches(&self, other: &Self) -> bool {
        self.sign() == other.sign()
    }

    pub fn opposes(&self, other: &Self) -> bool {
        self.sign() == -other.sign()
    }
}

impl<P: Piece> Default for PieceId<P> {
    fn default() -> Self {
        PieceId(P::none(), Sign::None, 0)
    }
}
