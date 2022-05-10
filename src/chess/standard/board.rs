use crate::chess::{
    board::{Board, BoardSlice},
    piece::{Piece, PieceId, PiecePos, Sign},
};
use std::{
    collections::{HashMap, HashSet},
    marker::PhantomData,
};

pub struct BoardHelper<const T_ROW_SIZE: usize, const T_COL_SIZE: usize, const T_BOARD_SIZE: usize>
{
    ids_per_position: [u64; T_BOARD_SIZE],
    positions_per_version: HashMap<isize, Vec<usize>>,
}

impl<const T_ROW_SIZE: usize, const T_COL_SIZE: usize, const T_BOARD_SIZE: usize>
    BoardHelper<T_ROW_SIZE, T_COL_SIZE, T_BOARD_SIZE>
{
    fn new<P: Piece>(initial_state: [isize; T_BOARD_SIZE]) -> Self {
        let mut ids_per_position = [0u64; T_BOARD_SIZE];
        let mut positions_per_version = HashMap::new();

        for (pos, i) in initial_state.iter().enumerate() {
            if i == &0 {
                continue;
            }
            positions_per_version
                .entry(*i)
                .or_insert(Vec::new())
                .push(pos);
            let piece_id =
                Self::convert_to_id::<P>(&PieceId::from((*i, positions_per_version[i].len())));
            ids_per_position[pos] = piece_id;
        }

        BoardHelper {
            ids_per_position,
            positions_per_version,
        }
    }

    fn row(&self, row: usize) -> [isize; T_ROW_SIZE] {
        assert!(
            row < T_ROW_SIZE,
            "the board only has {} rows. Received index: {}",
            T_ROW_SIZE,
            row
        );
        self.ids_per_position[row * T_ROW_SIZE..row * T_ROW_SIZE + T_COL_SIZE]
            .iter()
            .map(|id| Self::convert_to_i(*id))
            .collect::<Vec<isize>>()
            .try_into()
            .expect("unexpected slice length")
    }

    fn col(&self, col: usize) -> [isize; T_COL_SIZE] {
        assert!(
            col < T_COL_SIZE,
            "the board only has {} columns. Received index: {}",
            T_COL_SIZE,
            col,
        );
        let mut array = [0; T_COL_SIZE];
        for u in 0..T_COL_SIZE {
            array[u] = Self::convert_to_i(self.ids_per_position[col + u * T_ROW_SIZE]);
        }
        array
    }

    fn convert_to_id<P: Piece>(custom_identifier: &PieceId<P>) -> u64 {
        let (i, version) = (custom_identifier.i(), custom_identifier.version());
        (i as i32 as u64) << 32 | version as u32 as u64
    }

    fn convert_to_i(id: u64) -> isize {
        (id >> 32) as i32 as isize
    }

    fn convert_to_version(id: u64) -> usize {
        id as u32 as usize
    }

    fn convert_to_piece_id<P: Piece>(id: u64) -> PieceId<P> {
        PieceId::from((Self::convert_to_i(id), Self::convert_to_version(id)))
    }

    fn get_id(&self, pos: usize) -> u64 {
        self.ids_per_position[pos]
    }

    fn get_pos(&self, id: u64) -> usize {
        self.positions_per_version[&Self::convert_to_i(id)][Self::convert_to_version(id)]
    }

    fn set(&mut self, id: u64, pos: usize) {
        assert!(
            pos < T_BOARD_SIZE,
            "the board only has {} squares. Received index: {}",
            T_BOARD_SIZE,
            pos,
        );
        self.ids_per_position[pos] = id;
        self.positions_per_version
            .entry(Self::convert_to_i(id))
            .and_modify(|e| e[Self::convert_to_version(id)] = pos);
    }

    fn clear(&mut self) {
        self.ids_per_position = [0; T_BOARD_SIZE];
        self.positions_per_version = HashMap::new();
    }
}

pub struct StandardBoard<
    const T_ROW_SIZE: usize,
    const T_COL_SIZE: usize,
    const T_BOARD_SIZE: usize,
    P: Piece,
> {
    helper: BoardHelper<T_ROW_SIZE, T_COL_SIZE, T_BOARD_SIZE>,
    __: PhantomData<P>,
}

impl<const T_ROW_SIZE: usize, const T_COL_SIZE: usize, const T_BOARD_SIZE: usize, P: Piece>
    StandardBoard<T_ROW_SIZE, T_COL_SIZE, T_BOARD_SIZE, P>
{
    pub fn new(initial_state: [isize; T_BOARD_SIZE]) -> Self {
        StandardBoard {
            helper: BoardHelper::new::<P>(initial_state),
            __: PhantomData,
        }
    }

    pub fn row(&self, row: usize) -> [isize; T_ROW_SIZE] {
        self.helper.row(row)
    }

    pub fn col(&self, col: usize) -> [isize; T_COL_SIZE] {
        self.helper.col(col)
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

    fn get_all_pieces(&self) -> HashSet<Self::PieceType> {
        let mut all_ids = HashSet::new();
        for (i, _) in &self.helper.positions_per_version {
            all_ids.insert(P::from(*i));
        }
        all_ids
    }

    fn get_all_versions(&self, piece: P, sign: Sign) -> Vec<usize> {
        (0..self.helper.positions_per_version[&(piece.into() * sign)].len()).collect()
    }

    fn get_id(&self, pos: &PiecePos<P>) -> Option<PieceId<P>> {
        let u = pos.u();
        if u >= T_BOARD_SIZE {
            return None;
        }
        let id = self.helper.get_id(u);
        return Some(BoardHelper::<T_ROW_SIZE, T_COL_SIZE, T_BOARD_SIZE>::convert_to_piece_id(id));
    }

    fn get_pos(&self, id: &PieceId<P>) -> Option<PiecePos<P>> {
        if id.is_none() {
            return None;
        }
        let id = BoardHelper::<T_ROW_SIZE, T_COL_SIZE, T_BOARD_SIZE>::convert_to_id(id);
        return Some(PiecePos(self.helper.get_pos(id), self));
    }

    fn get_slice(&self) -> BoardSlice {
        BoardSlice(
            (0..T_BOARD_SIZE)
                .filter(|v| self.helper.ids_per_position[*v] != 0)
                .collect(),
        )
    }

    fn set_square(&mut self, id: &PieceId<P>, square: usize) {
        self.helper.set(
            BoardHelper::<T_ROW_SIZE, T_COL_SIZE, T_BOARD_SIZE>::convert_to_id(id),
            square,
        );
    }

    fn clear(&mut self) {
        self.helper.clear();
    }
}
