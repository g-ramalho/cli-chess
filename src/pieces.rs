use std::io;
use crate::{endgame::get_pieces_attacking_the_king, BOARD_LETTERS, BOARD_SIZE};

#[derive(PartialEq)]
pub enum MoveType {
    Normal,
    Castle
}

#[derive(PartialEq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

pub struct Piece {
    pub positions: Vec<(i8, i8)>, // (column, row) <- position of the piece
    pub color: bool, // true = white / false = black
    pub piece_type: PieceType,
    pub symbol: char
}

impl Piece {
    pub fn new(positions: Vec<(i8, i8)>, color: bool, piece_type: PieceType, symbol: char) -> Self {
        Self {
            positions,
            color,
            piece_type,
            symbol
        }
    }
}

pub fn setup_default_board_positions(color: bool) -> Vec<Piece> {
    let mut pieces_vector: Vec<Piece> = vec![];

    let mut symbol: char;
    
    let mut pawn_positions_vector: Vec<(i8, i8)> = vec![];
    if color { 
        for pawn_position in 0..8 {
            pawn_positions_vector.push((pawn_position, 1));
        }
        symbol = 'i';
    }else {
        for pawn_position in 0..8 {
            pawn_positions_vector.push((pawn_position, 6));
        }
        symbol = 'j';
    }

    pieces_vector.push(Piece::new(pawn_positions_vector, color, PieceType::Pawn, symbol));

    let mut knight_positions_vector: Vec<(i8, i8)> = vec![];
    if color {
        knight_positions_vector.push((1, 0));
        knight_positions_vector.push((6, 0));
        symbol = 'N';
    }else {
        knight_positions_vector.push((6, 7));
        knight_positions_vector.push((1, 7));
        symbol = 'n';
    }

    pieces_vector.push(Piece::new(knight_positions_vector, color, PieceType::Knight, symbol));

    let mut bishop_positions_vector: Vec<(i8, i8)> = vec![];
    if color {
        bishop_positions_vector.push((2, 0));
        bishop_positions_vector.push((5, 0));
        symbol = 'B';
    }else {
        bishop_positions_vector.push((2, 7));
        bishop_positions_vector.push((5, 7));
        symbol = 'b';
    }

    pieces_vector.push(Piece::new(bishop_positions_vector, color, PieceType::Bishop, symbol));

    let mut rook_positions_vector: Vec<(i8, i8)> = vec![];
    if color {
        rook_positions_vector.push((0, 0));
        rook_positions_vector.push((7, 0));
        symbol = 'R';
    }else {
        rook_positions_vector.push((0, 7));
        rook_positions_vector.push((7, 7));
        symbol = 'r';
    }

    pieces_vector.push(Piece::new(rook_positions_vector, color, PieceType::Rook, symbol));

    let mut queen_positions_vector: Vec<(i8, i8)> = vec![];
    if color {
        queen_positions_vector.push((3, 0));
        symbol = 'Q';
    }else {
        queen_positions_vector.push((3, 7));
        symbol = 'q';
    }

    pieces_vector.push(Piece::new(queen_positions_vector, color, PieceType::Queen, symbol));

    let mut king_positions_vector: Vec<(i8, i8)> = vec![];
    if color {
        king_positions_vector.push((4, 0));
        symbol = 'K';
    }else {
        king_positions_vector.push((4, 7));
        symbol = 'k';
    }

    pieces_vector.push(Piece::new(king_positions_vector, color, PieceType::King, symbol));

    pieces_vector

}

pub struct PlayerMovement {
    pub movement_type: MoveType,
    pub is_capture: bool,
    pub target_position: (i8, i8),
    pub p_type: PieceType,
    pub unambiguous_move_partial_position: (i8, i8)
}

pub struct VerifiedPlayerMovement {
    pub is_possible: bool,
    pub is_ambiguous: bool,
    pub index_position_to_move_from: usize
}

impl PlayerMovement {
    pub fn verify_if_move_is_possible(&self, piece: &Piece, board: &[[char; BOARD_SIZE]; BOARD_SIZE]) -> VerifiedPlayerMovement {
        let target_column = self.target_position.0;
        let target_row = self.target_position.1;

        let mut index_position_to_move_from = 27;
        let mut is_possible = false;
        let mut is_ambiguous = false;
        
        if let MoveType::Castle = self.movement_type {

        }else {
            match self.p_type {
                PieceType::Pawn => {
                    for pawn_position_index in 0..piece.positions.len() {
                        let current_column = piece.positions[pawn_position_index].0;
                        let current_row = piece.positions[pawn_position_index].1;
                        
                        if (current_row - target_row).abs() == 1 {
                            if self.is_capture {
                                if (current_column - target_column).abs() == 1 {
                                    if !is_possible {
                                        is_possible = true;
                                        index_position_to_move_from = pawn_position_index;
                                    }else {
                                        is_ambiguous = true;
                                        break;
                                    }
                                }
                            }else {
                                if current_column == target_column {
                                    if !is_possible {
                                        is_possible = true;
                                        index_position_to_move_from = pawn_position_index;
                                    }else {
                                        is_ambiguous = true;
                                        break;
                                    }
                                }
                            }
                        }else if (current_row - target_row).abs() == 2 {
                            if !self.is_capture {
                                if current_column == target_column {
                                    if piece.color {
                                        if current_row == 1 {
                                            if !is_possible {
                                                is_possible = true;
                                                index_position_to_move_from = pawn_position_index;
                                            }else {
                                                is_ambiguous = true;
                                                break;
                                            }
                                        }
                                    }else {
                                        if current_row == BOARD_SIZE as i8 - 2 {
                                            if !is_possible {
                                                is_possible = true;
                                                index_position_to_move_from = pawn_position_index;
                                            }else {
                                                is_ambiguous = true;
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                PieceType::Knight => {
                    for knight_position_index in 0..piece.positions.len() {
                        let current_column = piece.positions[knight_position_index].0;
                        let current_row = piece.positions[knight_position_index].1;

                        let is_valid_horizontal_movement: bool = (target_column - current_column).abs() == 2 && (target_row - current_row).abs() == 1;
                        let is_valid_vertical_movement: bool = (target_column - current_column).abs() == 1 && (target_row - current_row).abs() == 2;

                        if is_valid_horizontal_movement || is_valid_vertical_movement {
                            if !is_possible {
                                is_possible = true;
                                index_position_to_move_from = knight_position_index;
                            }else {
                                is_ambiguous = true;
                                break;
                            }
                        }
                    }
                },
                PieceType::Bishop => {
                    for bishop_position_index in 0..piece.positions.len() {
                        let current_column = piece.positions[bishop_position_index].0;
                        let current_row = piece.positions[bishop_position_index].1;

                        let is_diagonal_movement = (target_column - current_column).abs() == (target_row - current_row).abs();
                        if is_diagonal_movement {

                            for diagonal in 1..=(target_row - current_row).abs() {
                                let row_index_in_diagonal = (target_row - current_row).signum() * diagonal;
                                let column_index_in_diagonal = (target_column - current_column).signum() * diagonal;
                                let square_character = board[(column_index_in_diagonal + current_column) as usize][(row_index_in_diagonal + current_row) as usize];

                                let square_is_not_target = row_index_in_diagonal + current_row != target_row || column_index_in_diagonal + current_column != target_column;

                                if square_is_not_target {
                                    if is_white(square_character) || is_black(square_character) {
                                        break;
                                    }
                                }else {
                                    if !is_possible {
                                        is_possible = true;
                                        index_position_to_move_from = bishop_position_index;
                                    }else {
                                        is_ambiguous = true;
                                        break;
                                    }
                                }
                            }

                        }
                    }
                },
                PieceType::Rook => {
                    for rook_position_index in 0..piece.positions.len() {
                        let current_column = piece.positions[rook_position_index].0;
                        let current_row = piece.positions[rook_position_index].1;

                        let iterator: i8;
                        if current_column == target_column {
                            iterator = (target_row - current_row).abs();
                        }else if current_row == target_row {
                            iterator = (target_column - current_column).abs();
                        }else {
                            continue;
                        }

                        for square in 1..=iterator {
                            let row_index_in_square = (target_row - current_row).signum() * square;
                            let column_index_in_square = (target_column - current_column).signum() * square;
                            let square_character = board[(column_index_in_square + current_column) as usize][(row_index_in_square + current_row) as usize];
                        
                            let square_is_not_target = row_index_in_square + current_row != target_row || column_index_in_square + current_column != target_column;

                            if square_is_not_target {
                                if is_white(square_character) || is_black(square_character) {
                                    break;
                                }
                            }else {
                                if !is_possible {
                                    is_possible = true;
                                    index_position_to_move_from = rook_position_index;
                                }else {
                                    is_ambiguous = true;
                                    break;
                                }
                            }
                        }
                    }
                },
                PieceType::Queen => {
                    for queen_position_index in 0..piece.positions.len() {
                        let current_column = piece.positions[queen_position_index].0;
                        let current_row = piece.positions[queen_position_index].1;

                        let iterator: i8;
                        let is_diagonal_movement = (target_column - current_column).abs() == (target_row - current_row).abs();

                        if current_column == target_column || is_diagonal_movement {
                            iterator = (target_row - current_row).abs();
                        }else if current_row == target_row {
                            iterator = (target_column - current_column).abs();
                        }else {
                            continue;
                        }

                        for square in 1..=iterator {
                            let row_index_in_square = (target_row - current_row).signum() * square;
                            let column_index_in_square = (target_column - current_column).signum() * square;
                            let square_character = board[(column_index_in_square + current_column) as usize][(row_index_in_square + current_row) as usize];
                        
                            let square_is_not_target = row_index_in_square + current_row != target_row || column_index_in_square + current_column != target_column;

                            if square_is_not_target {
                                if is_white(square_character) || is_black(square_character) {
                                    break;
                                }
                            }else {
                                if !is_possible {
                                    is_possible = true;
                                    index_position_to_move_from = queen_position_index;
                                }else {
                                    is_ambiguous = true;
                                    break;
                                }
                            }
                        }
                    }
                },
                PieceType::King => ()
            }
        }

        VerifiedPlayerMovement {
            is_possible,
            is_ambiguous,
            index_position_to_move_from
        }

    }
}

pub fn get_piece_to_promote_to(pieces_vec: &mut Vec<Piece>) -> &mut Piece {

    println!("Indicate the pawn promotion: (Press Enter to promote it to a Queen)");
    println!("'N'=Knight,'R'=Rook,'B'=Bishop");

    let piece_to_promote_to: &mut Piece;

    loop {
        let mut promotion_input = String::new();
        io::stdin().read_line(&mut promotion_input).unwrap();
        let promotion_characters: Vec<char> = promotion_input.trim().chars().collect();

        if promotion_characters.len() > 0 {
            match promotion_characters[0] {
                'N' => {
                    piece_to_promote_to = pieces_vec.iter_mut().find(|piece| piece.piece_type == PieceType::Knight).unwrap();
                    break;
                },
                'R' => {
                    piece_to_promote_to = pieces_vec.iter_mut().find(|piece| piece.piece_type == PieceType::Rook).unwrap();
                    break;
                },
                'B' => {
                    piece_to_promote_to = pieces_vec.iter_mut().find(|piece| piece.piece_type == PieceType::Bishop).unwrap();
                    break;
                }
                _ => ()
            }
        }else { // only Enter key was pressed
            piece_to_promote_to = pieces_vec.iter_mut().find(|piece| piece.piece_type == PieceType::Queen).unwrap();
            break;
        }
    }

    piece_to_promote_to

}

fn translate_san_into_position(san_move: &Vec<char>, index_offset: &usize) -> (i8, i8) {
    let mut column = 27;
    let mut row: i8;

    for letter_index in 0..BOARD_LETTERS.len() {
        if BOARD_LETTERS[letter_index] == san_move[0 + index_offset] {
            column = letter_index as i8;
        }
    }

    row = match san_move[1 + index_offset] {
        '1' => 0,
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        '9' => 8,
        _ => 27
    };

    if san_move.len() > 2 + index_offset && row <= 1 {
        row = 10*row + match san_move[2 + index_offset] {
            '0' => 9,
            '1' => 10,
            '2' => 11,
            '3' => 12,
            '4' => 13,
            '5' => 14,
            '6' => 15,
            '7' => 16,
            '8' => 17,
            '9' => 18,
            _ => 0
        };

    }

    (column, row)

}

pub fn get_player_move() -> PlayerMovement {
    let mut target_position: (i8, i8) = (27, 27);
    let mut unambiguous_move_partial_position: (i8, i8) = (27, 27);
    
    let mut p_type: PieceType;
    let mut movement_type: MoveType = MoveType::Normal;
    let mut is_capture: bool = false;

    loop {
        let mut player_move = String::new();
        io::stdin().read_line(&mut player_move).unwrap();
        let san_move: Vec<char> = player_move.trim().chars().collect();

        let mut index_offset = 1;
        p_type = match san_move[0] {
            'N' => PieceType::Knight,
            'B' => PieceType::Bishop,
            'R' => PieceType::Rook,
            'Q' => PieceType::Queen,
            'K' => PieceType::King,
            _ => { if san_move.len() < 4 { index_offset -= 1; }; PieceType::Pawn }
        };

        if player_move == "O-O-O" || player_move == "0-0-0" || player_move == "O-O" || player_move == "0-0" {
            movement_type = MoveType::Castle;
        }else {
            if san_move.len() >= 4 {
                // only one character is used to make a move unambiguous, so it can be both a row number or a column letter
                // because of that, the same character is passed twice to check for either a column or a row value
                let unambiguous_san = vec![san_move[1], san_move[1]];
                unambiguous_move_partial_position = translate_san_into_position(&unambiguous_san, &0);

                if san_move[index_offset] == 'x' {
                    is_capture = true;
                }
                index_offset += 1;
            }

            target_position.0 = translate_san_into_position(&san_move, &index_offset).0;
            target_position.1 = translate_san_into_position(&san_move, &index_offset).1;

            if target_position.0 > BOARD_SIZE as i8 - 1 || target_position.1 > BOARD_SIZE as i8 - 1 {
                println!("Invalid move. Try again!");
            }else {
                break;
            }
        }
    }
    
    PlayerMovement {
        movement_type,
        is_capture,
        target_position,
        p_type,
        unambiguous_move_partial_position
    }

}

pub fn is_white(piece: char) -> bool {
    match piece {
        'i'|'N'|'B'|'R'|'Q'|'K' => true,
        _ => false
    }
}

pub fn is_black(piece: char) -> bool {
    match piece {
        'j'|'n'|'b'|'r'|'q'|'k' => true,
        _ => false
    }
}

