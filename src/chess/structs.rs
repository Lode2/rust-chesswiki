// define objects and apply methods to those objects
// #[derive(PartialEq, Eq, PartialOrd, Clone, Copy, Debug, Default, Hash)]
#[derive(Debug, Clone, Copy)]
pub struct BitBoard(pub u64);

#[derive(Debug)]
pub struct Position {
    /// Board for each side
    pub bb_sides: [BitBoard; 2],
    // BitBoards for all pieces and each side
    pub bb_pieces: [[BitBoard; 6]; 2],
    // pub state: State,
}

// #[derive(Debug)]
// pub struct State {
//     castling_rights: CastlingRights,
//     en_passant_square: Option<Square>,
//     half_move_counter: u8,
//     stm: usize,
// }

// Debug trait used for BitBoard and Position struct
pub trait Debug {
    fn pretty(&self) -> String;
}

// intuitive pointers to the position's sides bitboards
pub struct Sides;

// intuitive pointers to the position's pieces bitboards
pub struct Pieces;
