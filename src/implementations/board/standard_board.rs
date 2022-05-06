use crate::{
    board::{Board, board_slice::BoardSlice},
    piece::{Piece, piece_id::PieceId, piece_pos::PiecePos},
};
use std::{collections::HashMap, marker::PhantomData};

pub struct StandardBoard<
    const T_ROW_SIZE: usize,
    const T_COL_SIZE: usize,
    const T_BOARD_SIZE: usize,
    P: Piece,
> {
    state: [isize; T_BOARD_SIZE],
    repeats: HashMap<isize, Vec<usize>>,
    __: PhantomData<P>,
}

impl<const T_ROW_SIZE: usize, const T_COL_SIZE: usize, const T_BOARD_SIZE: usize, P: Piece>
    StandardBoard<T_ROW_SIZE, T_COL_SIZE, T_BOARD_SIZE, P>
{
    pub fn new(initial_state: [isize; T_BOARD_SIZE]) -> Self {
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

    pub fn row(&self, row: usize) -> [isize; T_ROW_SIZE] {
        assert!(row < T_ROW_SIZE, "the board only has {} rows", T_ROW_SIZE);
        self.state[row * T_ROW_SIZE..row * T_ROW_SIZE + T_COL_SIZE]
            .try_into()
            .expect("unexpected slice length")
    }

    pub fn col(&self, col: usize) -> [isize; T_COL_SIZE] {
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

impl<const T_ROW_SIZE: usize, const T_COL_SIZE: usize, const T_BOARD_SIZE: usize, P: Piece> Board
    for StandardBoard<T_ROW_SIZE, T_COL_SIZE, T_BOARD_SIZE, P>
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
                        return Some(PieceId::from((id, version)));
                    }
                }
                return None;
            })
            .or(Some(PieceId::from((id, 0))))
    }

    fn get_id_not_none(&self, pos: &PiecePos<P>) -> Option<PieceId<P>> {
        match self.get_id(pos) {
            Some(id) => {
                if !id.is_none() {
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
