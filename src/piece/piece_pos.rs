use super::sign::Sign;
use crate::board::Board;

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
