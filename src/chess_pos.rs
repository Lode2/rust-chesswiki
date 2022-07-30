// file containing bitboard and position struct

#[derive(PartialEq, Eq, PartialOrd, Clone, Copy, Debug, Default, Hash)]
pub struct BitBoard(pub u64);

// add methods to the BitBoard struct
impl BitBoard {
    pub fn _pretty(self) -> String {
        let BitBoard(pretty) = self;
        let mut pretty_board = String::new();

        for (i, x) in format!("{pretty:b}").bytes().enumerate() {
            if i % 8 == 0 {
                pretty_board += "\n";
                pretty_board += &(x as char).to_string();
                pretty_board += " ";
            } else {
                pretty_board += &(x as char).to_string();
                pretty_board += " ";
            }
        }
        return pretty_board;
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Position {
    /// Board for each side
    pub bb_sides: [BitBoard; 2],
    // BitBoards for all pieces and each side
    pub bb_pieces: [[BitBoard; 6]; 2],
}

impl Position {
    pub fn pretty(self) -> String {
        let mut pretty_array: [&str; 64] = ["x "; 64];

        // destructuring the position, white is the bitboard for white, wpawn the bitboard for white pawns etc.
        let Position {
            bb_sides: [_white, _black],
            bb_pieces:
                [[wpawn, wbishop, wknight, wrook, wqueen, wking], [bpawn, bbishop, bknight, brook, bqueen, bking]],
        } = self;

        // array of all pieces bitboards
        let all_pieces = [
            wpawn, wbishop, wknight, wrook, wqueen, wking, bpawn, bbishop, bknight, brook, bqueen,
            bking,
        ];

        let mut j: u8 = 0;

        // loop over all the pieces bitboards
        for i in all_pieces.iter() {
            // destructure bitboard to get the unsigned 64-bit integer
            let BitBoard(val) = i;

            // loop over a stringified u64, get bit position and bit value
            for k in format!("{val:b}").to_string().char_indices() {
                if k.1 == '1' {
                    let string_representation: &str = match j {
                        0 => "P ",   // wpawn
                        6 => "p ",   // bpawn
                        3 => "R ",   // wrook
                        9 => "r ",   // brook
                        1 => "B ",   // wbishop
                        7 => "b ",   // bbishop
                        2 => "N ",   // wknight
                        8 => "n ",   // bknight
                        5 => "K ",   // wking
                        11 => "k ",  // bking
                        4 => "Q ",   // wqueen
                        10 => "q ",  // bqueen
                        _ => "Err ", // in case of no match
                    };
                    pretty_array[k.0] = string_representation;
                }
            }
            j += 1;
        }

        // println!("{:#?}", pretty_array);

        // transforming the pretty array into a pretty string
        let mut pretty_pos: String = String::new();
        for i in 0..64 {
            if i % 8 == 0 && i != 0 {
                pretty_pos += "\n"
            }
            pretty_pos += pretty_array[i];
        }

        return pretty_pos;
    }
}

// intuitive pointers to the position's sides bitboard
pub struct Sides;
#[allow(dead_code)]
impl Sides {
    pub const WHITE: usize = 0;
    pub const BLACK: usize = 1;
}

// intuitive pointers to the position's pieces bitboard
pub struct Pieces;
#[allow(dead_code)]
impl Pieces {
    pub const PAWN: usize = 0;
    pub const BISHOP: usize = 1;
    pub const KNIGHT: usize = 2;
    pub const ROOK: usize = 3;
    pub const QUEEN: usize = 4;
    pub const KING: usize = 5;
}
