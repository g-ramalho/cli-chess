use crate::{get_piece_color, Piece, PieceType, BOARD_SIZE};

pub struct AttackingPieces {
    pub pieces_attacking_square: Vec<(i8, i8)>,
    pub pinned_pieces: Vec<(i8, i8)>
}

#[derive(PartialEq)]
pub enum EndgameType {
    NotEndgame,
    Checkmate,
    Stalemate,
    InsufficientMaterial,
}

pub fn get_pieces_attacking_square(attacked_piece_color: bool, square_position: (i8, i8), board: &[[char; BOARD_SIZE]; BOARD_SIZE]) -> AttackingPieces {

    let mut pieces_attacking_square: Vec<(i8, i8)> = vec![];
    let mut pinned_pieces: Vec<(i8, i8)> = vec![];
    let mut pinned_pieces_index_offset = 0;

    let current_column: i8 = square_position.0;
    let current_row: i8 = square_position.1;

    let queen_symbol: char;
    let rook_symbol: char;
    let bishop_symbol: char;
    let knight_symbol: char;

    if attacked_piece_color {
        queen_symbol = 'q';
        rook_symbol = 'r';
        bishop_symbol = 'b';
        knight_symbol = 'n';
    }else {
        queen_symbol = 'Q';
        rook_symbol = 'R';
        bishop_symbol = 'B';
        knight_symbol = 'N';
    }

    for (direction_index, cardinal_direction) in [(-1, 0), (0, -1), (1, 0), (0, 1), (-1, -1), (1, -1), (1, 1), (-1, 1)].iter().enumerate() {
    // left, down, right, up, lower-left, lower-right, upper-right, upper-left

    let mut path_that_attacking_piece_was_found_in: Vec<((i8, i8), usize)> = vec![];
    let mut pinned_pieces_briefing: Vec<((i8, i8), usize)> = vec![];

        for square in 1..BOARD_SIZE as i8 {
            let column_index_in_square = current_column + square*cardinal_direction.0;
            let row_index_in_square = current_row + square*cardinal_direction.1;
            let position = (column_index_in_square, row_index_in_square);

            let is_diagonal_movement = cardinal_direction.0 != 0 && cardinal_direction.1 != 0;

            let is_column_in_board = column_index_in_square >= 0 && column_index_in_square < BOARD_SIZE as i8;
            let is_row_in_board = row_index_in_square >= 0 && row_index_in_square < BOARD_SIZE as i8;

            if is_column_in_board && is_row_in_board {
                let square_character = board[position.0 as usize][position.1 as usize];

                match get_piece_color(square_character) {
                    Some(b) => { // a piece was identified
                        if b == attacked_piece_color {
                            pinned_pieces_briefing.push((position, direction_index));
                        }else {
                            if square_character == queen_symbol {
                                pieces_attacking_square.insert(pinned_pieces_index_offset, position);
                                path_that_attacking_piece_was_found_in.push((position, direction_index));
                                break;
                            }else if is_diagonal_movement {
                                if square_character == bishop_symbol {
                                    pieces_attacking_square.insert(pinned_pieces_index_offset, position);
                                    path_that_attacking_piece_was_found_in.push((position, direction_index));
                                    break;
                                }
                            }else {
                                if square_character == rook_symbol {
                                    pieces_attacking_square.insert(pinned_pieces_index_offset, position);
                                    path_that_attacking_piece_was_found_in.push((position, direction_index));
                                    break;
                                }else {
                                    break; // opposite color piece that is not attacking the square
                                }
                            }
                        }
                    },
                    None => (), // not a piece ('.' / FREE_SQUARE_SYMBOL)
                }
            }else {
                break;
            }
        }

        if pinned_pieces_briefing.len() == 1 && path_that_attacking_piece_was_found_in.len() == 1 {
            pinned_pieces.insert(pinned_pieces_index_offset, pinned_pieces_briefing[0].0);
        }

        // if pieces_attacking_square.len() < pinned_pieces.len() {
        //     // if there is a "pinned piece" but no "pinning piece",
        //     // the pinned piece is not actually being pinned by anything
        //     pinned_pieces.remove(pinned_pieces_index_offset);
        // }
        // // if the identified piece is the same color as the piece being attacked, it is a pinned piece
        // if pinned_pieces.len() >= pinned_pieces_index_offset+1 {
        //     // if there are more pieces that are the same color of the attacked on the way, none are pinned
        //     pinned_pieces.remove(pinned_pieces_index_offset);
        // }else{
        //     pinned_pieces.insert(pinned_pieces_index_offset, position);
        // }

        if pinned_pieces.len() == pinned_pieces_index_offset+1 {
            // only one pinned piece may exist per pinning piece
            pinned_pieces_index_offset += 1;
        }
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
    let row_to_search_for_pawn: i8;
    let pawn_symbol: char;
    if attacked_piece_color {
        row_to_search_for_pawn = 1;
        pawn_symbol = 'j';
    }else {
        row_to_search_for_pawn = -1;
        pawn_symbol = 'i';
    }

    if current_row + row_to_search_for_pawn >= 0 && current_row + row_to_search_for_pawn < BOARD_SIZE as i8 {
        if current_column + 1 < BOARD_SIZE as i8 {
            if board[(current_column + 1) as usize][(current_row + row_to_search_for_pawn) as usize] == pawn_symbol {
                pieces_attacking_square.push((current_column + 1, current_row + row_to_search_for_pawn));
            }
        }else if current_column - 1 >= 0 {
            if board[(current_column - 1) as usize][(current_row + row_to_search_for_pawn) as usize] == pawn_symbol {
                pieces_attacking_square.push((current_column - 1, current_row + row_to_search_for_pawn));
            }
        }
    }

    AttackingPieces {
        pieces_attacking_square,
        pinned_pieces
    }
}

pub fn get_safe_squares_for_king(attacked_piece_color: bool, square_position: (i8, i8), board: &[[char; BOARD_SIZE]; BOARD_SIZE]) -> Vec<(i8, i8)> {

    let current_column = square_position.0;
    let current_row = square_position.1;

    let mut safe_squares = vec![];

    for cardinal_direction in [(-1, -1), (-1, 1), (1, -1), (1, 1), (1, 0), (0, 1), (-1, 0), (0, -1)].iter() {
        let column_index_in_square = current_column + cardinal_direction.0;
        let row_index_in_square = current_row + cardinal_direction.1;
        let position = (column_index_in_square, row_index_in_square);

        let is_column_in_board = column_index_in_square >= 0 && column_index_in_square < BOARD_SIZE as i8;
        let is_row_in_board = row_index_in_square >= 0 && row_index_in_square < BOARD_SIZE as i8;

        if is_column_in_board && is_row_in_board {
            let attacking_pieces = get_pieces_attacking_square(attacked_piece_color, position, &board);
            if attacking_pieces.pieces_attacking_square.len() == 0 || attacking_pieces.pinned_pieces.len() == attacking_pieces.pieces_attacking_square.len() {
                if !get_piece_color(board[position.0 as usize][position.1 as usize]).is_some_and(|color| color == attacked_piece_color) {
                    safe_squares.push(position);
                }
            }
        }
    }

    safe_squares
}

pub fn get_pieces_that_can_block_attack(attacking_piece_position: (i8, i8), attacked_piece: (bool, &(i8, i8)), board: &[[char; BOARD_SIZE]; BOARD_SIZE]) -> Vec<(i8, i8)> {
    
    let mut pieces_that_can_block_check: Vec<(i8, i8)> = vec![];

    let current_column = attacked_piece.1.0;
    let current_row = attacked_piece.1.1;
    let target_column = attacking_piece_position.0;
    let target_row = attacking_piece_position.1;

    let mut iterator: i8 = 1;
    let is_diagonal_movement = (target_column - current_column).abs() == (target_row - current_row).abs();

    if current_column == target_column || is_diagonal_movement {
        iterator = (target_row - current_row).abs();
    }else if current_row == target_row {
        iterator = (target_column - current_column).abs();
    }

    let knight_symbol: char;
    if attacked_piece.0 {
        knight_symbol = 'n';
    }else {
        knight_symbol = 'N';
    }

    if board[attacking_piece_position.0 as usize][attacking_piece_position.1 as usize] != knight_symbol { // knight checks can't be blocked
        for square in 1..=iterator {
            let column_index_in_square = (target_column - current_column).signum() * square;
            let row_index_in_square = (target_row - current_row).signum() * square;
    
            let position = (column_index_in_square, row_index_in_square);
        
            let square_is_not_target = position.0 + current_column != target_column || position.1 + current_row != target_row;
    
            if square_is_not_target {
                let pieces_reaching_square = get_pieces_attacking_square(!attacked_piece.0, position, &board).pieces_attacking_square;
    
                for piece in pieces_reaching_square.iter() {
                    pieces_that_can_block_check.push(*piece);
                }
            }else{
                break;
            }
        }
    }

    pieces_that_can_block_check
}

pub fn is_piece_movable(piece: &Piece, piece_position: (i8, i8), board: &[[char; BOARD_SIZE]; BOARD_SIZE]) -> bool {

    let mut is_piece_movable = false;

    let current_column = piece_position.0;
    let current_row = piece_position.1;

    match piece.piece_type {
        PieceType::Knight => {
            for knight_movement_possibility in [(-2, -1), (-2, 1), (-1, -2), (-1, 2), (1, 2), (1, -2), (2, 1), (2, -1)].iter() {
                let column_index_in_square = current_column + knight_movement_possibility.0;
                let row_index_in_square = current_row + knight_movement_possibility.1;
                let position = (column_index_in_square, row_index_in_square);
                
                let is_column_in_board = column_index_in_square >= 0 && column_index_in_square < BOARD_SIZE as i8;
                let is_row_in_board = row_index_in_square >= 0 && row_index_in_square < BOARD_SIZE as i8;
        
                if is_column_in_board && is_row_in_board {
                    let square_character = board[position.0 as usize][position.1 as usize];
                    if !get_piece_color(square_character).is_some_and(|color| color == piece.color) {
                        is_piece_movable = true;
                        break;
                    }
                }
            }
        },
        PieceType::Pawn => {
            let iterator: std::slice::Iter<'_, (i8, i8)>;
            if piece.color {
                iterator = [(0, 2), (0, 1), (-1, 1), (1, 1)].iter();
            }else {
                iterator = [(0, -2), (0, -1), (1, -1), (-1, -1)].iter();
            }

            for possible_position in iterator {
                let column_index_in_square = current_column + possible_position.0;
                let row_index_in_square = current_row + possible_position.1;
                let position = (column_index_in_square, row_index_in_square);

                let is_column_in_board = column_index_in_square >= 0 && column_index_in_square < BOARD_SIZE as i8;
                let is_row_in_board = row_index_in_square >= 0 && row_index_in_square < BOARD_SIZE as i8;

                if is_column_in_board && is_row_in_board {
                    let square_character = board[position.0 as usize][position.1 as usize];
                    if (possible_position.0).abs() == (possible_position.1).abs() { // diagonal movement
                        if get_piece_color(square_character).is_some_and(|color| color != piece.color) {
                            is_piece_movable = true;
                            break;
                        }
                    }else {
                        if (possible_position.1).abs() == 2 {
                            if piece.color {
                                if current_row == 1 && get_piece_color(square_character).is_none() && get_piece_color(board[position.0 as usize][(position.1 - 1) as usize]).is_none() {
                                    is_piece_movable = true;
                                    break;
                                }
                            }else {
                                if current_row == BOARD_SIZE as i8 - 2 && get_piece_color(square_character).is_none() && get_piece_color(board[position.0 as usize][(position.1 + 1) as usize]).is_none() {
                                    is_piece_movable = true;
                                    break;
                                }
                            }
                        }else {
                            if get_piece_color(square_character).is_none() {
                                is_piece_movable = true;
                                break;
                            }
                        }
                    }
                }
            }
        },
        _ => {
            for cardinal_direction in [(-1, 0), (0, -1), (1, 0), (0, 1), (-1, -1), (1, -1), (1, 1), (-1, 1)].iter() {
                // left, up, right, down, upper-left, upper-right, lower-right, lower-left
                let column_index_in_square = current_column + cardinal_direction.0;
                let row_index_in_square = current_row + cardinal_direction.1;
                let position = (column_index_in_square, row_index_in_square);
    
                let is_diagonal_movement = cardinal_direction.0 != 0 && cardinal_direction.1 != 0;
    
                let is_column_in_board = position.0 >= 0 && position.0 < BOARD_SIZE as i8;
                let is_row_in_board = position.1 >= 0 && position.1 < BOARD_SIZE as i8;
    
                if is_column_in_board && is_row_in_board {
                    let square_character = board[position.0 as usize][position.1 as usize];

                    if (is_diagonal_movement && piece.piece_type != PieceType::Rook) || (!is_diagonal_movement && piece.piece_type != PieceType::Bishop) {
                        if !get_piece_color(square_character).is_some_and(|color| color == piece.color) {
                            is_piece_movable = true;
                            break;
                        }
                    }
                }
            }
        }
    }

    is_piece_movable
}

pub fn get_game_state(pieces: &Vec<Piece>, board: &[[char; BOARD_SIZE]; BOARD_SIZE], opposite_side_pieces: &Vec<Piece>) -> EndgameType {

    let king_piece: &Piece = pieces.iter().find(|piece: &&Piece| piece.piece_type == PieceType::King).unwrap();
    let king_position = king_piece.positions[0];

    let pieces_attacking_king = get_pieces_attacking_square(king_piece.color, king_position, &board);
    let kings_safe_squares = get_safe_squares_for_king(king_piece.color, king_position, &board);

    if kings_safe_squares.len() == 0 {
        if pieces_attacking_king.pieces_attacking_square.len() == 2 {
            return EndgameType::Checkmate
        }else if pieces_attacking_king.pieces_attacking_square.len() == 1 && pieces_attacking_king.pieces_attacking_square.len() != pieces_attacking_king.pinned_pieces.len() {
            let attacking_piece_position = pieces_attacking_king.pieces_attacking_square[0];
            if get_pieces_attacking_square(!king_piece.color, attacking_piece_position, &board).pieces_attacking_square.len() < 1 {
                // the player cant take the attacking piece
                if get_pieces_that_can_block_attack(attacking_piece_position, (king_piece.color, &king_position), &board).len() == 0 {
                    // the player has no pieces that can block the check
                    return EndgameType::Checkmate
                }
            }
        }else { // ideally no pieces are checking the king (this section tests if the game is drawn)
            for piece in pieces.iter() {
                if piece.piece_type != PieceType::King {
                    for piece_position in piece.positions.iter() {
                        if is_piece_movable(&piece, *piece_position, &board) {
                            if pieces_attacking_king.pinned_pieces.iter().find(|position| *position == piece_position).is_none() {
                                return EndgameType::NotEndgame;
                            }
                        }
                    }
                }
            }

            return EndgameType::Stalemate; // will only trigger if no piece is movable
        }
    }

    let white_pawns = pieces.iter().find(|piece| piece.piece_type == PieceType::Pawn).unwrap();
    let black_pawns = opposite_side_pieces.iter().find(|piece| piece.piece_type == PieceType::Pawn).unwrap();

    if white_pawns.positions.len() == 0 && black_pawns.positions.len() == 0 {

        let mut white_queen_quantity: usize = 0;
        let mut white_rook_quantity: usize = 0;
        let mut white_bishop_quantity: usize = 0;
        let mut white_knight_quantity: usize = 0;

        for piece in pieces.iter() {
            match piece.piece_type {
                PieceType::Queen => { white_queen_quantity = piece.positions.len() },
                PieceType::Rook => { white_rook_quantity = piece.positions.len() },
                PieceType::Bishop => { white_bishop_quantity = piece.positions.len() },
                PieceType::Knight => { white_knight_quantity = piece.positions.len() },
                PieceType::Pawn | PieceType::King => continue
            }
        }

        let mut black_queen_quantity: usize = 0;
        let mut black_rook_quantity: usize = 0;
        let mut black_bishop_quantity: usize = 0;
        let mut black_knight_quantity: usize = 0;

        for piece in pieces.iter() {
            match piece.piece_type {
                PieceType::Queen => { black_queen_quantity = piece.positions.len() },
                PieceType::Rook => { black_rook_quantity = piece.positions.len() },
                PieceType::Bishop => { black_bishop_quantity = piece.positions.len() },
                PieceType::Knight => { black_knight_quantity = piece.positions.len() },
                PieceType::Pawn | PieceType::King => continue
            }
        }

        if white_queen_quantity + white_rook_quantity + black_queen_quantity + black_rook_quantity == 0 {
            let is_bishops_checkmate_possible = white_bishop_quantity >= 2 || black_bishop_quantity >= 2;
            let is_knight_and_bishop_checkmate_possible = (white_bishop_quantity >= 1 && white_knight_quantity >= 1) || (black_bishop_quantity >= 1 && black_knight_quantity >= 1);

            if !is_bishops_checkmate_possible && !is_knight_and_bishop_checkmate_possible {
                return EndgameType::InsufficientMaterial;
            }
        } 
    }

    EndgameType::NotEndgame
}
