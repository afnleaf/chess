
#[derive(Debug)]
pub enum PieceColor {
    Black,
    White,
}

impl PieceColor {
    pub fn print_color(&self) -> &'static str {
        match *self {
            Self::Black => "B",
            Self::White => "W",

        }
    }
}

#[derive(Debug)]
#[repr(usize)]
pub enum Piece {
    Pawn(u64, PieceColor),
    Knight(u64, PieceColor),
    Bishop(u64, PieceColor),
    Rook(u64, PieceColor),
    Queen(u64, PieceColor),
    King(u64, PieceColor),
}

impl Piece {
    pub fn move_piece(&mut self, dest: u8) -> Result<(), String> {
        
        match self {
            Piece::Pawn(data, c) => {
                println!("im a pawn");
                *data = *data << dest;
            },
            _ => ()
        }

        Ok(())
    }
    
    pub fn at(&self, index: usize) -> (bool, &PieceColor) {
        let mask = 0b1 << index;

        if let Piece::Pawn(d, c) | Piece::Knight(d, c) | 
                Piece::Bishop(d, c) | Piece::Rook(d, c) | 
                Piece::Queen(d, c) | Piece::King(d, c) = self {
                    return (if d & mask != 0 { true } else {false}, c)
            
        }
        unreachable!()
    }

    pub fn get_data(&self) -> (u64, &PieceColor) {
        if let Piece::Pawn(d, c) | Piece::Knight(d, c) | 
                Piece::Bishop(d, c) | Piece::Rook(d, c) | 
                Piece::Queen(d, c) | Piece::King(d, c) = self {
            (*d, c)
        } else {
            unreachable!()
        }
    }

    pub fn print_piece(&self) -> &str {
        match self {
            Piece::Pawn(data, c) => "P",
            Piece::Knight(data, c) => "N",
            Piece::Bishop(data, c) => "B",
            Piece::Rook(data, c) => "R",
            Piece::Queen(data, c) => "Q",
            Piece::King(data, c) => "K",
        }
    }
}
