#![allow(dead_code, unused_imports, unused_variables)]
use crate::chess::structs::BitBoard;
use crate::chess::structs::Position;
use crate::chess::structs::State;
use std::num::Wrapping; // wrapping integer types to control overflow behaviour

// use crate::chess::structs_methods;
// use crate::structs_methods::Debug;

pub fn moves(pos: &Position) -> Vec<String> {
    let mut moves: Vec<String> = Vec::new();

    pseudo_legal_moves(pos, &mut moves); // push all pseudo legal moves to moves

    // legal_moves(pos, &mut moves); // remove the illegal moves from moves

    return moves;
}

fn pseudo_legal_moves(pos: &Position, moves: &mut Vec<String>) {
    let potential_move_pieces = pos.get_pieces(pos.state.stm);
    // println!("{:?}", potential_move_pieces);

    for i in potential_move_pieces.into_iter() {
        match i {
            (_, 0, _) => {
                // println!("{:?}", square_id(i.2));
                add_plegal_pawn_push(moves, &pos, i);
                add_plegal_pawn_capture(moves, &pos, i);
            }
            (_, 1, _) => add_plegal_bishop_move(moves, &pos, i),
            (_, 2, _) => add_plegal_knight_move(moves, &pos, i),
            (_, 3, _) => add_plegal_rook_move(moves, &pos, i),
            (_, 4, _) => add_plegal_queen_move(moves, &pos, i),
            (_, 5, _) => add_plegal_king_move(moves, &pos, i),
            (_, _, _) => println!("Piece not found!"),
        }
    }
}

fn legal_moves(pos: &Position, moves: &mut Vec<String>) {
    // test loop
    for i in pos.get_pieces(0).into_iter() {
        if i.0 == 0 {
            moves.push("Legal move".to_owned());
        }
    }
}

fn add_plegal_pawn_push(moves: &mut Vec<String>, pos: &Position, piece: (usize, usize, usize)) {
    /*
    evaluate if square in front is empty
        true->add push possible pawn moves (including promotion)
            evaluate if pawn is on starting rank
                true->evaluate if 2 squares in front is empty
                    true->add push 2 square to move list
                false->exit function by returning the updated move list
        false->exit function by returning the original move list
    */

    let color = pos.state.stm;
    let not_color = 1 - color;

    let next_square = piece.2 + 8 - 16 * color;
    let next_empty: bool =
        !pos.bb_sides[0].set_bit_at(next_square) && !pos.bb_sides[1].set_bit_at(next_square);
    if next_empty {
        let can_promote: bool =
            (piece.2 >= (not_color * 39 + 8)) && (piece.2 <= (not_color * 40 + 15));

        add_pawn_moves(moves, can_promote, color, square_id(next_square));

        let on_starting_rank: bool =
            (piece.2 >= (color * 39 + 8)) && (piece.2 <= (color * 40 + 15));
        if on_starting_rank {
            let next_2_squares = piece.2 + 16 - 32 * color;
            let next_2_empty: bool = !pos.bb_sides[0].set_bit_at(next_2_squares)
                && !pos.bb_sides[1].set_bit_at(next_2_squares);
            if next_2_empty {
                moves.push(square_id(next_2_squares));
            }
        }
    }
}
fn add_plegal_pawn_capture(moves: &mut Vec<String>, pos: &Position, piece: (usize, usize, usize)) {
    /*
    evaluate if piece not on a file
        true->evaluate if front left square has a piece of opposing color
            true->add takes front left (plus possible promotions) to move list
            false->evaluate if en passant square front left
                true->add takes front left (plus possible promotions) to move list
    (almost) same for front right, difference indicated by ! at beginning:
    !evaluate if piece not on h file
        true->evaluate if front right square has a piece of opposing color
            true->add takes front right (plus possible promotions) to move list
            !false->evaluate if there was NOT a front left en passant square
                true->evaluate if en passant square front right
                    true->add takes front right (plus possible promotions) to move list
    */

    let color = pos.state.stm;
    let not_color = 1 - color;
    let State {
        en_passant_square: en_passant_option,
        ..
    } = pos.state;

    let can_promote: bool = (piece.2 >= (not_color * 39 + 8)) && (piece.2 <= (not_color * 40 + 15));

    let on_a_file: bool = piece.2 % 8 == 0;
    let mut en_passant_front_left_occupied = false;

    // evaluate front left capture
    if !on_a_file {
        let front_left_square = piece.2 + 7 - 14 * color;
        let front_left_occupied: bool = pos.bb_sides[not_color].set_bit_at(front_left_square);
        let f_l_capture = format!(
            "{}{}{}",
            file_letter(piece.2),
            "x",
            square_id(front_left_square)
        );
        if front_left_occupied {
            add_pawn_moves(moves, can_promote, color, f_l_capture);
        } else if en_passant_option != None {
            let en_passant_u64 = en_passant_option.unwrap();
            en_passant_front_left_occupied = (en_passant_u64 >> front_left_square) & 1 == 1;
            if en_passant_front_left_occupied {
                add_pawn_moves(moves, can_promote, color, f_l_capture);
            }
        }
    }

    // evaluate front right capture
    let on_h_file: bool = (piece.2 + 1) % 8 == 0;
    if !on_h_file {
        let front_right_square = piece.2 + 9 - 18 * color;
        let front_right_occupied: bool = pos.bb_sides[not_color].set_bit_at(front_right_square);
        let f_r_capture = format!(
            "{}{}{}",
            file_letter(piece.2),
            "x",
            square_id(front_right_square)
        );
        if front_right_occupied {
            add_pawn_moves(moves, can_promote, color, f_r_capture);
        } else if en_passant_option != None {
            if !en_passant_front_left_occupied {
                let en_passant_u64 = en_passant_option.unwrap();
                let en_passant_front_right_occupied =
                    (en_passant_u64 >> front_right_square) & 1 == 1;
                if en_passant_front_right_occupied {
                    add_pawn_moves(moves, can_promote, color, f_r_capture);
                }
            }
        }
    }
}
// given a base move, this function evaluates if promotion is possible and adds moves accordingly
fn add_pawn_moves(moves: &mut Vec<String>, can_promote: bool, color: usize, base_move: String) {
    if can_promote {
        moves.push(format!("{}{}{}", base_move, "=", piece_name((color, 4)))); // queen
        moves.push(format!("{}{}{}", base_move, "=", piece_name((color, 3)))); // rook
        moves.push(format!("{}{}{}", base_move, "=", piece_name((color, 2)))); // knight
        moves.push(format!("{}{}{}", base_move, "=", piece_name((color, 1)))); // bishop
    } else {
        moves.push(base_move);
    }
}

fn add_plegal_bishop_move(moves: &mut Vec<String>, pos: &Position, piece: (usize, usize, usize)) {
    // println!("{}",);
}
fn add_plegal_knight_move(moves: &mut Vec<String>, pos: &Position, piece: (usize, usize, usize)) {
    let color = pos.state.stm;

    let piece_file: usize = file_number(piece.2); // as a number for quick calculation purposes
    let piece_rank: usize = rank_number(piece.2);

    // lookup squares given as winddirections
    let nne: usize = piece.2 + 17;
    let ene: usize = piece.2 + 10;
    let ese: usize = piece.2.wrapping_sub(6); // wrapping to not panic on overflow->checking for it later
    let sse: usize = piece.2.wrapping_sub(15);
    let ssw: usize = piece.2.wrapping_sub(17);
    let wsw: usize = piece.2.wrapping_sub(10);
    let wnw: usize = piece.2 + 6;
    let nnw: usize = piece.2 + 15;

    if piece_file < 8 && piece_rank < 7 && !pos.bb_sides[color].set_bit_at(nne) {
        add_move_or_take(pos, moves, piece.1, nne);
    }
    if piece_file < 7 && piece_rank < 8 && !pos.bb_sides[color].set_bit_at(ene) {
        add_move_or_take(pos, moves, piece.1, ene);
    }
    // checking for the next 4 if no overflow happened
    if ese < 64 && piece_file < 7 && piece_rank > 1 && !pos.bb_sides[color].set_bit_at(ese) {
        add_move_or_take(pos, moves, piece.1, ese);
    }
    if sse < 64 && piece_file < 8 && piece_rank > 1 && !pos.bb_sides[color].set_bit_at(sse) {
        add_move_or_take(pos, moves, piece.1, sse);
    }
    if ssw < 64 && piece_file > 1 && piece_rank > 2 && !pos.bb_sides[color].set_bit_at(ssw) {
        add_move_or_take(pos, moves, piece.1, ssw);
    }
    if wsw < 64 && piece_file > 2 && piece_rank > 1 && !pos.bb_sides[color].set_bit_at(wsw) {
        add_move_or_take(pos, moves, piece.1, wsw);
    }
    if piece_file > 2 && piece_rank < 7 && !pos.bb_sides[color].set_bit_at(wnw) {
        add_move_or_take(pos, moves, piece.1, wnw);
    }
    if piece_file > 1 && piece_rank < 7 && !pos.bb_sides[color].set_bit_at(nnw) {
        add_move_or_take(pos, moves, piece.1, nnw);
    }
}
fn add_plegal_rook_move(moves: &mut Vec<String>, pos: &Position, piece: (usize, usize, usize)) {}
fn add_plegal_queen_move(moves: &mut Vec<String>, pos: &Position, piece: (usize, usize, usize)) {
    add_plegal_rook_move(moves, pos, piece);
    add_plegal_bishop_move(moves, pos, piece)
}
fn add_plegal_king_move(moves: &mut Vec<String>, pos: &Position, piece: (usize, usize, usize)) {
    // check if file!=a or h (=0 or 7) and if rank!=1 or 8

    let color = pos.state.stm;

    let piece_file: usize = file_number(piece.2); // as a number for quick calculation purposes
    let piece_rank: usize = rank_number(piece.2);

    // lookup squares given as winddirections
    let no: usize = piece.2 + 8;
    let no_ea: usize = piece.2 + 9;
    let ea: usize = piece.2 + 1;
    let so_ea: usize = piece.2.wrapping_sub(7); // wrapping to not panic on overflow->checking for it later
    let so: usize = piece.2.wrapping_sub(8);
    let so_we: usize = piece.2.wrapping_sub(9);
    let we: usize = piece.2.wrapping_sub(1);
    let no_we: usize = piece.2 + 7;

    if piece_rank < 8 && !pos.bb_sides[color].set_bit_at(no) {
        add_move_or_take(pos, moves, piece.1, no);
    }
    if piece_file < 8 && piece_rank < 8 && !pos.bb_sides[color].set_bit_at(no_ea) {
        add_move_or_take(pos, moves, piece.1, no_ea);
    }
    if piece_file < 8 && !pos.bb_sides[color].set_bit_at(ea) {
        add_move_or_take(pos, moves, piece.1, ea);
    }
    // checking for the next 4 if no overflow happened
    if so_ea < 64 && piece_file < 8 && piece_rank > 1 && !pos.bb_sides[color].set_bit_at(so_ea) {
        add_move_or_take(pos, moves, piece.1, so_ea);
    }
    if so < 64 && piece_rank > 1 && !pos.bb_sides[color].set_bit_at(so) {
        add_move_or_take(pos, moves, piece.1, so);
    }
    if so_we < 64 && piece_file > 1 && piece_rank > 1 && !pos.bb_sides[color].set_bit_at(so_we) {
        add_move_or_take(pos, moves, piece.1, so_we);
    }
    if piece_file > 1 && !pos.bb_sides[color].set_bit_at(we) {
        add_move_or_take(pos, moves, piece.1, we);
    }
    if piece_file > 1 && piece_rank < 8 && !pos.bb_sides[color].set_bit_at(no_we) {
        add_move_or_take(pos, moves, piece.1, no_we);
    }
}

// this function adds a normal -or takes move to the moves vector
fn add_move_or_take(pos: &Position, moves: &mut Vec<String>, piece: usize, target: usize) {
    // this function assumes the target square is not occupied by a piece of the same color
    let color = pos.state.stm;
    let not_color = 1 - color;
    if pos.bb_sides[not_color].set_bit_at(target) {
        // takes
        moves.push(format!(
            "{}x{}",
            piece_name((color, piece)),
            square_id(target)
        ));
    } else {
        // moves
        moves.push(format!(
            "{}{}",
            piece_name((color, piece)),
            square_id(target)
        ));
    }
}

// input: an index of a bitboard (0<=index<=63), output: square id (a1 or a2, etc.)
fn square_id(idx: usize) -> String {
    return match idx {
        0 => "a1",
        1 => "b1",
        2 => "c1",
        3 => "d1",
        4 => "e1",
        5 => "f1",
        6 => "g1",
        7 => "h1",

        8 => "a2",
        9 => "b2",
        10 => "c2",
        11 => "d2",
        12 => "e2",
        13 => "f2",
        14 => "g2",
        15 => "h2",

        16 => "a3",
        17 => "b3",
        18 => "c3",
        19 => "d3",
        20 => "e3",
        21 => "f3",
        22 => "g3",
        23 => "h3",

        24 => "a4",
        25 => "b4",
        26 => "c4",
        27 => "d4",
        28 => "e4",
        29 => "f4",
        30 => "g4",
        31 => "h4",

        32 => "a5",
        33 => "b5",
        34 => "c5",
        35 => "d5",
        36 => "e5",
        37 => "f5",
        38 => "g5",
        39 => "h5",

        40 => "a6",
        41 => "b6",
        42 => "c6",
        43 => "d6",
        44 => "e6",
        45 => "f6",
        46 => "g6",
        47 => "h6",

        48 => "a7",
        49 => "b7",
        50 => "c7",
        51 => "d7",
        52 => "e7",
        53 => "f7",
        54 => "g7",
        55 => "h7",

        56 => "a8",
        57 => "b8",
        58 => "c8",
        59 => "d8",
        60 => "e8",
        61 => "f8",
        62 => "g8",
        63 => "h8",

        _ => "Err",
    }
    .to_owned();
}

// input: an index of a bitboard (0<=index<=63), output: rank number of this index (char)
fn rank_number_char(idx: usize) -> char {
    return match idx {
        0..=7 => '1',
        8..=15 => '2',
        16..=23 => '3',
        24..=31 => '4',
        32..=39 => '5',
        40..=47 => '6',
        48..=55 => '7',
        56..=63 => '8',
        _ => '?',
    };
}

// input: an index of a bitboard (0<=index<=63), output: rank number of this index
fn rank_number(idx: usize) -> usize {
    return match idx {
        0..=7 => 1,
        8..=15 => 2,
        16..=23 => 3,
        24..=31 => 4,
        32..=39 => 5,
        40..=47 => 6,
        48..=55 => 7,
        56..=63 => 8,
        _ => 0,
    };
}

// input: an index of a bitboard (0<=index<=63), output: file letter of this index
fn file_letter(idx: usize) -> char {
    return match idx {
        0 | 8 | 16 | 24 | 32 | 40 | 48 | 56 => 'a',
        1 | 9 | 17 | 25 | 33 | 41 | 49 | 57 => 'b',
        2 | 10 | 18 | 26 | 34 | 42 | 50 | 58 => 'c',
        3 | 11 | 19 | 27 | 35 | 43 | 51 | 59 => 'd',
        4 | 12 | 20 | 28 | 36 | 44 | 52 | 60 => 'e',
        5 | 13 | 21 | 29 | 37 | 45 | 53 | 61 => 'f',
        6 | 14 | 22 | 30 | 38 | 46 | 54 | 62 => 'g',
        7 | 15 | 23 | 31 | 39 | 47 | 55 | 63 => 'h',
        _ => '?',
    };
}

// input: an index of a bitboard (0<=index<=63), output: file number of this index
fn file_number(idx: usize) -> usize {
    return match idx {
        0 | 8 | 16 | 24 | 32 | 40 | 48 | 56 => 1,
        1 | 9 | 17 | 25 | 33 | 41 | 49 | 57 => 2,
        2 | 10 | 18 | 26 | 34 | 42 | 50 | 58 => 3,
        3 | 11 | 19 | 27 | 35 | 43 | 51 | 59 => 4,
        4 | 12 | 20 | 28 | 36 | 44 | 52 | 60 => 5,
        5 | 13 | 21 | 29 | 37 | 45 | 53 | 61 => 6,
        6 | 14 | 22 | 30 | 38 | 46 | 54 | 62 => 7,
        7 | 15 | 23 | 31 | 39 | 47 | 55 | 63 => 8,
        _ => 0,
    };
}

// input: a tuple of 2 usizes, output: letter corresponding to the piece
fn piece_name(id: (usize, usize)) -> char {
    return match id {
        (0, 0) => 'P',
        (0, 1) => 'B',
        (0, 2) => 'N',
        (0, 3) => 'R',
        (0, 4) => 'Q',
        (0, 5) => 'K',

        (1, 0) => 'p',
        (1, 1) => 'b',
        (1, 2) => 'n',
        (1, 3) => 'r',
        (1, 4) => 'q',
        (1, 5) => 'k',

        (_, _) => '?',
    };
}

/*
TODO:
    get_moves file:
        finish moves function:
            1. pseudo_legal_moves
                create move generation for all the pieces besides pawn and knight
            2. legal_moves
    structs_methods file:
        load method:
            1. check for allowed FEN
            2. add option to pass previous moves (PGN) and save these moves in state
        put piece and remove piece method: maybe change castling state when putting/removing king?
        add select move method: input: half move number -> output: position at that move
        add next move method: select move method with input of 1 less than current move
        add previous move method: select move method with input of 1 more than current move
*/
