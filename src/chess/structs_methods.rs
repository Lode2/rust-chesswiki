// file containing methods for the structs in structs.rs

// functionality in this file that is used outside this file
pub use crate::chess::structs::BitBoard;
pub use crate::chess::structs::Debug;
pub use crate::chess::structs::Pieces;
pub use crate::chess::structs::Position;
pub use crate::chess::structs::Sides;
pub use crate::chess::structs::State;

// imports that are not called in this file from other files
use crate::move_gen;
use regex::Regex;

// add methods to the BitBoard struct
impl BitBoard {
    // given an initial bitboard u64, flip the bit at position square_idx, not necessary due to bit_set and bit_unset
    // pub fn bit_flip(&self, square_idx: usize) -> BitBoard {
    //     return BitBoard(self.u64() ^ (1 << square_idx));
    // }
    // set a bit (change to 1) at position square_idx
    pub fn bit_set(&self, square_idx: usize) -> BitBoard {
        return BitBoard(self.u64() | (1 << square_idx));
    }
    // unset a bit (change to 0) at position square_idx
    pub fn bit_unset(&self, square_idx: usize) -> BitBoard {
        return BitBoard(self.u64() & !(1 << square_idx));
    }
    // returns a vector with the position of all 1-bits
    pub fn find_set_bits(&self) -> Vec<usize> {
        let mut set_bits: Vec<usize> = vec![];
        let &BitBoard(mut my_int) = self;
        let mut index: usize = 0;
        while my_int != 0 {
            if (my_int & 1) == 1 {
                set_bits.push(index);
            }
            my_int = my_int >> 1;
            index += 1;
        }
        return set_bits;
    }
    // extract the u64 inside the BitBoard struct
    pub fn u64(&self) -> &u64 {
        let BitBoard(my_integer) = self;
        return my_integer;
    }
}

impl Debug for BitBoard {
    // creates a pretty string of the bitboard (for debugging purposes)
    fn pretty(&self) -> String {
        let my_int = self.u64();
        let formatted_bitboard = format!("{my_int:064b}");
        let pretty_array: [char; 64] = rearrange_array(formatted_bitboard.chars());
        let mut pretty_string: String = String::new();

        for (idx, x) in pretty_array.into_iter().enumerate() {
            if idx % 8 == 0 {
                pretty_string += "\n";
            }
            pretty_string += &(x as char).to_string();
            pretty_string += " ";
        }
        return pretty_string;
    }
}

impl Debug for Position {
    // creates a pretty string of the position (for debugging purposes)
    fn pretty(&self) -> String {
        let mut pretty_array: [&str; 64] = ["x "; 64];

        // 1. find all occupied squares for both colors -> so 2 vectors of indices
        let white_occupies = self.bb_sides[0].find_set_bits();
        let black_occupies = self.bb_sides[1].find_set_bits();

        // 2. loop over the found indices for both colors
        for w_idx in white_occupies {
            // 3. find what the piece is that occupies a found index and make readable
            let found_piece = self.piece_id_finder(0, w_idx);
            let pretty_piece = match found_piece {
                (0, 0, _) => "P ",
                (0, 1, _) => "B ",
                (0, 2, _) => "N ",
                (0, 3, _) => "R ",
                (0, 4, _) => "Q ",
                (0, 5, _) => "K ",
                _ => "Err",
            };
            // 4. append this piece to pretty_array
            pretty_array[to_pretty_index(63 - w_idx)] = pretty_piece;
        }
        // again for the black pieces
        for b_idx in black_occupies {
            let found_piece = self.piece_id_finder(1, b_idx);
            let pretty_piece = match found_piece {
                (1, 0, _) => "p ",
                (1, 1, _) => "b ",
                (1, 2, _) => "n ",
                (1, 3, _) => "r ",
                (1, 4, _) => "q ",
                (1, 5, _) => "k ",
                _ => "Err",
            };
            pretty_array[to_pretty_index(63 - b_idx)] = pretty_piece;
        }

        // 5. transform pretty_array into a string
        let mut pretty_pos: String = String::new();
        for i in 0..64 {
            if i % 8 == 0 {
                pretty_pos += "\n"
            }
            pretty_pos += pretty_array[i];
        }

        // 6. add the state data
        pretty_pos += "\nSide to move (0 = w, 1 = b):";
        pretty_pos += format!("{:08b}", self.state.stm).as_str();
        pretty_pos += "\nCastling allowed (xxxxKQkq): ";
        pretty_pos += format!("{:08b}", self.state.castling_rights).as_str();
        pretty_pos += "\nEn passant square:";
        let state_eps = if self.state.en_passant_square == None {
            "No en passant square".to_owned()
        } else {
            format!("{:08b}", self.state.en_passant_square.unwrap())
        };
        pretty_pos.push_str(&state_eps);
        pretty_pos += "\nHalfmove counter (half moves since last pawn push): ";
        pretty_pos += &self.state.half_move_counter.to_string();
        pretty_pos += "\nFull moves: ";
        pretty_pos += &self.state.full_move_counter.to_string();

        return pretty_pos;
    }
}

impl Position {
    // execute this function to get the chess starting position
    pub fn starting_pos() -> Position {
        return Position {
            bb_sides: [
                BitBoard(0b0000000000000000000000000000000000000000000000001111111111111111),
                BitBoard(0b1111111111111111000000000000000000000000000000000000000000000000),
            ],
            bb_pieces: [
                [
                    BitBoard(0b0000000000000000000000000000000000000000000000001111111100000000),
                    BitBoard(0b0000000000000000000000000000000000000000000000000000000000100100),
                    BitBoard(0b0000000000000000000000000000000000000000000000000000000001000010),
                    BitBoard(0b0000000000000000000000000000000000000000000000000000000010000001),
                    BitBoard(0b0000000000000000000000000000000000000000000000000000000000001000),
                    BitBoard(0b0000000000000000000000000000000000000000000000000000000000010000),
                ],
                [
                    BitBoard(0b0000000011111111000000000000000000000000000000000000000000000000),
                    BitBoard(0b0010010000000000000000000000000000000000000000000000000000000000),
                    BitBoard(0b0100001000000000000000000000000000000000000000000000000000000000),
                    BitBoard(0b1000000100000000000000000000000000000000000000000000000000000000),
                    BitBoard(0b0000100000000000000000000000000000000000000000000000000000000000),
                    BitBoard(0b0001000000000000000000000000000000000000000000000000000000000000),
                ],
            ],
            state: State {
                stm: 0,                      // side to move
                castling_rights: 0b00001111, // all castling right available
                en_passant_square: None,
                half_move_counter: 0,
                full_move_counter: 0,
            },
        };
    }
    // execute this function to get an empty position
    pub fn empty_pos() -> Position {
        return Position {
            bb_sides: [BitBoard(0); 2],
            bb_pieces: [[BitBoard(0); 6]; 2],
            state: State {
                stm: 0,
                castling_rights: 0b00000000,
                en_passant_square: None,
                half_move_counter: 0,
                full_move_counter: 0,
            },
        };
    }
    // load a position using the FEN format
    pub fn load(&mut self, fen: &str) {
        // start with empty position
        self.bb_sides = [BitBoard(0); 2];
        self.bb_pieces = [[BitBoard(0); 6]; 2];

        // 1. unpack the FEN into a string containing only the positional info
        // define regex's
        let split_whitespace: Regex = Regex::new(" ").unwrap();
        let split_slash: Regex = Regex::new("/").unwrap();

        // remove the slashes from the piece data of the fen
        let slashless = split_slash.replace_all(fen, "");
        // split the piece data and meta data of the fen
        let fen_data_split: Vec<&str> = split_whitespace.splitn(&slashless, 2).collect();

        // 2. translate the FEN string into a piece on a bitboard
        let mut square_count: usize = 0;
        for fen_character in fen_data_split[0].bytes() {
            // find the square index of the analyzed character in the fen
            let div_by_8: f32 = square_count as f32 / 8.;
            let floor: usize = (div_by_8).floor() as usize;
            let square_idx: usize = (7 - floor) * 8 + ((div_by_8 % 1.) * 8.) as usize;

            // catch which piece bitboard needs to be updated
            match fen_character {
                0b110001 => (),                // 1, 1 is already added at each iteration
                0b110010 => square_count += 1, // 2
                0b110011 => square_count += 2, // 3
                0b110100 => square_count += 3, // 4
                0b110101 => square_count += 4, // 5
                0b110110 => square_count += 5, // 6
                0b110111 => square_count += 6, // 7
                0b111000 => square_count += 7, // 8
                0b1010000 => {
                    self.bb_pieces[0][0] = self.bb_pieces[0][0].bit_set(square_idx);
                    self.bb_sides[0] = self.bb_sides[0].bit_set(square_idx);
                } // P
                0b1010010 => {
                    self.bb_pieces[0][3] = self.bb_pieces[0][3].bit_set(square_idx);
                    self.bb_sides[0] = self.bb_sides[0].bit_set(square_idx);
                } // R
                0b1001110 => {
                    self.bb_pieces[0][2] = self.bb_pieces[0][2].bit_set(square_idx);
                    self.bb_sides[0] = self.bb_sides[0].bit_set(square_idx);
                } // N
                0b1000010 => {
                    self.bb_pieces[0][1] = self.bb_pieces[0][1].bit_set(square_idx);
                    self.bb_sides[0] = self.bb_sides[0].bit_set(square_idx);
                } // B
                0b1001011 => {
                    self.bb_pieces[0][5] = self.bb_pieces[0][5].bit_set(square_idx);
                    self.bb_sides[0] = self.bb_sides[0].bit_set(square_idx);
                } // K
                0b1010001 => {
                    self.bb_pieces[0][4] = self.bb_pieces[0][4].bit_set(square_idx);
                    self.bb_sides[0] = self.bb_sides[0].bit_set(square_idx);
                } // Q
                0b1110000 => {
                    self.bb_pieces[1][0] = self.bb_pieces[1][0].bit_set(square_idx);
                    self.bb_sides[1] = self.bb_sides[1].bit_set(square_idx);
                } // p
                0b1110010 => {
                    self.bb_pieces[1][3] = self.bb_pieces[1][3].bit_set(square_idx);
                    self.bb_sides[1] = self.bb_sides[1].bit_set(square_idx);
                } // r
                0b1101110 => {
                    self.bb_pieces[1][2] = self.bb_pieces[1][2].bit_set(square_idx);
                    self.bb_sides[1] = self.bb_sides[1].bit_set(square_idx);
                } // n
                0b1100010 => {
                    self.bb_pieces[1][1] = self.bb_pieces[1][1].bit_set(square_idx);
                    self.bb_sides[1] = self.bb_sides[1].bit_set(square_idx);
                } // b
                0b1101011 => {
                    self.bb_pieces[1][5] = self.bb_pieces[1][5].bit_set(square_idx);
                    self.bb_sides[1] = self.bb_sides[1].bit_set(square_idx);
                } // k
                0b1110001 => {
                    self.bb_pieces[1][4] = self.bb_pieces[1][4].bit_set(square_idx);
                    self.bb_sides[1] = self.bb_sides[1].bit_set(square_idx);
                } // q
                _ => println!("No match found in the fen data!"),
            };
            square_count += 1;
        }

        // 3. Add the meta data from the FEN to State in the Position struct
        let fen_meta_data: Vec<&str> = split_whitespace.split(fen_data_split[1]).collect();

        // side to move
        let state_stm: usize = if fen_meta_data[0] == "w" { 0 } else { 1 };

        // castling rights
        let mut state_castling_rights: u8 = 0b00000000;
        if fen_meta_data[1] != "-" {
            if Regex::new("K").unwrap().is_match(fen_meta_data[1]) {
                state_castling_rights = state_castling_rights | (1 << 3)
            }
            if Regex::new("Q").unwrap().is_match(fen_meta_data[1]) {
                state_castling_rights = state_castling_rights | (1 << 2)
            }
            if Regex::new("k").unwrap().is_match(fen_meta_data[1]) {
                state_castling_rights = state_castling_rights | (1 << 1)
            }
            if Regex::new("Q").unwrap().is_match(fen_meta_data[1]) {
                state_castling_rights = state_castling_rights | (1 << 0)
            }
        }

        let state_en_passant_square: Option<u8> = if fen_meta_data[2] == "-" {
            None
        } else {
            match fen_meta_data[2] {
                "a1" => Some(0),
                "b1" => Some(1),
                "c1" => Some(2),
                "d1" => Some(3),
                "e1" => Some(4),
                "f1" => Some(5),
                "g1" => Some(6),
                "h1" => Some(7),

                "a2" => Some(8),
                "b2" => Some(9),
                "c2" => Some(10),
                "d2" => Some(11),
                "e2" => Some(12),
                "f2" => Some(13),
                "g2" => Some(14),
                "h2" => Some(15),

                "a3" => Some(16),
                "b3" => Some(17),
                "c3" => Some(18),
                "d3" => Some(19),
                "e3" => Some(20),
                "f3" => Some(21),
                "g3" => Some(22),
                "h3" => Some(23),

                "a4" => Some(24),
                "b4" => Some(25),
                "c4" => Some(26),
                "d4" => Some(27),
                "e4" => Some(28),
                "f4" => Some(29),
                "g4" => Some(30),
                "h4" => Some(31),

                "a5" => Some(32),
                "b5" => Some(33),
                "c5" => Some(34),
                "d5" => Some(35),
                "e5" => Some(36),
                "f5" => Some(37),
                "g5" => Some(38),
                "h5" => Some(39),

                "a6" => Some(40),
                "b6" => Some(41),
                "c6" => Some(42),
                "d6" => Some(43),
                "e6" => Some(44),
                "f6" => Some(45),
                "g6" => Some(46),
                "h6" => Some(47),

                "a7" => Some(48),
                "b7" => Some(49),
                "c7" => Some(50),
                "d7" => Some(51),
                "e7" => Some(52),
                "f7" => Some(53),
                "g7" => Some(54),
                "h7" => Some(55),

                "a8" => Some(56),
                "b8" => Some(57),
                "c8" => Some(58),
                "d8" => Some(59),
                "e8" => Some(60),
                "f8" => Some(61),
                "g8" => Some(62),
                "h8" => Some(63),
                _ => None,
            }
        };

        // add to state
        self.state = State {
            stm: state_stm,
            castling_rights: state_castling_rights,
            en_passant_square: state_en_passant_square,
            half_move_counter: fen_meta_data[3].parse().unwrap(),
            full_move_counter: fen_meta_data[4].parse().unwrap(),
        };

        println!("{:?}", fen_meta_data);
    }
    // put a piece on a desired square
    pub fn put_piece(&mut self, piece: usize, piece_color: usize, square_idx: usize) {
        let piece_info: [usize; 2] = [piece, piece_color];
        match piece_info {
            [0, 0] => {
                self.bb_pieces[0][0] = self.bb_pieces[0][0].bit_set(square_idx);
                self.bb_sides[0] = self.bb_sides[0].bit_set(square_idx);
            } // P
            [3, 0] => {
                self.bb_pieces[0][3] = self.bb_pieces[0][3].bit_set(square_idx);
                self.bb_sides[0] = self.bb_sides[0].bit_set(square_idx);
                self.state.castling_rights = update_state_castling(self, 0, square_idx);
            } // R
            [2, 0] => {
                self.bb_pieces[0][2] = self.bb_pieces[0][2].bit_set(square_idx);
                self.bb_sides[0] = self.bb_sides[0].bit_set(square_idx);
            } // N
            [1, 0] => {
                self.bb_pieces[0][1] = self.bb_pieces[0][1].bit_set(square_idx);
                self.bb_sides[0] = self.bb_sides[0].bit_set(square_idx);
            } // B
            [5, 0] => {
                self.bb_pieces[0][5] = self.bb_pieces[0][5].bit_set(square_idx);
                self.bb_sides[0] = self.bb_sides[0].bit_set(square_idx);
            } // K
            [4, 0] => {
                self.bb_pieces[0][4] = self.bb_pieces[0][4].bit_set(square_idx);
                self.bb_sides[0] = self.bb_sides[0].bit_set(square_idx);
            } // Q
            [0, 1] => {
                self.bb_pieces[1][0] = self.bb_pieces[1][0].bit_set(square_idx);
                self.bb_sides[1] = self.bb_sides[1].bit_set(square_idx);
            } // p
            [3, 1] => {
                self.bb_pieces[1][3] = self.bb_pieces[1][3].bit_set(square_idx);
                self.bb_sides[1] = self.bb_sides[1].bit_set(square_idx);
                self.state.castling_rights = update_state_castling(self, 1, square_idx);
            } // r
            [2, 1] => {
                self.bb_pieces[1][2] = self.bb_pieces[1][2].bit_set(square_idx);
                self.bb_sides[1] = self.bb_sides[1].bit_set(square_idx);
            } // n
            [1, 1] => {
                self.bb_pieces[1][1] = self.bb_pieces[1][1].bit_set(square_idx);
                self.bb_sides[1] = self.bb_sides[1].bit_set(square_idx);
            } // b
            [5, 1] => {
                self.bb_pieces[1][5] = self.bb_pieces[1][5].bit_set(square_idx);
                self.bb_sides[1] = self.bb_sides[1].bit_set(square_idx);
            } // k
            [4, 1] => {
                self.bb_pieces[1][4] = self.bb_pieces[1][4].bit_set(square_idx);
                self.bb_sides[1] = self.bb_sides[1].bit_set(square_idx);
            } // q
            _ => println!("No matching piece found!"),
        };
    }
    // clear a square of a piece
    pub fn remove_piece(&mut self, square_idx: usize) {
        // find piece color
        let color = if (self.bb_sides[0].u64() >> square_idx) & 1 == 1 {
            0
        } else {
            1
        };
        // clear all pieces of the found color of the square
        self.bb_sides[color] = self.bb_sides[color].bit_unset(square_idx);
        self.bb_pieces[color][0] = self.bb_pieces[color][0].bit_unset(square_idx);
        self.bb_pieces[color][1] = self.bb_pieces[color][1].bit_unset(square_idx);
        self.bb_pieces[color][2] = self.bb_pieces[color][2].bit_unset(square_idx);
        self.bb_pieces[color][3] = self.bb_pieces[color][3].bit_unset(square_idx);
        self.bb_pieces[color][4] = self.bb_pieces[color][4].bit_unset(square_idx);
        self.bb_pieces[color][5] = self.bb_pieces[color][5].bit_unset(square_idx);

        // update the castling rights in state if necessary
        self.state.castling_rights = update_state_castling(self, color, square_idx);
    }
    // return all the legal moves of the position
    pub fn moves(&self) -> Vec<String> {
        return move_gen::get_moves::moves(&self);
    }
    // output -> vector of tuples: (piece color (0=white), piece id (0=pawn), piece position (a1=0))
    pub fn get_pieces(&self, piece_color: usize) -> Vec<(usize, usize, usize)> {
        let occupied_idx = self.bb_sides[piece_color].find_set_bits();

        let mut pieces: Vec<(usize, usize, usize)> = vec![];

        // append the piece that occupies a square
        for idx in occupied_idx {
            pieces.push(self.piece_id_finder(piece_color, idx));
        }
        return pieces;
    }
    // finds what piece occupies a provided square index
    pub fn piece_id_finder(&self, color: usize, idx: usize) -> (usize, usize, usize) {
        #[allow(unused_assignments)]
        let mut piece_id: usize = 5; // standard
        if (self.bb_pieces[color][0].u64() >> idx) & 1 == 1 {
            piece_id = 0;
        } else if (self.bb_pieces[color][1].u64() >> idx) & 1 == 1 {
            piece_id = 1;
        } else if (self.bb_pieces[color][2].u64() >> idx) & 1 == 1 {
            piece_id = 2;
        } else if (self.bb_pieces[color][3].u64() >> idx) & 1 == 1 {
            piece_id = 3;
        } else if (self.bb_pieces[color][4].u64() >> idx) & 1 == 1 {
            piece_id = 4;
        } // 5 check not needed, default
        return (color, piece_id, idx);
    }
}

#[allow(dead_code)]
impl Sides {
    pub const WHITE: usize = 0;
    pub const BLACK: usize = 1;
}

#[allow(dead_code)]
impl Pieces {
    pub const PAWN: usize = 0;
    pub const BISHOP: usize = 1;
    pub const KNIGHT: usize = 2;
    pub const ROOK: usize = 3;
    pub const QUEEN: usize = 4;
    pub const KING: usize = 5;
}

// generally used functions for this file
// transform the an index of a square in a bitboard to an index for display
fn to_pretty_index(ugly_index: usize) -> usize {
    let div_by_8: f32 = ugly_index as f32 / 8.;
    let floor: usize = (div_by_8).floor() as usize;
    let pretty_index: usize = (floor) * 8 + 7 - ((div_by_8 % 1.) * 8.) as usize;
    return pretty_index;
}

// transform a bitboard-ordered-array to an array that can be used for display
fn rearrange_array(old_array: std::str::Chars) -> [char; 64] {
    let mut new_array: [char; 64] = ['0'; 64];
    for (old_idx, itm) in old_array.into_iter().enumerate() {
        if itm == '1' {
            let new_idx: usize = to_pretty_index(old_idx);
            new_array[new_idx] = '1';
        }
    }
    return new_array;
}

// function to update the castling state if needed
fn update_state_castling(pos: &mut Position, color: usize, square_id: usize) -> u8 {
    if (color == 0) & ((pos.bb_pieces[0][5].u64() >> 4) & 1 == 1) {
        if square_id == 0 {
            return pos.state.castling_rights ^ (1 << 2);
        } else if square_id == 7 {
            return pos.state.castling_rights ^ (1 << 3);
        }
    } else if (color == 1) & ((pos.bb_pieces[1][5].u64() >> 60) & 1 == 1) {
        if square_id == 56 {
            return pos.state.castling_rights ^ (1 << 0);
        } else if square_id == 63 {
            return pos.state.castling_rights ^ (1 << 1);
        }
    }
    return pos.state.castling_rights;
}
