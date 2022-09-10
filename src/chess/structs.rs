// define objects and apply methods to those objects
// #[derive(PartialEq, Eq, PartialOrd, Clone, Copy, Debug, Default, Hash)]
#[derive(Debug, Clone, Copy)]
pub struct BitBoard(pub u64);

#[derive(Debug)]
pub struct State {
    pub stm: usize,                    // side to move
    pub castling_rights: u8, // first 4 bits not used, 0000abcd: a=B queen side, b=B king side, c&d same for W
    pub en_passant_square: Option<u8>, // either None (no en passant capture squares) or Some(square index)
    pub half_move_counter: u8,         // half moves since the last pawn push
    pub full_move_counter: u8,
}

#[derive(Debug)]
pub struct Position {
    /// Board for each side
    pub bb_sides: [BitBoard; 2],
    // BitBoards for all pieces and each side
    pub bb_pieces: [[BitBoard; 6]; 2],
    // State for castling rights, move counter and en passant capture squares
    pub state: State,
}

// Debug trait used for BitBoard and Position struct
pub trait Debug {
    fn pretty(&self) -> String;
}

// intuitive pointers to the position's sides bitboards
pub struct Sides;

// intuitive pointers to the position's pieces bitboards
pub struct Pieces;
