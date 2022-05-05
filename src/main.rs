use std::{collections::HashMap, marker::PhantomData};

struct MoveStep {
    x: isize,
    y: isize,
}

struct Move {
    step: MoveStep,
    max_steps: usize,
}

impl Move {
    const fn new(step_x: isize, step_y: isize, max_steps: usize) -> Self {
        Move {
            step: MoveStep {
                x: step_x,
                y: step_y,
            },
            max_steps,
        }
    }

    fn calculate<P: IsizeLike>(
        &self,
        piece_id: &PieceId<P>,
        piece_pos: &PiecePos<P>,
        can_capture: &CanCapture<P>,
        board: &dyn Board<PieceType = P>,
    ) -> Vec<usize> {
        let mut calculated = Vec::new();
        let (px, py): (isize, isize) = piece_pos.into();
        let MoveStep { x, y } = self.step;
        let mut mx = px as isize + x;
        let mut my = py as isize + y;
        let mut iters: usize = 0;
        let mut captured: usize = 0;

        while iters < self.max_steps && PiecePos::is_inbounds(mx, my, board) {
            let from_xy = PiecePos::from((mx, my, board));
            match board.get_id(&from_xy) {
                Some(p) => {
                    if !can_capture.check(piece_id, &p, &mut captured) {
                        break;
                    }
                }
                None => (),
            };
            calculated.push(from_xy.u());

            mx += x;
            my += y;
            iters += 1;
        }
        calculated
    }
}

enum CanCapture<'a, P> {
    None,
    Matching(usize),
    Opposing(usize),
    Specific(&'a dyn Fn(&PieceId<P>, &PieceId<P>, &mut usize) -> bool),
    All,
}

impl<'a, P: IsizeLike> CanCapture<'a, P> {
    fn check(&self, id: &PieceId<P>, other: &PieceId<P>, captured: &mut usize) -> bool {
        match self {
            CanCapture::None => other.piece() == P::none(),
            CanCapture::Matching(max) => {
                other.piece() == P::none()
                    || (*captured < *max && (*captured += 1) == () && id.matches(other))
            }
            CanCapture::Opposing(max) => {
                other.piece() == P::none()
                    || (*captured < *max && (*captured += 1) == () && id.opposes(other))
            }
            CanCapture::Specific(s) => s(id, other, captured),
            CanCapture::All => true,
        }
    }
}

enum CanMove<'a, P> {
    Free(Move, CanCapture<'a, P>),
    Conditional(
        &'a dyn Fn(
            &PieceId<P>,
            &dyn Board<PieceType = P>,
            &History,
        ) -> Option<(Move, CanCapture<'a, P>)>,
    ),
}

#[derive(PartialEq, PartialOrd, Clone, Copy)]
enum Sign {
    None,
    Positive,
    Negative = -1,
}

impl Default for Sign {
    fn default() -> Self {
        Sign::None
    }
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

trait PieceSet<'a> {
    type PieceType;
    fn moveset(&self, piece: &Self::PieceType) -> Option<&[CanMove<'a, Self::PieceType>]>;
    fn valid_slice(
        &self,
        piece_id: &PieceId<Self::PieceType>,
        board: &dyn Board<PieceType = Self::PieceType>,
        history: &History,
    ) -> BoardSlice;
}

#[derive(PartialEq, PartialOrd, Clone, Copy, Eq, Hash)]
enum Piece {
    None,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl Default for Piece {
    fn default() -> Self {
        Piece::None
    }
}

impl IsizeLike for Piece {
    fn none() -> Self {
        Self::None
    }
}

impl From<Piece> for isize {
    fn from(i: Piece) -> Self {
        match i {
            Piece::None => 0,
            Piece::Pawn => 1,
            Piece::Knight => 2,
            Piece::Bishop => 3,
            Piece::Rook => 4,
            Piece::Queen => 5,
            Piece::King => 6,
        }
    }
}

impl From<isize> for Piece {
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

struct StaticPieceSet;

impl StaticPieceSet {
    const PAWN_MOVESET: [CanMove<'static, Piece>; 6] = [
        CanMove::Free(Move::new(0, 1, 1), CanCapture::None),
        CanMove::Conditional(&|id, _, history| {
            if history.get_slice(id).is_none() {
                return Some((Move::new(0, 1, 2), CanCapture::None));
            }
            return None;
        }),
        CanMove::Conditional(&|id, board, _| {
            board.get_pos(id).and_then(|op| {
                let idx = op.topleft(id.sign());
                board.get_id(&idx).and_then(|other| {
                    if other.opposes(id) {
                        return Some((Move::new(-1, 1, 1), CanCapture::Opposing(1)));
                    }
                    return None;
                })
            })
        }),
        CanMove::Conditional(&|id, board, _| {
            board.get_pos(id).and_then(|op| {
                let idx = op.topright(id.sign());
                board.get_id(&idx).and_then(|other| {
                    if other.opposes(id) {
                        return Some((Move::new(1, 1, 1), CanCapture::Opposing(1)));
                    }
                    return None;
                })
            })
        }),
        CanMove::Conditional(&|id, board, history| {
            board.get_pos(id).and_then(|op| {
                let op_left = op.left(id.sign());
                board.get_id(&op_left).and_then(|other| {
                    if other.opposes(id) && other.piece() == Piece::Pawn {
                        return history.get_slice(&other).and_then(|prev| {
                            prev.inner().last().and_then(|last| {
                                if last == &op_left.bottom(other.sign()).bottom(other.sign()).u() {
                                    return Some((Move::new(-1, 1, 1), CanCapture::None));
                                } else {
                                    return None;
                                }
                            })
                        });
                    }
                    return None;
                })
            })
        }),
        CanMove::Conditional(&|id, board, history| {
            board.get_pos(id).and_then(|op| {
                let op_right = op.right(id.sign());
                board.get_id(&op_right).and_then(|other| {
                    if other.opposes(id) && other.piece() == Piece::Pawn {
                        return history.get_slice(&other).and_then(|prev| {
                            prev.inner().last().and_then(|last| {
                                if last == &op_right.bottom(other.sign()).bottom(other.sign()).u() {
                                    return Some((Move::new(1, 1, 1), CanCapture::None));
                                } else {
                                    return None;
                                }
                            })
                        });
                    }
                    return None;
                })
            })
        }),
    ];
    const KNIGHT_MOVESET: [CanMove<'static, Piece>; 8] = [
        CanMove::Free(Move::new(1, 2, 1), CanCapture::Opposing(1)),
        CanMove::Free(Move::new(-1, 2, 1), CanCapture::Opposing(1)),
        CanMove::Free(Move::new(1, -2, 1), CanCapture::Opposing(1)),
        CanMove::Free(Move::new(-1, -2, 1), CanCapture::Opposing(1)),
        CanMove::Free(Move::new(2, 1, 1), CanCapture::Opposing(1)),
        CanMove::Free(Move::new(-2, 1, 1), CanCapture::Opposing(1)),
        CanMove::Free(Move::new(2, -1, 1), CanCapture::Opposing(1)),
        CanMove::Free(Move::new(-2, -1, 1), CanCapture::Opposing(1)),
    ];
    const BISHOP_MOVESET: [CanMove<'static, Piece>; 4] = [
        CanMove::Free(Move::new(1, 1, usize::MAX), CanCapture::Opposing(1)),
        CanMove::Free(Move::new(-1, 1, usize::MAX), CanCapture::Opposing(1)),
        CanMove::Free(Move::new(1, -1, usize::MAX), CanCapture::Opposing(1)),
        CanMove::Free(Move::new(-1, -1, usize::MAX), CanCapture::Opposing(1)),
    ];
    const ROOK_MOVESET: [CanMove<'static, Piece>; 6] = [
        CanMove::Free(Move::new(1, 0, usize::MAX), CanCapture::Opposing(1)),
        CanMove::Free(Move::new(-1, 0, usize::MAX), CanCapture::Opposing(1)),
        CanMove::Free(Move::new(0, 1, usize::MAX), CanCapture::Opposing(1)),
        CanMove::Free(Move::new(0, -1, usize::MAX), CanCapture::Opposing(1)),
        CanMove::Conditional(&|id, board, history| {
            board.get_pos(id).and_then(|op| {
                let idx1 = op.left(id.sign());
                let idx2 = idx1.left(id.sign());
                let idx3 = idx2.left(id.sign());
                let idx4 = idx3.left(id.sign());
                let other_match = |distance: isize| {
                    move |other: PieceId<Piece>| {
                        if other.matches(id)
                            && other.piece() == Piece::King
                            && history.get_slice(id).is_none()
                            && history.get_slice(&other).is_none()
                        {
                            return Some((Move::new(distance, 0, 1), CanCapture::None));
                        }
                        return None;
                    }
                };
                if board.get_id_not_none(&idx1).is_none() && board.get_id_not_none(&idx2).is_none()
                {
                    return board
                        .get_id_not_none(&idx3)
                        .and_then(other_match(-2))
                        .or_else(|| board.get_id_not_none(&idx4).and_then(other_match(-3)));
                }
                return None;
            })
        }),
        CanMove::Conditional(&|id, board, history| {
            board.get_pos(id).and_then(|op| {
                let idx1 = op.right(id.sign());
                let idx2 = idx1.right(id.sign());
                let idx3 = idx2.right(id.sign());
                let idx4 = idx3.right(id.sign());
                let other_match = |distance: isize| {
                    move |other: PieceId<Piece>| {
                        if other.matches(id)
                            && other.piece() == Piece::King
                            && history.get_slice(id).is_none()
                            && history.get_slice(&other).is_none()
                        {
                            return Some((Move::new(distance, 0, 1), CanCapture::None));
                        }
                        return None;
                    }
                };
                if board.get_id_not_none(&idx1).is_none() && board.get_id_not_none(&idx2).is_none()
                {
                    return board
                        .get_id_not_none(&idx3)
                        .and_then(other_match(2))
                        .or_else(|| board.get_id_not_none(&idx4).and_then(other_match(3)));
                }
                return None;
            })
        }),
    ];
    const QUEEN_MOVESET: [CanMove<'static, Piece>; 8] = [
        CanMove::Free(Move::new(1, 0, usize::MAX), CanCapture::Opposing(1)),
        CanMove::Free(Move::new(-1, 0, usize::MAX), CanCapture::Opposing(1)),
        CanMove::Free(Move::new(0, 1, usize::MAX), CanCapture::Opposing(1)),
        CanMove::Free(Move::new(0, -1, usize::MAX), CanCapture::Opposing(1)),
        CanMove::Free(Move::new(1, 1, usize::MAX), CanCapture::Opposing(1)),
        CanMove::Free(Move::new(-1, 1, usize::MAX), CanCapture::Opposing(1)),
        CanMove::Free(Move::new(1, -1, usize::MAX), CanCapture::Opposing(1)),
        CanMove::Free(Move::new(-1, -1, usize::MAX), CanCapture::Opposing(1)),
    ];
    const KING_MOVESET: [CanMove<'static, Piece>; 10] = [
        CanMove::Free(Move::new(1, 0, 1), CanCapture::Opposing(1)),
        CanMove::Free(Move::new(-1, 0, 1), CanCapture::Opposing(1)),
        CanMove::Free(Move::new(0, 1, 1), CanCapture::Opposing(1)),
        CanMove::Free(Move::new(0, -1, 1), CanCapture::Opposing(1)),
        CanMove::Free(Move::new(1, 1, 1), CanCapture::Opposing(1)),
        CanMove::Free(Move::new(-1, 1, 1), CanCapture::Opposing(1)),
        CanMove::Free(Move::new(1, -1, 1), CanCapture::Opposing(1)),
        CanMove::Free(Move::new(-1, -1, 1), CanCapture::Opposing(1)),
        CanMove::Conditional(&|id, board, history| {
            board.get_pos(id).and_then(|op| {
                let idx1 = op.left(id.sign());
                let idx2 = idx1.left(id.sign());
                let idx3 = idx2.left(id.sign());
                let idx4 = idx3.left(id.sign());
                let other_match = |other: PieceId<Piece>| {
                    if other.matches(id)
                        && other.piece() == Piece::Rook
                        && history.get_slice(id).is_none()
                        && history.get_slice(&other).is_none()
                    {
                        return Some((Move::new(-2, 0, 1), CanCapture::None));
                    }
                    return None;
                };
                if board.get_id_not_none(&idx1).is_none() && board.get_id_not_none(&idx2).is_none()
                {
                    return board
                        .get_id_not_none(&idx3)
                        .and_then(other_match)
                        .or_else(|| board.get_id_not_none(&idx4).and_then(other_match));
                }
                return None;
            })
        }),
        CanMove::Conditional(&|id, board, history| {
            board.get_pos(id).and_then(|op| {
                let idx1 = op.right(id.sign());
                let idx2 = idx1.right(id.sign());
                let idx3 = idx2.right(id.sign());
                let idx4 = idx3.right(id.sign());
                let other_match = |other: PieceId<Piece>| {
                    if other.matches(id)
                        && other.piece() == Piece::Rook
                        && history.get_slice(id).is_none()
                        && history.get_slice(&other).is_none()
                    {
                        return Some((Move::new(2, 0, 1), CanCapture::None));
                    }
                    return None;
                };
                if board.get_id_not_none(&idx1).is_none() && board.get_id_not_none(&idx2).is_none()
                {
                    return board
                        .get_id_not_none(&idx3)
                        .and_then(other_match)
                        .or_else(|| board.get_id_not_none(&idx4).and_then(other_match));
                }
                return None;
            })
        }),
    ];

    fn valid_moves(
        &self,
        piece_id: &PieceId<Piece>,
        board: &dyn Board<PieceType = Piece>,
        history: &History,
    ) -> Option<Vec<usize>> {
        let mut valid = Vec::new();
        let pos = board.get_pos(piece_id)?;
        let moveset = self.moveset(&piece_id.piece())?;
        for can_move in moveset {
            let mut move_op = match can_move {
                CanMove::Free(m, c) => m.calculate(piece_id, &pos, c, board),
                CanMove::Conditional(c) => match c(piece_id, board, history) {
                    Some((m, c)) => m.calculate(piece_id, &pos, &c, board),
                    None => Vec::new(),
                },
            };
            valid.append(&mut move_op);
        }
        Some(valid)
    }
}

impl PieceSet<'static> for StaticPieceSet {
    type PieceType = Piece;

    fn moveset(&self, piece: &Piece) -> Option<&[CanMove<'static, Self::PieceType>]> {
        let moveset = match piece {
            Piece::None => &[],
            Piece::Pawn => &Self::PAWN_MOVESET[..],
            Piece::Knight => &Self::KNIGHT_MOVESET[..],
            Piece::Bishop => &Self::BISHOP_MOVESET[..],
            Piece::Rook => &Self::ROOK_MOVESET[..],
            Piece::Queen => &Self::QUEEN_MOVESET[..],
            Piece::King => &Self::KING_MOVESET[..],
        };
        return Some(moveset);
    }

    fn valid_slice(
        &self,
        piece_id: &PieceId<Piece>,
        board: &dyn Board<PieceType = Piece>,
        history: &History,
    ) -> BoardSlice {
        BoardSlice::new(self.valid_moves(piece_id, board, history))
    }
}

#[derive(Default)]
struct DynamicPieceSet<'a, T>(HashMap<T, Vec<CanMove<'a, T>>>)
where
    T: std::hash::Hash + Eq;

impl<'a, T> PieceSet<'a> for DynamicPieceSet<'a, T>
where
    T: std::hash::Hash + Eq,
{
    type PieceType = T;

    fn moveset(&self, piece: &T) -> Option<&[CanMove<'a, Self::PieceType>]> {
        if let Some(moveset) = self.0.get(piece) {
            return Some(&moveset[..]);
        }
        return None;
    }

    fn valid_slice(
        &self,
        piece_id: &PieceId<T>,
        board: &dyn Board<PieceType = T>,
        history: &History,
    ) -> BoardSlice {
        BoardSlice::new(self.valid_moves(piece_id, board, history))
    }
}

impl<'a, T> DynamicPieceSet<'a, T>
where
    T: std::hash::Hash + Eq,
{
    fn insert(&mut self, piece: T, piece_move: CanMove<'a, T>) {
        if let Some(moveset) = self.0.get_mut(&piece) {
            moveset.push(piece_move);
        } else {
            self.0.insert(piece, vec![piece_move]);
        }
    }

    fn valid_moves(
        &self,
        piece_id: &PieceId<T>,
        board: &dyn Board<PieceType = T>,
        history: &History,
    ) -> Option<Vec<usize>> {
        todo!()
    }
}

trait IsizeLike:
    Copy + Default + std::convert::From<isize> + std::convert::Into<isize> + PartialEq
{
    fn none() -> Self;
}

trait PieceMask {
    type PieceType: IsizeLike;
    fn fromi(i: isize, version: usize) -> Self
    where
        Self: Sized;
    fn piece(&self) -> Self::PieceType;
    fn sign(&self) -> Sign;
    fn version(&self) -> usize;
    fn i(&self) -> isize;
    fn matches(&self, other: &Self) -> bool;
    fn opposes(&self, other: &Self) -> bool;
}

#[derive(Default)]
struct PieceId<T>(T, Sign, usize);

impl<T> PieceMask for PieceId<T>
where
    T: IsizeLike,
{
    type PieceType = T;

    fn fromi(i: isize, version: usize) -> Self {
        PieceId(i.into(), i.signum().into(), version)
    }

    fn piece(&self) -> Self::PieceType {
        self.0
    }

    fn sign(&self) -> Sign {
        self.1
    }

    fn version(&self) -> usize {
        self.2
    }

    fn i(&self) -> isize {
        self.0.into() * self.1
    }

    fn matches(&self, other: &Self) -> bool {
        self.sign() == other.sign()
    }

    fn opposes(&self, other: &Self) -> bool {
        self.sign() == -other.sign()
    }
}

struct PiecePos<'a, P>(usize, &'a dyn Board<PieceType = P>);

impl<'a, P> From<(isize, isize, &'a dyn Board<PieceType = P>)> for PiecePos<'a, P> {
    fn from((x, y, board): (isize, isize, &'a dyn Board<PieceType = P>)) -> Self {
        PiecePos(x as usize + y as usize * board.get_row_size(), board)
    }
}

impl<'a, P> From<PiecePos<'a, P>> for (isize, isize) {
    fn from(pos: PiecePos<'a, P>) -> Self {
        (
            (pos.0 % pos.1.get_row_size()) as isize,
            (pos.0 / pos.1.get_row_size()) as isize,
        )
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
    fn u(&self) -> usize {
        self.0
    }

    fn is_inbounds(x: isize, y: isize, board: &dyn Board<PieceType = P>) -> bool {
        return x >= 0
            && x < board.get_row_size() as isize
            && y >= 0
            && y < board.get_col_size() as isize;
    }

    fn top(&self, sign: Sign) -> Self {
        PiecePos(
            (self.0 as isize + (sign * self.1.get_row_size() as isize)) as usize,
            self.1,
        )
    }

    fn bottom(&self, sign: Sign) -> Self {
        PiecePos(
            (self.0 as isize - (sign * self.1.get_row_size() as isize)) as usize,
            self.1,
        )
    }

    fn left(&self, sign: Sign) -> Self {
        PiecePos((self.0 as isize - sign) as usize, self.1)
    }

    fn right(&self, sign: Sign) -> Self {
        PiecePos((self.0 as isize + sign) as usize, self.1)
    }

    fn topleft(&self, sign: Sign) -> Self {
        PiecePos(
            (self.0 as isize + (sign * self.1.get_row_size() as isize) - 1) as usize,
            self.1,
        )
    }

    fn topright(&self, sign: Sign) -> Self {
        PiecePos(
            (self.0 as isize + (sign * self.1.get_row_size() as isize) + 1) as usize,
            self.1,
        )
    }

    fn bottomleft(&self, sign: Sign) -> Self {
        PiecePos(
            (self.0 as isize - (sign * self.1.get_row_size() as isize) - 1) as usize,
            self.1,
        )
    }

    fn bottomright(&self, sign: Sign) -> Self {
        PiecePos(
            (self.0 as isize - (sign * self.1.get_row_size() as isize) + 1) as usize,
            self.1,
        )
    }

    fn offset(&self, sign: Sign, other: usize) -> Self {
        PiecePos((self.0 as isize + sign * other as isize) as usize, self.1)
    }
}

trait Board {
    type PieceType;
    fn get_row_size(&self) -> usize;
    fn get_col_size(&self) -> usize;
    fn get_board_size(&self) -> usize;
    fn piece_slice(&self, ids: &[PieceId<Self::PieceType>]) -> BoardSlice;
    fn get_id(&self, pos: &PiecePos<Self::PieceType>) -> Option<PieceId<Self::PieceType>>;
    fn get_id_not_none(&self, pos: &PiecePos<Self::PieceType>) -> Option<PieceId<Self::PieceType>>;
    fn get_pos(&self, id: &PieceId<Self::PieceType>) -> Option<PiecePos<Self::PieceType>>;
    fn set_square(&mut self, id: &PieceId<Self::PieceType>, square: usize);
    fn clear(&mut self);
}

struct StandardBoard<
    const T_ROW_SIZE: usize,
    const T_COL_SIZE: usize,
    const T_BOARD_SIZE: usize,
    P: IsizeLike,
> {
    state: [isize; T_BOARD_SIZE],
    repeats: HashMap<isize, Vec<usize>>,
    __: PhantomData<P>,
}

impl<const T_ROW_SIZE: usize, const T_COL_SIZE: usize, const T_BOARD_SIZE: usize, P: IsizeLike>
    StandardBoard<T_ROW_SIZE, T_COL_SIZE, T_BOARD_SIZE, P>
{
    fn new(initial_state: [isize; T_BOARD_SIZE]) -> Self {
        let mut repeats: HashMap<isize, Vec<usize>> = HashMap::new();
        let mut last_seen_pos: HashMap<isize, usize> = HashMap::new();
        for (pos, id) in initial_state.iter().enumerate() {
            if id == &0 {
                continue;
            }
            match last_seen_pos.insert(*id, pos) {
                Some(v) => {
                    if repeats.contains_key(id) {
                        let existing_vec = repeats.get_mut(id).unwrap();
                        existing_vec.push(pos);
                    } else {
                        repeats.insert(*id, vec![v, pos]);
                    }
                }
                None => (),
            }
        }
        StandardBoard {
            state: initial_state,
            repeats,
            __: PhantomData,
        }
    }

    fn row(&self, row: usize) -> [isize; T_ROW_SIZE] {
        assert!(row < T_ROW_SIZE, "the board only has {} rows", T_ROW_SIZE);
        self.state[row * T_ROW_SIZE..row * T_ROW_SIZE + T_COL_SIZE]
            .try_into()
            .expect("unexpected slice length")
    }

    fn col(&self, col: usize) -> [isize; T_COL_SIZE] {
        assert!(
            col < T_COL_SIZE,
            "the board only has {} columns",
            T_COL_SIZE
        );
        let mut array = [0; T_COL_SIZE];
        for u in 0..T_COL_SIZE {
            array[u] = self.state[col + u * T_ROW_SIZE];
        }
        array
    }
}

impl<const T_ROW_SIZE: usize, const T_COL_SIZE: usize, const T_BOARD_SIZE: usize, P: IsizeLike>
    Board for StandardBoard<T_ROW_SIZE, T_COL_SIZE, T_BOARD_SIZE, P>
{
    type PieceType = P;
    fn get_row_size(&self) -> usize {
        return T_ROW_SIZE;
    }

    fn get_col_size(&self) -> usize {
        return T_COL_SIZE;
    }

    fn get_board_size(&self) -> usize {
        return T_BOARD_SIZE;
    }

    fn piece_slice(&self, ids: &[PieceId<P>]) -> BoardSlice {
        BoardSlice(
            (0..T_BOARD_SIZE - 1)
                .collect::<Vec<usize>>()
                .into_iter()
                .filter(|v| ids.iter().any(|id| self.state[*v] == id.i()))
                .collect::<Vec<_>>(),
        )
    }

    fn get_id(&self, pos: &PiecePos<P>) -> Option<PieceId<P>> {
        let u = pos.u();
        if u > T_BOARD_SIZE {
            return None;
        }
        let id = self.state[u];
        self.repeats
            .get(&id)
            .and_then(|repeats| {
                for (version, existing_pos) in repeats.iter().enumerate() {
                    if &u == existing_pos {
                        return Some(PieceId::fromi(id, version));
                    }
                }
                return None;
            })
            .or(Some(PieceId::fromi(id, 0)))
    }

    fn get_id_not_none(&self, pos: &PiecePos<P>) -> Option<PieceId<P>> {
        match self.get_id(pos) {
            Some(id) => {
                if id.piece() != P::none() {
                    return Some(id);
                }
                return None;
            }
            None => None,
        }
    }

    fn get_pos(&self, id: &PieceId<P>) -> Option<PiecePos<P>> {
        self.repeats
            .get(&id.i())
            .and_then(|versions| {
                versions.get(id.version()).and_then(|specific| {
                    return Some(PiecePos(*specific, self));
                })
            })
            .or_else(|| {
                for (idx, v) in self.state.iter().enumerate() {
                    if v == &id.i() {
                        return Some(PiecePos(idx, self));
                    }
                }
                return None;
            })
    }

    fn set_square(&mut self, id: &PieceId<P>, square: usize) {
        if self.repeats.contains_key(&id.i()) {
            self.repeats.get_mut(&id.i()).unwrap()[id.version()] = square;
        }
        self.state[square] = id.i();
    }

    fn clear(&mut self) {
        self.state = [0; T_BOARD_SIZE];
        self.repeats = HashMap::new();
    }
}

struct History {
    past: HashMap<(isize, usize), BoardSlice>,
}

impl History {
    fn new(initial: Option<HashMap<(isize, usize), BoardSlice>>) -> Self {
        match initial {
            Some(past) => History { past },
            None => History {
                past: HashMap::new(),
            },
        }
    }

    fn get_slice<P: IsizeLike>(&self, id: &PieceId<P>) -> Option<&BoardSlice> {
        self.past.get(&(id.i(), id.version()))
    }

    fn push<P: IsizeLike>(&mut self, id: &PieceId<P>, pos: &PiecePos<P>) {
        let old_slice = self.past.get(&(id.i(), id.version()));
        match old_slice {
            Some(slice) => {
                let mut new_slice = BoardSlice::new(Some(slice.inner().to_vec()));
                new_slice.push(pos.u());
                self.past.insert((id.i(), id.version()), new_slice);
            }
            None => {
                let new_slice = BoardSlice::new(Some(vec![pos.u()]));
                self.past.insert((id.i(), id.version()), new_slice);
            }
        }
    }

    fn clear(&mut self) {
        self.past = HashMap::new();
    }
}

struct BoardSlice(Vec<usize>);

impl BoardSlice {
    fn new(default: Option<Vec<usize>>) -> Self {
        match default {
            Some(n) => BoardSlice(n),
            None => BoardSlice(Vec::new()),
        }
    }

    fn inner(&self) -> &Vec<usize> {
        &self.0
    }

    fn push(&mut self, pos: usize) {
        self.0.push(pos);
    }

    fn visualize<
        const T_ROW_SIZE: usize,
        const T_COL_SIZE: usize,
        const T_BOARD_SIZE: usize,
        P: IsizeLike,
    >(
        &self,
        fill: isize,
    ) -> StandardBoard<T_ROW_SIZE, T_COL_SIZE, T_BOARD_SIZE, P> {
        let mut visual = StandardBoard::new([0; T_BOARD_SIZE]);
        for v in &self.0 {
            visual.set_square(&PieceId(fill.into(), fill.into(), 0), *v);
        }
        return visual;
    }
}

struct Game<T, S>
where
    T: Board,
    S: PieceSet<'static>,
{
    board: T,
    history: History,
    piece_set: S,
}

impl<
        const T_ROW_SIZE: usize,
        const T_COL_SIZE: usize,
        const T_BOARD_SIZE: usize,
        P: IsizeLike,
        S: PieceSet<'static, PieceType = P>,
    > Game<StandardBoard<T_ROW_SIZE, T_COL_SIZE, T_BOARD_SIZE, P>, S>
{
    fn new(initial_state: [isize; T_BOARD_SIZE], piece_set: S) -> Self {
        Game {
            board: StandardBoard::new(initial_state),
            history: History::new(None),
            piece_set,
        }
    }

    fn move_piece(&mut self, id: &PieceId<P>, square: usize) {
        let pos = &PiecePos(square, &self.board);
        let existing_piece = self.board.get_pos(id);
        if existing_piece.is_some() {
            let old_pos = existing_piece.unwrap();
            let old_square = old_pos.u();
            self.history.push(id, pos);
            self.board.set_square(&PieceId::default(), old_square);
            self.board.set_square(id, square);
        }
    }

    fn move_piece_relative(&mut self, id: &PieceId<P>, distance: usize) {
        let existing_piece = self.board.get_pos(id);
        if existing_piece.is_some() {
            let old_pos = existing_piece.unwrap();
            let old_square = old_pos.u();
            let relative_pos = old_pos.offset(id.sign(), distance);
            let relative_square = relative_pos.u();
            self.history.push(id, &relative_pos);
            self.board.set_square(&PieceId::default(), old_square);
            self.board.set_square(id, relative_square);
        }
    }

    fn clear(&mut self) {
        self.board.clear();
        self.history.clear();
    }

    fn print(&self) {
        println!("{:?}", self.board.row(0));
        println!("{:?}", self.board.row(1));
        println!("{:?}", self.board.row(2));
        println!("{:?}", self.board.row(3));
        println!("{:?}", self.board.row(4));
        println!("{:?}", self.board.row(5));
        println!("{:?}", self.board.row(6));
        println!("{:?}", self.board.row(7));
    }

    fn visualize_moves(&self, id: &PieceId<P>) {
        let slice = self.piece_set.valid_slice(id, &self.board, &self.history);
        let mirror: StandardBoard<T_ROW_SIZE, T_COL_SIZE, T_BOARD_SIZE, P> =
            slice.visualize(id.i());
        println!("{:?}", mirror.row(0));
        println!("{:?}", mirror.row(1));
        println!("{:?}", mirror.row(2));
        println!("{:?}", mirror.row(3));
        println!("{:?}", mirror.row(4));
        println!("{:?}", mirror.row(5));
        println!("{:?}", mirror.row(6));
        println!("{:?}", mirror.row(7));
    }
}

fn main() {
    let mut game = Game::<StandardBoard<8, 8, 64, Piece>, StaticPieceSet>::new(
        [
            4, 2, 3, 5, 6, 3, 2, 4, //
            1, 1, 1, 1, 1, 1, 1, 1, //
            0, 0, 0, 0, 0, 0, 0, 0, //
            0, 0, 0, 0, 0, 0, 0, 0, //
            0, 0, 0, 0, 0, 0, 0, 0, //
            0, 0, 0, 0, 0, 0, 0, 0, //
            -1, -1, -1, -1, -1, -1, -1, -1, //
            -4, -2, -3, -5, -6, -3, -2, -4, //
        ],
        StaticPieceSet,
    );
    let my_pawn = &PieceId(Piece::Pawn, Sign::Positive, 3);
    let my_rook = &PieceId(Piece::Rook, Sign::Positive, 0);
    let my_knight = &PieceId(Piece::Knight, Sign::Positive, 0);
    let my_bishop = &PieceId(Piece::Bishop, Sign::Positive, 0);
    let my_queen = &PieceId(Piece::Queen, Sign::Positive, 0);
    let my_king = &PieceId(Piece::King, Sign::Positive, 0);

    game.print();
    println!("");
    game.visualize_moves(my_pawn);
    println!("");
    game.move_piece_relative(my_pawn, 16);
    game.print();
    println!("");
    game.visualize_moves(my_pawn);
    println!("");
    game.move_piece_relative(my_pawn, 8);
    game.print();
    println!("");
    game.visualize_moves(my_pawn);
    println!("");
    game.move_piece_relative(my_pawn, 8);
    game.print();
    println!("");
    game.visualize_moves(my_pawn);
    println!("");
    game.move_piece_relative(my_pawn, 7);
    game.print();
    println!("");
    game.visualize_moves(my_knight);
    println!("");
    game.move_piece_relative(my_knight, 15);
    game.print();
    println!("");
    game.visualize_moves(my_bishop);
    println!("");
    game.move_piece_relative(my_bishop, 27);
    game.print();
    println!("");
    game.visualize_moves(my_queen);
    game.move_piece_relative(my_queen, 8);
    println!("");
    game.print();
    println!("");
    game.visualize_moves(my_king);
    println!("");
    game.move_piece(my_king, 2);
    game.move_piece(my_rook, 3);
    game.print();
    println!("");
    game.visualize_moves(my_rook);
    game.move_piece_relative(my_rook, 1);
    println!("");
    game.print();
    println!("");
    game.clear();
    game.print();
}
