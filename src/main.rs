mod pieces;
mod endgame;
use pieces::*;

const BOARD_SIZE: usize = 8; // max board size: 26x26 / min board size: 8x8
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

    let mut processed_move = ProcessedMove {
        // from left to right, each bit keeps track of an initial rook to verify if it has ever made a move or not
        // first and second bit are the kingside and queenside's black rooks, respectively
        // third and fourth are the kingside and queenside's white rooks
        has_rook_moved: 0b0000,
        en_passant_column: (27, true),
        captured_piece_symbols: vec![]
    };

    loop {
        show_board(&board, &processed_move.captured_piece_symbols);
        println!("White moves:");
        processed_move = play_turn(&mut white_pieces, &mut board, &mut black_pieces, processed_move);

        show_board(&board, &processed_move.captured_piece_symbols);
        println!("Black moves:");
        processed_move = play_turn(&mut black_pieces, &mut board, &mut white_pieces, processed_move);
    }
}

struct ProcessedMove {
    has_rook_moved: i8,
    en_passant_column: (i8, bool),
    captured_piece_symbols: Vec<char>
}

fn show_board(board: &[[char;BOARD_SIZE];BOARD_SIZE], captured_piece_symbols: &Vec<char> ) {
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

    if captured_piece_symbols.len() > 0 {
        let mut white_pieces = vec![];
        let mut black_pieces = vec![];

        for piece in captured_piece_symbols.iter() {
            match get_piece_color(*piece) {
                Some(color) => {
                    if color {
                        white_pieces.push(piece);
                    }else {
                        black_pieces.push(piece);
                    }
                },
                None => ()
            }
        }

        if white_pieces.len() > 0 {
            print!("[");
            for piece in white_pieces.iter() {
                print!(" {} ", piece);
            }
            println!("]");
        }
        if black_pieces.len() > 0 {
            print!("[");
            for piece in black_pieces.iter() {
                print!(" {} ", piece);
            }
            println!("]");
        }
        println!();
    }
}

fn play_turn(pieces: &mut Vec<Piece>, board: &mut [[char;BOARD_SIZE];BOARD_SIZE], opposite_side_pieces: &mut Vec<Piece>, processed_move: ProcessedMove) -> ProcessedMove {

    let mut has_rook_moved = processed_move.has_rook_moved;
    let mut en_passant_column = processed_move.en_passant_column;
    let mut captured_piece_symbols = processed_move.captured_piece_symbols;
    
    let mut turn_ongoing = true;
    while turn_ongoing {
        let mut player_move = get_player_move();
        let mut player_move_piece_type: &mut Piece = pieces.iter_mut().find(|piece: &&mut Piece| piece.piece_type == player_move.p_type).unwrap();
        let player_move_verified: VerifiedPlayerMovement = player_move.verify_if_move_is_possible(player_move_piece_type, &board, processed_move.en_passant_column.0);

        if en_passant_column.1 == player_move_piece_type.color && en_passant_column.0 == player_move_verified.en_passant_column {
            en_passant_column.0 = 27;
        }else if en_passant_column.0 != player_move_verified.en_passant_column {
            en_passant_column.0 = player_move_verified.en_passant_column;
            en_passant_column.1 = player_move_piece_type.color;
        }

        if player_move_verified.is_possible {
            
            if player_move.movement_type == MoveType::Castle {
                let has_white_rook_moved = (player_move.target_position.0 == 0 && ((has_rook_moved >> 3) & 1) == 1) || (player_move.target_position.0 == 1 && ((has_rook_moved >> 2) & 1) == 1);
                let has_black_rook_moved = (player_move.target_position.0 == 0 && ((has_rook_moved >> 1) & 1) == 1) || (player_move.target_position.0 == 1 && ((has_rook_moved >> 0) & 1) == 1);
                if (player_move_piece_type.color && has_white_rook_moved) || (!player_move_piece_type.color && has_black_rook_moved) {
                    println!("Either your king or this side's rook has moved and castling is not possible anymore!");
                    continue;
                }

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

            let mut is_en_passant: bool = false;
            let actual_position = player_move.target_position;
            if player_move.is_capture && player_move_piece_type.piece_type == PieceType::Pawn && en_passant_column.0 == player_move.target_position.0 {
                if player_move_piece_type.color {
                    if player_move.target_position.1-1 >= 0 {
                        let target_char = board[player_move.target_position.0 as usize][(player_move.target_position.1-1) as usize];

                        is_en_passant = player_move_piece_type.color && get_piece_color(target_char).is_some_and(|x|x==false);
                        if is_en_passant {
                            player_move.target_position.1 -= 1;
                        }
                    }
                }else {
                    if player_move.target_position.1+1 < BOARD_SIZE as i8 {
                        let target_char = board[player_move.target_position.0 as usize][(player_move.target_position.1+1) as usize];

                        is_en_passant = !player_move_piece_type.color && get_piece_color(target_char).is_some_and(|x|x==true);
                        if is_en_passant {
                            player_move.target_position.1 += 1;
                        }
                    }
                }
            }

            let target_square_character = board[player_move.target_position.0 as usize][player_move.target_position.1 as usize];

            let is_normal_move: bool = !player_move.is_capture && target_square_character == FREE_SQUARE_SYMBOL;
            let is_valid_capture: bool = player_move.is_capture && get_piece_color(target_square_character).is_some_and(|value| value  == !player_move_piece_type.color);

            if is_normal_move || is_valid_capture || is_en_passant {
                let target_piece_opt = get_piece_type(target_square_character);

                match player_move_piece_type.piece_type { // keeps track of king and rook movement for castling
                    PieceType::King => {
                        if player_move_piece_type.color {
                            // set both rooks' bits to 1 (4th and 3rd bits)
                            has_rook_moved |= 0b1100;
                        }else {
                            // set both rooks' bits to 1 (2nd and 1st bits)
                            has_rook_moved |= 0b0011;
                        }
                    },
                    PieceType::Rook => {
                        if player_move_piece_type.positions[player_move_verified.index_position_to_move_from].0 == 0 {
                            if player_move_piece_type.color {
                                has_rook_moved |= 1 << 3;
                            }else {
                                has_rook_moved |= 1 << 1;
                            }
                        }else if player_move_piece_type.positions[player_move_verified.index_position_to_move_from].0 == BOARD_SIZE as i8 - 1 {
                            if player_move_piece_type.color {
                                has_rook_moved |= 1 << 2;
                            }else {
                                has_rook_moved |= 1 << 0;
                            }
                        }
                    },
                    _ => ()
                }

                if !player_move_verified.is_ambiguous {

                    let position_to_move_from = &mut player_move_piece_type.positions[player_move_verified.index_position_to_move_from];

                    board[position_to_move_from.0 as usize][position_to_move_from.1 as usize] = FREE_SQUARE_SYMBOL;
                    board[player_move.target_position.0 as usize][player_move.target_position.1 as usize] = FREE_SQUARE_SYMBOL;

                    position_to_move_from.0 = actual_position.0;
                    position_to_move_from.1 = actual_position.1;

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

                    board[actual_position.0 as usize][actual_position.1 as usize] = player_move_piece_type.symbol;
                    
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
                        board[player_move.target_position.0 as usize][player_move.target_position.1 as usize] = FREE_SQUARE_SYMBOL;

                        position_to_move_from.0 = actual_position.0;
                        position_to_move_from.1 = actual_position.1;

                        board[actual_position.0 as usize][actual_position.1 as usize] = player_move_piece_type.symbol;
                
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
    
                    captured_piece_symbols.push(target_piece.symbol);
                    target_piece.positions.swap_remove(captured_piece_position_index);
                }
            }
        }
        
        if turn_ongoing {
            println!("Invalid move, try again!");
        }
    }
    
    ProcessedMove {
        has_rook_moved,
        en_passant_column,
        captured_piece_symbols
    }

}

const BOARD_LETTERS: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', '#', 'y', 'z']; 
// notice how '#' was used instead of 'x'
