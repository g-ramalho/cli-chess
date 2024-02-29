mod pieces;
mod endgame;
use pieces::*;

const BOARD_SIZE: usize = 8; // max board size: 26x26
const FREE_SQUARE_SYMBOL: char = '.';

fn main() {
    let mut board = [[FREE_SQUARE_SYMBOL; BOARD_SIZE]; BOARD_SIZE];

    let mut white_pieces = setup_default_board_positions(true);
    let mut black_pieces = setup_default_board_positions(false);

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
        play_turn(&mut white_pieces, &mut board, &mut black_pieces);

        show_board(&board);
        println!("Black moves:");
        play_turn(&mut black_pieces, &mut board, &mut white_pieces);
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

fn play_turn(pieces: &mut Vec<Piece>, board: &mut [[char;BOARD_SIZE];BOARD_SIZE], opposite_side_pieces: &mut Vec<Piece>) {
    let mut turn_ongoing = true;
    while turn_ongoing {
        let player_move = get_player_move();
        let mut player_move_piece_type: &mut Piece = pieces.iter_mut().find(|piece: &&mut Piece| piece.piece_type == player_move.p_type).unwrap();
        let player_move_verified: VerifiedPlayerMovement = player_move.verify_if_move_is_possible(player_move_piece_type, &board);

        if player_move_verified.is_possible {
            let target_square_character = board[player_move.target_position.0 as usize][player_move.target_position.1 as usize];
            
            if player_move.movement_type == MoveType::Castle {
                board[player_move_piece_type.positions[0].0 as usize][player_move_piece_type.positions[0].1 as usize] = FREE_SQUARE_SYMBOL;

                let rook_old_column: i8;
                let rook_new_column: i8;
                if player_move.target_position.0 == 0 {
                    player_move_piece_type.positions[0].0 = 2;
                    board[player_move_piece_type.positions[0].0 as usize][player_move_piece_type.positions[0].1 as usize] = player_move_piece_type.symbol;
                    rook_old_column = 0;
                    rook_new_column = 3;
                }else {
                    player_move_piece_type.positions[0].0 = 6;
                    board[player_move_piece_type.positions[0].0 as usize][player_move_piece_type.positions[0].1 as usize] = player_move_piece_type.symbol;
                    rook_old_column = 7;
                    rook_new_column = 5;
                }

                let rook_to_castle = pieces.iter_mut().find(|piece| piece.piece_type == PieceType::Rook).unwrap();
                let rook_to_castle_index = rook_to_castle.positions.iter().position(|position| position.0 == rook_old_column).unwrap();

                board[rook_to_castle.positions[rook_to_castle_index].0 as usize][rook_to_castle.positions[rook_to_castle_index].1 as usize] = FREE_SQUARE_SYMBOL;
                rook_to_castle.positions[rook_to_castle_index].0 = rook_new_column;
                board[rook_to_castle.positions[rook_to_castle_index].0 as usize][rook_to_castle.positions[rook_to_castle_index].1 as usize] = rook_to_castle.symbol;

                break;
            }

            let is_playable: bool;
            if player_move_piece_type.color {
                is_playable = (player_move.is_capture && is_black(target_square_character)) || (!player_move.is_capture && target_square_character == FREE_SQUARE_SYMBOL);
            }else {
                is_playable = (player_move.is_capture && is_white(target_square_character)) || (!player_move.is_capture && target_square_character == FREE_SQUARE_SYMBOL);
            }

            if is_playable {
                let target_piece_opt = get_piece_type(target_square_character);

                if !player_move_verified.is_ambiguous {

                    let position_to_move_from = &mut player_move_piece_type.positions[player_move_verified.index_position_to_move_from];

                    board[position_to_move_from.0 as usize][position_to_move_from.1 as usize] = FREE_SQUARE_SYMBOL;

                    position_to_move_from.0 = player_move.target_position.0;
                    position_to_move_from.1 = player_move.target_position.1;

                    // pawn promotion:
                    let is_promotion: bool;
                    if player_move_piece_type.color {
                        is_promotion = player_move_piece_type.piece_type == PieceType::Pawn && player_move.target_position.1 == BOARD_SIZE as i8 - 1;
                    }else {
                        is_promotion = player_move_piece_type.piece_type == PieceType::Pawn && player_move.target_position.1 == 0;
                    }
                    if is_promotion {
                        player_move_piece_type.positions.swap_remove(player_move_verified.index_position_to_move_from);
                        let piece_to_promote_to = get_piece_to_promote_to(pieces);
                        piece_to_promote_to.positions.push((player_move.target_position.0, player_move.target_position.1));
                        player_move_piece_type = piece_to_promote_to;
                    }

                    board[player_move.target_position.0 as usize][player_move.target_position.1 as usize] = player_move_piece_type.symbol;
                    
                    turn_ongoing = false;
                }else {
                    if player_move.unambiguous_move_partial_position.0 != 27 || player_move.unambiguous_move_partial_position.1 != 27 {
                        let position_to_move_from: &mut (i8, i8);
                        
                        if player_move.unambiguous_move_partial_position.0 != 27 {
                            position_to_move_from = player_move_piece_type.positions.iter_mut().find(|position| position.0 == player_move.unambiguous_move_partial_position.0).unwrap();
                        }else {
                            position_to_move_from = player_move_piece_type.positions.iter_mut().find(|position| position.1 == player_move.unambiguous_move_partial_position.1).unwrap();
                        }

                        board[position_to_move_from.0 as usize][position_to_move_from.1 as usize] = FREE_SQUARE_SYMBOL;

                        position_to_move_from.0 = player_move.target_position.0;
                        position_to_move_from.1 = player_move.target_position.1;

                        board[player_move.target_position.0 as usize][player_move.target_position.1 as usize] = player_move_piece_type.symbol;
                
                        turn_ongoing = false;
                    }else{
                        println!("Your move is ambiguous! Indicate the piece you choose to move using Short Algebraic Notation!");
                        continue;
                    }
                }

                if player_move.is_capture && !turn_ongoing {
                    let target_piece_type = target_piece_opt.unwrap();
                    let target_piece = opposite_side_pieces.iter_mut().find(|piece| piece.piece_type == target_piece_type).unwrap();
                    let captured_piece_position_index = target_piece.positions.iter().position(|position| position == &player_move.target_position).unwrap();
    
                    target_piece.positions.swap_remove(captured_piece_position_index);
                }
            }
        }
        
        if turn_ongoing {
            println!("Invalid move, try again!");
        }
    }
}

const BOARD_LETTERS: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', '#', 'y', 'z']; 
// notice how '#' was used instead of 'x'
