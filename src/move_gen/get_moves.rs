use crate::chess_pos::Position;
pub fn moves(pos: &Position) -> Vec<&'static str> {
    let pseudo_legal: Vec<&str> = pseudo_legal_moves(pos);
    let legal_moves: Vec<&str> = legal_moves(pos, pseudo_legal);
    return legal_moves;
}
fn pseudo_legal_moves(pos: &Position) -> Vec<&'static str> {
    return vec![];
}
fn legal_moves(pos: &Position, pseudo_legal: Vec<&str>) -> Vec<&'static str> {
    return vec![];
}
