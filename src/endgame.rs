use crate::{is_black, is_white, BOARD_SIZE};

pub fn get_pieces_attacking_king(color: bool, king_position: (i8, i8), board: &[[char; BOARD_SIZE]; BOARD_SIZE]) -> Vec<(i8, i8)> {

    let mut pieces_attacking_king: Vec<(i8, i8)> = vec![];

    let current_column: i8 = king_position.0;
    let current_row: i8 = king_position.1;

    for right_side_square in 1..BOARD_SIZE as i8 {
        let column_index_in_square = current_column + right_side_square;
        
        if column_index_in_square < BOARD_SIZE as i8 {
            let square_character = board[column_index_in_square as usize][current_row as usize];
            if color {
                if square_character == 'q' || square_character == 'r' {
                    pieces_attacking_king.push((column_index_in_square, current_row));
                }else if is_black(square_character) || is_white(square_character) {
                    break;
                }
            }else {
                if square_character == 'Q' || square_character == 'R' {
                    pieces_attacking_king.push((column_index_in_square, current_row));
                }else if is_black(square_character) || is_white(square_character) {
                    break;
                }
            }
        }else {
            break;
        }
    }

    for left_side_square in 1..BOARD_SIZE as i8 {
        let column_index_in_square = current_column - left_side_square;
        
        if column_index_in_square >= 0 {
            let square_character = board[column_index_in_square as usize][current_row as usize];
            if color {
                if square_character == 'q' || square_character == 'r' {
                    pieces_attacking_king.push((column_index_in_square, current_row));
                }else if is_black(square_character) || is_white(square_character) {
                    break;
                }
            }else {
                if square_character == 'Q' || square_character == 'R' {
                    pieces_attacking_king.push((column_index_in_square, current_row));
                }else if is_black(square_character) || is_white(square_character) {
                    break;
                }
            }
        }else {
            break;
        }
    }

    for below_square in 1..BOARD_SIZE as i8 {
        let row_index_in_square = current_row + below_square;
        
        if row_index_in_square < BOARD_SIZE as i8 {
            let square_character = board[current_column as usize][row_index_in_square as usize];
            if color {
                if square_character == 'q' || square_character == 'r' {
                    pieces_attacking_king.push((current_column, current_row));
                    break;
                }else if is_black(square_character) || is_white(square_character) {
                    break;
                }
            }else {
                if square_character == 'Q' || square_character == 'R' {
                    pieces_attacking_king.push((current_column, current_row));
                    break;
                }else if is_black(square_character) || is_white(square_character) {
                    break;
                }
            }
        }else {
            break;
        }
    }

    for above_square in 1..BOARD_SIZE as i8 {
        let row_index_in_square = current_row - above_square;
        
        if row_index_in_square >= 0 {
            let square_character = board[current_column as usize][row_index_in_square as usize];
            if color {
                if square_character == 'q' || square_character == 'r' {
                    pieces_attacking_king.push((current_column, current_row));
                    break;
                }else if is_black(square_character) || is_white(square_character) {
                    break;
                }
            }else {
                if square_character == 'Q' || square_character == 'R' {
                    pieces_attacking_king.push((current_column, current_row));
                    break;
                }else if is_black(square_character) || is_white(square_character) {
                    break;
                }
            }
        }else {
            break;
        }
    }

    for upper_right_diagonal in 1..BOARD_SIZE as i8 {
        let column_index_in_square = current_column + upper_right_diagonal;
        let row_index_in_square = current_row - upper_right_diagonal;
        
        if column_index_in_square < BOARD_SIZE as i8 && row_index_in_square >= 0 {
            let square_character = board[column_index_in_square as usize][row_index_in_square as usize];
            if color {
                if square_character == 'q' || square_character == 'b' {
                    pieces_attacking_king.push((current_column, current_row));
                    break;
                }else if is_black(square_character) || is_white(square_character) {
                    break;
                }
            }else {
                if square_character == 'Q' || square_character == 'B' {
                    pieces_attacking_king.push((current_column, current_row));
                    break;
                }else if is_black(square_character) || is_white(square_character) {
                    break;
                }
            }
        }else {
            break;
        }
    }

    for upper_left_diagonal in 1..BOARD_SIZE as i8 {
        let column_index_in_square = current_column - upper_left_diagonal;
        let row_index_in_square = current_row - upper_left_diagonal;
        
        if column_index_in_square >= 0 && row_index_in_square >= 0 {
            let square_character = board[column_index_in_square as usize][row_index_in_square as usize];
            if color {
                if square_character == 'q' || square_character == 'b' {
                    pieces_attacking_king.push((current_column, current_row));
                    break;
                }else if is_black(square_character) || is_white(square_character) {
                    break;
                }
            }else {
                if square_character == 'Q' || square_character == 'B' {
                    pieces_attacking_king.push((current_column, current_row));
                    break;
                }else if is_black(square_character) || is_white(square_character) {
                    break;
                }
            }
        }else {
            break;
        }
    }

    for lower_left_diagonal in 1..BOARD_SIZE as i8 {
        let column_index_in_square = current_column - lower_left_diagonal;
        let row_index_in_square = current_row + lower_left_diagonal;
        
        if column_index_in_square >= 0 && row_index_in_square < BOARD_SIZE as i8 {
            let square_character = board[column_index_in_square as usize][row_index_in_square as usize];
            if color {
                if square_character == 'q' || square_character == 'b' {
                    pieces_attacking_king.push((current_column, current_row));
                    break;
                }else if is_black(square_character) || is_white(square_character) {
                    break;
                }
            }else {
                if square_character == 'Q' || square_character == 'B' {
                    pieces_attacking_king.push((current_column, current_row));
                    break;
                }else if is_black(square_character) || is_white(square_character) {
                    break;
                }
            }
        }else {
            break;
        }
    }

    for lower_right_diagonal in 1..BOARD_SIZE as i8 {
        let column_index_in_square = current_column + lower_right_diagonal;
        let row_index_in_square = current_row + lower_right_diagonal;
        
        if column_index_in_square < BOARD_SIZE as i8 && row_index_in_square < BOARD_SIZE as i8 {
            let square_character = board[column_index_in_square as usize][row_index_in_square as usize];
            if color {
                if square_character == 'q' || square_character == 'b' {
                    pieces_attacking_king.push((current_column, current_row));
                    break;
                }else if is_black(square_character) || is_white(square_character) {
                    break;
                }
            }else {
                if square_character == 'Q' || square_character == 'B' {
                    pieces_attacking_king.push((current_column, current_row));
                    break;
                }else if is_black(square_character) || is_white(square_character) {
                    break;
                }
            }
        }else {
            break;
        }
    }

    // pawn verification:
    if color {
        if king_position.1 - 1 >= 0 {
            if king_position.0 - 1 >= 0 {
                if board[(king_position.0-1) as usize][(king_position.1-1) as usize] == 'j' {
                    pieces_attacking_king.push((king_position.0-1, king_position.1-1));
                }
            }else if king_position.0 + 1 < BOARD_SIZE as i8 {
                if board[(king_position.0+1) as usize][(king_position.1-1) as usize] == 'j' {
                    pieces_attacking_king.push((king_position.0+1, king_position.1-1));
                }
            }
        }
    }else {
        if king_position.1 + 1 < BOARD_SIZE as i8 {
            if king_position.0-1 >= 0 {
                if board[(king_position.0-1) as usize][(king_position.1+1) as usize] == 'i' {
                    pieces_attacking_king.push((king_position.0-1, king_position.1+1));
                }
            }else if king_position.0+1 < BOARD_SIZE as i8 {
                if board[(king_position.0+1) as usize][(king_position.1+1) as usize] == 'i' {
                    pieces_attacking_king.push((king_position.0+1, king_position.1+1));
                }
            }
        }
    }

    let knight_symbol: char;
    if color {
        knight_symbol = 'n';
    }else {
        knight_symbol = 'N';
    }

    if current_column - 2 >= 0 && current_row - 1 >= 0 {
        if board[(current_column - 2) as usize][(current_row - 1) as usize] == knight_symbol {
            pieces_attacking_king.push((current_column - 2, current_row - 1));
        }
    }
    if current_column - 2 >= 0 && current_row + 1 < BOARD_SIZE as i8 {
        if board[(current_column - 2) as usize][(current_row + 1) as usize] == knight_symbol {
            pieces_attacking_king.push((current_column - 2, current_row + 1));
        }
    }
    if current_column - 1 >= 0 && current_row - 2 >= 0 {
        if board[(current_column - 1) as usize][(current_row - 2) as usize] == knight_symbol {
            pieces_attacking_king.push((current_column - 1, current_row - 2));
        }
    }
    if current_column - 1 >= 0 && current_row + 2 < BOARD_SIZE as i8 {
        if board[(current_column - 1) as usize][(current_row + 2) as usize] == knight_symbol {
            pieces_attacking_king.push((current_column - 1, current_row + 2));
        }
    }
    if current_column + 1 < BOARD_SIZE as i8 && current_row + 2 < BOARD_SIZE as i8 {
        if board[(current_column + 1) as usize][(current_row + 2) as usize] == knight_symbol {
            pieces_attacking_king.push((current_column + 1, current_row + 2));
        }
    }
    if current_column + 1 < BOARD_SIZE as i8 && current_row - 2 >= 0 {
        if board[(current_column + 1) as usize][(current_row - 2) as usize] == knight_symbol {
            pieces_attacking_king.push((current_column + 1, current_row - 2));
        }
    }
    if current_column + 2 < BOARD_SIZE as i8 && current_row + 1 < BOARD_SIZE as i8 {
        if board[(current_column + 2) as usize][(current_row + 1) as usize] == knight_symbol {
            pieces_attacking_king.push((current_column + 2, current_row + 1));
        }
    }
    if current_column + 2 < BOARD_SIZE as i8 && current_row - 1 >= 0 {
        if board[(current_column + 2) as usize][(current_row - 1) as usize] == knight_symbol {
            pieces_attacking_king.push((current_column + 2, current_row - 1));
        }
    }

    pieces_attacking_king
}