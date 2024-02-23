use std::io;

use crate::{BOARD_LETTERS, BOARD_SIZE};

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
    pub fn new(optional_positions: Option<Vec<(i8, i8)>>, color: bool, piece_type: PieceType, optional_symbol: Option<char>) -> Self {
        let mut positions: Vec<(i8, i8)> = vec![];
        let symbol: char;

        match optional_positions {
            None => {
                match piece_type {
                    PieceType::Pawn => {
                        if color { 
                            for pawn_position in 0..8 {
                                positions.push((pawn_position, 1));
                            }
                        }else {
                            for pawn_position in 0..8 {
                                positions.push((pawn_position, 6));
                            }
                        }
                    },
                    PieceType::Knight => {
                        if color {
                            positions.push((1, 0));
                            positions.push((6, 0));
                        }else {
                            positions.push((6, 7));
                            positions.push((1, 7));
                        }
                    },
                    PieceType::Bishop => {
                        if color {
                            positions.push((2, 0));
                            positions.push((5, 0));
                        }else {
                            positions.push((2, 7));
                            positions.push((5, 7));
                        }
                    },
                    PieceType::Rook => {
                        if color {
                            positions.push((0, 0));
                            positions.push((7, 0));
                        }else {
                            positions.push((0, 7));
                            positions.push((7, 7));
                        }
                    },
                    PieceType::Queen => {
                        if color {
                            positions.push((3, 0));
                        }else {
                            positions.push((3, 7));
                        }
                    },
                    PieceType::King => {
                        if color {
                            positions.push((4, 0));
                        }else {
                            positions.push((4, 7));
                        }
                    }
                }
            },
            Some(p) => {
                positions = p;
            }
        }

        match optional_symbol {
            None => {
                match piece_type {
                    PieceType::Pawn => {
                        if color { symbol = 'i'; } else { symbol = 'j'; }
                    },
                    PieceType::Knight => {
                        if color { symbol = 'N'; } else { symbol = 'n'; }
                    },
                    PieceType::Bishop => {
                        if color { symbol = 'B'; } else { symbol = 'b'; }
                    },
                    PieceType::Rook => {
                        if color { symbol = 'R'; } else { symbol = 'r'; }
                    },
                    PieceType::Queen => {
                        if color { symbol = 'Q'; } else { symbol = 'q'; }
                    },
                    PieceType::King => {
                        if color { symbol = 'K'; } else { symbol = 'k'; }
                    }
                }
            },
            Some(s) => {
                symbol = s;
            }
        }
        
        Self {
            positions,
            color,
            piece_type,
            symbol
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

pub fn setup_default_board(color: bool) -> Vec<Piece> {
    let mut pieces_vector: Vec<Piece> = vec![];

    pieces_vector.push(Piece::new(None, color, PieceType::Pawn,     None));
    pieces_vector.push(Piece::new(None, color, PieceType::Knight,   None));
    pieces_vector.push(Piece::new(None, color, PieceType::Bishop,   None));
    pieces_vector.push(Piece::new(None, color, PieceType::Rook,     None));
    pieces_vector.push(Piece::new(None, color, PieceType::Queen,    None));
    pieces_vector.push(Piece::new(None, color, PieceType::King,     None));

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
    pub fn verify_if_move_is_possible(&self, piece: &Piece) -> VerifiedPlayerMovement {
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
                PieceType::Knight => (),
                PieceType::Bishop => (),
                PieceType::Rook => (),
                PieceType::Queen => (),
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
    
    let mut movement_type: MoveType = MoveType::Normal;
    let mut is_capture: bool = false;

    let mut player_move = String::new();
    io::stdin().read_line(&mut player_move).unwrap();
    let san_move: Vec<char> = player_move.trim().chars().collect();

    let mut index_offset = 0;
    let p_type = match san_move[0] {
        'N' => { index_offset += 1; PieceType::Knight },
        'B' => { index_offset += 1; PieceType::Bishop },
        'R' => { index_offset += 1; PieceType::Rook },
        'Q' => { index_offset += 1; PieceType::Queen },
        'K' => { index_offset += 1; PieceType::King },
        _ => PieceType::Pawn
    };

    if player_move == "O-O-O" || player_move == "0-0-0" || player_move == "O-O" || player_move == "0-0" {
        movement_type = MoveType::Castle;
    }else {
        if san_move[1 + index_offset] == 'x' && san_move.len() > 3 {
            is_capture = true;
            index_offset += 1;
        }
        if san_move.len() >= 4 {
            // only one character is used to make a move unambiguous, so it can be both a row number or a column letter
            // because of that, the same character is passed twice to check for either a column or a row value
            let unambiguous_san = vec![san_move[1], san_move[1]]; 
            unambiguous_move_partial_position = translate_san_into_position(&unambiguous_san, &0);
            index_offset += 1;
        }

        target_position.0 = translate_san_into_position(&san_move, &index_offset).0;
        target_position.1 = translate_san_into_position(&san_move, &index_offset).1;

        if target_position.0 > BOARD_SIZE as i8 - 1 || target_position.1 > BOARD_SIZE as i8 - 1 {
            println!("Invalid move. Try again!");
            get_player_move();
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
