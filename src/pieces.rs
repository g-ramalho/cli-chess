use std::io;

use crate::{BOARD_LETTERS, BOARD_SIZE};

pub enum MoveType {
    Normal,
    Castle,
    Promotion
}

pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

pub struct PlayerMovement {
    pub movement_type: MoveType,
    pub is_capture: bool,
    pub target_position: (i8, i8),
    pub p_type: PieceType,
    pub unambiguous_move_partial_position: (i8, i8)
}

pub struct Piece {
    pub positions: Vec<(i8, i8)>, // (column, row) <- position of the piece
    pub color: bool, // true = white / false = black
    pub piece_type: PieceType,
    pub symbol: char
}

impl PlayerMovement {
    pub fn verify_if_move_is_possible(&self, piece: Piece) -> Vec<bool> {
        let target_column = self.target_position.0;
        let target_row = self.target_position.1;

        let mut movable_pieces: Vec<bool> = vec![];
        
        if let MoveType::Castle = self.movement_type {

        }else {
            match self.p_type {
                PieceType::Pawn => {
                    match self.movement_type {
                        MoveType::Normal => {
                            for pawn_position in piece.positions.iter() {
                                let current_column = pawn_position.0;
                                let current_row = pawn_position.1;
                                
                                if (current_row - target_row).abs() == 1 {
                                    if self.is_capture {
                                        if (current_column - target_column).abs() == 1 {
                                            movable_pieces.push(true);
                                            continue;
                                        }
                                    }else {
                                        if current_column == target_column {
                                            movable_pieces.push(true);
                                            continue;
                                        }
                                    }
                                }else if (current_row - target_row).abs() == 2 {
                                    if !self.is_capture {
                                        if current_column == target_column {
                                            if piece.color {
                                                if current_row == 1 {
                                                    movable_pieces.push(true);
                                                    continue;
                                                }
                                            }else {
                                                if current_row == BOARD_SIZE as i8 - 2 {
                                                    movable_pieces.push(true);
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                }
        
                                movable_pieces.push(false);
                            }
                        },
                        MoveType::Promotion => {
                            for pawn_position in piece.positions.iter() {
                                let current_column = pawn_position.0;
                                let current_row = pawn_position.1;

                                if (current_row - target_row).abs() == 1 {
                                    if self.is_capture {
                                        if (current_column - target_column).abs() == 1 {
                                            if piece.color {
                                                if target_row == BOARD_SIZE as i8 - 1 {
                                                    movable_pieces.push(true);
                                                    continue;
                                                }
                                            }else {
                                                if target_row == 0 {
                                                    movable_pieces.push(true);
                                                    continue;
                                                }
                                            }
                                        }
                                    }else {
                                        if current_column == target_column {
                                            movable_pieces.push(true);
                                            continue;
                                        }
                                    }
                                }else if (current_row - target_row).abs() == 2 {
                                    if !self.is_capture {
                                        if current_column == target_column {
                                            if piece.color {
                                                if current_row == 1 {
                                                    if target_row == BOARD_SIZE as i8 - 1 {
                                                        movable_pieces.push(true);
                                                        continue;
                                                    }
                                                }
                                            }else {
                                                if current_row == BOARD_SIZE as i8 - 2 {
                                                    if target_row == 0 {
                                                        movable_pieces.push(true);
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
        
                                movable_pieces.push(false);
                            }
                        },
                        _ => movable_pieces.push(false)
                    }
                },
                PieceType::Knight => movable_pieces.push(false),
                PieceType::Bishop => movable_pieces.push(false),
                PieceType::Rook => movable_pieces.push(false),
                PieceType::Queen => movable_pieces.push(false),
                PieceType::King => movable_pieces.push(false)
            }
        }

        movable_pieces

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
        row = row + 10*row + match san_move[2 + index_offset] {
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
    let mut unambiguous_move_partial_position: (i8, i8) = (0b0, 0b0);
    
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


        if target_position.1 == (BOARD_SIZE as i8) - 1 || target_position.1 == 0{ // if the inputted move takes a piece to the other side of the board
            if let PieceType::Pawn = &p_type { // and it is a pawn move
                movement_type = MoveType::Promotion;
            }
        }

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
