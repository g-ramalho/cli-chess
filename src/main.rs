mod pieces;
use std::vec;
use pieces::*;

const BOARD_SIZE: usize = 8; // max board size: 26x26

fn main() {
    const FREE_SQUARE_SYMBOL: char = '.';
    let mut board = [[FREE_SQUARE_SYMBOL; BOARD_SIZE]; BOARD_SIZE];

    let mut white_pieces = setup_default_board(true);
    let mut black_pieces = setup_default_board(false);

    for pieces in white_pieces.iter() {
        for piece_position in pieces.positions.iter() {
            board[piece_position.0 as usize][piece_position.1 as usize] = pieces.symbol;
        }
    }

    for pieces in black_pieces.iter() {
        for piece_position in pieces.positions.iter() {
            board[piece_position.0 as usize][piece_position.1 as usize] = pieces.symbol;
        }
    }

    loop {
        show_board(&board);
        println!("White moves:");
        let player_move = get_player_move();
        let player_move_piece_type: &Piece = white_pieces.iter_mut().find(|piece| piece.piece_type == player_move.p_type).unwrap();
        let player_move_verified: VerifiedPlayerMovement = player_move.verify_if_move_is_possible(player_move_piece_type);

        if player_move_verified.is_possible {
            if !player_move_verified.is_ambiguous {
                for piece in white_pieces.iter_mut() {
                    if piece.piece_type == player_move.p_type {
                        for position in piece.positions.iter_mut() {
                            if *position == player_move_verified.position_to_move_from {
                                board[position.0 as usize][position.1 as usize] = FREE_SQUARE_SYMBOL;
                                board[player_move.target_position.0 as usize][player_move.target_position.1 as usize] = piece.symbol;
    
                                position.0 = player_move.target_position.0;
                                position.1 = player_move.target_position.1;
                                break;
                            }
                        }
                    }
                }
            }
        }

        show_board(&board);
        println!("Black moves:");
        let player_move = get_player_move();
        let player_move_piece_type: &Piece = black_pieces.iter_mut().find(|piece| piece.piece_type == player_move.p_type).unwrap();
        let player_move_verified: VerifiedPlayerMovement = player_move.verify_if_move_is_possible(player_move_piece_type);

        if player_move_verified.is_possible {
            if !player_move_verified.is_ambiguous {
                for piece in black_pieces.iter_mut() {
                    if piece.piece_type == player_move.p_type {
                        for position in piece.positions.iter_mut() {
                            if *position == player_move_verified.position_to_move_from {
                                board[position.0 as usize][position.1 as usize] = FREE_SQUARE_SYMBOL;
                                board[player_move.target_position.0 as usize][player_move.target_position.1 as usize] = piece.symbol;
    
                                position.0 = player_move.target_position.0;
                                position.1 = player_move.target_position.1;
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
}

fn show_board(board: &[[char;BOARD_SIZE];BOARD_SIZE]) {
    for row in (0..BOARD_SIZE).rev() {
        for column in 0..BOARD_SIZE {
            if column == 0 {
                if row+1 >= 10 {
                    print!("[{}] {} ", (row+1), board[column][row]);
                }else {
                    print!("[{}]  {} ", (row+1), board[column][row]);
                }
            }else if column == BOARD_SIZE - 1 {
                println!(" {}", board[column][row]);
            }else {
                print!(" {} ", board[column][row]);
            }


        }
    }

    print!("    ");
    for letter in 0..BOARD_SIZE {
        if letter == BOARD_SIZE - 1 {
            println!("[{}]\n", BOARD_LETTERS[letter]);
        }else {
            print!("[{}]", BOARD_LETTERS[letter]);
        }
    }
}

const BOARD_LETTERS: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', '#', 'y', 'z']; 
// notice how '#' was used instead of 'x'
