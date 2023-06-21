pub mod board;

use crate::board::Board;

fn main() {
    let mut board = Board::new();
    board.print_board();
    board.pieces[0].move_piece(8, 16).expect("something went wrong with mvoing");
    board.print_board();
}
