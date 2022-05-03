use std::collections::HashMap;

struct Move(isize, isize, usize);

impl Move {
    fn calculate(
        &self,
        piece_id: &PieceId,
        piece_pos: &PiecePos,
        board: &Board,
        can_capture: &CanCapture,
    ) -> Vec<usize> {
        let mut calculated = Vec::new();
        let (px, py): (isize, isize) = piece_pos.into();
        let (x, y, max) = (self.0, self.1, self.2);
        let mut mx = px as isize + x;
        let mut my = py as isize + y;
        let mut iters: usize = 0;
        let mut captured: usize = 0;

        while iters < max && PiecePos::is_inbounds(mx, my) {
            let from_xy = PiecePos::from((mx, my));
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

#[derive(Clone, Copy)]
enum CanCapture<'a> {
    None,
    Matching(usize),
    Opposing(usize),
    Specific(&'a dyn Fn(&PieceId, &PieceId, &mut usize) -> bool),
    All,
}

impl<'a> CanCapture<'a> {
    fn check(&self, id: &PieceId, other: &PieceId, captured: &mut usize) -> bool {
        match self {
            CanCapture::None => other.piece() == Piece::None,
            CanCapture::Matching(max) => {
                other.piece() == Piece::None
                    || (*captured < *max && (*captured += 1) == () && id.matches(other))
            }
            CanCapture::Opposing(max) => {
                other.piece() == Piece::None
                    || (*captured < *max && (*captured += 1) == () && id.opposes(other))
            }
            CanCapture::Specific(s) => s(id, other, captured),
            CanCapture::All => true,
        }
    }
}

enum CanMove<'a> {
    Free(Move, CanCapture<'a>),
    Conditional(&'a dyn Fn(&PieceId, &Board, &History) -> Option<(Move, CanCapture<'a>)>),
}

const PAWN_MOVESET: [CanMove; 6] = [
    CanMove::Free(Move(0, 1, 1), CanCapture::None),
    CanMove::Conditional(&|id, _, history| {
        if history.get_slice(id).is_none() {
            return Some((Move(0, 1, 2), CanCapture::None));
        }
        return None;
    }),
    CanMove::Conditional(&|id, board, _| {
        board.get_pos(id).and_then(|op| {
            let idx = op.topleft(id.sign());
            board.get_id(&idx).and_then(|other| {
                if other.opposes(id) {
                    return Some((Move(-1, 1, 1), CanCapture::Opposing(1)));
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
                    return Some((Move(1, 1, 1), CanCapture::Opposing(1)));
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
                        prev.0.last().and_then(|last| {
                            if last == &op_left.bottom(other.sign()).bottom(other.sign()).u() {
                                return Some((Move(-1, 1, 1), CanCapture::None));
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
                        prev.0.last().and_then(|last| {
                            if last == &op_right.bottom(other.sign()).bottom(other.sign()).u() {
                                return Some((Move(1, 1, 1), CanCapture::None));
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
const KNIGHT_MOVESET: [CanMove; 8] = [
    CanMove::Free(Move(1, 2, 1), CanCapture::Opposing(1)),
    CanMove::Free(Move(-1, 2, 1), CanCapture::Opposing(1)),
    CanMove::Free(Move(1, -2, 1), CanCapture::Opposing(1)),
    CanMove::Free(Move(-1, -2, 1), CanCapture::Opposing(1)),
    CanMove::Free(Move(2, 1, 1), CanCapture::Opposing(1)),
    CanMove::Free(Move(-2, 1, 1), CanCapture::Opposing(1)),
    CanMove::Free(Move(2, -1, 1), CanCapture::Opposing(1)),
    CanMove::Free(Move(-2, -1, 1), CanCapture::Opposing(1)),
];
const BISHOP_MOVESET: [CanMove; 4] = [
    CanMove::Free(Move(1, 1, BOARD_SIZE), CanCapture::Opposing(1)),
    CanMove::Free(Move(-1, 1, BOARD_SIZE), CanCapture::Opposing(1)),
    CanMove::Free(Move(1, -1, BOARD_SIZE), CanCapture::Opposing(1)),
    CanMove::Free(Move(-1, -1, BOARD_SIZE), CanCapture::Opposing(1)),
];
const ROOK_MOVESET: [CanMove; 6] = [
    CanMove::Free(Move(1, 0, BOARD_SIZE), CanCapture::Opposing(1)),
    CanMove::Free(Move(-1, 0, BOARD_SIZE), CanCapture::Opposing(1)),
    CanMove::Free(Move(0, 1, BOARD_SIZE), CanCapture::Opposing(1)),
    CanMove::Free(Move(0, -1, BOARD_SIZE), CanCapture::Opposing(1)),
    CanMove::Conditional(&|id, board, history| {
        board.get_pos(id).and_then(|op| {
            let idx1 = op.left(id.sign());
            let idx2 = idx1.left(id.sign());
            let idx3 = idx2.left(id.sign());
            let idx4 = idx3.left(id.sign());
            let other_match = |distance: isize| {
                move |other: PieceId| {
                    if other.matches(id)
                        && other.piece() == Piece::King
                        && history.get_slice(id).is_none()
                        && history.get_slice(&other).is_none()
                    {
                        return Some((Move(distance, 0, 1), CanCapture::None));
                    }
                    return None;
                }
            };
            if board.get_id_not_none(&idx1).is_none() && board.get_id_not_none(&idx2).is_none() {
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
                move |other: PieceId| {
                    if other.matches(id)
                        && other.piece() == Piece::King
                        && history.get_slice(id).is_none()
                        && history.get_slice(&other).is_none()
                    {
                        return Some((Move(distance, 0, 1), CanCapture::None));
                    }
                    return None;
                }
            };
            if board.get_id_not_none(&idx1).is_none() && board.get_id_not_none(&idx2).is_none() {
                return board
                    .get_id_not_none(&idx3)
                    .and_then(other_match(2))
                    .or_else(|| board.get_id_not_none(&idx4).and_then(other_match(3)));
            }
            return None;
        })
    }),
];
const QUEEN_MOVESET: [CanMove; 8] = [
    CanMove::Free(Move(1, 0, BOARD_SIZE), CanCapture::Opposing(1)),
    CanMove::Free(Move(-1, 0, BOARD_SIZE), CanCapture::Opposing(1)),
    CanMove::Free(Move(0, 1, BOARD_SIZE), CanCapture::Opposing(1)),
    CanMove::Free(Move(0, -1, BOARD_SIZE), CanCapture::Opposing(1)),
    CanMove::Free(Move(1, 1, BOARD_SIZE), CanCapture::Opposing(1)),
    CanMove::Free(Move(-1, 1, BOARD_SIZE), CanCapture::Opposing(1)),
    CanMove::Free(Move(1, -1, BOARD_SIZE), CanCapture::Opposing(1)),
    CanMove::Free(Move(-1, -1, BOARD_SIZE), CanCapture::Opposing(1)),
];
const KING_MOVESET: [CanMove; 10] = [
    CanMove::Free(Move(1, 0, 1), CanCapture::Opposing(1)),
    CanMove::Free(Move(-1, 0, 1), CanCapture::Opposing(1)),
    CanMove::Free(Move(0, 1, 1), CanCapture::Opposing(1)),
    CanMove::Free(Move(0, -1, 1), CanCapture::Opposing(1)),
    CanMove::Free(Move(1, 1, 1), CanCapture::Opposing(1)),
    CanMove::Free(Move(-1, 1, 1), CanCapture::Opposing(1)),
    CanMove::Free(Move(1, -1, 1), CanCapture::Opposing(1)),
    CanMove::Free(Move(-1, -1, 1), CanCapture::Opposing(1)),
    CanMove::Conditional(&|id, board, history| {
        board.get_pos(id).and_then(|op| {
            let idx1 = op.left(id.sign());
            let idx2 = idx1.left(id.sign());
            let idx3 = idx2.left(id.sign());
            let idx4 = idx3.left(id.sign());
            let other_match = |other: PieceId| {
                if other.matches(id)
                    && other.piece() == Piece::Rook
                    && history.get_slice(id).is_none()
                    && history.get_slice(&other).is_none()
                {
                    return Some((Move(-2, 0, 1), CanCapture::None));
                }
                return None;
            };
            if board.get_id_not_none(&idx1).is_none() && board.get_id_not_none(&idx2).is_none() {
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
            let other_match = |other: PieceId| {
                if other.matches(id)
                    && other.piece() == Piece::Rook
                    && history.get_slice(id).is_none()
                    && history.get_slice(&other).is_none()
                {
                    return Some((Move(2, 0, 1), CanCapture::None));
                }
                return None;
            };
            if board.get_id_not_none(&idx1).is_none() && board.get_id_not_none(&idx2).is_none() {
                return board
                    .get_id_not_none(&idx3)
                    .and_then(other_match)
                    .or_else(|| board.get_id_not_none(&idx4).and_then(other_match));
            }
            return None;
        })
    }),
];

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

#[derive(PartialEq, PartialOrd, Clone, Copy)]
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

impl From<usize> for Piece {
    fn from(u: usize) -> Self {
        match u {
            0 => Self::None,
            1 => Self::Pawn,
            2 => Self::Knight,
            3 => Self::Bishop,
            4 => Self::Rook,
            5 => Self::Queen,
            6 => Self::King,
            _ => panic!("unknown piece {u}"),
        }
    }
}

impl Piece {
    fn moveset(&self) -> &[CanMove<'static>] {
        let moveset = match self {
            Piece::None => &[],
            Piece::Pawn => &PAWN_MOVESET[..],
            Piece::Knight => &KNIGHT_MOVESET[..],
            Piece::Bishop => &BISHOP_MOVESET[..],
            Piece::Rook => &ROOK_MOVESET[..],
            Piece::Queen => &QUEEN_MOVESET[..],
            Piece::King => &KING_MOVESET[..],
        };
        return moveset;
    }
}

#[derive(Default)]
struct PieceId(Piece, Sign, usize);

impl PieceId {
    fn fromi(i: isize, version: usize) -> Self {
        PieceId(i.into(), i.signum().into(), version)
    }

    fn piece(&self) -> Piece {
        self.0
    }

    fn sign(&self) -> Sign {
        self.1
    }

    fn version(&self) -> usize {
        self.2
    }

    fn i(&self) -> isize {
        self.0 as isize * self.1
    }

    fn matches(&self, other: &PieceId) -> bool {
        self.sign() == other.sign()
    }

    fn opposes(&self, other: &PieceId) -> bool {
        self.sign() == -other.sign()
    }

    fn valid_moves(&self, board: &Board, history: &History) -> Option<Vec<usize>> {
        let mut valid = Vec::new();
        let pos = board.get_pos(self)?;
        for can_move in self.piece().moveset() {
            let mut move_op = match can_move {
                CanMove::Free(m, c) => m.calculate(self, &pos, board, c),
                CanMove::Conditional(c) => match c(self, board, history) {
                    Some((m, c)) => m.calculate(self, &pos, board, &c),
                    None => Vec::new(),
                },
            };
            valid.append(&mut move_op);
        }
        Some(valid)
    }

    fn valid_slice(&self, board: &Board, history: &History) -> BoardSlice {
        BoardSlice::new(self.valid_moves(board, history))
    }
}

#[derive(Debug)]
struct PiecePos(usize);

impl From<(isize, isize)> for PiecePos {
    fn from((x, y): (isize, isize)) -> Self {
        PiecePos(x as usize + y as usize * ROW_SIZE)
    }
}

impl From<PiecePos> for (isize, isize) {
    fn from(pos: PiecePos) -> Self {
        ((pos.0 % ROW_SIZE) as isize, (pos.0 / ROW_SIZE) as isize)
    }
}

impl From<&PiecePos> for (isize, isize) {
    fn from(pos: &PiecePos) -> Self {
        ((pos.0 % ROW_SIZE) as isize, (pos.0 / ROW_SIZE) as isize)
    }
}

impl PiecePos {
    fn u(&self) -> usize {
        self.0
    }

    fn is_inbounds(x: isize, y: isize) -> bool {
        return x >= 0 && x < ROW_SIZE as isize && y >= 0 && y < COL_SIZE as isize;
    }

    fn top(&self, sign: Sign) -> Self {
        PiecePos((self.0 as isize + (sign * ROW_SIZE as isize)) as usize)
    }

    fn bottom(&self, sign: Sign) -> Self {
        PiecePos((self.0 as isize - (sign * ROW_SIZE as isize)) as usize)
    }

    fn left(&self, sign: Sign) -> Self {
        PiecePos((self.0 as isize - sign) as usize)
    }

    fn right(&self, sign: Sign) -> Self {
        PiecePos((self.0 as isize + sign) as usize)
    }

    fn topleft(&self, sign: Sign) -> Self {
        PiecePos((self.0 as isize + (sign * ROW_SIZE as isize) - 1) as usize)
    }

    fn topright(&self, sign: Sign) -> Self {
        PiecePos((self.0 as isize + (sign * ROW_SIZE as isize) + 1) as usize)
    }

    fn bottomleft(&self, sign: Sign) -> Self {
        PiecePos((self.0 as isize - (sign * ROW_SIZE as isize) - 1) as usize)
    }

    fn bottomright(&self, sign: Sign) -> Self {
        PiecePos((self.0 as isize - (sign * ROW_SIZE as isize) + 1) as usize)
    }

    fn add(&self, sign: Sign, other: &PiecePos) -> Self {
        PiecePos((self.0 as isize + sign * other.0 as isize) as usize)
    }
}

const ROW_SIZE: usize = 8;
const COL_SIZE: usize = 8;
const BOARD_SIZE: usize = ROW_SIZE * COL_SIZE;

#[derive(Debug, Clone)]
struct Board([isize; BOARD_SIZE], HashMap<isize, Vec<usize>>);

impl Board {
    fn new(initial_state: [isize; BOARD_SIZE]) -> Self {
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
        Board(initial_state, repeats)
    }

    fn row(&self, row: usize) -> [isize; ROW_SIZE] {
        assert!(row < ROW_SIZE, "the board only has {ROW_SIZE} rows");
        self.0[row * ROW_SIZE..row * ROW_SIZE + COL_SIZE]
            .try_into()
            .expect("unexpected slice length")
    }

    fn col(&self, col: usize) -> [isize; COL_SIZE] {
        assert!(col < COL_SIZE, "the board only has {COL_SIZE} columns");
        let mut array = [0; COL_SIZE];
        for u in 0..COL_SIZE {
            array[u] = self.0[col + u * ROW_SIZE];
        }
        array
    }

    fn piece_slice(&self, ids: &[PieceId]) -> BoardSlice {
        BoardSlice(
            (0..BOARD_SIZE - 1)
                .collect::<Vec<usize>>()
                .into_iter()
                .filter(|v| ids.iter().any(|id| self.0[*v] == id.i()))
                .collect::<Vec<_>>(),
        )
    }

    fn get_id(&self, pos: &PiecePos) -> Option<PieceId> {
        let u = pos.u();
        if u > BOARD_SIZE {
            return None;
        }
        let id = self.0[u];
        self.1
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

    fn get_id_not_none(&self, pos: &PiecePos) -> Option<PieceId> {
        match self.get_id(pos) {
            Some(id) => {
                if id.piece() != Piece::None {
                    return Some(id);
                }
                return None;
            }
            None => None,
        }
    }

    fn get_pos(&self, id: &PieceId) -> Option<PiecePos> {
        self.1
            .get(&id.i())
            .and_then(|versions| {
                versions.get(id.version()).and_then(|specific| {
                    return Some(PiecePos(*specific));
                })
            })
            .or_else(|| {
                for (idx, v) in self.0.iter().enumerate() {
                    if v == &id.i() {
                        return Some(PiecePos(idx));
                    }
                }
                return None;
            })
    }

    fn set_square(&mut self, id: &PieceId, pos: &PiecePos) {
        if self.1.contains_key(&id.i()) {
            self.1.get_mut(&id.i()).unwrap()[id.version()] = pos.u();
        }
        self.0[pos.u()] = id.i();
    }

    fn clear(&mut self) {
        self.0 = [0; BOARD_SIZE];
        self.1 = HashMap::new();
    }
}

#[derive(Debug)]
struct History(HashMap<(isize, usize), BoardSlice>);

impl History {
    fn new(default: Option<HashMap<(isize, usize), BoardSlice>>) -> Self {
        match default {
            Some(n) => History(n),
            None => History(HashMap::new()),
        }
    }

    fn get_slice(&self, id: &PieceId) -> Option<&BoardSlice> {
        self.0.get(&(id.i(), id.version()))
    }

    fn push(&mut self, id: &PieceId, pos: &PiecePos) {
        let old_slice = self.0.get(&(id.i(), id.version()));
        match old_slice {
            Some(slice) => {
                let mut new_slice = BoardSlice::new(Some(slice.0.to_vec()));
                new_slice.push(pos.u());
                self.0.insert((id.i(), id.version()), new_slice);
            }
            None => {
                let new_slice = BoardSlice::new(Some(vec![pos.u()]));
                self.0.insert((id.i(), id.version()), new_slice);
            }
        }
    }

    fn clear(&mut self) {
        self.0 = HashMap::new();
    }
}

#[derive(Debug)]
struct BoardSlice(Vec<usize>);

impl BoardSlice {
    fn new(default: Option<Vec<usize>>) -> Self {
        match default {
            Some(n) => BoardSlice(n),
            None => BoardSlice(Vec::new()),
        }
    }

    fn push(&mut self, pos: usize) {
        self.0.push(pos);
    }

    fn visualize(&self, fill: isize) -> Board {
        let mut visual = Board::new([0; BOARD_SIZE]);
        for v in &self.0 {
            visual.0[*v] = fill;
        }
        return visual;
    }
}

struct Game {
    board: Board,
    history: History,
}

impl Game {
    fn new() -> Self {
        Game {
            board: Board::new([
                4, 2, 3, 5, 6, 3, 2, 4, //
                1, 1, 1, 1, 1, 1, 1, 1, //
                0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, //
                -1, -1, -1, -1, -1, -1, -1, -1, //
                -4, -2, -3, -5, -6, -3, -2, -4, //
            ]),
            history: History::new(None),
        }
    }

    fn move_piece(&mut self, id: &PieceId, pos: &PiecePos) {
        let existing_piece = self.board.get_pos(id);
        if existing_piece.is_some() {
            let old_pos = existing_piece.unwrap();
            self.history.push(id, pos);
            self.board.set_square(&PieceId::default(), &old_pos);
            self.board.set_square(id, pos);
        }
    }

    fn move_piece_relative(&mut self, id: &PieceId, pos: &PiecePos) {
        let existing_piece = self.board.get_pos(id);
        if existing_piece.is_some() {
            let old_pos = existing_piece.unwrap();
            let relative_pos = old_pos.add(id.sign(), pos);
            self.history.push(id, &relative_pos);
            self.board.set_square(&PieceId::default(), &old_pos);
            self.board.set_square(id, &relative_pos);
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

    fn visualize_moves(&self, id: &PieceId) {
        let mirror = id.valid_slice(&self.board, &self.history).visualize(id.i());
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
    let mut game = Game::new();
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
    game.move_piece_relative(my_pawn, &PiecePos(16));
    game.print();
    println!("");
    game.visualize_moves(my_pawn);
    println!("");
    game.move_piece_relative(my_pawn, &PiecePos(8));
    game.print();
    println!("");
    game.visualize_moves(my_pawn);
    println!("");
    game.move_piece_relative(my_pawn, &PiecePos(8));
    game.print();
    println!("");
    game.visualize_moves(my_pawn);
    println!("");
    game.move_piece_relative(my_pawn, &PiecePos(7));
    game.print();
    println!("");
    game.visualize_moves(my_knight);
    println!("");
    game.move_piece_relative(my_knight, &PiecePos(15));
    game.print();
    println!("");
    game.visualize_moves(my_bishop);
    println!("");
    game.move_piece_relative(my_bishop, &PiecePos(27));
    game.print();
    println!("");
    game.visualize_moves(my_queen);
    game.move_piece_relative(my_queen, &PiecePos(8));
    println!("");
    game.print();
    println!("");
    game.visualize_moves(my_king);
    println!("");
    game.move_piece(my_king, &PiecePos(2));
    game.move_piece(my_rook, &PiecePos(3));
    game.print();
    println!("");
    game.visualize_moves(my_rook);
    game.move_piece_relative(my_rook, &PiecePos(1));
    println!("");
    game.print();
    println!("");
    game.clear();
    game.print();
}
