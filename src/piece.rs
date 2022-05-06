use crate::{
    board::{Board, BoardHistory, BoardSlice},
    movement::CanMove,
};

pub trait Piece: Copy + std::convert::From<isize> + std::convert::Into<isize> + PartialEq {
    fn none() -> Self;
}

pub trait PieceSet<'a> {
    type PieceType: Piece;
    fn moveset(&self, piece: &Self::PieceType) -> Option<&[CanMove<'a, Self::PieceType>]>;
    fn valid_slice(
        &self,
        piece_id: &PieceId<Self::PieceType>,
        board: &dyn Board<PieceType = Self::PieceType>,
        history: &BoardHistory,
    ) -> BoardSlice;
}

#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub enum Sign {
    None,
    Positive,
    Negative = -1,
}

impl From<isize> for Sign {
    fn from(i: isize) -> Self {
        match i.signum() {
            1 => Self::Positive,
            -1 => Self::Negative,
            _ => Self::None,
        }
    }
}

impl std::ops::Neg for Sign {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Sign::None => Sign::None,
            Sign::Positive => Sign::Negative,
            Sign::Negative => Sign::Positive,
        }
    }
}

impl std::ops::Mul<isize> for Sign {
    type Output = isize;

    fn mul(self, rhs: isize) -> Self::Output {
        match self {
            Sign::None => 0,
            Sign::Positive => rhs,
            Sign::Negative => -rhs,
        }
    }
}

impl std::ops::Add<isize> for Sign {
    type Output = isize;

    fn add(self, rhs: isize) -> Self::Output {
        match self {
            Sign::None => rhs,
            Sign::Positive => rhs + 1,
            Sign::Negative => rhs - 1,
        }
    }
}

impl std::ops::Sub<isize> for Sign {
    type Output = isize;

    fn sub(self, rhs: isize) -> Self::Output {
        match self {
            Sign::None => rhs,
            Sign::Positive => rhs - 1,
            Sign::Negative => rhs + 1,
        }
    }
}

impl std::ops::Mul<Sign> for isize {
    type Output = isize;

    fn mul(self, rhs: Sign) -> Self::Output {
        match rhs {
            Sign::None => 0,
            Sign::Positive => self,
            Sign::Negative => -self,
        }
    }
}

impl std::ops::Add<Sign> for isize {
    type Output = isize;

    fn add(self, rhs: Sign) -> Self::Output {
        match rhs {
            Sign::None => self,
            Sign::Positive => self + 1,
            Sign::Negative => self - 1,
        }
    }
}

impl std::ops::Sub<Sign> for isize {
    type Output = isize;

    fn sub(self, rhs: Sign) -> Self::Output {
        match rhs {
            Sign::None => self,
            Sign::Positive => self - 1,
            Sign::Negative => self + 1,
        }
    }
}

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

pub struct PiecePos<'a, P>(pub usize, pub &'a dyn Board<PieceType = P>);

impl<'a, P> From<(isize, isize, &'a dyn Board<PieceType = P>)> for PiecePos<'a, P> {
    fn from((x, y, board): (isize, isize, &'a dyn Board<PieceType = P>)) -> Self {
        PiecePos(x as usize + y as usize * board.get_row_size(), board)
    }
}

impl<'a, P> From<PiecePos<'a, P>> for (isize, isize) {
    fn from(pos: PiecePos<'a, P>) -> Self {
        (&pos).into()
    }
}

impl<'a, P> From<&PiecePos<'a, P>> for (isize, isize) {
    fn from(pos: &PiecePos<'a, P>) -> Self {
        (
            (pos.0 % pos.1.get_row_size()) as isize,
            (pos.0 / pos.1.get_row_size()) as isize,
        )
    }
}

impl<'a, P> PiecePos<'a, P> {
    pub fn u(&self) -> usize {
        self.0
    }

    pub fn is_inbounds(x: isize, y: isize, board: &dyn Board<PieceType = P>) -> bool {
        return x >= 0
            && x < board.get_row_size() as isize
            && y >= 0
            && y < board.get_col_size() as isize;
    }

    pub fn top(&self, sign: Sign) -> Self {
        PiecePos(
            (self.0 as isize + (sign * self.1.get_row_size() as isize)) as usize,
            self.1,
        )
    }

    pub fn bottom(&self, sign: Sign) -> Self {
        PiecePos(
            (self.0 as isize - (sign * self.1.get_row_size() as isize)) as usize,
            self.1,
        )
    }

    pub fn left(&self, sign: Sign) -> Self {
        PiecePos((self.0 as isize - sign) as usize, self.1)
    }

    pub fn right(&self, sign: Sign) -> Self {
        PiecePos((self.0 as isize + sign) as usize, self.1)
    }

    pub fn topleft(&self, sign: Sign) -> Self {
        PiecePos(
            (self.0 as isize + (sign * self.1.get_row_size() as isize) - 1) as usize,
            self.1,
        )
    }

    pub fn topright(&self, sign: Sign) -> Self {
        PiecePos(
            (self.0 as isize + (sign * self.1.get_row_size() as isize) + 1) as usize,
            self.1,
        )
    }

    pub fn bottomleft(&self, sign: Sign) -> Self {
        PiecePos(
            (self.0 as isize - (sign * self.1.get_row_size() as isize) - 1) as usize,
            self.1,
        )
    }

    pub fn bottomright(&self, sign: Sign) -> Self {
        PiecePos(
            (self.0 as isize - (sign * self.1.get_row_size() as isize) + 1) as usize,
            self.1,
        )
    }

    pub fn offset(&self, sign: Sign, other: usize) -> Self {
        PiecePos((self.0 as isize + sign * other as isize) as usize, self.1)
    }
}
