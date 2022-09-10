#[allow(unused_imports)]
use crate::chess::structs::BitBoard;
use crate::chess::structs::Position;
// use crate::chess::structs_methods;
// use crate::structs_methods::Debug;

pub fn moves(pos: &Position, current_move: usize) -> Vec<&str> {
    let mut moves: Vec<&str> = Vec::new();

    pseudo_legal_moves(pos, &mut moves, current_move); // push all pseudo legal moves to moves
    legal_moves(pos, &mut moves); // remove the illegal moves from moves

    return moves;
}

fn pseudo_legal_moves(pos: &Position, moves: &mut Vec<&str>, current_move: usize) {
    for i in pos.get_pieces(current_move).into_iter() {
        if i.0 == 0 {
            moves.push("Pseudo legal move");
        }
    }
}

fn legal_moves(pos: &Position, moves: &mut Vec<&str>) {
    for i in pos.get_pieces(0).into_iter() {
        if i.0 == 0 {
            moves.push("Legal move");
        }
    }
}

/*
TODO:
    structs_methods file:
        load method:
            1. load the state from the FEN: I think this works now. UPDATE: It does!
            2. check for allowed FEN
        pretty method: print the state data. UPDATE: Works!
        put_piece method: if placing rook on home square, should castling rights return?
        remove_piece method: same as put_piece but reversed
*/
