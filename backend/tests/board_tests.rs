use backend::board::Board;

#[test]
fn board_init(){
    let board = Board::new();
    board.print_board();
    //panic!("");
    assert!(true);
}
