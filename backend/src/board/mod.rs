pub mod piece;

use crate::board::piece::*;
use crate::board::piece::PieceColor::*;

const N_PIECE_TYPES: usize = 12;
const BOARD_WIDTH: usize = 8;
const BOARD_HEIGHT: usize = 8;
const BOARD_SIZE: usize = BOARD_WIDTH * BOARD_HEIGHT;

// a1 is lsb, then b2 ...
// h8 is msb

#[derive(Debug)]
pub struct Board {
    pub pieces: [Piece; N_PIECE_TYPES],
}

impl Board {
    pub fn new() -> Board {
        Board {
            pieces: [

                //white pieces
                Piece::Pawn(0b1111_1111 << (BOARD_WIDTH * 1), White),
                Piece::Knight(0b0100_0010, White),
                Piece::Bishop(0b0010_0100, White),
                Piece::Rook(0b1000_0001, White),
                Piece::Queen(0b0001_0000, White),
                Piece::King(0b0000_1000, White),

                // black pieces
                Piece::Pawn(0b1111_1111 << (BOARD_WIDTH * 6), Black),
                Piece::Knight(0b0100_0010 << (BOARD_WIDTH * 7), Black),
                Piece::Bishop(0b0010_0100 << (BOARD_WIDTH  * 7), Black),
                Piece::Rook(0b1000_0001 << (BOARD_WIDTH* 7), Black),
                Piece::Queen(0b0001_0000 << (BOARD_WIDTH * 7), Black),
                Piece::King(0b0000_1000 << (BOARD_WIDTH * 7), Black),
            ],
        }
    }

    pub fn print_board(&self) {
        let mut board_string: Vec<String> = vec![String::from("--"); BOARD_SIZE];
        for piece in &self.pieces {
            for i in 0..64 {
                let result = piece.at(i);
                if result.0 {
                    let p = piece.print_piece(); 
                    let c = result.1.print_color();
                    let v = [c, p].join("");
                    board_string[i] = v;
                }
            }
        }

        for i in 0..64 {if i % 8 == 0 { println!("")}
            print!(" {} ", board_string[i]);
            
        }
        println!("");
    }


}

