use crate::{get_piece_color, BOARD_SIZE};

pub struct AttackingPieces {
    pub pieces_attacking_square: Vec<(i8, i8)>,
    pub pinned_pieces: Vec<(i8, i8)>
}

pub fn get_pieces_attacking_square(attacked_piece_color: bool, square_position: (i8, i8), board: &[[char; BOARD_SIZE]; BOARD_SIZE]) -> AttackingPieces {

    let mut pieces_attacking_square: Vec<(i8, i8)> = vec![];
    let mut pinned_pieces: Vec<(i8, i8)> = vec![];
    let mut pinned_pieces_index_offset = 0;

    let current_column: i8 = square_position.0;
    let current_row: i8 = square_position.1;

    for cardinal_direction in [(-1, 0), (0, -1), (1, 0), (0, 1), (-1, -1), (1, -1), (1, 1), (-1, 1)].iter() {
    // left, up, right, down, upper-left, upper-right, lower-right, lower-left
        for square in 1..BOARD_SIZE as i8 {
            let column_index_in_square = current_column + square*cardinal_direction.0;
            let row_index_in_square = current_row + square*cardinal_direction.1;
            let position = (column_index_in_square, row_index_in_square);

            let is_diagonal_movement = cardinal_direction.0 != 0 && cardinal_direction.1 != 0;

            let is_column_in_board = column_index_in_square >= 0 && column_index_in_square < BOARD_SIZE as i8;
            let is_row_in_board = row_index_in_square >= 0 && row_index_in_square < BOARD_SIZE as i8;

            if is_column_in_board && is_row_in_board {
                let square_character = board[column_index_in_square as usize][current_row as usize];

                if attacked_piece_color {
                    if is_diagonal_movement {
                        if square_character == 'q' || square_character == 'b' {
                            pieces_attacking_square.insert(pinned_pieces_index_offset, position);
                        }
                    }else {
                        if square_character == 'q' || square_character == 'r' {
                            pieces_attacking_square.insert(pinned_pieces_index_offset, position);
                        }
                    }
                }else {
                    if is_diagonal_movement {
                        if square_character == 'Q' || square_character == 'B' {
                            pieces_attacking_square.insert(pinned_pieces_index_offset, position);
                        }
                    }else {
                        if square_character == 'Q' || square_character == 'R' {
                            pieces_attacking_square.insert(pinned_pieces_index_offset, position);
                        }
                    }
                }
                match get_piece_color(square_character) {
                    Some(b) => { // a piece was identified
                        if b == attacked_piece_color { 
                            // if the identified piece is the same color as the piece being attacked, it is a pinned piece
                            if pinned_pieces.len() >= pinned_pieces_index_offset+1 {
                                // if there are more pieces that are the same color of the attacked on the way, none are pinned
                                pinned_pieces.remove(pinned_pieces_index_offset);
                            }else{
                                pinned_pieces.insert(pinned_pieces_index_offset, position)
                            }
                        }else {
                            break;
                        }
                    },
                    None => (), // not a piece ('.' / FREE_SQUARE_SYMBOL)
                }
            }else {
                break;
            }
        }

        if pieces_attacking_square.len() < pinned_pieces.len() {
            // if there is a "pinned piece" but no "pinning piece",
            // the pinned piece is not actually being pinned by anything
            pinned_pieces.remove(pinned_pieces_index_offset);
        }
        if pinned_pieces.len() == pinned_pieces_index_offset+1 {
            pinned_pieces_index_offset += 1;
        }
    }

    let knight_symbol: char;
    if attacked_piece_color {
        knight_symbol = 'n';
    }else {
        knight_symbol = 'N';
    }

    for knight_movement_possibility in [(-2, -1), (-2, 1), (-1, -2), (-1, 2), (1, 2), (1, -2), (2, 1), (2, -1)].iter() {
        let column_index_in_square = current_column + knight_movement_possibility.0;
        let row_index_in_square = current_row + knight_movement_possibility.1;
        let position = (column_index_in_square, row_index_in_square);
        
        let is_column_in_board = column_index_in_square >= 0 && column_index_in_square < BOARD_SIZE as i8;
        let is_row_in_board = row_index_in_square >= 0 && row_index_in_square < BOARD_SIZE as i8;

        if is_column_in_board && is_row_in_board {
            let square_character = board[position.0 as usize][position.1 as usize];
            if square_character == knight_symbol {
                pieces_attacking_square.push(position);
            }
        }
    }

    // pawn verification:
    if attacked_piece_color {
        if square_position.1 - 1 >= 0 {
            if square_position.0 - 1 >= 0 {
                if board[(square_position.0-1) as usize][(square_position.1-1) as usize] == 'j' {
                    pieces_attacking_square.push((square_position.0-1, square_position.1-1));
                }
            }else if square_position.0 + 1 < BOARD_SIZE as i8 {
                if board[(square_position.0+1) as usize][(square_position.1-1) as usize] == 'j' {
                    pieces_attacking_square.push((square_position.0+1, square_position.1-1));
                }
            }
        }
    }else {
        if square_position.1 + 1 < BOARD_SIZE as i8 {
            if square_position.0-1 >= 0 {
                if board[(square_position.0-1) as usize][(square_position.1+1) as usize] == 'i' {
                    pieces_attacking_square.push((square_position.0-1, square_position.1+1));
                }
            }else if square_position.0+1 < BOARD_SIZE as i8 {
                if board[(square_position.0+1) as usize][(square_position.1+1) as usize] == 'i' {
                    pieces_attacking_square.push((square_position.0+1, square_position.1+1));
                }
            }
        }
    }

    AttackingPieces {
        pieces_attacking_square,
        pinned_pieces
    }
}
