// file containing bitboard and position struct

#[derive(PartialEq, Eq, PartialOrd, Clone, Copy, Debug, Default, Hash)]
pub struct BitBoard(pub u64);

// add methods to the BitBoard struct
impl BitBoard {
    pub fn pretty(self) -> String {
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
        let mut pretty_pos = String::new();

        // destructuring the position
        let Position {
            bb_sides: [white, black],
            bb_pieces:
                [[wpawn, wbishop, wknight, wrook, wqueen, wking], [bpawn, bbishop, bknight, brook, bqueen, bking]],
        } = self;
        // array of all pieces bitboards
        let all_pieces = [
            wpawn, wbishop, wknight, wrook, wqueen, wking, bpawn, bbishop, bknight, brook, bqueen,
            bking,
        ];

        for i in all_pieces.iter() {
            let BitBoard(val) = i;
            pretty_pos += " ";
        }
        println!("inside position impl white pretty: {}", white.pretty());
        pretty_pos += "put pretty position";
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
