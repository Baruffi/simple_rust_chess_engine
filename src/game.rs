use crate::{
    board::{board_history::BoardHistory, Board},
    implementations::board::standard_board::StandardBoard,
    piece::{piece_id::PieceId, piece_pos::PiecePos, piece_set::PieceSet, Piece},
};

pub struct Game<T, S>
where
    T: Board,
    S: PieceSet<'static>,
{
    board: T,
    history: BoardHistory,
    piece_set: S,
}

impl<
        const T_ROW_SIZE: usize,
        const T_COL_SIZE: usize,
        const T_BOARD_SIZE: usize,
        P: Piece,
        S: PieceSet<'static, PieceType = P>,
    > Game<StandardBoard<T_ROW_SIZE, T_COL_SIZE, T_BOARD_SIZE, P>, S>
{
    pub fn new(initial_state: [isize; T_BOARD_SIZE], piece_set: S) -> Self {
        Game {
            board: StandardBoard::new(initial_state),
            history: BoardHistory::new(None),
            piece_set,
        }
    }

    pub fn move_piece(&mut self, id: &PieceId<P>, square: usize) {
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

    pub fn move_piece_relative(&mut self, id: &PieceId<P>, distance: usize) {
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

    pub fn clear(&mut self) {
        self.board.clear();
        self.history.clear();
    }

    pub fn print(&self) {
        println!("{:?}", self.board.row(0));
        println!("{:?}", self.board.row(1));
        println!("{:?}", self.board.row(2));
        println!("{:?}", self.board.row(3));
        println!("{:?}", self.board.row(4));
        println!("{:?}", self.board.row(5));
        println!("{:?}", self.board.row(6));
        println!("{:?}", self.board.row(7));
    }

    pub fn visualize_moves(&self, id: &PieceId<P>) {
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
