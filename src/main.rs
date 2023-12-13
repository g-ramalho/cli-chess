use std::io;
use std::string::String;

const NOTHING: char = '.';

const WHITE_PAWN: char = 'i';
const WHITE_ROOK: char = 'R';
const WHITE_KNIGHT: char = 'N';
const WHITE_BISHOP: char = 'B';
const WHITE_QUEEN: char = 'Q';
const WHITE_KING: char = 'K';

const BLACK_PAWN: char = 'j';
const BLACK_ROOK: char = 'r';
const BLACK_KNIGHT: char = 'n';
const BLACK_BISHOP: char = 'b';
const BLACK_QUEEN: char = 'q';
const BLACK_KING: char = 'k';
fn main() {

// PAWNS:
    /* Columns are made into vectors because of the pawn capture system.
       Since pawns capture diagonally, everytime a pawn captures something it changes columns
       and since columns may have many pawns in different positions, 
       every column was made into a vector, with each element being an i8 
       holding the last position of a pawn.
    */

    //WHITE PAWNS:
    let mut white_column_a = vec![48];
    let mut white_column_b = vec![49];
    let mut white_column_c = vec![50];
    let mut white_column_d = vec![51];
    let mut white_column_e = vec![52];
    let mut white_column_f = vec![53];
    let mut white_column_g = vec![54];
    let mut white_column_h = vec![55];
    let mut white_column_a_enpassant: bool = false;
    let mut white_column_b_enpassant: bool = false;
    let mut white_column_c_enpassant: bool = false;
    let mut white_column_d_enpassant: bool = false;
    let mut white_column_e_enpassant: bool = false;
    let mut white_column_f_enpassant: bool = false;
    let mut white_column_g_enpassant: bool = false;
    let mut white_column_h_enpassant: bool = false;

    //BLACK PAWNS:
    let mut black_column_a = vec![8];
    let mut black_column_b = vec![9];
    let mut black_column_c = vec![10];
    let mut black_column_d = vec![11];
    let mut black_column_e = vec![12];
    let mut black_column_f = vec![13];
    let mut black_column_g = vec![14];
    let mut black_column_h = vec![15];
    let mut black_column_a_enpassant: bool = false;
    let mut black_column_b_enpassant: bool = false;
    let mut black_column_c_enpassant: bool = false;
    let mut black_column_d_enpassant: bool = false;
    let mut black_column_e_enpassant: bool = false;
    let mut black_column_f_enpassant: bool = false;
    let mut black_column_g_enpassant: bool = false;
    let mut black_column_h_enpassant: bool = false;

//PIECES:
    /* Once again vectors were used because of the pawn promotion system.
       Whenever a pawn reaches the opposite side of the board,
       it needs to be transformed from a pawn into a piece
       that's why vectors were used: 
       to grow in size and hold a promoted pawn.
    */

    //WHITE PIECES:
    let mut white_rooks: Vec<i8> = vec![56, 63];
    /* There was a need to make variables that indicate whether
       the initial rooks have moved or not. That's because, by the rules of chess, 
       castling cannot happen after the king or a rook of their 
       respective side's castle has moved (getting their initial
       position isn't enough because they can simply come back 
       to their initial position, but that still invalidates castling). 
       So, whenever a rook moves, their variable
       recieve the "true" value (both do when the king has moved).
    */
    let mut has_white_rook1_moved = false;
    let mut has_white_rook2_moved = false;

    let mut white_knights = vec![57, 62];
    let mut white_bishops = vec![58, 61];
    let mut white_queens = vec![59];
    let mut white_king: i8 = 60; // the only exception are the kings, there may only be one for each side
    
    //BLACK PIECES:
    let mut black_rooks: Vec<i8> = vec![0, 7];
    let mut has_black_rook1_moved = false;
    let mut has_black_rook2_moved = false;

    let mut black_knights = vec![1, 6];
    let mut black_bishops = vec![2, 5];
    let mut black_queens = vec![3];
    let mut black_king: i8 = 4;

//create the board
    let mut board = [NOTHING; 64];

    // pawns setup
    for i in 0..8 {
        board[i+8] = BLACK_PAWN;
        board[i+48] = WHITE_PAWN;
    };

    //rook setup 0, 7, 56, 63
    board[0] = BLACK_ROOK;
    board[7] = BLACK_ROOK;
    board[56] = WHITE_ROOK;
    board[63] = WHITE_ROOK;

    //knight setup 1, 6, 57, 62
    board[1] = BLACK_KNIGHT;
    board[6] = BLACK_KNIGHT;
    board[57] = WHITE_KNIGHT;
    board[62] = WHITE_KNIGHT;

    //bishop setup 2, 5, 58, 61
    board[2] = BLACK_BISHOP;
    board[5] = BLACK_BISHOP;
    board[58] = WHITE_BISHOP;
    board[61] = WHITE_BISHOP;

    //queen setup 3, 59
    board[3] = BLACK_QUEEN;
    board[59] = WHITE_QUEEN;

    //king setup 4, 60
    board[4] = BLACK_KING;
    board[60] = WHITE_KING;

    loop {
    
        let mut player_move = String::new();

        let mut column: i8;
        let mut line: i8;
        let mut desired_position: i8;

        let mut try_again: bool;

        // WHITE'S TURN:
        let wking_checks = get_pieces_checking_the_white_king(white_king, &board); // will never have more than 2 elements
        try_again = true;
        show_board(board); //print the board

        while try_again { // white's turn

            player_move.clear(); //has to be cleared, otherwise read_line would just append the string to the last move registered in player_move

            println!("White moves");

            io::stdin()
                .read_line(&mut player_move)
                .expect("Read error");

            //san = short algebraic notation
            let mut san_move: Vec<char> = player_move.trim().chars().collect();

            if san_move.len() <= 1 {
                println!("To move, input atleast a letter from 'a' to 'h' and a number from 1 to 8 (i.e. 'e4')");
                continue
            }

            if is_piece(san_move[0]) == true {
                column = match san_move[1] {
                    'a' => 0,
                    'b' => 1,
                    'c' => 2,
                    'd' => 3,
                    'e' => 4,
                    'f' => 5,
                    'g' => 6,
                    'h' => 7,
                    _ => 100
                };
            
                //must be in reverse because we view the board as white
                line = match san_move[2] {
                    '1' => 56,
                    '2' => 48,
                    '3' => 40,
                    '4' => 32,
                    '5' => 24,
                    '6' => 16,
                    '7' => 8,
                    '8' => 0,
                    _ => 100
                };

                if column >= 100 || line >= 100 {
                    println!("Not a possible move, try again!\n");
                    continue;
                }

                desired_position = column + line;

                if wking_checks.len() == 1 { // king is being checked by a single piece/pawn
                    if san_move[0] == 'K' {
                        if match desired_position - white_king { // check if it's a possible king move (otherwise just ignore the following)
                            -9|-8|-7|-1|1|7|8|9 => true,
                            _ => false
                        } {
                            if get_pieces_checking_the_white_king(desired_position, &board).len() > 0 {
                                println!("Your king is in check and that position would still keep it in check!\n");
                                continue;
                            }
                        }else{ // not even a possible king move
                            println!("Not a possible move, try again!\n");
                            continue;
                        }
                    }else if (wking_checks[0] > white_king && desired_position < white_king) // if the checking piece has a higher index than the king, the blocking piece must also be higher
                        || (wking_checks[0] < white_king && desired_position > white_king) // if the checking piece has a lower index than the king, the blocking piece must also be lower
                        || ((wking_checks[0]-white_king)%7 == 0 && (desired_position-white_king)%7 != 0) // the piece is not blocking the diagonal
                        || ((wking_checks[0]-white_king)%9 == 0 && (desired_position-white_king)%9 != 0) // the piece is not blocking the diagonal
                        || ((wking_checks[0]-white_king)%8 == 0 && (desired_position-white_king)%8 != 0) { // the piece is not blocking the file

                        // the piece to be moved does not block the check
                        println!("That move did not block the check completely!\n");
                        continue;
                    }
                }else if wking_checks.len() > 1 {
                    // incase of a double check, the king MUST move
                    if san_move[0] == 'K' {
                        if match desired_position - white_king { // check if it's a possible king move (otherwise just ignore the following)
                            -9|-8|-7|-1|1|7|8|9 => true,
                            _ => false
                        } {
                            if get_pieces_checking_the_white_king(desired_position, &board).len() > 0 {
                                println!("Your king is in check and that position would still keep it in check!\n");
                                continue;
                            }
                        }else{ // not even a possible king move
                            println!("Not a possible move, try again!\n");
                            continue;
                        }
                    }else{
                        println!("Your king is being double checked. You have to move it!\n");
                        continue;
                    }
                }

                //pieces' movement checks
                if !is_white(board[desired_position as usize]) {
                    // check and store the piece that is about to be captured:
                    let captured_piece = board[desired_position as usize];

                    match san_move[0] { // check if the piece can actually be captured (and capture it)
                        'N' => {
                            if test_multiple_knights(&mut white_knights, desired_position) {
                                println!("Specify the current square of the knight to be moved");
                                player_move.clear();
                                san_move.clear();
                            
                                io::stdin()
                                        .read_line(&mut player_move)
                                        .expect("Read error");
                                
                                san_move = player_move.trim().chars().collect();
                            
                                let knight_column: i8 = match san_move[0] {
                                            'a' => 0,
                                            'b' => 1,
                                            'c' => 2,
                                            'd' => 3,
                                            'e' => 4,
                                            'f' => 5,
                                            'g' => 6,
                                            'h' => 7,
                                            _ => 100
                                        };
                                    
                                //must be in reverse because we view the board as white
                                let knight_line: i8 = match san_move[1] {
                                            '1' => 56,
                                            '2' => 48,
                                            '3' => 40,
                                            '4' => 32,
                                            '5' => 24,
                                            '6' => 16,
                                            '7' => 8,
                                            '8' => 0,
                                            _ => 100
                                        };

                                for w_knight in white_knights.iter_mut() {
                                    if knight_column + knight_line == *w_knight {
                                        match *w_knight - desired_position {
                                            -17 | -15 | -10 | -6 | 6 | 10 | 15 | 17 => {
                                                //last position is freed
                                                board[*w_knight as usize] = NOTHING;

                                                //piece is moved to new position
                                                board[desired_position as usize] = WHITE_KNIGHT;

                                                //current position is updated
                                                *w_knight = desired_position;

                                                try_again = false;
                                                break;
                                            },
                                            _ => ()
                                        }
                                    }
                                }
                            }else{
                                for w_knight in white_knights.iter_mut() {
                                    match *w_knight - desired_position {
                                        -17 | -15 | -10 | -6 | 6 | 10 | 15 | 17 => {
                                            board[*w_knight as usize] = NOTHING;
                                            board[desired_position as usize] = WHITE_KNIGHT;
                                            *w_knight = desired_position;

                                            try_again = false;
                                            break;
                                        },
                                        _ => ()
                                    }
                                };
                            }
                        },
                        'B' => {
                            if test_multiple_bishops(&mut white_bishops, desired_position) == true {
                                println!("Specify the current square of the bishop to be moved");
                                player_move.clear();
                                san_move.clear();
                            
                                io::stdin()
                                        .read_line(&mut player_move)
                                        .expect("Read error");
                                
                                san_move = player_move.trim().chars().collect();
                            
                                let bishop_column: i8 = match san_move[0] {
                                    'a' => 0,
                                    'b' => 1,
                                    'c' => 2,
                                    'd' => 3,
                                    'e' => 4,
                                    'f' => 5,
                                    'g' => 6,
                                    'h' => 7,
                                    _ => 100
                                };
                                    
                                //must be in reverse because we view the board as white
                                let bishop_line: i8 = match san_move[1] {
                                    '1' => 56,
                                    '2' => 48,
                                    '3' => 40,
                                    '4' => 32,
                                    '5' => 24,
                                    '6' => 16,
                                    '7' => 8,
                                    '8' => 0,
                                    _ => 100
                                };
                                
                                for w_bishop in white_bishops.iter_mut() { // multiple bishops may reach the square
                                    if bishop_line + bishop_column == *w_bishop {
                                            // if the desired square is "above" the initial position
                                        if *w_bishop > desired_position {
                                            // and if the distance is divisible by 7
                                            if (*w_bishop - desired_position)%7 == 0 {
                                                for diagonal in 1..8 {
                                                    // count each possible diagonal (until the maximum of 7 diagonals)
                                                    if *w_bishop - diagonal*7 == desired_position && !upper_right_diagonal(*w_bishop, diagonal) {
                                                        /* check if any of the squares in the bishop's diagonal is the desired square,
                                                        check if any of the calculated diagonals are forbidden (done using the 'upper_right_diagonal' function),
                                                        and finally, check if the desired square has no white pieces that may block the movement
                                                        if all of those checks are true, the bishop may be moved */
                                                        board[*w_bishop as usize] = NOTHING;
                                                        board[desired_position as usize] = WHITE_BISHOP;
                                                        *w_bishop = desired_position;
    
                                                        try_again = false;
                                                        break;
                                                    }else if is_white(board[(*w_bishop - diagonal*7) as usize]) 
                                                    || is_black(board[((*w_bishop - diagonal*7) as usize)]) 
                                                    || upper_right_diagonal(*w_bishop, diagonal){
                                                        // otherwise, if there are any white/black pieces on the way, the square is unreachable
                                                        break;
                                                    }
                                                }
                                            }else if (*w_bishop - desired_position)%9 == 0 {
                                                for diagonal in 1..8 {
                                                    if *w_bishop - diagonal*9 == desired_position && !upper_left_diagonal(*w_bishop, diagonal) {
                                                        board[*w_bishop as usize] = NOTHING;
                                                        board[desired_position as usize] = WHITE_BISHOP;
                                                        *w_bishop = desired_position;
    
                                                        try_again = false;
                                                        break;
                                                    }else if is_white(board[(*w_bishop - diagonal*9) as usize]) 
                                                    || is_black(board[((*w_bishop - diagonal*9) as usize)]) 
                                                    || upper_left_diagonal(*w_bishop, diagonal) {
                                                        break;
                                                    }
                                                }
                                            }
                                        }else if *w_bishop < desired_position {
                                            if (*w_bishop - desired_position)%7 == 0 {
                                                for diagonal in 1..8 {
                                                    if *w_bishop + diagonal*7 == desired_position && !inferior_left_diagonal(*w_bishop, diagonal) {
                                                        board[*w_bishop as usize] = NOTHING;
                                                        board[desired_position as usize] = WHITE_BISHOP;
                                                        *w_bishop = desired_position;
    
                                                        try_again = false;
                                                        break;
                                                    }else if is_white(board[(*w_bishop + 7*diagonal) as usize]) 
                                                    || is_black(board[(*w_bishop + 7*diagonal) as usize])
                                                    || inferior_left_diagonal(*w_bishop, diagonal) {
                                                        break;
                                                    }
                                                }
                                            }else if (*w_bishop - desired_position)%9 == 0 {
                                                for diagonal in 1..8 {
                                                    if *w_bishop + diagonal*9 == desired_position && !inferior_right_diagonal(*w_bishop, diagonal) {
                                                        board[*w_bishop as usize] = NOTHING;
                                                        board[desired_position as usize] = WHITE_BISHOP;
                                                        *w_bishop = desired_position;
    
                                                        try_again = false;
                                                        break;
                                                    }else if is_white(board[(*w_bishop + 9*diagonal) as usize]) 
                                                    || is_black(board[(*w_bishop + 9*diagonal) as usize]) 
                                                    || inferior_right_diagonal(*w_bishop, diagonal){
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                };
                            }else{ // only one bishop reaches the square
                                for w_bishop in white_bishops.iter_mut() {
                                        // if the desired square is "above" the initial position
                                    if *w_bishop > desired_position {
                                        // and if the distance is divisible by 7
                                        if (*w_bishop - desired_position)%7 == 0 {
                                            for diagonal in 1..8 {
                                                // count each possible diagonal (until the maximum of 7 diagonals)
                                                if *w_bishop - diagonal*7 == desired_position && !upper_right_diagonal(*w_bishop, diagonal) {
                                                    board[*w_bishop as usize] = NOTHING;
                                                    board[desired_position as usize] = WHITE_BISHOP;
                                                    *w_bishop = desired_position;

                                                    try_again = false;
                                                    break;
                                                }else if is_white(board[(*w_bishop - diagonal*7) as usize]) 
                                                || is_black(board[((*w_bishop - diagonal*7) as usize)]) 
                                                || upper_right_diagonal(*w_bishop, diagonal) {
                                                    // otherwise, if there are any white/black pieces on the way, the square is unreachable
                                                    break;
                                                }
                                            }
                                        }else if (*w_bishop - desired_position)%9 == 0 {
                                            for diagonal in 1..8 {
                                                if *w_bishop - diagonal*9 == desired_position && !upper_left_diagonal(*w_bishop, diagonal) {
                                                    board[*w_bishop as usize] = NOTHING;
                                                    board[desired_position as usize] = WHITE_BISHOP;
                                                    *w_bishop = desired_position;

                                                    try_again = false;
                                                    break;
                                                }else if is_white(board[(*w_bishop - diagonal*9) as usize]) 
                                                || is_black(board[((*w_bishop - diagonal*9) as usize)]) 
                                                || upper_left_diagonal(*w_bishop, diagonal) {
                                                    break;
                                                }
                                            }
                                        }
                                    }else if *w_bishop < desired_position {
                                        if (*w_bishop - desired_position)%7 == 0 {
                                            for diagonal in 1..8 {
                                                if *w_bishop + diagonal*7 == desired_position && !inferior_left_diagonal(*w_bishop, diagonal) {
                                                    board[*w_bishop as usize] = NOTHING;
                                                    board[desired_position as usize] = WHITE_BISHOP;
                                                    *w_bishop = desired_position;

                                                    try_again = false;
                                                    break;
                                                }else if is_white(board[(*w_bishop + 7*diagonal) as usize]) 
                                                || is_black(board[(*w_bishop + 7*diagonal) as usize]) 
                                                || inferior_left_diagonal(*w_bishop, diagonal) {
                                                    break;
                                                }
                                            }
                                        }else if (*w_bishop - desired_position)%9 == 0 {
                                            for diagonal in 1..8 {
                                                if *w_bishop + diagonal*9 == desired_position && !inferior_right_diagonal(*w_bishop, diagonal) {
                                                    board[*w_bishop as usize] = NOTHING;
                                                    board[desired_position as usize] = WHITE_BISHOP;
                                                    *w_bishop = desired_position;

                                                    try_again = false;
                                                    break;
                                                }else if is_white(board[(*w_bishop + 9*diagonal) as usize]) 
                                                || is_black(board[(*w_bishop + 9*diagonal) as usize]) 
                                                || inferior_right_diagonal(*w_bishop, diagonal) {
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                };
                            }
                        },
                        'R' => {
                            // check if more than one rook can reach the desired square
                            if test_multiple_rooks(&mut white_rooks, desired_position) == true {
                                println!("Specify the current square of the rook to be moved");
                                player_move.clear();
                                san_move.clear();
                            
                                io::stdin()
                                        .read_line(&mut player_move)
                                        .expect("Read error");
                                
                                san_move = player_move.trim().chars().collect();
                            
                                let rook_column: i8 = match san_move[0] {
                                            'a' => 0,
                                            'b' => 1,
                                            'c' => 2,
                                            'd' => 3,
                                            'e' => 4,
                                            'f' => 5,
                                            'g' => 6,
                                            'h' => 7,
                                            _ => 100
                                        };
                                    
                                //must be in reverse because we view the board as white
                                let rook_line: i8 = match san_move[1] {
                                            '1' => 56,
                                            '2' => 48,
                                            '3' => 40,
                                            '4' => 32,
                                            '5' => 24,
                                            '6' => 16,
                                            '7' => 8,
                                            '8' => 0,
                                            _ => 100
                                        };
                                for w_rook in &mut white_rooks.iter_mut() {
                                    if rook_column + rook_line == *w_rook {
                                        if *w_rook > desired_position {
                                            // if the desired square is on the same rank as the initial position
                                            if get_line(*w_rook) == get_line(desired_position) {
                                                for square in 1..8 {
                                                    if *w_rook - square == desired_position && !rook_left(*w_rook, square) {
                                                        board[*w_rook as usize] = NOTHING;
                                                        board[desired_position as usize] = WHITE_ROOK;
                                                        match *w_rook { // check if the moved rook was in its initial position
                                                            56 => has_white_rook1_moved = true,
                                                            63 => has_white_rook2_moved = true,
                                                            _ => ()
                                                        }
                                                        *w_rook = desired_position;
                
                                                        try_again = false;
                                                        break;
                                                    }else if is_white(board[(*w_rook - square) as usize]) 
                                                    || is_black(board[(*w_rook - square) as usize])
                                                    || rook_left(*w_rook, square) {
                                                        break;
                                                    }
                                                }
                                            }else if (*w_rook - desired_position)%8 == 0 {
                                                // otherwise, test if it is on the same file
                                                for square in 1..8 {
                                                    if *w_rook - square*8 == desired_position && !rook_up(*w_rook, square) {
                                                        board[*w_rook as usize] = NOTHING;
                                                        board[desired_position as usize] = WHITE_ROOK;
                                                        match *w_rook {
                                                            56 => has_white_rook1_moved = true,
                                                            63 => has_white_rook2_moved = true,
                                                            _ => ()
                                                        }
                                                        *w_rook = desired_position;
                
                                                        try_again = false;
                                                        break;
                                                    }else if is_white(board[(*w_rook - square*8) as usize]) 
                                                    || is_black(board[(*w_rook - square*8) as usize])
                                                    || rook_up(*w_rook, square) {
                                                        break;
                                                    }
                                                }
                                            }
                                        }else if *w_rook < desired_position {
                                            if get_line(*w_rook) == get_line(desired_position) {
                                                for square in 1..8 {
                                                    if *w_rook + square == desired_position && !rook_right(*w_rook, square) {
                                                        board[*w_rook as usize] = NOTHING;
                                                        board[desired_position as usize] = WHITE_ROOK;
                                                        match *w_rook {
                                                            56 => has_white_rook1_moved = true,
                                                            63 => has_white_rook2_moved = true,
                                                            _ => ()
                                                        }
                                                        *w_rook = desired_position;
                
                                                        try_again = false;
                                                        break;
                                                    }else if is_white(board[(*w_rook + square) as usize]) 
                                                    || is_black(board[(*w_rook + square) as usize]) 
                                                    || rook_right(*w_rook, square) {
                                                        break;
                                                    }
                                                }
                                            }else if (desired_position - *w_rook)%8 == 0 {
                                                for square in 1..8 {
                                                    if *w_rook + square*8 == desired_position && !rook_down(*w_rook, square) {
                                                        board[*w_rook as usize] = NOTHING;
                                                        board[desired_position as usize] = WHITE_ROOK;
                                                        match *w_rook {
                                                            56 => has_white_rook1_moved = true,
                                                            63 => has_white_rook2_moved = true,
                                                            _ => ()
                                                        }
                                                        *w_rook = desired_position;
                
                                                        try_again = false;
                                                        break;
                                                    }else if is_white(board[(*w_rook + square*8) as usize]) 
                                                    || is_black(board[(*w_rook + square*8) as usize])
                                                    || rook_down(*w_rook, square) {
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                };
                            }else{ // only one rook may reach the desired square
                                for w_rook in &mut white_rooks.iter_mut() {    
                                    if *w_rook > desired_position {
                                        if get_line(*w_rook) == get_line(desired_position) {
                                            for square in 1..8 {
                                                if *w_rook - square == desired_position && !rook_left(*w_rook, square) {
                                                    board[*w_rook as usize] = NOTHING;
                                                    board[desired_position as usize] = WHITE_ROOK;
                                                    match *w_rook {
                                                        56 => has_white_rook1_moved = true,
                                                        63 => has_white_rook2_moved = true,
                                                        _ => ()
                                                    }
                                                    *w_rook = desired_position;
            
                                                    try_again = false;
                                                    break;
                                                }else if is_white(board[(*w_rook - square) as usize]) 
                                                || is_black(board[(*w_rook - square) as usize]) 
                                                || rook_left(*w_rook, square) {
                                                    break;
                                                }
                                            }
                                        }else if (*w_rook - desired_position)%8 == 0 {
                                            for square in 1..8 {
                                                if *w_rook - square*8 == desired_position && !rook_up(*w_rook, square) {
                                                    board[*w_rook as usize] = NOTHING;
                                                    board[desired_position as usize] = WHITE_ROOK;
                                                    match *w_rook {
                                                        56 => has_white_rook1_moved = true,
                                                        63 => has_white_rook2_moved = true,
                                                        _ => ()
                                                    }
                                                    *w_rook = desired_position;
            
                                                    try_again = false;
                                                    break;
                                                }else if is_white(board[(*w_rook - square*8) as usize]) 
                                                || is_black(board[(*w_rook - square*8) as usize]) 
                                                || rook_up(*w_rook, square) {
                                                    break;
                                                }
                                            }
                                        }
                                    }else if *w_rook < desired_position {
                                        if get_line(*w_rook) == get_line(desired_position) {
                                            for square in 1..8 {
                                                if *w_rook + square == desired_position && !rook_right(*w_rook, square) {
                                                    board[*w_rook as usize] = NOTHING;
                                                    board[desired_position as usize] = WHITE_ROOK;
                                                    match *w_rook {
                                                        56 => has_white_rook1_moved = true,
                                                        63 => has_white_rook2_moved = true,
                                                        _ => ()
                                                    }
                                                    *w_rook = desired_position;
            
                                                    try_again = false;
                                                    break;
                                                }else if is_white(board[(*w_rook + square) as usize]) 
                                                || is_black(board[(*w_rook + square) as usize]) 
                                                || rook_right(*w_rook, square) {
                                                    break;
                                                }
                                            }
                                        }else if (desired_position - *w_rook)%8 == 0 {
                                            for square in 1..8 {
                                                if *w_rook + square*8 == desired_position && !rook_down(*w_rook, square) {
                                                    board[*w_rook as usize] = NOTHING;
                                                    board[desired_position as usize] = WHITE_ROOK;
                                                    match *w_rook {
                                                        56 => has_white_rook1_moved = true,
                                                        63 => has_white_rook2_moved = true,
                                                        _ => ()
                                                    }
                                                    *w_rook = desired_position;
            
                                                    try_again = false;
                                                    break;
                                                }else if is_white(board[(*w_rook + square*8) as usize]) 
                                                || is_black(board[(*w_rook + square*8) as usize]) 
                                                || rook_down(*w_rook, square) {
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                };
                            }
                        },
                        'K' => {
                            match desired_position - white_king {
                                -9 => {
                                    if !upper_left_diagonal(white_king, 1) {
                                        board[white_king as usize] = NOTHING;
                                        board[desired_position as usize] = WHITE_KING;
                                        white_king = desired_position;
                                        try_again = false;

                                        // the rooks might have not actually moved,
                                        // but a king move invalidates both castling maneuvers
                                        has_white_rook1_moved = true;
                                        has_white_rook2_moved = true;
                                        }
                                    },
                                -8 => {
                                    if !rook_up(white_king, 1) {
                                        board[white_king as usize] = NOTHING;
                                        board[desired_position as usize] = WHITE_KING;
                                        white_king = desired_position;
                                        try_again = false;

                                        has_white_rook1_moved = true;
                                        has_white_rook2_moved = true;
                                        }
                                    },
                                -7 => {
                                    if !upper_right_diagonal(white_king, 1) {
                                        board[white_king as usize] = NOTHING;
                                        board[desired_position as usize] = WHITE_KING;
                                        white_king = desired_position;
                                        try_again = false;

                                        has_white_rook1_moved = true;
                                        has_white_rook2_moved = true;
                                        }
                                    },
                                -1 => {
                                    if !rook_left(white_king, 1) {
                                        board[white_king as usize] = NOTHING;
                                        board[desired_position as usize] = WHITE_KING;
                                        white_king = desired_position;
                                        try_again = false;

                                        has_white_rook1_moved = true;
                                        has_white_rook2_moved = true;
                                        }
                                    },
                                1 => {
                                    if !rook_right(white_king, 1) {
                                        board[white_king as usize] = NOTHING;
                                        board[desired_position as usize] = WHITE_KING;
                                        white_king = desired_position;
                                        try_again = false;

                                        has_white_rook1_moved = true;
                                        has_white_rook2_moved = true;
                                        }
                                    },
                                7 => {
                                    if !inferior_left_diagonal(white_king, 1) {
                                        board[white_king as usize] = NOTHING;
                                        board[desired_position as usize] = WHITE_KING;
                                        white_king = desired_position;
                                        try_again = false;

                                        has_white_rook1_moved = true;
                                        has_white_rook2_moved = true;
                                        }
                                    },
                                8 => {
                                    if !rook_down(white_king, 1) {
                                        board[white_king as usize] = NOTHING;
                                        board[desired_position as usize] = WHITE_KING;
                                        white_king = desired_position;
                                        try_again = false;

                                        has_white_rook1_moved = true;
                                        has_white_rook2_moved = true;
                                        }
                                    },
                                9 => {
                                    if !inferior_right_diagonal(white_king, 1) {
                                        board[white_king as usize] = NOTHING;
                                        board[desired_position as usize] = WHITE_KING;
                                        white_king = desired_position;
                                        try_again = false;

                                        has_white_rook1_moved = true;
                                        has_white_rook2_moved = true;
                                        }
                                    },
                                _ => ()
                            }
                        },
                        'Q' => {
                            if test_multiple_queens(&mut white_queens, desired_position) == true {
                                println!("Specify the current square of the queen to be moved");
                                player_move.clear();
                                san_move.clear();
                            
                                io::stdin()
                                        .read_line(&mut player_move)
                                        .expect("Read error");
                                
                                san_move = player_move.trim().chars().collect();
                            
                                let queen_column: i8 = match san_move[0] {
                                            'a' => 0,
                                            'b' => 1,
                                            'c' => 2,
                                            'd' => 3,
                                            'e' => 4,
                                            'f' => 5,
                                            'g' => 6,
                                            'h' => 7,
                                            _ => 100
                                        };
                                    
                                //must be in reverse because we view the board as white
                                let queen_line: i8 = match san_move[1] {
                                            '1' => 56,
                                            '2' => 48,
                                            '3' => 40,
                                            '4' => 32,
                                            '5' => 24,
                                            '6' => 16,
                                            '7' => 8,
                                            '8' => 0,
                                            _ => 100
                                        };
                                
                                for w_queen in white_queens.iter_mut() {
                                    if queen_column + queen_line == *w_queen {
                                        if *w_queen > desired_position {
                                            // DIAGONAL MOVEMENT:
                                            if (*w_queen - desired_position)%7 == 0 {
                                                for diagonal in 1..8 {
                                                    if *w_queen - diagonal*7 == desired_position && !upper_right_diagonal(*w_queen, diagonal) {
                                                        board[*w_queen as usize] = NOTHING;
                                                        board[desired_position as usize] = WHITE_QUEEN;
                                                        *w_queen = desired_position;
        
                                                        try_again = false;
                                                        break;
                                                    }else if is_white(board[(*w_queen - diagonal*7) as usize]) || is_black(board[((*w_queen - diagonal*7) as usize)]) {
                                                        break;
                                                    }
                                                }
                                            }else if (*w_queen - desired_position)%9 == 0 {
                                                for diagonal in 1..8 {
                                                    if *w_queen - diagonal*9 == desired_position && !upper_left_diagonal(*w_queen, diagonal) {
                                                        board[*w_queen as usize] = NOTHING;
                                                        board[desired_position as usize] = WHITE_QUEEN;
                                                        *w_queen = desired_position;
        
                                                        try_again = false;
                                                        break;
                                                    }else if is_white(board[(*w_queen - diagonal*9) as usize]) || is_black(board[((*w_queen - diagonal*9) as usize)]){
                                                        break;
                                                    }
                                                }
                                            }
                                            // ROOK-LIKE MOVEMENT:
                                            if get_line(*w_queen) == get_line(desired_position) {
                                                for square in 1..8 {
                                                    if *w_queen - square == desired_position && !rook_left(*w_queen, square) {
                                                        board[*w_queen as usize] = NOTHING;
                                                        board[desired_position as usize] = WHITE_QUEEN;
                                                        *w_queen = desired_position;
                
                                                        try_again = false;
                                                        break;
                                                    }else if is_white(board[(*w_queen - square) as usize]) || is_black(board[(*w_queen - square) as usize]) {
                                                        break;
                                                    }
                                                }
                                            }else if (*w_queen - desired_position)%8 == 0 {
                                                for square in 1..8 {
                                                    if *w_queen - square*8 == desired_position && !rook_up(*w_queen, square) {
                                                        board[*w_queen as usize] = NOTHING;
                                                        board[desired_position as usize] = WHITE_QUEEN;
                                                        *w_queen = desired_position;
                
                                                        try_again = false;
                                                        break;
                                                    }else if is_white(board[(*w_queen - square*8) as usize]) || is_black(board[(*w_queen - square*8) as usize]) {
                                                        break;
                                                    }
                                                }
                                            }
                                        }else if *w_queen < desired_position {
                                            // DIAGONAL MOVEMENT:
                                            if (*w_queen - desired_position)%7 == 0 {
                                                for diagonal in 1..8 {
                                                    if *w_queen + diagonal*7 == desired_position && !inferior_left_diagonal(*w_queen, diagonal) {
                                                        board[*w_queen as usize] = NOTHING;
                                                        board[desired_position as usize] = WHITE_QUEEN;
                                                        *w_queen = desired_position;
        
                                                        try_again = false;
                                                        break;
                                                    }else if is_white(board[(*w_queen + 7*diagonal) as usize]) || is_black(board[(*w_queen + 7*diagonal) as usize]){
                                                        break;
                                                    }
                                                }
                                            }else if (*w_queen - desired_position)%9 == 0 {
                                                for diagonal in 1..8 {
                                                    if *w_queen + diagonal*9 == desired_position && !inferior_right_diagonal(*w_queen, diagonal) {
                                                        board[*w_queen as usize] = NOTHING;
                                                        board[desired_position as usize] = WHITE_QUEEN;
                                                        *w_queen = desired_position;
        
                                                        try_again = false;
                                                        break;
                                                    }else if is_white(board[(*w_queen + 9*diagonal) as usize]) || is_black(board[(*w_queen + 9*diagonal) as usize]) {
                                                        break;
                                                    }
                                                }
                                            }
                                            // ROOK-LIKE MOVEMENT:
                                            if get_line(*w_queen) == get_line(desired_position) {
                                                for square in 1..8 {
                                                    if *w_queen + square == desired_position && !rook_right(*w_queen, square) {
                                                        board[*w_queen as usize] = NOTHING;
                                                        board[desired_position as usize] = WHITE_QUEEN;
                                                        *w_queen = desired_position;
                
                                                        try_again = false;
                                                        break;
                                                    }else if is_white(board[(*w_queen + square) as usize]) || is_black(board[(*w_queen + square) as usize]) {
                                                        break;
                                                    }
                                                }
                                            }else if (desired_position - *w_queen)%8 == 0 {
                                                for square in 1..8 {
                                                    if *w_queen + square*8 == desired_position && !rook_down(*w_queen, square) {
                                                        board[*w_queen as usize] = NOTHING;
                                                        board[desired_position as usize] = WHITE_QUEEN;
                                                        *w_queen = desired_position;
                
                                                        try_again = false;
                                                        break;
                                                    }else if is_white(board[(*w_queen + square*8) as usize]) || is_black(board[(*w_queen + square*8) as usize]) {
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                };
                            }else{
                                for w_queen in white_queens.iter_mut() {
                                    if *w_queen > desired_position {
                                        // DIAGONAL MOVEMENT:
                                        if (*w_queen - desired_position)%7 == 0 {
                                            for diagonal in 1..8 {
                                                if *w_queen - diagonal*7 == desired_position && !upper_right_diagonal(*w_queen, diagonal) {
                                                    board[*w_queen as usize] = NOTHING;
                                                    board[desired_position as usize] = WHITE_QUEEN;
                                                    *w_queen = desired_position;
    
                                                    try_again = false;
                                                    break;
                                                }else if is_white(board[(*w_queen - diagonal*7) as usize]) 
                                                || is_black(board[((*w_queen - diagonal*7) as usize)]) 
                                                || upper_right_diagonal(*w_queen, diagonal) {
                                                    break;
                                                }
                                            }
                                        }else if (*w_queen - desired_position)%9 == 0 {
                                            for diagonal in 1..8 {
                                                if *w_queen - diagonal*9 == desired_position && !upper_left_diagonal(*w_queen, diagonal) {
                                                    board[*w_queen as usize] = NOTHING;
                                                    board[desired_position as usize] = WHITE_QUEEN;
                                                    *w_queen = desired_position;
    
                                                    try_again = false;
                                                    break;
                                                }else if is_white(board[(*w_queen - diagonal*9) as usize]) 
                                                || is_black(board[((*w_queen - diagonal*9) as usize)]) 
                                                || upper_left_diagonal(*w_queen, diagonal) {
                                                    break;
                                                }
                                            }
                                        }
                                        // ROOK-LIKE MOVEMENT:
                                        if get_line(*w_queen) == get_line(desired_position) {
                                            for square in 1..8 {
                                                if *w_queen - square == desired_position && !rook_left(*w_queen, square) {
                                                    board[*w_queen as usize] = NOTHING;
                                                    board[desired_position as usize] = WHITE_QUEEN;
                                                    *w_queen = desired_position;
            
                                                    try_again = false;
                                                    break;
                                                }else if is_white(board[(*w_queen - square) as usize]) 
                                                || is_black(board[(*w_queen - square) as usize])
                                                || rook_left(*w_queen, square) {
                                                    break;
                                                }
                                            }
                                        }else if (*w_queen - desired_position)%8 == 0 {
                                            for square in 1..8 {
                                                if *w_queen - square*8 == desired_position && !rook_up(*w_queen, square) {
                                                    board[*w_queen as usize] = NOTHING;
                                                    board[desired_position as usize] = WHITE_QUEEN;
                                                    *w_queen = desired_position;
            
                                                    try_again = false;
                                                    break;
                                                }else if is_white(board[(*w_queen - square*8) as usize]) 
                                                || is_black(board[(*w_queen - square*8) as usize])
                                                || rook_up(*w_queen, square) {
                                                    break;
                                                }
                                            }
                                        }
                                    }else if *w_queen < desired_position {
                                        // DIAGONAL MOVEMENT:
                                        if (*w_queen - desired_position)%7 == 0 {
                                            for diagonal in 1..8 {
                                                if *w_queen + diagonal*7 == desired_position && !inferior_left_diagonal(*w_queen, diagonal) {
                                                    board[*w_queen as usize] = NOTHING;
                                                    board[desired_position as usize] = WHITE_QUEEN;
                                                    *w_queen = desired_position;
    
                                                    try_again = false;
                                                    break;
                                                }else if is_white(board[(*w_queen + 7*diagonal) as usize]) 
                                                || is_black(board[(*w_queen + 7*diagonal) as usize]) 
                                                || inferior_left_diagonal(*w_queen, diagonal) {
                                                    break;
                                                }
                                            }
                                        }else if (*w_queen - desired_position)%9 == 0 {
                                            for diagonal in 1..8 {
                                                if *w_queen + diagonal*9 == desired_position && !inferior_right_diagonal(*w_queen, diagonal) {
                                                    board[*w_queen as usize] = NOTHING;
                                                    board[desired_position as usize] = WHITE_QUEEN;
                                                    *w_queen = desired_position;
    
                                                    try_again = false;
                                                    break;
                                                }else if is_white(board[(*w_queen + 9*diagonal) as usize]) 
                                                || is_black(board[(*w_queen + 9*diagonal) as usize])
                                                || inferior_right_diagonal(*w_queen, diagonal) {
                                                    break;
                                                }
                                            }
                                        }
                                        // ROOK-LIKE MOVEMENT:
                                        if get_line(*w_queen) == get_line(desired_position) {
                                            for square in 1..8 {
                                                if *w_queen + square == desired_position && !rook_right(*w_queen, square) {
                                                    board[*w_queen as usize] = NOTHING;
                                                    board[desired_position as usize] = WHITE_QUEEN;
                                                    *w_queen = desired_position;
            
                                                    try_again = false;
                                                    break;
                                                }else if is_white(board[(*w_queen + square) as usize]) 
                                                || is_black(board[(*w_queen + square) as usize])
                                                || rook_right(*w_queen, square) {
                                                    break;
                                                }
                                            }
                                        }else if (desired_position - *w_queen)%8 == 0 {
                                            for square in 1..8 {
                                                if *w_queen + square*8 == desired_position && !rook_down(*w_queen, square) {
                                                    board[*w_queen as usize] = NOTHING;
                                                    board[desired_position as usize] = WHITE_QUEEN;
                                                    *w_queen = desired_position;
            
                                                    try_again = false;
                                                    break;
                                                }else if is_white(board[(*w_queen + square*8) as usize]) 
                                                || is_black(board[(*w_queen + square*8) as usize])
                                                || rook_down(*w_queen, square) {
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        },
                        _ => ()
                    }

                    if try_again == false { // prevents captured pawns/pieces to be moved later
                        match captured_piece {
                            'j' => {
                                match san_move[1] {
                                    'a' => {
                                        for pawn in 0..black_column_a.len() {
                                            if black_column_a[pawn] == desired_position {
                                                black_column_a.swap_remove(pawn);
                                                break;
                                            }
                                        };
                                    },
                                    'b' => {
                                        for pawn in 0..black_column_b.len() {
                                            if black_column_b[pawn] == desired_position {
                                                black_column_b.swap_remove(pawn);
                                                break;
                                            }
                                        };
                                    },
                                    'c' => {
                                        for pawn in 0..black_column_c.len() {
                                            if black_column_c[pawn] == desired_position {
                                                black_column_c.swap_remove(pawn);
                                                break;
                                            }
                                        };
                                    },
                                    'd' => {
                                        for pawn in 0..black_column_d.len() {
                                            if black_column_d[pawn] == desired_position {
                                                black_column_d.swap_remove(pawn);
                                                break;
                                            }
                                        };
                                    },
                                    'e' => {
                                        for pawn in 0..black_column_e.len() {
                                            if black_column_e[pawn] == desired_position {
                                                black_column_e.swap_remove(pawn);
                                                break;
                                            }
                                        };
                                    },
                                    'f' => {
                                        for pawn in 0..black_column_f.len() {
                                            if black_column_f[pawn] == desired_position {
                                                black_column_f.swap_remove(pawn);
                                                break;
                                            }
                                        };
                                    },
                                    'g' => {
                                        for pawn in 0..black_column_g.len() {
                                            if black_column_g[pawn] == desired_position {
                                                black_column_g.swap_remove(pawn);
                                                break;
                                            }
                                        };
                                    },
                                    'h' => {
                                        for pawn in 0..black_column_h.len() {
                                            if black_column_h[pawn] == desired_position {
                                                black_column_h.swap_remove(pawn);
                                                break;
                                            }
                                        };
                                    },
                                    _ => ()
                                }
                            },
                            'q' => {
                                for i in 0..black_queens.len() {
                                    if black_queens[i] == desired_position {
                                        black_queens.swap_remove(i);
                                        break;
                                    }
                                };
                            },
                            'b' => {
                                for i in 0..black_bishops.len() {
                                    if black_bishops[i] == desired_position {
                                        black_bishops.swap_remove(i);
                                        break;
                                    }
                                };
                            },
                            'n' => {
                                for i in 0..black_knights.len() {
                                    if black_knights[i] == desired_position {
                                        black_knights.swap_remove(i);
                                        break;
                                    }
                                };
                            },
                            'r' => {
                                for i in 0..black_rooks.len() {
                                    if black_rooks[i] == desired_position {
                                        black_rooks.swap_remove(i);
                                        break;
                                    }
                                };
                            },
                            _ => ()
                        }
                    }
                }
            }else if player_move.trim() == "O-O" || player_move.trim() == "0-0" { // kingside castling
                if has_white_rook2_moved == false {
                    if board[62] == NOTHING && board[61] == NOTHING { // squares between the rook and king must be empty
                        if get_pieces_checking_the_white_king(62, &board).len() == 0
                        && get_pieces_checking_the_white_king(61, &board).len() == 0 { // any black piece attacking the square makes it impossible to castle
                            board[white_king as usize] = NOTHING;
                            board[63] = NOTHING; // kingside rook

                            white_king = 62;
                            board[white_king as usize] = WHITE_KING;
                            for w_rook in white_rooks.iter_mut() {
                                if *w_rook == 63 {
                                    *w_rook = 61;
                                    board[*w_rook as usize] = WHITE_ROOK;
                                    break;
                                }
                            }

                            try_again = false;
                        }
                    }
                }
            }else if player_move.trim() == "O-O-O" || player_move.trim() == "0-0-0" { // queenside castling
                if has_white_rook1_moved == false {
                    if board[59] == NOTHING && board[58] == NOTHING && board[57] == NOTHING { // squares between the rook and king must be empty
                        if get_pieces_checking_the_white_king(59, &board).len() == 0 
                        && get_pieces_checking_the_white_king(58, &board).len() == 0
                        && get_pieces_checking_the_white_king(57, &board).len() == 0 {
                            board[white_king as usize] = NOTHING;
                            board[56] = NOTHING; // kingside rook

                            white_king = 58;
                            board[white_king as usize] = WHITE_KING;
                            for w_rook in white_rooks.iter_mut() {
                                if *w_rook == 56 {
                                    *w_rook = 59;
                                    board[*w_rook as usize] = WHITE_ROOK;
                                    break;
                                }
                            }

                            try_again = false;
                        }
                    }
                }
            }else{ // pawn movement
                if san_move[1] != 'x' {
                    column = match san_move[0] {
                        'a' => 0,
                        'b' => 1,
                        'c' => 2,
                        'd' => 3,
                        'e' => 4,
                        'f' => 5,
                        'g' => 6,
                        'h' => 7,
                        _ => 100
                    };
                
                    //must be in reverse because we view the board as white
                    line = match san_move[1] {
                        '1' => 56,
                        '2' => 48,
                        '3' => 40,
                        '4' => 32,
                        '5' => 24,
                        '6' => 16,
                        '7' => 8,
                        '8' => 0,
                        _ => 100
                    };

                    if column >= 100 || line >= 100 {
                        println!("Not a possible move, try again!\n");
                        continue;
                    }

                    desired_position = column + line;

                    if wking_checks.len() == 1 {
                        if (wking_checks[0] > white_king && desired_position < white_king) // if the checking piece has a higher index than the king, the blocking pawn must also be higher
                        || (wking_checks[0] < white_king && desired_position > white_king) // if the checking piece has a lower index than the king, the blocking pawn must also be lower
                        || ((wking_checks[0]-white_king)%7 == 0 && (desired_position-white_king)%7 != 0) // the pawn is not blocking the diagonal
                        || ((wking_checks[0]-white_king)%9 == 0 && (desired_position-white_king)%9 != 0) // the pawn is not blocking the diagonal
                        || ((wking_checks[0]-white_king)%8 == 0 && (desired_position-white_king)%8 != 0) { // the pawn is not blocking the file

                            // the piece to be moved does not block the check
                            println!("That move did not block the check completely!\n");
                            continue;
                        }
                    }else if wking_checks.len() > 1 {
                        println!("Your king is being double checked. You have to move it!\n");
                        continue;
                    }

                    if board[desired_position as usize] == NOTHING {
                        match san_move[0] {
                            'a' => {
                                // for every pawn in the column
                                for pawn in &mut white_column_a.iter_mut() {
                                    // if the pawn is in it's starting position
                                    if *pawn >= 48 {
                                        if desired_position - *pawn == -16 
                                        && board[(*pawn-8) as usize] == NOTHING {
                                            board[*pawn as usize] = NOTHING;
                                            board[desired_position as usize] = WHITE_PAWN;
                                            *pawn = desired_position;
                                            
                                            try_again = false;
                                            white_column_a_enpassant = true;

                                            break;
                                        }else if desired_position - *pawn == -8 {
                                            board[*pawn as usize] = NOTHING;
                                            board[desired_position as usize] = WHITE_PAWN;
                                            *pawn = desired_position;
                                            
                                            try_again = false;
                                            break;
                                        }
                                    }else{
                                        if desired_position - *pawn == -8 {
                                            board[*pawn as usize] = NOTHING;
                                            board[desired_position as usize] = WHITE_PAWN;
                                            *pawn = desired_position;
                                            
                                            try_again = false;
                                            break;
                                        }
                                    }
                                };
                            },
                            'b' => {
                                // for every pawn in the column
                                for pawn in &mut white_column_b.iter_mut() {
                                    // if the pawn is in it's starting position
                                    if *pawn >= 48 {
                                        if desired_position - *pawn == -16 
                                        && board[(*pawn-8) as usize] == NOTHING {
                                            board[*pawn as usize] = NOTHING;
                                            board[desired_position as usize] = WHITE_PAWN;
                                            *pawn = desired_position;
                                            
                                            try_again = false;
                                            white_column_b_enpassant = true;

                                            break;
                                        }else if desired_position - *pawn == -8 {
                                            board[*pawn as usize] = NOTHING;
                                            board[desired_position as usize] = WHITE_PAWN;
                                            *pawn = desired_position;
                                            
                                            try_again = false;
                                            break;
                                        }
                                    }else{
                                        if desired_position - *pawn == -8 {
                                            board[*pawn as usize] = NOTHING;
                                            board[desired_position as usize] = WHITE_PAWN;
                                            *pawn = desired_position;
                                            
                                            try_again = false;
                                            break;
                                        }
                                    }
                                };
                            },
                            'c' => {
                                // for every pawn in the column
                                for pawn in &mut white_column_c.iter_mut() {
                                    // if the pawn is in it's starting position
                                    if *pawn >= 48 {
                                        if desired_position - *pawn == -16 
                                        && board[(*pawn-8) as usize] == NOTHING {
                                            board[*pawn as usize] = NOTHING;
                                            board[desired_position as usize] = WHITE_PAWN;
                                            *pawn = desired_position;
                                            
                                            try_again = false;
                                            white_column_c_enpassant = true;

                                            break;
                                        }else if desired_position - *pawn == -8 {
                                            board[*pawn as usize] = NOTHING;
                                            board[desired_position as usize] = WHITE_PAWN;
                                            *pawn = desired_position;
                                            
                                            try_again = false;
                                            break;
                                        }
                                    }else{
                                        if desired_position - *pawn == -8 {
                                            board[*pawn as usize] = NOTHING;
                                            board[desired_position as usize] = WHITE_PAWN;
                                            *pawn = desired_position;
                                            
                                            try_again = false;
                                            break;
                                        }
                                    }
                                };
                            },
                            'd' => {
                                // for every pawn in the column
                                for pawn in &mut white_column_d.iter_mut() {
                                    // if the pawn is in it's starting position
                                    if *pawn >= 48 {
                                        if desired_position - *pawn == -16 
                                        && board[(*pawn-8) as usize] == NOTHING {
                                            board[*pawn as usize] = NOTHING;
                                            board[desired_position as usize] = WHITE_PAWN;
                                            *pawn = desired_position;
                                            
                                            try_again = false;
                                            white_column_d_enpassant = true;

                                            break;
                                        }else if desired_position - *pawn == -8 {
                                            board[*pawn as usize] = NOTHING;
                                            board[desired_position as usize] = WHITE_PAWN;
                                            *pawn = desired_position;
                                            
                                            try_again = false;
                                            break;
                                        }
                                    }else{
                                        if desired_position - *pawn == -8 {
                                            board[*pawn as usize] = NOTHING;
                                            board[desired_position as usize] = WHITE_PAWN;
                                            *pawn = desired_position;
                                            
                                            try_again = false;
                                            break;
                                        }
                                    }
                                };
                            },
                            'e' => {
                                // for every pawn in the column
                                for pawn in &mut white_column_e.iter_mut() {
                                    // if the pawn is in it's starting position
                                    if *pawn >= 48 {
                                        if desired_position - *pawn == -16 
                                        && board[(*pawn-8) as usize] == NOTHING {
                                            board[*pawn as usize] = NOTHING;
                                            board[desired_position as usize] = WHITE_PAWN;
                                            *pawn = desired_position;
                                            
                                            try_again = false;
                                            white_column_e_enpassant = true;

                                            break;
                                        }else if desired_position - *pawn == -8 {
                                            board[*pawn as usize] = NOTHING;
                                            board[desired_position as usize] = WHITE_PAWN;
                                            *pawn = desired_position;
                                            
                                            try_again = false;
                                            break;
                                        }
                                    }else{
                                        if desired_position - *pawn == -8 {
                                            board[*pawn as usize] = NOTHING;
                                            board[desired_position as usize] = WHITE_PAWN;
                                            *pawn = desired_position;
                                            
                                            try_again = false;
                                            break;
                                        }
                                    }
                                };
                            },
                            'f' => {
                                // for every pawn in the column
                                for pawn in &mut white_column_f.iter_mut() {
                                    // if the pawn is in it's starting position
                                    if *pawn >= 48 {
                                        if desired_position - *pawn == -16 
                                        && board[(*pawn-8) as usize] == NOTHING {
                                            board[*pawn as usize] = NOTHING;
                                            board[desired_position as usize] = WHITE_PAWN;
                                            *pawn = desired_position;
                                            
                                            try_again = false;
                                            white_column_f_enpassant = true;

                                            break;
                                        }else if desired_position - *pawn == -8 {
                                            board[*pawn as usize] = NOTHING;
                                            board[desired_position as usize] = WHITE_PAWN;
                                            *pawn = desired_position;
                                            
                                            try_again = false;
                                            break;
                                        }
                                    }else{
                                        if desired_position - *pawn == -8 {
                                            board[*pawn as usize] = NOTHING;
                                            board[desired_position as usize] = WHITE_PAWN;
                                            *pawn = desired_position;
                                            
                                            try_again = false;
                                            break;
                                        }
                                    }
                                };
                            },
                            'g' => {
                                // for every pawn in the column
                                for pawn in &mut white_column_g.iter_mut() {
                                    // if the pawn is in it's starting position
                                    if *pawn >= 48 {
                                        if desired_position - *pawn == -16 
                                        && board[(*pawn-8) as usize] == NOTHING {
                                            board[*pawn as usize] = NOTHING;
                                            board[desired_position as usize] = WHITE_PAWN;
                                            *pawn = desired_position;
                                            
                                            try_again = false;
                                            white_column_g_enpassant = true;

                                            break;
                                        }else if desired_position - *pawn == -8 {
                                            board[*pawn as usize] = NOTHING;
                                            board[desired_position as usize] = WHITE_PAWN;
                                            *pawn = desired_position;
                                            
                                            try_again = false;
                                            break;
                                        }
                                    }else{
                                        if desired_position - *pawn == -8 {
                                            board[*pawn as usize] = NOTHING;
                                            board[desired_position as usize] = WHITE_PAWN;
                                            *pawn = desired_position;
                                            
                                            try_again = false;
                                            break;
                                        }
                                    }
                                };
                            },
                            'h' => {
                                // for every pawn in the column
                                for pawn in &mut white_column_h.iter_mut() {
                                    // if the pawn is in it's starting position
                                    if *pawn >= 48 {
                                        if desired_position - *pawn == -16 
                                        && board[(*pawn-8) as usize] == NOTHING {
                                            board[*pawn as usize] = NOTHING;
                                            board[desired_position as usize] = WHITE_PAWN;
                                            *pawn = desired_position;
                                            
                                            try_again = false;
                                            white_column_h_enpassant = true;

                                            break;
                                        }else if desired_position - *pawn == -8 {
                                            board[*pawn as usize] = NOTHING;
                                            board[desired_position as usize] = WHITE_PAWN;
                                            *pawn = desired_position;
                                            
                                            try_again = false;
                                            break;
                                        }
                                    }else{
                                        if desired_position - *pawn == -8 {
                                            board[*pawn as usize] = NOTHING;
                                            board[desired_position as usize] = WHITE_PAWN;
                                            *pawn = desired_position;
                                            
                                            try_again = false;
                                            break;
                                        }
                                    }
                                };
                            },
                            _ => ()
                        };

                        if line == 0 && try_again == false{ // if the pawn moved to the other side of the board successfully
                            println!("Indicate the pawn promotion: (Press Enter to promote it to a Queen)");
                            println!("'N'=Knight,'R'=Rook,'B'=Bishop");
                            let mut promotion_move = String::new();
                            let mut prom_loop: bool = true;

                            while prom_loop {
                                promotion_move.clear();
                                io::stdin()
                                    .read_line(&mut promotion_move)
                                    .expect("Read error");
                                
                                let prom_vec: Vec<char> = promotion_move.trim().chars().collect();

                                if prom_vec.len() > 0 {
                                    match san_move[0] {
                                        'a' => {
                                            match prom_vec[0] {
                                                'N' => { // promotion to knight
                                                    white_column_a.remove(0);
                                                    board[desired_position as usize] = WHITE_KNIGHT;
                                                    white_knights.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                'R' => { // promotion to rook
                                                    white_column_a.remove(0);
                                                    board[desired_position as usize] = WHITE_ROOK;
                                                    white_rooks.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                'B' => { // promotion to bishop
                                                    white_column_a.remove(0);
                                                    board[desired_position as usize] = WHITE_BISHOP;
                                                    white_bishops.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                _ => ()
                                            }
                                        },
                                        'b' => {
                                            match prom_vec[0] {
                                                'N' => { // promotion to knight
                                                    white_column_b.remove(0);
                                                    board[desired_position as usize] = WHITE_KNIGHT;
                                                    white_knights.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                'R' => { // promotion to rook
                                                    white_column_b.remove(0);
                                                    board[desired_position as usize] = WHITE_ROOK;
                                                    white_rooks.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                'B' => { // promotion to bishop
                                                    white_column_b.remove(0);
                                                    board[desired_position as usize] = WHITE_BISHOP;
                                                    white_bishops.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                _ => ()
                                            }
                                        },
                                        'c' => {
                                            match prom_vec[0] {
                                                'N' => { // promotion to knight
                                                    white_column_c.remove(0);
                                                    board[desired_position as usize] = WHITE_KNIGHT;
                                                    white_knights.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                'R' => { // promotion to rook
                                                    white_column_c.remove(0);
                                                    board[desired_position as usize] = WHITE_ROOK;
                                                    white_rooks.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                'B' => { // promotion to bishop
                                                    white_column_c.remove(0);
                                                    board[desired_position as usize] = WHITE_BISHOP;
                                                    white_bishops.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                _ => ()
                                            }
                                        },
                                        'd' => {
                                            match prom_vec[0] {
                                                'N' => { // promotion to knight
                                                    white_column_d.remove(0);
                                                    board[desired_position as usize] = WHITE_KNIGHT;
                                                    white_knights.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                'R' => { // promotion to rook
                                                    white_column_d.remove(0);
                                                    board[desired_position as usize] = WHITE_ROOK;
                                                    white_rooks.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                'B' => { // promotion to bishop
                                                    white_column_d.remove(0);
                                                    board[desired_position as usize] = WHITE_BISHOP;
                                                    white_bishops.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                _ => ()
                                            }
                                        },
                                        'e' => {
                                            match prom_vec[0] {
                                                'N' => { // promotion to knight
                                                    white_column_e.remove(0);
                                                    board[desired_position as usize] = WHITE_KNIGHT;
                                                    white_knights.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                'R' => { // promotion to rook
                                                    white_column_e.remove(0);
                                                    board[desired_position as usize] = WHITE_ROOK;
                                                    white_rooks.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                'B' => { // promotion to bishop
                                                    white_column_e.remove(0);
                                                    board[desired_position as usize] = WHITE_BISHOP;
                                                    white_bishops.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                _ => ()
                                            }
                                        },
                                        'f' => {
                                            match prom_vec[0] {
                                                'N' => { // promotion to knight
                                                    white_column_f.remove(0);
                                                    board[desired_position as usize] = WHITE_KNIGHT;
                                                    white_knights.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                'R' => { // promotion to rook
                                                    white_column_f.remove(0);
                                                    board[desired_position as usize] = WHITE_ROOK;
                                                    white_rooks.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                'B' => { // promotion to bishop
                                                    white_column_f.remove(0);
                                                    board[desired_position as usize] = WHITE_BISHOP;
                                                    white_bishops.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                _ => ()
                                            }
                                        },
                                        'g' => {
                                            match prom_vec[0] {
                                                'N' => { // promotion to knight
                                                    white_column_g.remove(0);
                                                    board[desired_position as usize] = WHITE_KNIGHT;
                                                    white_knights.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                'R' => { // promotion to rook
                                                    white_column_g.remove(0);
                                                    board[desired_position as usize] = WHITE_ROOK;
                                                    white_rooks.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                'B' => { // promotion to bishop
                                                    white_column_g.remove(0);
                                                    board[desired_position as usize] = WHITE_BISHOP;
                                                    white_bishops.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                _ => ()
                                            }
                                        },
                                        _ => ()
                                    };
                                }else{ // queen promotion (Enter key doesnt add a character to the vector, so it's lenght stays 0)
                                    match san_move[0] {
                                        'a' => {
                                            white_column_a.remove(0);
                                            board[desired_position as usize] = WHITE_QUEEN;
                                            white_queens.insert(0, desired_position);
                                            prom_loop = false;
                                        },
                                        'b' => {
                                            white_column_b.remove(0);
                                            board[desired_position as usize] = WHITE_QUEEN;
                                            white_queens.insert(0, desired_position);
                                            prom_loop = false;
                                        },
                                        'c' => {
                                            white_column_c.remove(0);
                                            board[desired_position as usize] = WHITE_QUEEN;
                                            white_queens.insert(0, desired_position);
                                            prom_loop = false;
                                        },
                                        'd' => {
                                            white_column_d.remove(0);
                                            board[desired_position as usize] = WHITE_QUEEN;
                                            white_queens.insert(0, desired_position);
                                            prom_loop = false;
                                        },
                                        'e' => {
                                            white_column_e.remove(0);
                                            board[desired_position as usize] = WHITE_QUEEN;
                                            white_queens.insert(0, desired_position);
                                            prom_loop = false;
                                        },
                                        'f' => {
                                            white_column_f.remove(0);
                                            board[desired_position as usize] = WHITE_QUEEN;
                                            white_queens.insert(0, desired_position);
                                            prom_loop = false;
                                        },
                                        'g' => {
                                            white_column_g.remove(0);
                                            board[desired_position as usize] = WHITE_QUEEN;
                                            white_queens.insert(0, desired_position);
                                            prom_loop = false;
                                        },
                                        _ => ()
                                    };
                                }
                                if prom_loop == true {
                                    println!("Not a possible promotion, try again!\n");
                                }
                            }
                        }
                    }

                }else if san_move.len() >= 4 { // if the second letter in the SAN notation move is 'x' (which means a capture) AND is longer than 3 characters:
                    column = match san_move[2] {
                        'a' => 0,
                        'b' => 1,
                        'c' => 2,
                        'd' => 3,
                        'e' => 4,
                        'f' => 5,
                        'g' => 6,
                        'h' => 7,
                        _ => 100
                    };

                    line = match san_move[3] {
                        '1' => 56,
                        '2' => 48,
                        '3' => 40,
                        '4' => 32,
                        '5' => 24,
                        '6' => 16,
                        '7' => 8,
                        '8' => 0,
                        _ => 100
                    };

                    if column >= 100 || line >= 100 {
                        println!("Not a possible move, try again!\n");
                        continue;
                    }

                    desired_position = column + line;

                    if wking_checks.len() == 1 {
                        if (wking_checks[0] > white_king && desired_position < white_king) // if the checking piece has a higher index than the king, the blocking pawn must also be higher
                        || (wking_checks[0] < white_king && desired_position > white_king) // if the checking piece has a lower index than the king, the blocking pawn must also be lower
                        || ((wking_checks[0]-white_king)%7 == 0 && (desired_position-white_king)%7 != 0) // the pawn is not blocking the diagonal
                        || ((wking_checks[0]-white_king)%9 == 0 && (desired_position-white_king)%9 != 0) // the pawn is not blocking the diagonal
                        || ((wking_checks[0]-white_king)%8 == 0 && (desired_position-white_king)%8 != 0) { // the pawn is not blocking the file

                            // the piece to be moved does not block the check
                            println!("That move did not block the check completely!\n");
                            continue;
                        }
                    }else if wking_checks.len() > 1 {
                        println!("Your king is being double checked. You have to move it!\n");
                        continue;
                    }

                    if is_black(board[desired_position as usize])
                    || (get_line(desired_position) == 6 && board[(desired_position+8) as usize] == BLACK_PAWN && match san_move[2] { // checks if the desired square's column can be a victim of en passant
                        'a' => {
                            if black_column_a_enpassant == true {
                                true
                            }else{
                                false
                            }
                        },
                        'b' => {
                            if black_column_b_enpassant == true {
                                true
                            }else{
                                false
                            }
                        },
                        'c' => {
                            if black_column_c_enpassant == true {
                                true
                            }else{
                                false
                            }
                        },
                        'd' => {
                            if black_column_d_enpassant == true {
                                true
                            }else{
                                false
                            }
                        },
                        'e' => {
                            if black_column_e_enpassant == true {
                                true
                            }else{
                                false
                            }
                        },
                        'f' => {
                            if black_column_f_enpassant == true {
                                true
                            }else{
                                false
                            }
                        },
                        'g' => {
                            if black_column_g_enpassant == true {
                                true
                            }else{
                                false
                            }
                        },
                        'h' => {
                            if black_column_h_enpassant == true {
                                true
                            }else{
                                false
                            }
                        },
                        _ => false
                    }) {
                        let captured_piece: char;
                        if board[desired_position as usize] == NOTHING {
                            captured_piece = board[(desired_position+8) as usize];
                        }else{
                            captured_piece = board[desired_position as usize];
                        }

                        match san_move[0] {
                            'a' => {
                                let mut pawn_index: usize = 0;
                                // for every *pawn in the column
                                for pawn in &mut white_column_a.iter_mut() {
                                    if desired_position - *pawn == -7 && !upper_right_diagonal(*pawn, 1) {
                                        board[*pawn as usize] = NOTHING;
                                        if board[desired_position as usize] == NOTHING { // en passant
                                            board[(desired_position+8) as usize] = NOTHING;
                                            // removes the pawn from the board
                                        }
                                        board[desired_position as usize] = WHITE_PAWN;
                                        *pawn = desired_position;

                                        white_column_b.insert(0, *pawn);

                                        white_column_a.remove(pawn_index);
                                        
                                        try_again = false;
                                        break;
                                    }
                                    // goes up each time a pawn in the column vector can't go to the desired square
                                    pawn_index += 1;
                                };
                            },
                            'b' => {
                                let mut pawn_index: usize = 0;
                                // for every *pawn in the column
                                for pawn in &mut white_column_b.iter_mut() {
                                    if (desired_position - *pawn == -7 && !upper_right_diagonal(*pawn, 1)) 
                                    || (desired_position  - *pawn == -9 && !upper_left_diagonal(*pawn, 1)) {
                                        board[*pawn as usize] = NOTHING;
                                        if board[desired_position as usize] == NOTHING { // en passant
                                            board[(desired_position+8) as usize] = NOTHING;
                                        }
                                        board[desired_position as usize] = WHITE_PAWN;
                                        *pawn = desired_position;

                                        match san_move[2]{
                                            'a' => white_column_a.insert(0, *pawn),
                                            'c' => white_column_c.insert(0, *pawn),
                                            _ => ()
                                        }

                                        white_column_b.remove(pawn_index);
                                        
                                        try_again = false;
                                        break;
                                    }
                                    pawn_index += 1;
                                };
                            },
                            'c' => {
                                let mut pawn_index: usize = 0;
                                // for every pawn in the column
                                for pawn in &mut white_column_c.iter_mut() {
                                    if (desired_position - *pawn == -7 && !upper_right_diagonal(*pawn, 1)) 
                                    || (desired_position  - *pawn == -9 && !upper_left_diagonal(*pawn, 1)) {
                                        board[*pawn as usize] = NOTHING;
                                        if board[desired_position as usize] == NOTHING { // en passant
                                            board[(desired_position+8) as usize] = NOTHING;
                                        }
                                        board[desired_position as usize] = WHITE_PAWN;
                                        *pawn = desired_position;
                                        
                                        match san_move[2]{
                                            'b' => white_column_b.insert(0, *pawn),
                                            'd' => white_column_d.insert(0, *pawn),
                                            _ => ()
                                        }

                                        white_column_c.remove(pawn_index);
                                        
                                        try_again = false;
                                        break;
                                    }
                                    pawn_index += 1;
                                };
                            },
                            'd' => {
                                let mut pawn_index: usize = 0;
                                // for every *pawn in the column
                                for pawn in &mut white_column_d.iter_mut() {
                                    if (desired_position - *pawn == -7 && !upper_right_diagonal(*pawn, 1)) 
                                    || (desired_position - *pawn == -9 && !upper_left_diagonal(*pawn, 1)) {
                                        board[*pawn as usize] = NOTHING;
                                        if board[desired_position as usize] == NOTHING { // en passant
                                            board[(desired_position+8) as usize] = NOTHING;
                                        }
                                        board[desired_position as usize] = WHITE_PAWN;
                                        *pawn = desired_position;
                                        
                                        match san_move[2]{
                                            'c' => white_column_c.insert(0, *pawn),
                                            'e' => white_column_e.insert(0, *pawn),
                                            _ => ()
                                        }

                                        white_column_d.remove(pawn_index);

                                        try_again = false;
                                        break;
                                    }
                                    pawn_index += 1;
                                };
                            },
                            'e' => {
                                let mut pawn_index: usize = 0;
                                // for every *pawn in the column
                                for pawn in &mut white_column_e.iter_mut() {
                                    if (desired_position - *pawn == -7 && !upper_right_diagonal(*pawn, 1)) 
                                    || (desired_position - *pawn == -9 && !upper_left_diagonal(*pawn, 1)) {
                                        board[*pawn as usize] = NOTHING;
                                        if board[desired_position as usize] == NOTHING { // en passant
                                            board[(desired_position+8) as usize] = NOTHING;
                                        }
                                        board[desired_position as usize] = WHITE_PAWN;
                                        *pawn = desired_position;
                                        
                                        match san_move[2]{
                                            'd' => white_column_d.insert(0, *pawn),
                                            'f' => white_column_f.insert(0, *pawn),
                                            _ => ()
                                        }
                                        
                                        white_column_e.remove(pawn_index);

                                        try_again = false;
                                        break;
                                    }
                                    pawn_index += 1;
                                };
                            },
                            'f' => {
                                let mut pawn_index: usize = 0;
                                // for every *pawn in the column
                                for pawn in &mut white_column_f.iter_mut() {
                                    if (desired_position - *pawn == -7 && !upper_right_diagonal(*pawn, 1)) 
                                    || (desired_position  - *pawn == -9 && !upper_left_diagonal(*pawn, 1)) {
                                        board[*pawn as usize] = NOTHING;
                                        if board[desired_position as usize] == NOTHING { // en passant
                                            board[(desired_position+8) as usize] = NOTHING;
                                        }
                                        board[desired_position as usize] = WHITE_PAWN;
                                        *pawn = desired_position;
                                        
                                        match san_move[2]{
                                            'e' => white_column_e.insert(0, *pawn),
                                            'g' => white_column_g.insert(0, *pawn),
                                            _ => ()
                                        }
                                        
                                        white_column_f.remove(pawn_index);

                                        try_again = false;
                                        break;
                                    }
                                    pawn_index += 1;
                                };
                            },
                            'g' => {
                                let mut pawn_index: usize = 0;
                                // for every *pawn in the column
                                for pawn in &mut white_column_g.iter_mut() {
                                    if (desired_position - *pawn == -7 && !upper_right_diagonal(*pawn, 1)) 
                                    || (desired_position  - *pawn == -9 && !upper_left_diagonal(*pawn, 1)) {
                                        board[*pawn as usize] = NOTHING;
                                        if board[desired_position as usize] == NOTHING { // en passant
                                            board[(desired_position+8) as usize] = NOTHING;
                                        }
                                        board[desired_position as usize] = WHITE_PAWN;
                                        *pawn = desired_position;
                                        
                                        match san_move[2]{
                                            'f' => white_column_f.insert(0, *pawn),
                                            'h' => white_column_h.insert(0, *pawn),
                                            _ => ()
                                        }

                                        white_column_g.remove(pawn_index);

                                        try_again = false;
                                        break;
                                    }
                                    pawn_index += 1;
                                };
                            },
                            'h' => {
                                let mut pawn_index: usize = 0;
                                // for every *pawn in the column
                                for pawn in &mut white_column_h.iter_mut() {
                                    if desired_position - *pawn == -9 && !upper_left_diagonal(*pawn, 1) {
                                        board[*pawn as usize] = NOTHING;
                                        if board[desired_position as usize] == NOTHING { // en passant
                                            board[(desired_position+8) as usize] = NOTHING;
                                        }
                                        board[desired_position as usize] = WHITE_PAWN;
                                        *pawn = desired_position;
                                        
                                        white_column_g.insert(0, *pawn);

                                        white_column_h.remove(pawn_index);

                                        try_again = false;
                                        break;
                                    }
                                    pawn_index += 1;
                                };
                            },
                            _ => ()
                        }
                    
                        if try_again == false {
                            match captured_piece {
                                'j' => {
                                    match san_move[2] {
                                        'a' => {
                                            if black_column_a_enpassant == false {
                                                for pawn in 0..black_column_a.len() {
                                                    if black_column_a[pawn] == desired_position {
                                                        black_column_a.swap_remove(pawn);
                                                        break;
                                                    }
                                                };
                                            }else{
                                                for pawn in 0..black_column_a.len() {
                                                    if black_column_a[pawn] == desired_position+8 {
                                                        black_column_a.swap_remove(pawn);
                                                        break;
                                                    }
                                                };
                                            }
                                        },
                                        'b' => {
                                            if black_column_b_enpassant == false {
                                                for pawn in 0..black_column_b.len() {
                                                    if black_column_b[pawn] == desired_position {
                                                        black_column_b.swap_remove(pawn);
                                                        break;
                                                    }
                                                };
                                            }else{
                                                for pawn in 0..black_column_b.len() {
                                                    if black_column_b[pawn] == desired_position+8 {
                                                        black_column_b.swap_remove(pawn);
                                                        break;
                                                    }
                                                };
                                            }
                                        },
                                        'c' => {
                                            if black_column_c_enpassant == false {
                                                for pawn in 0..black_column_c.len() {
                                                    if black_column_c[pawn] == desired_position {
                                                        black_column_c.swap_remove(pawn);
                                                        break;
                                                    }
                                                };
                                            }else{
                                                for pawn in 0..black_column_c.len() {
                                                    if black_column_c[pawn] == desired_position+8 {
                                                        black_column_c.swap_remove(pawn);
                                                        break;
                                                    }
                                                };
                                            }
                                        },
                                        'd' => {
                                            if black_column_d_enpassant == false {
                                                for pawn in 0..black_column_d.len() {
                                                    if black_column_d[pawn] == desired_position {
                                                        black_column_d.swap_remove(pawn);
                                                        break;
                                                    }
                                                };
                                            }else{
                                                for pawn in 0..black_column_d.len() {
                                                    if black_column_d[pawn] == desired_position+8 {
                                                        black_column_d.swap_remove(pawn);
                                                        break;
                                                    }
                                                };
                                            }
                                        },
                                        'e' => {
                                            if black_column_e_enpassant == false {
                                                for pawn in 0..black_column_d.len() {
                                                    if black_column_d[pawn] == desired_position {
                                                        black_column_d.swap_remove(pawn);
                                                        break;
                                                    }
                                                };
                                            }else{
                                                for pawn in 0..black_column_d.len() {
                                                    if black_column_d[pawn] == desired_position+8 {
                                                        black_column_d.swap_remove(pawn);
                                                        break;
                                                    }
                                                };
                                            }
                                        },
                                        'f' => {
                                            if black_column_f_enpassant == false {
                                                for pawn in 0..black_column_f.len() {
                                                    if black_column_f[pawn] == desired_position {
                                                        black_column_f.swap_remove(pawn);
                                                        break;
                                                    }
                                                };
                                            }else{
                                                for pawn in 0..black_column_f.len() {
                                                    if black_column_f[pawn] == desired_position+8 {
                                                        black_column_f.swap_remove(pawn);
                                                        break;
                                                    }
                                                };
                                            }
                                        },
                                        'g' => {
                                            if black_column_g_enpassant == false {
                                                for pawn in 0..black_column_g.len() {
                                                    if black_column_g[pawn] == desired_position {
                                                        black_column_g.swap_remove(pawn);
                                                        break;
                                                    }
                                                };
                                            }else{
                                                for pawn in 0..black_column_g.len() {
                                                    if black_column_g[pawn] == desired_position+8 {
                                                        black_column_g.swap_remove(pawn);
                                                        break;
                                                    }
                                                };
                                            }
                                        },
                                        'h' => {
                                            if black_column_h_enpassant == false {
                                                for pawn in 0..black_column_h.len() {
                                                    if black_column_h[pawn] == desired_position {
                                                        black_column_h.swap_remove(pawn);
                                                        break;
                                                    }
                                                };
                                            }else{
                                                for pawn in 0..black_column_h.len() {
                                                    if black_column_h[pawn] == desired_position+8{
                                                        black_column_h.swap_remove(pawn);
                                                        break;
                                                    }
                                                };
                                            }
                                        },
                                        _ => ()
                                    }
                                },
                                'q' => {
                                    for i in 0..black_queens.len() {
                                        if black_queens[i] == desired_position {
                                            black_queens.swap_remove(i);
                                            break;
                                        }
                                    };
                                },
                                'b' => {
                                    for i in 0..black_bishops.len() {
                                        if black_bishops[i] == desired_position {
                                            black_bishops.swap_remove(i);
                                            break;
                                        }
                                    };
                                },
                                'n' => {
                                    for i in 0..black_knights.len() {
                                        if black_knights[i] == desired_position {
                                            black_knights.swap_remove(i);
                                            break;
                                        }
                                    };
                                },
                                'r' => {
                                    for i in 0..black_rooks.len() {
                                        if black_rooks[i] == desired_position {
                                            black_rooks.swap_remove(i);
                                            break;
                                        }
                                    };
                                },
                                _ => ()
                            }
                        }

                        if line == 0 && try_again == false{ // if the pawn moved to the other side of the board successfully
                            println!("Indicate the pawn promotion: (Press Enter to promote it to a Queen)");
                            println!("'N'=Knight,'R'=Rook,'B'=Bishop");
                            let mut promotion_move = String::new();
                            let mut prom_loop: bool = true;

                            while prom_loop {
                                promotion_move.clear();
                                io::stdin()
                                    .read_line(&mut promotion_move)
                                    .expect("Read error");
                                
                                let prom_vec: Vec<char> = promotion_move.trim().chars().collect();

                                if prom_vec.len() > 0 {
                                    match san_move[2] {
                                        'a' => {
                                            match prom_vec[0] {
                                                'N' => { // promotion to knight
                                                    white_column_a.remove(0);
                                                    board[desired_position as usize] = WHITE_KNIGHT;
                                                    white_knights.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                'R' => { // promotion to rook
                                                    white_column_a.remove(0);
                                                    board[desired_position as usize] = WHITE_ROOK;
                                                    white_rooks.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                'B' => { // promotion to bishop
                                                    white_column_a.remove(0);
                                                    board[desired_position as usize] = WHITE_BISHOP;
                                                    white_bishops.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                _ => ()
                                            }
                                        },
                                        'b' => {
                                            match prom_vec[0] {
                                                'N' => { // promotion to knight
                                                    white_column_b.remove(0);
                                                    board[desired_position as usize] = WHITE_KNIGHT;
                                                    white_knights.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                'R' => { // promotion to rook
                                                    white_column_b.remove(0);
                                                    board[desired_position as usize] = WHITE_ROOK;
                                                    white_rooks.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                'B' => { // promotion to bishop
                                                    white_column_b.remove(0);
                                                    board[desired_position as usize] = WHITE_BISHOP;
                                                    white_bishops.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                _ => ()
                                            }
                                        },
                                        'c' => {
                                            match prom_vec[0] {
                                                'N' => { // promotion to knight
                                                    white_column_c.remove(0);
                                                    board[desired_position as usize] = WHITE_KNIGHT;
                                                    white_knights.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                'R' => { // promotion to rook
                                                    white_column_c.remove(0);
                                                    board[desired_position as usize] = WHITE_ROOK;
                                                    white_rooks.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                'B' => { // promotion to bishop
                                                    white_column_c.remove(0);
                                                    board[desired_position as usize] = WHITE_BISHOP;
                                                    white_bishops.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                _ => ()
                                            }
                                        },
                                        'd' => {
                                            match prom_vec[0] {
                                                'N' => { // promotion to knight
                                                    white_column_d.remove(0);
                                                    board[desired_position as usize] = WHITE_KNIGHT;
                                                    white_knights.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                'R' => { // promotion to rook
                                                    white_column_d.remove(0);
                                                    board[desired_position as usize] = WHITE_ROOK;
                                                    white_rooks.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                'B' => { // promotion to bishop
                                                    white_column_d.remove(0);
                                                    board[desired_position as usize] = WHITE_BISHOP;
                                                    white_bishops.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                _ => ()
                                            }
                                        },
                                        'e' => {
                                            match prom_vec[0] {
                                                'N' => { // promotion to knight
                                                    white_column_e.remove(0);
                                                    board[desired_position as usize] = WHITE_KNIGHT;
                                                    white_knights.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                'R' => { // promotion to rook
                                                    white_column_e.remove(0);
                                                    board[desired_position as usize] = WHITE_ROOK;
                                                    white_rooks.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                'B' => { // promotion to bishop
                                                    white_column_e.remove(0);
                                                    board[desired_position as usize] = WHITE_BISHOP;
                                                    white_bishops.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                _ => ()
                                            }
                                        },
                                        'f' => {
                                            match prom_vec[0] {
                                                'N' => { // promotion to knight
                                                    white_column_f.remove(0);
                                                    board[desired_position as usize] = WHITE_KNIGHT;
                                                    white_knights.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                'R' => { // promotion to rook
                                                    white_column_f.remove(0);
                                                    board[desired_position as usize] = WHITE_ROOK;
                                                    white_rooks.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                'B' => { // promotion to bishop
                                                    white_column_f.remove(0);
                                                    board[desired_position as usize] = WHITE_BISHOP;
                                                    white_bishops.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                _ => ()
                                            }
                                        },
                                        'g' => {
                                            match prom_vec[0] {
                                                'N' => { // promotion to knight
                                                    white_column_g.remove(0);
                                                    board[desired_position as usize] = WHITE_KNIGHT;
                                                    white_knights.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                'R' => { // promotion to rook
                                                    white_column_g.remove(0);
                                                    board[desired_position as usize] = WHITE_ROOK;
                                                    white_rooks.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                'B' => { // promotion to bishop
                                                    white_column_g.remove(0);
                                                    board[desired_position as usize] = WHITE_BISHOP;
                                                    white_bishops.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                _ => ()
                                            }
                                        },
                                        _ => ()
                                    };
                                }else{ // queen promotion (Enter key doesnt add a character to the vector, so it's lenght stays 0)
                                    match san_move[2] {
                                        'a' => {
                                            white_column_a.remove(0);
                                            board[desired_position as usize] = WHITE_QUEEN;
                                            white_queens.insert(0, desired_position);
                                            prom_loop = false;
                                        },
                                        'b' => {
                                            white_column_b.remove(0);
                                            board[desired_position as usize] = WHITE_QUEEN;
                                            white_queens.insert(0, desired_position);
                                            prom_loop = false;
                                        },
                                        'c' => {
                                            white_column_c.remove(0);
                                            board[desired_position as usize] = WHITE_QUEEN;
                                            white_queens.insert(0, desired_position);
                                            prom_loop = false;
                                        },
                                        'd' => {
                                            white_column_d.remove(0);
                                            board[desired_position as usize] = WHITE_QUEEN;
                                            white_queens.insert(0, desired_position);
                                            prom_loop = false;
                                        },
                                        'e' => {
                                            white_column_e.remove(0);
                                            board[desired_position as usize] = WHITE_QUEEN;
                                            white_queens.insert(0, desired_position);
                                            prom_loop = false;
                                        },
                                        'f' => {
                                            white_column_f.remove(0);
                                            board[desired_position as usize] = WHITE_QUEEN;
                                            white_queens.insert(0, desired_position);
                                            prom_loop = false;
                                        },
                                        'g' => {
                                            white_column_g.remove(0);
                                            board[desired_position as usize] = WHITE_QUEEN;
                                            white_queens.insert(0, desired_position);
                                            prom_loop = false;
                                        },
                                        _ => ()
                                    };
                                }
                                if prom_loop == true {
                                    println!("Not a possible promotion, try again!\n");
                                }
                            }
                        }
                    }
                }
            }

            if try_again == true{
                println!("Not a possible move, try again!\n");
            }
        }


        // BLACK'S TURN:
        let bking_checks = get_pieces_checking_the_black_king(black_king, &board);
        try_again = true;
        show_board(board); //print the board

        black_column_a_enpassant = false;
        black_column_b_enpassant = false;
        black_column_c_enpassant = false;
        black_column_d_enpassant = false;
        black_column_e_enpassant = false;
        black_column_f_enpassant = false;
        black_column_g_enpassant = false;
        black_column_h_enpassant = false;

        while try_again{ // black's turn
            player_move.clear();

            println!("Black moves");

            io::stdin()
                .read_line(&mut player_move)
                .expect("Read error");

            let mut san_move: Vec<char> = player_move.trim().chars().collect();

            if san_move.len() <= 1 {
                println!("To move, input atleast a letter from 'a' to 'h' and a number from 1 to 8 (i.e. 'e4')");
                continue
            }

            if is_piece(san_move[0]) == true {
                column = match san_move[1] {
                    'a' => 0,
                    'b' => 1,
                    'c' => 2,
                    'd' => 3,
                    'e' => 4,
                    'f' => 5,
                    'g' => 6,
                    'h' => 7,
                    _ => 100
                };

                //must be in reverse because we view the board as white
                line = match san_move[2] {
                    '1' => 56,
                    '2' => 48,
                    '3' => 40,
                    '4' => 32,
                    '5' => 24,
                    '6' => 16,
                    '7' => 8,
                    '8' => 0,
                    _ => 100
                };

                if column >= 100 || line >= 100 {
                    println!("Not a possible move, try again!\n");
                    continue;
                }

                desired_position = column + line;

                if bking_checks.len() == 1 { // king is being checked by a single piece/pawn
                    if san_move[0] == 'K' {
                        if match desired_position - black_king { // check if it's a possible king move (otherwise just ignore the following)
                            -9|-8|-7|-1|1|7|8|9 => true,
                            _ => false
                        } {
                            if get_pieces_checking_the_black_king(desired_position, &board).len() > 0 {
                                println!("Your king is in check and that position would still keep it in check!\n");
                                continue;
                            }
                        }else{ // not even a possible king move
                            println!("Not a possible move, try again!\n");
                            continue;
                        }
                    }else if (bking_checks[0] > black_king && desired_position < black_king) // if the checking piece has a higher index than the king, the blocking piece must also be higher
                        || (bking_checks[0] < black_king && desired_position > black_king) // if the checking piece has a lower index than the king, the blocking piece must also be lower
                        || ((bking_checks[0]-black_king)%7 == 0 && (desired_position-black_king)%7 != 0) // the piece is not blocking the diagonal
                        || ((bking_checks[0]-black_king)%9 == 0 && (desired_position-black_king)%9 != 0) // the piece is not blocking the diagonal
                        || ((bking_checks[0]-black_king)%8 == 0 && (desired_position-black_king)%8 != 0) { // the piece is not blocking the file

                        // the piece to be moved does not block the check
                        println!("That move did not block the check completely!\n");
                        continue;
                    }
                }else if bking_checks.len() > 1 {
                    // incase of a double check, the king MUST move
                    if san_move[0] == 'K' {
                        if match desired_position - black_king { // check if it's a possible king move (otherwise just ignore the following)
                            -9|-8|-7|-1|1|7|8|9 => true,
                            _ => false
                        } {
                            if get_pieces_checking_the_black_king(desired_position, &board).len() > 0 {
                                println!("Your king is in check and that position would still keep it in check!\n");
                                continue;
                            }
                        }else{ // not even a possible king move
                            println!("Not a possible move, try again!\n");
                            continue;
                        }
                    }else{
                        println!("Your king is being double checked. You have to move it!\n");
                        continue;
                    }
                }

                if !is_black(board[desired_position as usize]) {
                    let captured_piece = board[desired_position as usize];

                    match san_move[0] {
                        'N' => {
                            if test_multiple_knights(&mut black_knights, desired_position) {
                                println!("Specify the current square of the knight to be moved");
                                player_move.clear();
                                san_move.clear();
                            
                                io::stdin()
                                        .read_line(&mut player_move)
                                        .expect("Read error");
                                
                                san_move = player_move.trim().chars().collect();
                            
                                let knight_column: i8 = match san_move[0] {
                                            'a' => 0,
                                            'b' => 1,
                                            'c' => 2,
                                            'd' => 3,
                                            'e' => 4,
                                            'f' => 5,
                                            'g' => 6,
                                            'h' => 7,
                                            _ => 100
                                        };
                                    
                                let knight_line: i8 = match san_move[1] {
                                            '1' => 56,
                                            '2' => 48,
                                            '3' => 40,
                                            '4' => 32,
                                            '5' => 24,
                                            '6' => 16,
                                            '7' => 8,
                                            '8' => 0,
                                            _ => 100
                                        };

                                for b_knight in black_knights.iter_mut() {
                                    if knight_column + knight_line == *b_knight {
                                        match *b_knight - desired_position {
                                            -17 | -15 | -10 | -6 | 6 | 10 | 15 | 17 => {
                                                    board[*b_knight as usize] = NOTHING;
                                                    board[desired_position as usize] = BLACK_KNIGHT;
                                                    *b_knight = desired_position;

                                                    try_again = false;
                                                    break;
                                            },
                                            _ => ()
                                        }
                                    }
                                }
                            }else{
                                for b_knight in black_knights.iter_mut() {
                                    match *b_knight - desired_position {
                                        -17 | -15 | -10 | -6 | 6 | 10 | 15 | 17 => {
                                                board[*b_knight as usize] = NOTHING;
                                                board[desired_position as usize] = BLACK_KNIGHT;
                                                *b_knight = desired_position;

                                                try_again = false;
                                                break;
                                        },
                                        _ => ()
                                    }
                                };
                            }
                        },
                        'B' => {
                            if test_multiple_bishops(&mut black_bishops, desired_position) == true {
                                println!("Specify the current square of the bishop to be moved");
                                player_move.clear();
                                san_move.clear();
                            
                                io::stdin()
                                        .read_line(&mut player_move)
                                        .expect("Read error");
                                
                                san_move = player_move.trim().chars().collect();
                            
                                let bishop_column: i8 = match san_move[0] {
                                            'a' => 0,
                                            'b' => 1,
                                            'c' => 2,
                                            'd' => 3,
                                            'e' => 4,
                                            'f' => 5,
                                            'g' => 6,
                                            'h' => 7,
                                            _ => 100
                                        };
                                    
                                let bishop_line: i8 = match san_move[1] {
                                            '1' => 56,
                                            '2' => 48,
                                            '3' => 40,
                                            '4' => 32,
                                            '5' => 24,
                                            '6' => 16,
                                            '7' => 8,
                                            '8' => 0,
                                            _ => 100
                                        };
                                
                                for b_bishop in black_bishops.iter_mut() {
                                    if bishop_line + bishop_column == *b_bishop {
                                        if *b_bishop > desired_position {
                                            if (*b_bishop - desired_position)%7 == 0 {
                                                for diagonal in 1..8 {
                                                    if *b_bishop - diagonal*7 == desired_position && !upper_right_diagonal(*b_bishop, diagonal) {
                                                        board[*b_bishop as usize] = NOTHING;
                                                        board[desired_position as usize] = BLACK_BISHOP;
                                                        *b_bishop = desired_position;
    
                                                        try_again = false;
                                                        break;
                                                    }else if is_white(board[(*b_bishop - diagonal*7) as usize]) 
                                                    || is_black(board[((*b_bishop - diagonal*7) as usize)])
                                                    || upper_right_diagonal(*b_bishop, diagonal) {
                                                        break;
                                                    }
                                                }
                                            }else if (*b_bishop - desired_position)%9 == 0 {
                                                for diagonal in 1..8 {
                                                    if *b_bishop - diagonal*9 == desired_position && !upper_left_diagonal(*b_bishop, diagonal) {
                                                        board[*b_bishop as usize] = NOTHING;
                                                        board[desired_position as usize] = BLACK_BISHOP;
                                                        *b_bishop = desired_position;
    
                                                        try_again = false;
                                                        break;
                                                    }else if is_white(board[(*b_bishop - diagonal*9) as usize]) 
                                                    || is_black(board[((*b_bishop - diagonal*9) as usize)]) 
                                                    || upper_left_diagonal(*b_bishop, diagonal) {
                                                        break;
                                                    }
                                                }
                                            }
                                        }else if *b_bishop < desired_position {
                                            if (*b_bishop - desired_position)%7 == 0 {
                                                for diagonal in 1..8 {
                                                    if *b_bishop + diagonal*7 == desired_position && !inferior_left_diagonal(*b_bishop, diagonal) {
                                                        board[*b_bishop as usize] = NOTHING;
                                                        board[desired_position as usize] = BLACK_BISHOP;
                                                        *b_bishop = desired_position;
    
                                                        try_again = false;
                                                        break;
                                                    }else if is_white(board[(*b_bishop + 7*diagonal) as usize]) 
                                                    || is_black(board[(*b_bishop + 7*diagonal) as usize]) 
                                                    || inferior_left_diagonal(*b_bishop, diagonal) {
                                                        break;
                                                    }
                                                }
                                            }else if (*b_bishop - desired_position)%9 == 0 {
                                                for diagonal in 1..8 {
                                                    if *b_bishop + diagonal*9 == desired_position && !inferior_right_diagonal(*b_bishop, diagonal) {
                                                        board[*b_bishop as usize] = NOTHING;
                                                        board[desired_position as usize] = BLACK_BISHOP;
                                                        *b_bishop = desired_position;
    
                                                        try_again = false;
                                                        break;
                                                    }else if is_white(board[(*b_bishop + 9*diagonal) as usize]) 
                                                    || is_black(board[(*b_bishop + 9*diagonal) as usize])
                                                    || inferior_right_diagonal(*b_bishop, diagonal) {
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                };
                            }else{
                                for b_bishop in black_bishops.iter_mut() {
                                    if *b_bishop > desired_position {
                                        if (*b_bishop - desired_position)%7 == 0 {
                                            for diagonal in 1..8 {
                                                if *b_bishop - diagonal*7 == desired_position && !upper_right_diagonal(*b_bishop, diagonal) {
                                                    board[*b_bishop as usize] = NOTHING;
                                                    board[desired_position as usize] = BLACK_BISHOP;
                                                    *b_bishop = desired_position;

                                                    try_again = false;
                                                    break;
                                                }else if is_white(board[(*b_bishop - diagonal*7) as usize]) 
                                                || is_black(board[((*b_bishop - diagonal*7) as usize)])
                                                || upper_right_diagonal(*b_bishop, diagonal) {
                                                    break;
                                                }
                                            }
                                        }else if (*b_bishop - desired_position)%9 == 0 {
                                            for diagonal in 1..8 {
                                                if *b_bishop - diagonal*9 == desired_position && !upper_left_diagonal(*b_bishop, diagonal) {
                                                    board[*b_bishop as usize] = NOTHING;
                                                    board[desired_position as usize] = BLACK_BISHOP;
                                                    *b_bishop = desired_position;

                                                    try_again = false;
                                                    break;
                                                }else if is_white(board[(*b_bishop - diagonal*9) as usize]) 
                                                || is_black(board[((*b_bishop - diagonal*9) as usize)]) 
                                                || upper_left_diagonal(*b_bishop, diagonal) {
                                                    break;
                                                }
                                            }
                                        }
                                    }else if *b_bishop < desired_position {
                                        if (*b_bishop - desired_position)%7 == 0 {
                                            for diagonal in 1..8 {
                                                if *b_bishop + diagonal*7 == desired_position && !inferior_left_diagonal(*b_bishop, diagonal) {
                                                    board[*b_bishop as usize] = NOTHING;
                                                    board[desired_position as usize] = BLACK_BISHOP;
                                                    *b_bishop = desired_position;

                                                    try_again = false;
                                                    break;
                                                }else if is_white(board[(*b_bishop + 7*diagonal) as usize]) 
                                                || is_black(board[(*b_bishop + 7*diagonal) as usize]) 
                                                || inferior_left_diagonal(*b_bishop, diagonal) {
                                                    break;
                                                }
                                            }
                                        }else if (*b_bishop - desired_position)%9 == 0 {
                                            for diagonal in 1..8 {
                                                if *b_bishop + diagonal*9 == desired_position && !inferior_right_diagonal(*b_bishop, diagonal) {
                                                    board[*b_bishop as usize] = NOTHING;
                                                    board[desired_position as usize] = BLACK_BISHOP;
                                                    *b_bishop = desired_position;

                                                    try_again = false;
                                                    break;
                                                }else if is_white(board[(*b_bishop + 9*diagonal) as usize]) 
                                                || is_black(board[(*b_bishop + 9*diagonal) as usize])
                                                || inferior_right_diagonal(*b_bishop, diagonal) {
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                };
                            }
                        },
                        'R' => {
                            // check if more than one rook can reach the desired square
                            if test_multiple_rooks(&mut black_rooks, desired_position) == true {
                                println!("Specify the current square of the rook to be moved");
                                player_move.clear();
                                san_move.clear();
                            
                                io::stdin()
                                        .read_line(&mut player_move)
                                        .expect("Read error");
                                
                                san_move = player_move.trim().chars().collect();
                            
                                let rook_column: i8 = match san_move[0] {
                                            'a' => 0,
                                            'b' => 1,
                                            'c' => 2,
                                            'd' => 3,
                                            'e' => 4,
                                            'f' => 5,
                                            'g' => 6,
                                            'h' => 7,
                                            _ => 100
                                        };
                                    
                                let rook_line: i8 = match san_move[1] {
                                            '1' => 56,
                                            '2' => 48,
                                            '3' => 40,
                                            '4' => 32,
                                            '5' => 24,
                                            '6' => 16,
                                            '7' => 8,
                                            '8' => 0,
                                            _ => 100
                                        };
                                for b_rook in &mut black_rooks.iter_mut() {
                                    if rook_column + rook_line == *b_rook {
                                        if *b_rook > desired_position {
                                            if get_line(*b_rook) == get_line(desired_position) {
                                                for square in 1..8 {
                                                    if *b_rook - square == desired_position && !rook_left(*b_rook, square) {
                                                        board[*b_rook as usize] = NOTHING;
                                                        board[desired_position as usize] = BLACK_ROOK;
                                                        match *b_rook {
                                                            0 => has_black_rook1_moved = true,
                                                            7 => has_black_rook2_moved = true,
                                                            _ => ()
                                                        }
                                                        *b_rook = desired_position;
                
                                                        try_again = false;
                                                        break;
                                                    }else if is_white(board[(*b_rook - square) as usize]) 
                                                    || is_black(board[(*b_rook - square) as usize])
                                                    || rook_left(*b_rook, square) {
                                                        break;
                                                    }
                                                }
                                            }else if (*b_rook - desired_position)%8 == 0 {
                                                for square in 1..8 {
                                                    if *b_rook - square*8 == desired_position && !rook_up(*b_rook, square) {
                                                        board[*b_rook as usize] = NOTHING;
                                                        board[desired_position as usize] = BLACK_ROOK;
                                                        match *b_rook {
                                                            0 => has_black_rook1_moved = true,
                                                            7 => has_black_rook2_moved = true,
                                                            _ => ()
                                                        }
                                                        *b_rook = desired_position;
                
                                                        try_again = false;
                                                        break;
                                                    }else if is_white(board[(*b_rook - square*8) as usize]) 
                                                    || is_black(board[(*b_rook - square*8) as usize])
                                                    || rook_up(*b_rook, square) {
                                                        break;
                                                    }
                                                }
                                            }
                                        }else if *b_rook < desired_position {
                                            if get_line(*b_rook) == get_line(desired_position) {
                                                for square in 1..8 {
                                                    if *b_rook + square == desired_position && !rook_right(*b_rook, square) {
                                                        board[*b_rook as usize] = NOTHING;
                                                        board[desired_position as usize] = BLACK_ROOK;
                                                        match *b_rook {
                                                            0 => has_black_rook1_moved = true,
                                                            7 => has_black_rook2_moved = true,
                                                            _ => ()
                                                        }
                                                        *b_rook = desired_position;
                
                                                        try_again = false;
                                                        break;
                                                    }else if is_white(board[(*b_rook + square) as usize]) 
                                                    || is_black(board[(*b_rook + square) as usize])
                                                    || rook_right(*b_rook, square) {
                                                        break;
                                                    }
                                                }
                                            }else if (desired_position - *b_rook)%8 == 0 {
                                                for square in 1..8 {
                                                    if *b_rook + square*8 == desired_position && !rook_down(*b_rook, square) {
                                                        board[*b_rook as usize] = NOTHING;
                                                        board[desired_position as usize] = BLACK_ROOK;
                                                        match *b_rook {
                                                            0 => has_black_rook1_moved = true,
                                                            7 => has_black_rook2_moved = true,
                                                            _ => ()
                                                        }
                                                        *b_rook = desired_position;
                
                                                        try_again = false;
                                                        break;
                                                    }else if is_white(board[(*b_rook + square*8) as usize]) 
                                                    || is_black(board[(*b_rook + square*8) as usize])
                                                    || rook_down(*b_rook, square) {
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                };
                            }else{ // only one rook may reach the desired square
                                for b_rook in &mut black_rooks.iter_mut() {    
                                    if *b_rook > desired_position && !is_black(board[desired_position as usize]) {
                                        if get_line(*b_rook) == get_line(desired_position) {
                                            for square in 1..8 {
                                                if *b_rook - square == desired_position && !rook_left(*b_rook, square) {
                                                    board[*b_rook as usize] = NOTHING;
                                                    board[desired_position as usize] = BLACK_ROOK;
                                                    match *b_rook {
                                                        0 => has_black_rook1_moved = true,
                                                        7 => has_black_rook2_moved = true,
                                                        _ => ()
                                                    }
                                                    *b_rook = desired_position;
            
                                                    try_again = false;
                                                    break;
                                                }else if is_white(board[(*b_rook - square) as usize]) 
                                                || is_black(board[(*b_rook - square) as usize])
                                                || rook_left(*b_rook, square) {
                                                    break;
                                                }
                                            }
                                        }else if (*b_rook - desired_position)%8 == 0 {
                                            for square in 1..8 {
                                                if *b_rook - square*8 == desired_position && !rook_up(*b_rook, square) {
                                                    board[*b_rook as usize] = NOTHING;
                                                    board[desired_position as usize] = BLACK_ROOK;
                                                    match *b_rook {
                                                        0 => has_black_rook1_moved = true,
                                                        7 => has_black_rook2_moved = true,
                                                        _ => ()
                                                    }
                                                    *b_rook = desired_position;
            
                                                    try_again = false;
                                                    break;
                                                }else if is_white(board[(*b_rook - square*8) as usize]) 
                                                || is_black(board[(*b_rook - square*8) as usize])
                                                || rook_up(*b_rook, square) {
                                                    break;
                                                }
                                            }
                                        }
                                    }else if *b_rook < desired_position && !is_black(board[desired_position as usize]) {
                                        if get_line(*b_rook) == get_line(desired_position) {
                                            for square in 1..8 {
                                                if *b_rook + square == desired_position && !rook_right(*b_rook, square) {
                                                    board[*b_rook as usize] = NOTHING;
                                                    board[desired_position as usize] = BLACK_ROOK;
                                                    match *b_rook {
                                                        0 => has_black_rook1_moved = true,
                                                        7 => has_black_rook2_moved = true,
                                                        _ => ()
                                                    }
                                                    *b_rook = desired_position;
            
                                                    try_again = false;
                                                    break;
                                                }else if is_white(board[(*b_rook + square) as usize]) 
                                                || is_black(board[(*b_rook + square) as usize])
                                                || rook_right(*b_rook, square) {
                                                    break;
                                                }
                                            }
                                        }else if (desired_position - *b_rook)%8 == 0 {
                                            for square in 1..8 {
                                                if *b_rook + square*8 == desired_position && !rook_down(*b_rook, square) {
                                                    board[*b_rook as usize] = NOTHING;
                                                    board[desired_position as usize] = BLACK_ROOK;
                                                    match *b_rook {
                                                        0 => has_black_rook1_moved = true,
                                                        7 => has_black_rook2_moved = true,
                                                        _ => ()
                                                    }
                                                    *b_rook = desired_position;
            
                                                    try_again = false;
                                                    break;
                                                }else if is_white(board[(*b_rook + square*8) as usize]) 
                                                || is_black(board[(*b_rook + square*8) as usize])
                                                || rook_down(*b_rook, square) {
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                };
                            }
                        },
                        'K' => {
                            match desired_position - black_king {
                                -9 => {
                                    if !upper_left_diagonal(black_king, 1) {
                                        board[black_king as usize] = NOTHING;
                                        board[desired_position as usize] = BLACK_KING;
                                        black_king = desired_position;
                                        try_again = false;

                                        has_black_rook1_moved = true;
                                        has_black_rook2_moved = true;
                                        }
                                    },
                                -8 => {
                                    if !rook_up(black_king, 1) {
                                        board[black_king as usize] = NOTHING;
                                        board[desired_position as usize] = BLACK_KING;
                                        black_king = desired_position;
                                        try_again = false;

                                        has_black_rook1_moved = true;
                                        has_black_rook2_moved = true;
                                        }
                                    },
                                -7 => {
                                    if !upper_right_diagonal(black_king, 1) {
                                        board[black_king as usize] = NOTHING;
                                        board[desired_position as usize] = BLACK_KING;
                                        black_king = desired_position;
                                        try_again = false;

                                        has_black_rook1_moved = true;
                                        has_black_rook2_moved = true;
                                        }
                                    },
                                -1 => {
                                    if !rook_left(black_king, 1) {
                                        board[black_king as usize] = NOTHING;
                                        board[desired_position as usize] = BLACK_KING;
                                        black_king = desired_position;
                                        try_again = false;

                                        has_black_rook1_moved = true;
                                        has_black_rook2_moved = true;
                                        }
                                    },
                                1 => {
                                    if !rook_right(black_king, 1) {
                                        board[black_king as usize] = NOTHING;
                                        board[desired_position as usize] = BLACK_KING;
                                        black_king = desired_position;
                                        try_again = false;

                                        has_black_rook1_moved = true;
                                        has_black_rook2_moved = true;
                                        }
                                    },
                                7 => {
                                    if !inferior_left_diagonal(black_king, 1) {
                                        board[black_king as usize] = NOTHING;
                                        board[desired_position as usize] = BLACK_KING;
                                        black_king = desired_position;
                                        try_again = false;

                                        has_black_rook1_moved = true;
                                        has_black_rook2_moved = true;
                                        }
                                    },
                                8 => {
                                    if !rook_down(black_king, 1) {
                                        board[black_king as usize] = NOTHING;
                                        board[desired_position as usize] = BLACK_KING;
                                        black_king = desired_position;
                                        try_again = false;

                                        has_black_rook1_moved = true;
                                        has_black_rook2_moved = true;
                                        }
                                    },
                                9 => {
                                    if !inferior_right_diagonal(black_king, 1) {
                                        board[black_king as usize] = NOTHING;
                                        board[desired_position as usize] = BLACK_KING;
                                        black_king = desired_position;
                                        try_again = false;

                                        has_black_rook1_moved = true;
                                        has_black_rook2_moved = true;
                                        }
                                    },
                                _ => ()
                            }
                        },
                        'Q' => {
                            if test_multiple_queens(&mut black_queens, desired_position) == true {
                                println!("Specify the current square of the queen to be moved");
                                player_move.clear();
                                san_move.clear();
                            
                                io::stdin()
                                        .read_line(&mut player_move)
                                        .expect("Read error");
                                
                                san_move = player_move.trim().chars().collect();
                            
                                let queen_column: i8 = match san_move[0] {
                                            'a' => 0,
                                            'b' => 1,
                                            'c' => 2,
                                            'd' => 3,
                                            'e' => 4,
                                            'f' => 5,
                                            'g' => 6,
                                            'h' => 7,
                                            _ => 100
                                        };
                                    
                                let queen_line: i8 = match san_move[1] {
                                            '1' => 56,
                                            '2' => 48,
                                            '3' => 40,
                                            '4' => 32,
                                            '5' => 24,
                                            '6' => 16,
                                            '7' => 8,
                                            '8' => 0,
                                            _ => 100
                                        };
                                
                                for b_queen in black_queens.iter_mut() {
                                    if queen_column + queen_line == *b_queen {
                                        if *b_queen > desired_position {
                                            // DIAGONAL MOVEMENT:
                                            if (*b_queen - desired_position)%7 == 0 {
                                                for diagonal in 1..8 {
                                                    if *b_queen - diagonal*7 == desired_position && !upper_right_diagonal(*b_queen, diagonal) {
                                                        board[*b_queen as usize] = NOTHING;
                                                        board[desired_position as usize] = BLACK_QUEEN;
                                                        *b_queen = desired_position;
        
                                                        try_again = false;
                                                        break;
                                                    }else if is_white(board[(*b_queen - diagonal*7) as usize]) 
                                                    || is_black(board[((*b_queen - diagonal*7) as usize)])
                                                    || upper_right_diagonal(*b_queen, diagonal) {
                                                        break;
                                                    }
                                                }
                                            }else if (*b_queen - desired_position)%9 == 0 {
                                                for diagonal in 1..8 {
                                                    if *b_queen - diagonal*9 == desired_position && !upper_left_diagonal(*b_queen, diagonal) {
                                                        board[*b_queen as usize] = NOTHING;
                                                        board[desired_position as usize] = BLACK_QUEEN;
                                                        *b_queen = desired_position;
        
                                                        try_again = false;
                                                        break;
                                                    }else if is_white(board[(*b_queen - diagonal*9) as usize]) 
                                                    || is_black(board[((*b_queen - diagonal*9) as usize)]) 
                                                    || upper_left_diagonal(*b_queen, diagonal) {
                                                        break;
                                                    }
                                                }
                                            }
                                            // ROOK-LIKE MOVEMENT:
                                            if get_line(*b_queen) == get_line(desired_position) {
                                                for square in 1..8 {
                                                    if *b_queen - square == desired_position && !rook_left(*b_queen, square) {
                                                        board[*b_queen as usize] = NOTHING;
                                                        board[desired_position as usize] = BLACK_QUEEN;
                                                        *b_queen = desired_position;
                
                                                        try_again = false;
                                                        break;
                                                    }else if is_white(board[(*b_queen - square) as usize]) 
                                                    || is_black(board[(*b_queen - square) as usize])
                                                    || rook_left(*b_queen, square) {
                                                        break;
                                                    }
                                                }
                                            }else if (*b_queen - desired_position)%8 == 0 {
                                                for square in 1..8 {
                                                    if *b_queen - square*8 == desired_position && !rook_up(*b_queen, square) {
                                                        board[*b_queen as usize] = NOTHING;
                                                        board[desired_position as usize] = BLACK_QUEEN;
                                                        *b_queen = desired_position;
                
                                                        try_again = false;
                                                        break;
                                                    }else if is_white(board[(*b_queen - square*8) as usize]) 
                                                    || is_black(board[(*b_queen - square*8) as usize])
                                                    || rook_up(*b_queen, square) {
                                                        break;
                                                    }
                                                }
                                            }
                                        }else if *b_queen < desired_position {
                                            // DIAGONAL MOVEMENT:
                                            if (*b_queen - desired_position)%7 == 0 {
                                                for diagonal in 1..8 {
                                                    if *b_queen + diagonal*7 == desired_position && !inferior_left_diagonal(*b_queen, diagonal) {
                                                        board[*b_queen as usize] = NOTHING;
                                                        board[desired_position as usize] = BLACK_QUEEN;
                                                        *b_queen = desired_position;
        
                                                        try_again = false;
                                                        break;
                                                    }else if is_white(board[(*b_queen + 7*diagonal) as usize]) 
                                                    || is_black(board[(*b_queen + 7*diagonal) as usize])
                                                    || inferior_left_diagonal(*b_queen, diagonal) {
                                                        break;
                                                    }
                                                }
                                            }else if (*b_queen - desired_position)%9 == 0 {
                                                for diagonal in 1..8 {
                                                    if *b_queen + diagonal*9 == desired_position && !inferior_right_diagonal(*b_queen, diagonal) {
                                                        board[*b_queen as usize] = NOTHING;
                                                        board[desired_position as usize] = BLACK_QUEEN;
                                                        *b_queen = desired_position;
        
                                                        try_again = false;
                                                        break;
                                                    }else if is_white(board[(*b_queen + 9*diagonal) as usize]) 
                                                    || is_black(board[(*b_queen + 9*diagonal) as usize])
                                                    || inferior_right_diagonal(*b_queen, diagonal) {
                                                        break;
                                                    }
                                                }
                                            }
                                            // ROOK-LIKE MOVEMENT:
                                            if get_line(*b_queen) == get_line(desired_position) {
                                                for square in 1..8 {
                                                    if *b_queen + square == desired_position && !rook_right(*b_queen, square) {
                                                        board[*b_queen as usize] = NOTHING;
                                                        board[desired_position as usize] = BLACK_QUEEN;
                                                        *b_queen = desired_position;
                
                                                        try_again = false;
                                                        break;
                                                    }else if is_white(board[(*b_queen + square) as usize]) 
                                                    || is_black(board[(*b_queen + square) as usize])
                                                    || rook_right(*b_queen, square) {
                                                        break;
                                                    }
                                                }
                                            }else if (desired_position - *b_queen)%8 == 0 {
                                                for square in 1..8 {
                                                    if *b_queen + square*8 == desired_position && !rook_down(*b_queen, square) {
                                                        board[*b_queen as usize] = NOTHING;
                                                        board[desired_position as usize] = BLACK_QUEEN;
                                                        *b_queen = desired_position;
                
                                                        try_again = false;
                                                        break;
                                                    }else if is_white(board[(*b_queen + square*8) as usize]) 
                                                    || is_black(board[(*b_queen + square*8) as usize])
                                                    || rook_down(*b_queen, square) {
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                };
                            }else{
                                for b_queen in black_queens.iter_mut() {
                                    if *b_queen > desired_position {
                                        // DIAGONAL MOVEMENT:
                                        if (*b_queen - desired_position)%7 == 0 {
                                            for diagonal in 1..8 {
                                                if *b_queen - diagonal*7 == desired_position && !upper_right_diagonal(*b_queen, diagonal) {
                                                    board[*b_queen as usize] = NOTHING;
                                                    board[desired_position as usize] = BLACK_QUEEN;
                                                    *b_queen = desired_position;
    
                                                    try_again = false;
                                                    break;
                                                }else if is_white(board[(*b_queen - diagonal*7) as usize]) || is_black(board[((*b_queen - diagonal*7) as usize)]) {
                                                    break;
                                                }
                                            }
                                        }else if (*b_queen - desired_position)%9 == 0 {
                                            for diagonal in 1..8 {
                                                if *b_queen - diagonal*9 == desired_position && !upper_left_diagonal(*b_queen, diagonal) {
                                                    board[*b_queen as usize] = NOTHING;
                                                    board[desired_position as usize] = BLACK_QUEEN;
                                                    *b_queen = desired_position;
    
                                                    try_again = false;
                                                    break;
                                                }else if is_white(board[(*b_queen - diagonal*9) as usize]) || is_black(board[((*b_queen - diagonal*9) as usize)]){
                                                    break;
                                                }
                                            }
                                        }
                                        // ROOK-LIKE MOVEMENT:
                                        if get_line(*b_queen) == get_line(desired_position) {
                                            for square in 1..8 {
                                                if *b_queen - square == desired_position && !rook_left(*b_queen, square) {
                                                    board[*b_queen as usize] = NOTHING;
                                                    board[desired_position as usize] = BLACK_QUEEN;
                                                    *b_queen = desired_position;
            
                                                    try_again = false;
                                                    break;
                                                }else if is_white(board[(*b_queen - square) as usize]) || is_black(board[(*b_queen - square) as usize]) {
                                                    break;
                                                }
                                            }
                                        }else if (*b_queen - desired_position)%8 == 0 {
                                            for square in 1..8 {
                                                if *b_queen - square*8 == desired_position && !rook_up(*b_queen, square) {
                                                    board[*b_queen as usize] = NOTHING;
                                                    board[desired_position as usize] = BLACK_QUEEN;
                                                    *b_queen = desired_position;
            
                                                    try_again = false;
                                                    break;
                                                }else if is_white(board[(*b_queen - square*8) as usize]) || is_black(board[(*b_queen - square*8) as usize]) {
                                                    break;
                                                }
                                            }
                                        }
                                    }else if *b_queen < desired_position {
                                        // DIAGONAL MOVEMENT:
                                        if (*b_queen - desired_position)%7 == 0 {
                                            for diagonal in 1..8 {
                                                if *b_queen + diagonal*7 == desired_position && !inferior_left_diagonal(*b_queen, diagonal) {
                                                    board[*b_queen as usize] = NOTHING;
                                                    board[desired_position as usize] = BLACK_QUEEN;
                                                    *b_queen = desired_position;
    
                                                    try_again = false;
                                                    break;
                                                }else if is_white(board[(*b_queen + 7*diagonal) as usize]) || is_black(board[(*b_queen + 7*diagonal) as usize]){
                                                    break;
                                                }
                                            }
                                        }else if (*b_queen - desired_position)%9 == 0 {
                                            for diagonal in 1..8 {
                                                if *b_queen + diagonal*9 == desired_position && !inferior_right_diagonal(*b_queen, diagonal) {
                                                    board[*b_queen as usize] = NOTHING;
                                                    board[desired_position as usize] = BLACK_QUEEN;
                                                    *b_queen = desired_position;
    
                                                    try_again = false;
                                                    break;
                                                }else if is_white(board[(*b_queen + 9*diagonal) as usize]) || is_black(board[(*b_queen + 9*diagonal) as usize]) {
                                                    break;
                                                }
                                            }
                                        }
                                        // ROOK-LIKE MOVEMENT:
                                        if get_line(*b_queen) == get_line(desired_position) {
                                            for square in 1..8 {
                                                if *b_queen + square == desired_position && !rook_right(*b_queen, square) {
                                                    board[*b_queen as usize] = NOTHING;
                                                    board[desired_position as usize] = BLACK_QUEEN;
                                                    *b_queen = desired_position;
            
                                                    try_again = false;
                                                    break;
                                                }else if is_white(board[(*b_queen + square) as usize]) || is_black(board[(*b_queen + square) as usize]) {
                                                    break;
                                                }
                                            }
                                        }else if (desired_position - *b_queen)%8 == 0 {
                                            for square in 1..8 {
                                                if *b_queen + square*8 == desired_position && !rook_down(*b_queen, square) {
                                                    board[*b_queen as usize] = NOTHING;
                                                    board[desired_position as usize] = BLACK_QUEEN;
                                                    *b_queen = desired_position;
            
                                                    try_again = false;
                                                    break;
                                                }else if is_white(board[(*b_queen + square*8) as usize]) || is_black(board[(*b_queen + square*8) as usize]) {
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        },
                        _ => ()
                    }

                    if try_again == false {
                        match captured_piece {
                            'i' => {
                                match san_move[1] {
                                    'a' => {
                                        for pawn in 0..white_column_a.len() {
                                            if white_column_a[pawn] == desired_position {
                                                white_column_a.swap_remove(pawn);
                                                break;
                                            }
                                        };
                                    },
                                    'b' => {
                                        for pawn in 0..white_column_b.len() {
                                            if white_column_b[pawn] == desired_position {
                                                white_column_b.swap_remove(pawn);
                                                break;
                                            }
                                        };
                                    },
                                    'c' => {
                                        for pawn in 0..white_column_c.len() {
                                            if white_column_c[pawn] == desired_position {
                                                white_column_c.swap_remove(pawn);
                                                break;
                                            }
                                        };
                                    },
                                    'd' => {
                                        for pawn in 0..white_column_d.len() {
                                            if white_column_d[pawn] == desired_position {
                                                white_column_d.swap_remove(pawn);
                                                break;
                                            }
                                        };
                                    },
                                    'e' => {
                                        for pawn in 0..white_column_e.len() {
                                            if white_column_e[pawn] == desired_position {
                                                white_column_e.swap_remove(pawn);
                                                break;
                                            }
                                        };
                                    },
                                    'f' => {
                                        for pawn in 0..white_column_f.len() {
                                            if white_column_f[pawn] == desired_position {
                                                white_column_f.swap_remove(pawn);
                                                break;
                                            }
                                        };
                                    },
                                    'g' => {
                                        for pawn in 0..white_column_g.len() {
                                            if white_column_g[pawn] == desired_position {
                                                white_column_g.swap_remove(pawn);
                                                break;
                                            }
                                        };
                                    },
                                    'h' => {
                                        for pawn in 0..white_column_h.len() {
                                            if white_column_h[pawn] == desired_position {
                                                white_column_h.swap_remove(pawn);
                                                break;
                                            }
                                        };
                                    },
                                    _ => ()
                                }
                            },
                            'Q' => {
                                for i in 0..white_queens.len() {
                                    if white_queens[i] == desired_position {
                                        white_queens.swap_remove(i);
                                        break;
                                    }
                                };
                            },
                            'B' => {
                                for i in 0..white_bishops.len() {
                                    if white_bishops[i] == desired_position {
                                        white_bishops.swap_remove(i);
                                        break;
                                    }
                                };
                            },
                            'N' => {
                                for i in 0..white_knights.len() {
                                    if white_knights[i] == desired_position {
                                        white_knights.swap_remove(i);
                                        break;
                                    }
                                };
                            },
                            'R' => {
                                for i in 0..white_rooks.len() {
                                    if white_rooks[i] == desired_position {
                                        white_rooks.swap_remove(i);
                                        break;
                                    }
                                };
                            },
                            _ => ()
                        }
                    }
                }
            }else if player_move.trim() == "O-O" || player_move.trim() == "0-0" { // kingside castling
                if has_black_rook2_moved == false {
                    if board[6] == NOTHING && board[5] == NOTHING { // squares between the rook and king must be empty
                        if get_pieces_checking_the_black_king(6, &board).len() == 0
                        && get_pieces_checking_the_black_king(5, &board).len() == 0 {
                            board[black_king as usize] = NOTHING;
                            board[7] = NOTHING; // kingside rook

                            black_king = 6;
                            board[black_king as usize] = BLACK_KING;
                            for b_rook in black_rooks.iter_mut() {
                                if *b_rook == 7 {
                                    *b_rook = 5;
                                    board[*b_rook as usize] = BLACK_ROOK;
                                    break;
                                }
                            }

                            try_again = false;
                        }
                    }
                }
            }else if player_move.trim() == "O-O-O" || player_move.trim() == "0-0-0" { // queenside castling
                if has_black_rook1_moved == false {
                    if board[3] == NOTHING && board[2] == NOTHING && board[1] == NOTHING { // squares between the rook and king must be empty
                        if get_pieces_checking_the_black_king(3, &board).len() == 0
                        && get_pieces_checking_the_black_king(2, &board).len() == 0
                        && get_pieces_checking_the_black_king(1, &board).len() == 0 {
                            board[black_king as usize] = NOTHING;
                            board[0] = NOTHING; // kingside rook

                            black_king = 2;
                            board[black_king as usize] = BLACK_KING;
                            for b_rook in black_rooks.iter_mut() {
                                if *b_rook == 0 {
                                    *b_rook = 3;
                                    board[*b_rook as usize] = BLACK_ROOK;
                                    break;
                                }
                            }

                            try_again = false;
                        }
                    }
                }
            }else{ // pawn movement
                if san_move[1] != 'x' {
                    column = match san_move[0] {
                        'a' => 0,
                        'b' => 1,
                        'c' => 2,
                        'd' => 3,
                        'e' => 4,
                        'f' => 5,
                        'g' => 6,
                        'h' => 7,
                        _ => 100
                    };
                
                    //must be in reverse because we view the board as white
                    line = match san_move[1] {
                        '1' => 56,
                        '2' => 48,
                        '3' => 40,
                        '4' => 32,
                        '5' => 24,
                        '6' => 16,
                        '7' => 8,
                        '8' => 0,
                        _ => 100
                    };

                    if column >= 100 || line >= 100 {
                        println!("Not a possible move, try again!\n");
                        continue;
                    }

                    desired_position = column + line;

                    if bking_checks.len() == 1 { // king is being checked by a single piece
                        if (bking_checks[0] > black_king && desired_position < black_king) // if the checking piece has a higher index than the king, the blocking pawn must also be higher
                        || (bking_checks[0] < black_king && desired_position > black_king) // if the checking piece has a lower index than the king, the blocking pawn must also be lower
                        || ((bking_checks[0]-black_king)%7 == 0 && (desired_position-black_king)%7 != 0) // the pawn is not blocking the diagonal
                        || ((bking_checks[0]-black_king)%9 == 0 && (desired_position-black_king)%9 != 0) // the pawn is not blocking the diagonal
                        || ((bking_checks[0]-black_king)%8 == 0 && (desired_position-black_king)%8 != 0) { // the pawn is not blocking the file

                            // the piece to be moved does not block the check
                            println!("That move did not block the check completely!\n");
                            continue;
                        }
                    }else if bking_checks.len() > 1 {
                        println!("Your king is being double checked. You have to move it!\n");
                        continue;
                    }

                    if board[desired_position as usize] == NOTHING {
                        match san_move[0] {
                            'a' => {
                                // for every pawn in the column
                                for pawn in &mut black_column_a.iter_mut() {
                                    // if the pawn is in it's starting position
                                    if *pawn <= 15 {
                                        if desired_position - *pawn == 16 
                                        && board[(*pawn+8) as usize] == NOTHING {
                                            board[*pawn as usize] = NOTHING;
                                            board[desired_position as usize] = BLACK_PAWN;
                                            *pawn = desired_position;
                                            
                                            try_again = false;
                                            black_column_a_enpassant = true;

                                            break;
                                        }else if desired_position - *pawn == 8 {
                                            board[*pawn as usize] = NOTHING;
                                            board[desired_position as usize] = BLACK_PAWN;
                                            *pawn = desired_position;
                                            
                                            try_again = false;
                                            break;
                                        }
                                    }else{
                                        if desired_position - *pawn == 8 {
                                            board[*pawn as usize] = NOTHING;
                                            board[desired_position as usize] = BLACK_PAWN;
                                            *pawn = desired_position;
                                            
                                            try_again = false;
                                            break;
                                        }
                                    }
                                };
                            },
                            'b' => {
                                // for every pawn in the column
                                for pawn in &mut black_column_b.iter_mut() {
                                    // if the pawn is in it's starting position
                                    if *pawn <= 15 {
                                        if desired_position - *pawn == 16 
                                        && board[(*pawn+8) as usize] == NOTHING {
                                            board[*pawn as usize] = NOTHING;
                                            board[desired_position as usize] = BLACK_PAWN;
                                            *pawn = desired_position;
                                            
                                            try_again = false;
                                            black_column_b_enpassant = true;

                                            break;
                                        }else if desired_position - *pawn == 8 {
                                            board[*pawn as usize] = NOTHING;
                                            board[desired_position as usize] = BLACK_PAWN;
                                            *pawn = desired_position;
                                            
                                            try_again = false;
                                            break;
                                        }
                                    }else{
                                        if desired_position - *pawn == 8 {
                                            board[*pawn as usize] = NOTHING;
                                            board[desired_position as usize] = BLACK_PAWN;
                                            *pawn = desired_position;
                                            
                                            try_again = false;
                                            break;
                                        }
                                    }
                                };
                            },
                            'c' => {
                                // for every pawn in the column
                                for pawn in &mut black_column_c.iter_mut() {
                                    // if the pawn is in it's starting position
                                    if *pawn <= 15 {
                                        if desired_position - *pawn == 16 
                                        && board[(*pawn+8) as usize] == NOTHING {
                                            board[*pawn as usize] = NOTHING;
                                            board[desired_position as usize] = BLACK_PAWN;
                                            *pawn = desired_position;
                                            
                                            try_again = false;
                                            black_column_c_enpassant = true;

                                            break;
                                        }else if desired_position - *pawn == 8 {
                                            board[*pawn as usize] = NOTHING;
                                            board[desired_position as usize] = BLACK_PAWN;
                                            *pawn = desired_position;
                                            
                                            try_again = false;
                                            break;
                                        }
                                    }else{
                                        if desired_position - *pawn == 8 {
                                            board[*pawn as usize] = NOTHING;
                                            board[desired_position as usize] = BLACK_PAWN;
                                            *pawn = desired_position;
                                            
                                            try_again = false;
                                            break;
                                        }
                                    }
                                };
                            },
                            'd' => {
                                // for every pawn in the column
                                for pawn in &mut black_column_d.iter_mut() {
                                    // if the pawn is in it's starting position
                                    if *pawn <= 15 {
                                        if desired_position - *pawn == 16 
                                        && board[(*pawn+8) as usize] == NOTHING {
                                            board[*pawn as usize] = NOTHING;
                                            board[desired_position as usize] = BLACK_PAWN;
                                            *pawn = desired_position;
                                            
                                            try_again = false;
                                            black_column_d_enpassant = true;

                                            break;
                                        }else if desired_position - *pawn == 8 {
                                            board[*pawn as usize] = NOTHING;
                                            board[desired_position as usize] = BLACK_PAWN;
                                            *pawn = desired_position;
                                            
                                            try_again = false;
                                            break;
                                        }
                                    }else{
                                        if desired_position - *pawn == 8 {
                                            board[*pawn as usize] = NOTHING;
                                            board[desired_position as usize] = BLACK_PAWN;
                                            *pawn = desired_position;
                                            
                                            try_again = false;
                                            break;
                                        }
                                    }
                                };
                            },
                            'e' => {
                                // for every pawn in the column
                                for pawn in &mut black_column_e.iter_mut() {
                                    // if the pawn is in it's starting position
                                    if *pawn <= 15 {
                                        if desired_position - *pawn == 16 
                                        && board[(*pawn+8) as usize] == NOTHING {
                                            board[*pawn as usize] = NOTHING;
                                            board[desired_position as usize] = BLACK_PAWN;
                                            *pawn = desired_position;
                                            
                                            try_again = false;
                                            black_column_e_enpassant = true;

                                            break;
                                        }else if desired_position - *pawn == 8 {
                                            board[*pawn as usize] = NOTHING;
                                            board[desired_position as usize] = BLACK_PAWN;
                                            *pawn = desired_position;
                                            
                                            try_again = false;
                                            break;
                                        }
                                    }else{
                                        if desired_position - *pawn == 8 {
                                            board[*pawn as usize] = NOTHING;
                                            board[desired_position as usize] = BLACK_PAWN;
                                            *pawn = desired_position;
                                            
                                            try_again = false;
                                            break;
                                        }
                                    }
                                };
                            },
                            'f' => {
                                // for every pawn in the column
                                for pawn in &mut black_column_f.iter_mut() {
                                    // if the pawn is in it's starting position
                                    if *pawn <= 15 {
                                        if desired_position - *pawn == 16 
                                        && board[(*pawn+8) as usize] == NOTHING {
                                            board[*pawn as usize] = NOTHING;
                                            board[desired_position as usize] = BLACK_PAWN;
                                            *pawn = desired_position;
                                            
                                            try_again = false;
                                            black_column_f_enpassant = true;

                                            break;
                                        }else if desired_position - *pawn == 8 {
                                            board[*pawn as usize] = NOTHING;
                                            board[desired_position as usize] = BLACK_PAWN;
                                            *pawn = desired_position;
                                            
                                            try_again = false;
                                            break;
                                        }
                                    }else{
                                        if desired_position - *pawn == 8 {
                                            board[*pawn as usize] = NOTHING;
                                            board[desired_position as usize] = BLACK_PAWN;
                                            *pawn = desired_position;
                                            
                                            try_again = false;
                                            break;
                                        }
                                    }
                                };
                            },
                            'g' => {
                                // for every pawn in the column
                                for pawn in &mut black_column_g.iter_mut() {
                                    // if the pawn is in it's starting position
                                    if *pawn <= 15 {
                                        if desired_position - *pawn == 16 
                                        && board[(*pawn+8) as usize] == NOTHING {
                                            board[*pawn as usize] = NOTHING;
                                            board[desired_position as usize] = BLACK_PAWN;
                                            *pawn = desired_position;
                                            
                                            try_again = false;
                                            black_column_g_enpassant = true;

                                            break;
                                        }else if desired_position - *pawn == 8 {
                                            board[*pawn as usize] = NOTHING;
                                            board[desired_position as usize] = BLACK_PAWN;
                                            *pawn = desired_position;
                                            
                                            try_again = false;
                                            break;
                                        }
                                    }else{
                                        if desired_position - *pawn == 8 {
                                            board[*pawn as usize] = NOTHING;
                                            board[desired_position as usize] = BLACK_PAWN;
                                            *pawn = desired_position;
                                            
                                            try_again = false;
                                            break;
                                        }
                                    }
                                };
                            },
                            'h' => {
                                // for every pawn in the column
                                for pawn in &mut black_column_h.iter_mut() {
                                    // if the pawn is in it's starting position
                                    if *pawn <= 15 {
                                        if desired_position - *pawn == 16 
                                        && board[(*pawn+8) as usize] == NOTHING {
                                            board[*pawn as usize] = NOTHING;
                                            board[desired_position as usize] = BLACK_PAWN;
                                            *pawn = desired_position;
                                            
                                            try_again = false;
                                            black_column_h_enpassant = true;

                                            break;
                                        }else if desired_position - *pawn == 8 {
                                            board[*pawn as usize] = NOTHING;
                                            board[desired_position as usize] = BLACK_PAWN;
                                            *pawn = desired_position;
                                            
                                            try_again = false;
                                            break;
                                        }
                                    }else{
                                        if desired_position - *pawn == 8 {
                                            board[*pawn as usize] = NOTHING;
                                            board[desired_position as usize] = BLACK_PAWN;
                                            *pawn = desired_position;
                                            
                                            try_again = false;
                                            break;
                                        }
                                    }
                                };
                            },
                            _ => ()
                        };

                        if line == 56 && try_again == false{ // if the pawn moved to the other side of the board successfully
                            println!("Indicate the pawn promotion: (Press Enter to promote it to a Queen)");
                            println!("'N'=Knight,'R'=Rook,'B'=Bishop");
                            let mut promotion_move = String::new();
                            let mut prom_loop: bool = true;

                            while prom_loop {
                                promotion_move.clear();
                                io::stdin()
                                    .read_line(&mut promotion_move)
                                    .expect("Read error");
                                
                                let prom_vec: Vec<char> = promotion_move.trim().chars().collect();

                                if prom_vec.len() > 0 {
                                    match san_move[0] {
                                        'a' => {
                                            match prom_vec[0] {
                                                'N' => { // promotion to knight
                                                    black_column_a.remove(0);
                                                    board[desired_position as usize] = BLACK_KNIGHT;
                                                    black_knights.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                'R' => { // promotion to rook
                                                    black_column_a.remove(0);
                                                    board[desired_position as usize] = BLACK_ROOK;
                                                    black_rooks.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                'B' => { // promotion to bishop
                                                    black_column_a.remove(0);
                                                    board[desired_position as usize] = BLACK_BISHOP;
                                                    black_bishops.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                _ => ()
                                            }
                                        },
                                        'b' => {
                                            match prom_vec[0] {
                                                'N' => { // promotion to knight
                                                    black_column_b.remove(0);
                                                    board[desired_position as usize] = BLACK_KNIGHT;
                                                    black_knights.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                'R' => { // promotion to rook
                                                    black_column_b.remove(0);
                                                    board[desired_position as usize] = BLACK_ROOK;
                                                    black_rooks.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                'B' => { // promotion to bishop
                                                    black_column_b.remove(0);
                                                    board[desired_position as usize] = BLACK_BISHOP;
                                                    black_bishops.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                _ => ()
                                            }
                                        },
                                        'c' => {
                                            match prom_vec[0] {
                                                'N' => { // promotion to knight
                                                    black_column_c.remove(0);
                                                    board[desired_position as usize] = BLACK_KNIGHT;
                                                    black_knights.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                'R' => { // promotion to rook
                                                    black_column_c.remove(0);
                                                    board[desired_position as usize] = BLACK_ROOK;
                                                    black_rooks.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                'B' => { // promotion to bishop
                                                    black_column_c.remove(0);
                                                    board[desired_position as usize] = BLACK_BISHOP;
                                                    black_bishops.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                _ => ()
                                            }
                                        },
                                        'd' => {
                                            match prom_vec[0] {
                                                'N' => { // promotion to knight
                                                    black_column_d.remove(0);
                                                    board[desired_position as usize] = BLACK_KNIGHT;
                                                    black_knights.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                'R' => { // promotion to rook
                                                    black_column_d.remove(0);
                                                    board[desired_position as usize] = BLACK_ROOK;
                                                    black_rooks.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                'B' => { // promotion to bishop
                                                    black_column_d.remove(0);
                                                    board[desired_position as usize] = BLACK_BISHOP;
                                                    black_bishops.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                _ => ()
                                            }
                                        },
                                        'e' => {
                                            match prom_vec[0] {
                                                'N' => { // promotion to knight
                                                    black_column_e.remove(0);
                                                    board[desired_position as usize] = BLACK_KNIGHT;
                                                    black_knights.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                'R' => { // promotion to rook
                                                    black_column_e.remove(0);
                                                    board[desired_position as usize] = BLACK_ROOK;
                                                    black_rooks.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                'B' => { // promotion to bishop
                                                    black_column_e.remove(0);
                                                    board[desired_position as usize] = BLACK_BISHOP;
                                                    black_bishops.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                _ => ()
                                            }
                                        },
                                        'f' => {
                                            match prom_vec[0] {
                                                'N' => { // promotion to knight
                                                    black_column_f.remove(0);
                                                    board[desired_position as usize] = BLACK_KNIGHT;
                                                    black_knights.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                'R' => { // promotion to rook
                                                    black_column_f.remove(0);
                                                    board[desired_position as usize] = BLACK_ROOK;
                                                    black_rooks.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                'B' => { // promotion to bishop
                                                    black_column_f.remove(0);
                                                    board[desired_position as usize] = BLACK_BISHOP;
                                                    black_bishops.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                _ => ()
                                            }
                                        },
                                        'g' => {
                                            match prom_vec[0] {
                                                'N' => { // promotion to knight
                                                    black_column_g.remove(0);
                                                    board[desired_position as usize] = BLACK_KNIGHT;
                                                    black_knights.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                'R' => { // promotion to rook
                                                    black_column_g.remove(0);
                                                    board[desired_position as usize] = BLACK_ROOK;
                                                    black_rooks.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                'B' => { // promotion to bishop
                                                    black_column_g.remove(0);
                                                    board[desired_position as usize] = BLACK_BISHOP;
                                                    black_bishops.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                _ => ()
                                            }
                                        },
                                        _ => ()
                                    };
                                }else{ // queen promotion (Enter key doesnt add a character to the vector, so it's lenght stays 0)
                                    match san_move[0] {
                                        'a' => {
                                            black_column_a.remove(0);
                                            board[desired_position as usize] = BLACK_QUEEN;
                                            black_queens.insert(0, desired_position);
                                            prom_loop = false;
                                        },
                                        'b' => {
                                            black_column_b.remove(0);
                                            board[desired_position as usize] = BLACK_QUEEN;
                                            black_queens.insert(0, desired_position);
                                            prom_loop = false;
                                        },
                                        'c' => {
                                            black_column_c.remove(0);
                                            board[desired_position as usize] = BLACK_QUEEN;
                                            black_queens.insert(0, desired_position);
                                            prom_loop = false;
                                        },
                                        'd' => {
                                            black_column_d.remove(0);
                                            board[desired_position as usize] = BLACK_QUEEN;
                                            black_queens.insert(0, desired_position);
                                            prom_loop = false;
                                        },
                                        'e' => {
                                            black_column_e.remove(0);
                                            board[desired_position as usize] = BLACK_QUEEN;
                                            black_queens.insert(0, desired_position);
                                            prom_loop = false;
                                        },
                                        'f' => {
                                            black_column_f.remove(0);
                                            board[desired_position as usize] = BLACK_QUEEN;
                                            black_queens.insert(0, desired_position);
                                            prom_loop = false;
                                        },
                                        'g' => {
                                            black_column_g.remove(0);
                                            board[desired_position as usize] = BLACK_QUEEN;
                                            black_queens.insert(0, desired_position);
                                            prom_loop = false;
                                        },
                                        _ => ()
                                    };
                                }
                                if prom_loop == true {
                                    println!("Not a possible promotion, try again!\n");
                                }
                            }
                        }
                    }

                }else if san_move.len() >= 4 {
                    column = match san_move[2] {
                        'a' => 0,
                        'b' => 1,
                        'c' => 2,
                        'd' => 3,
                        'e' => 4,
                        'f' => 5,
                        'g' => 6,
                        'h' => 7,
                        _ => 100
                    };

                    line = match san_move[3] {
                        '1' => 56,
                        '2' => 48,
                        '3' => 40,
                        '4' => 32,
                        '5' => 24,
                        '6' => 16,
                        '7' => 8,
                        '8' => 0,
                        _ => 100
                    };

                    if column >= 100 || line >= 100 {
                        println!("Not a possible move, try again!\n");
                        continue;
                    }

                    desired_position = column + line;

                    if bking_checks.len() == 1 { // king is being checked by a single piece
                        if (bking_checks[0] > black_king && desired_position < black_king) // if the checking piece has a higher index than the king, the blocking pawn must also be higher
                        || (bking_checks[0] < black_king && desired_position > black_king) // if the checking piece has a lower index than the king, the blocking pawn must also be lower
                        || ((bking_checks[0]-black_king)%7 == 0 && (desired_position-black_king)%7 != 0) // the pawn is not blocking the diagonal
                        || ((bking_checks[0]-black_king)%9 == 0 && (desired_position-black_king)%9 != 0) // the pawn is not blocking the diagonal
                        || ((bking_checks[0]-black_king)%8 == 0 && (desired_position-black_king)%8 != 0) { // the pawn is not blocking the file

                            // the piece to be moved does not block the check
                            println!("That move did not block the check completely!\n");
                            continue;
                        }
                    }else if bking_checks.len() > 1 {
                        println!("Your king is being double checked. You have to move it!\n");
                        continue;
                    }

                    if is_white(board[desired_position as usize]) 
                    || (get_line(desired_position) == 3 && board[(desired_position-8) as usize] == WHITE_PAWN && match san_move[2] { // checks if the desired square's column can be a victim of en passant
                    'a' => {
                        if white_column_a_enpassant == true {
                            true
                        }else{
                            false
                        }
                    },
                    'b' => {
                        if white_column_b_enpassant == true {
                            true
                        }else{
                            false
                        }
                    },
                    'c' => {
                        if white_column_c_enpassant == true {
                            true
                        }else{
                            false
                        }
                    },
                    'd' => {
                        if white_column_d_enpassant == true {
                            true
                        }else{
                            false
                        }
                    },
                    'e' => {
                        if white_column_e_enpassant == true {
                            true
                        }else{
                            false
                        }
                    },
                    'f' => {
                        if white_column_f_enpassant == true {
                            true
                        }else{
                            false
                        }
                    },
                    'g' => {
                        if white_column_g_enpassant == true {
                            true
                        }else{
                            false
                        }
                    },
                    'h' => {
                        if white_column_h_enpassant == true {
                            true
                        }else{
                            false
                        }
                    },
                    _ => false
                    }) {
                        let captured_piece: char;
                        if board[desired_position as usize] == NOTHING {
                            captured_piece = board[(desired_position-8) as usize];
                        }else{
                            captured_piece = board[desired_position as usize];
                        }

                        match san_move[0] {
                            'a' => {
                                let mut pawn_index: usize = 0;
                                for pawn in &mut black_column_a.iter_mut() {
                                    if desired_position - *pawn == 9 && !inferior_right_diagonal(*pawn, 1) {
                                        board[*pawn as usize] = NOTHING;
                                        if board[desired_position as usize] == NOTHING {
                                            board[(desired_position-8) as usize] = NOTHING;
                                        }
                                        board[desired_position as usize] = BLACK_PAWN;
                                        *pawn = desired_position;

                                        black_column_b.insert(0, *pawn);
                                        
                                        black_column_a.remove(pawn_index);

                                        try_again = false;
                                        break;
                                    }
                                    pawn_index += 1;
                                };
                            },
                            'b' => {
                                let mut pawn_index: usize = 0;
                                for pawn in &mut black_column_b.iter_mut() {
                                    if (desired_position - *pawn == 9 && !inferior_right_diagonal(*pawn, 1)) 
                                    || (desired_position - *pawn == 7 && !inferior_left_diagonal(*pawn, 1)) {
                                        board[*pawn as usize] = NOTHING;
                                        if board[desired_position as usize] == NOTHING {
                                            board[(desired_position-8) as usize] = NOTHING;
                                        }
                                        board[desired_position as usize] = BLACK_PAWN;
                                        *pawn = desired_position;

                                        match san_move[2]{
                                            'a' => black_column_a.insert(0, *pawn),
                                            'c' => black_column_c.insert(0, *pawn),
                                            _ => ()
                                        }
                                        
                                        black_column_b.remove(pawn_index);

                                        try_again = false;
                                        break;
                                    }
                                    pawn_index += 1;
                                };
                            },
                            'c' => {
                                let mut pawn_index: usize = 0;
                                for pawn in &mut black_column_c.iter_mut() {
                                    if (desired_position - *pawn == 9 && !inferior_right_diagonal(*pawn, 1)) 
                                    || (desired_position - *pawn == 7 && !inferior_left_diagonal(*pawn, 1)) {
                                        board[*pawn as usize] = NOTHING;
                                        if board[desired_position as usize] == NOTHING {
                                            board[(desired_position-8) as usize] = NOTHING;
                                        }
                                        board[desired_position as usize] = BLACK_PAWN;
                                        *pawn = desired_position;
                                        
                                        match san_move[2]{
                                            'b' => black_column_b.insert(0, *pawn),
                                            'd' => black_column_d.insert(0, *pawn),
                                            _ => ()
                                        }

                                        black_column_c.remove(pawn_index);

                                        try_again = false;
                                        break;
                                    }
                                    pawn_index += 1;
                                };
                            },
                            'd' => {
                                let mut pawn_index: usize = 0;
                                for pawn in &mut black_column_d.iter_mut() {
                                    if (desired_position - *pawn == 9 && !inferior_right_diagonal(*pawn, 1)) 
                                    || (desired_position - *pawn == 7 && !inferior_left_diagonal(*pawn, 1)) {
                                        board[*pawn as usize] = NOTHING;
                                        if board[desired_position as usize] == NOTHING {
                                            board[(desired_position-8) as usize] = NOTHING;
                                        }
                                        board[desired_position as usize] = BLACK_PAWN;
                                        *pawn = desired_position;
                                        
                                        match san_move[2]{
                                            'c' => black_column_c.insert(0, *pawn),
                                            'e' => black_column_e.insert(0, *pawn),
                                            _ => ()
                                        }

                                        black_column_d.remove(pawn_index);

                                        try_again = false;
                                        break;
                                    }
                                    pawn_index += 1;
                                };
                            },
                            'e' => {
                                let mut pawn_index: usize = 0;
                                for pawn in &mut black_column_e.iter_mut() {
                                    if (desired_position - *pawn == 9 && !inferior_right_diagonal(*pawn, 1)) 
                                    || (desired_position - *pawn == 7 && !inferior_left_diagonal(*pawn, 1)) {
                                        board[*pawn as usize] = NOTHING;
                                        if board[desired_position as usize] == NOTHING {
                                            board[(desired_position-8) as usize] = NOTHING;
                                        }
                                        board[desired_position as usize] = BLACK_PAWN;
                                        *pawn = desired_position;
                                        
                                        match san_move[2]{
                                            'd' => black_column_d.insert(0, *pawn),
                                            'f' => black_column_f.insert(0, *pawn),
                                            _ => ()
                                        }

                                        black_column_e.remove(pawn_index);

                                        try_again = false;
                                        break;
                                    }
                                    pawn_index += 1;
                                };
                            },
                            'f' => {
                                let mut pawn_index: usize = 0;
                                for pawn in &mut black_column_f.iter_mut() {
                                    if (desired_position - *pawn == 9 && !inferior_right_diagonal(*pawn, 1)) 
                                    || (desired_position - *pawn == 7 && !inferior_left_diagonal(*pawn, 1)) {
                                        board[*pawn as usize] = NOTHING;
                                        if board[desired_position as usize] == NOTHING {
                                            board[(desired_position-8) as usize] = NOTHING;
                                        }
                                        board[desired_position as usize] = BLACK_PAWN;
                                        *pawn = desired_position;
                                        
                                        match san_move[2]{
                                            'e' => black_column_e.insert(0, *pawn),
                                            'g' => black_column_g.insert(0, *pawn),
                                            _ => ()
                                        }

                                        black_column_f.remove(pawn_index);

                                        try_again = false;
                                        break;
                                    }
                                    pawn_index += 1;
                                };
                            },
                            'g' => {
                                let mut pawn_index: usize = 0;
                                for pawn in &mut black_column_g.iter_mut() {
                                    if (desired_position - *pawn == 9 && !inferior_right_diagonal(*pawn, 1)) 
                                    || (desired_position - *pawn == 7 && !inferior_left_diagonal(*pawn, 1)) {
                                        board[*pawn as usize] = NOTHING;
                                        if board[desired_position as usize] == NOTHING {
                                            board[(desired_position-8) as usize] = NOTHING;
                                        }
                                        board[desired_position as usize] = BLACK_PAWN;
                                        *pawn = desired_position;
                                        
                                        match san_move[2]{
                                            'f' => black_column_f.insert(0, *pawn),
                                            'h' => black_column_h.insert(0, *pawn),
                                            _ => ()
                                        }

                                        black_column_g.remove(pawn_index);

                                        try_again = false;
                                        break;
                                    }
                                    pawn_index += 1;
                                };
                            },
                            'h' => {
                                let mut pawn_index: usize = 0;
                                for pawn in &mut black_column_h.iter_mut() {
                                    if desired_position - *pawn == 7 && !inferior_left_diagonal(*pawn, 1) {
                                        board[*pawn as usize] = NOTHING;
                                        if board[desired_position as usize] == NOTHING {
                                            board[(desired_position-8) as usize] = NOTHING;
                                        }
                                        board[desired_position as usize] = BLACK_PAWN;
                                        *pawn = desired_position;
                                        
                                        black_column_g.insert(0, *pawn);

                                        black_column_h.remove(pawn_index);

                                        try_again = false;
                                        break;
                                    }
                                    pawn_index += 1;
                                };
                            },
                            _ => ()
                        }
                    
                        if try_again == false {
                            match captured_piece {
                                'i' => {
                                    match san_move[2] {
                                        'a' => {
                                            if white_column_a_enpassant == false {
                                                for pawn in 0..white_column_a.len() {
                                                    if white_column_a[pawn] == desired_position {
                                                        white_column_a.swap_remove(pawn);
                                                        break;
                                                    }
                                                };
                                            }else{
                                                for pawn in 0..white_column_a.len() {
                                                    if white_column_a[pawn] == desired_position-8 {
                                                        white_column_a.swap_remove(pawn);
                                                        break;
                                                    }
                                                };
                                            }
                                        },
                                        'b' => {
                                            if white_column_b_enpassant == false {
                                                for pawn in 0..white_column_b.len() {
                                                    if white_column_b[pawn] == desired_position {
                                                        white_column_b.swap_remove(pawn);
                                                        break;
                                                    }
                                                };
                                            }else{
                                                for pawn in 0..white_column_b.len() {
                                                    if white_column_b[pawn] == desired_position-8 {
                                                        white_column_b.swap_remove(pawn);
                                                        break;
                                                    }
                                                };
                                            }
                                        },
                                        'c' => {
                                            if white_column_c_enpassant == false {
                                                for pawn in 0..white_column_c.len() {
                                                    if white_column_c[pawn] == desired_position {
                                                        white_column_c.swap_remove(pawn);
                                                        break;
                                                    }
                                                };
                                            }else{
                                                for pawn in 0..white_column_c.len() {
                                                    if white_column_c[pawn] == desired_position-8 {
                                                        white_column_c.swap_remove(pawn);
                                                        break;
                                                    }
                                                };
                                            }
                                        },
                                        'd' => {
                                            if white_column_d_enpassant == false {
                                                for pawn in 0..white_column_d.len() {
                                                    if white_column_d[pawn] == desired_position {
                                                        white_column_d.swap_remove(pawn);
                                                        break;
                                                    }
                                                };
                                            }else{
                                                for pawn in 0..white_column_d.len() {
                                                    if white_column_d[pawn] == desired_position-8 {
                                                        white_column_d.swap_remove(pawn);
                                                        break;
                                                    }
                                                };
                                            }
                                        },
                                        'e' => {
                                            if white_column_e_enpassant == false {
                                                for pawn in 0..white_column_e.len() {
                                                    if white_column_e[pawn] == desired_position {
                                                        white_column_e.swap_remove(pawn);
                                                        break;
                                                    }
                                                };
                                            }else{
                                                for pawn in 0..white_column_e.len() {
                                                    if white_column_e[pawn] == desired_position-8 {
                                                        white_column_e.swap_remove(pawn);
                                                        break;
                                                    }
                                                };
                                            }
                                        },
                                        'f' => {
                                            if white_column_f_enpassant == false {
                                                for pawn in 0..white_column_f.len() {
                                                    if white_column_f[pawn] == desired_position {
                                                        white_column_f.swap_remove(pawn);
                                                        break;
                                                    }
                                                };
                                            }else{
                                                for pawn in 0..white_column_f.len() {
                                                    if white_column_f[pawn] == desired_position-8 {
                                                        white_column_f.swap_remove(pawn);
                                                        break;
                                                    }
                                                };
                                            }
                                        },
                                        'g' => {
                                            if white_column_g_enpassant == false {
                                                for pawn in 0..white_column_g.len() {
                                                    if white_column_g[pawn] == desired_position {
                                                        white_column_g.swap_remove(pawn);
                                                        break;
                                                    }
                                                };
                                            }else{
                                                for pawn in 0..white_column_g.len() {
                                                    if white_column_g[pawn] == desired_position-8 {
                                                        white_column_g.swap_remove(pawn);
                                                        break;
                                                    }
                                                };
                                            }
                                        },
                                        'h' => {
                                            if white_column_h_enpassant == false {
                                                for pawn in 0..white_column_h.len() {
                                                    if white_column_h[pawn] == desired_position {
                                                        white_column_h.swap_remove(pawn);
                                                        break;
                                                    }
                                                };
                                            }else{
                                                for pawn in 0..white_column_h.len() {
                                                    if white_column_h[pawn] == desired_position-8 {
                                                        white_column_h.swap_remove(pawn);
                                                        break;
                                                    }
                                                };
                                            }
                                        },
                                        _ => ()
                                    }
                                },
                                'Q' => {
                                    for i in 0..white_queens.len() {
                                        if white_queens[i] == desired_position {
                                            white_queens.swap_remove(i);
                                            break;
                                        }
                                    };
                                },
                                'B' => {
                                    for i in 0..white_bishops.len() {
                                        if white_bishops[i] == desired_position {
                                            white_bishops.swap_remove(i);
                                            break;
                                        }
                                    };
                                },
                                'N' => {
                                    for i in 0..white_knights.len() {
                                        if white_knights[i] == desired_position {
                                            white_knights.swap_remove(i);
                                            break;
                                        }
                                    };
                                },
                                'R' => {
                                    for i in 0..white_rooks.len() {
                                        if white_rooks[i] == desired_position {
                                            white_rooks.swap_remove(i);
                                            break;
                                        }
                                    };
                                },
                                _ => ()
                            }
                        }
                    
                        if line == 56 && try_again == false{ // if the pawn moved to the other side of the board successfully
                            println!("Indicate the pawn promotion: (Press Enter to promote it to a Queen)");
                            println!("'N'=Knight,'R'=Rook,'B'=Bishop");
                            let mut promotion_move = String::new();
                            let mut prom_loop: bool = true;

                            while prom_loop {
                                promotion_move.clear();
                                io::stdin()
                                    .read_line(&mut promotion_move)
                                    .expect("Read error");
                                
                                let prom_vec: Vec<char> = promotion_move.trim().chars().collect();

                                if prom_vec.len() > 0 {
                                    match san_move[2] {
                                        'a' => {
                                            match prom_vec[0] {
                                                'N' => { // promotion to knight
                                                    black_column_a.remove(0);
                                                    board[desired_position as usize] = BLACK_KNIGHT;
                                                    black_knights.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                'R' => { // promotion to rook
                                                    black_column_a.remove(0);
                                                    board[desired_position as usize] = BLACK_ROOK;
                                                    black_rooks.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                'B' => { // promotion to bishop
                                                    black_column_a.remove(0);
                                                    board[desired_position as usize] = BLACK_BISHOP;
                                                    black_bishops.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                _ => ()
                                            }
                                        },
                                        'b' => {
                                            match prom_vec[0] {
                                                'N' => { // promotion to knight
                                                    black_column_b.remove(0);
                                                    board[desired_position as usize] = BLACK_KNIGHT;
                                                    black_knights.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                'R' => { // promotion to rook
                                                    black_column_b.remove(0);
                                                    board[desired_position as usize] = BLACK_ROOK;
                                                    black_rooks.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                'B' => { // promotion to bishop
                                                    black_column_b.remove(0);
                                                    board[desired_position as usize] = BLACK_BISHOP;
                                                    black_bishops.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                _ => ()
                                            }
                                        },
                                        'c' => {
                                            match prom_vec[0] {
                                                'N' => { // promotion to knight
                                                    black_column_c.remove(0);
                                                    board[desired_position as usize] = BLACK_KNIGHT;
                                                    black_knights.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                'R' => { // promotion to rook
                                                    black_column_c.remove(0);
                                                    board[desired_position as usize] = BLACK_ROOK;
                                                    black_rooks.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                'B' => { // promotion to bishop
                                                    black_column_c.remove(0);
                                                    board[desired_position as usize] = BLACK_BISHOP;
                                                    black_bishops.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                _ => ()
                                            }
                                        },
                                        'd' => {
                                            match prom_vec[0] {
                                                'N' => { // promotion to knight
                                                    black_column_d.remove(0);
                                                    board[desired_position as usize] = BLACK_KNIGHT;
                                                    black_knights.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                'R' => { // promotion to rook
                                                    black_column_d.remove(0);
                                                    board[desired_position as usize] = BLACK_ROOK;
                                                    black_rooks.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                'B' => { // promotion to bishop
                                                    black_column_d.remove(0);
                                                    board[desired_position as usize] = BLACK_BISHOP;
                                                    black_bishops.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                _ => ()
                                            }
                                        },
                                        'e' => {
                                            match prom_vec[0] {
                                                'N' => { // promotion to knight
                                                    black_column_e.remove(0);
                                                    board[desired_position as usize] = BLACK_KNIGHT;
                                                    black_knights.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                'R' => { // promotion to rook
                                                    black_column_e.remove(0);
                                                    board[desired_position as usize] = BLACK_ROOK;
                                                    black_rooks.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                'B' => { // promotion to bishop
                                                    black_column_e.remove(0);
                                                    board[desired_position as usize] = BLACK_BISHOP;
                                                    black_bishops.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                _ => ()
                                            }
                                        },
                                        'f' => {
                                            match prom_vec[0] {
                                                'N' => { // promotion to knight
                                                    black_column_f.remove(0);
                                                    board[desired_position as usize] = BLACK_KNIGHT;
                                                    black_knights.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                'R' => { // promotion to rook
                                                    black_column_f.remove(0);
                                                    board[desired_position as usize] = BLACK_ROOK;
                                                    black_rooks.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                'B' => { // promotion to bishop
                                                    black_column_f.remove(0);
                                                    board[desired_position as usize] = BLACK_BISHOP;
                                                    black_bishops.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                _ => ()
                                            }
                                        },
                                        'g' => {
                                            match prom_vec[0] {
                                                'N' => { // promotion to knight
                                                    black_column_g.remove(0);
                                                    board[desired_position as usize] = BLACK_KNIGHT;
                                                    black_knights.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                'R' => { // promotion to rook
                                                    black_column_g.remove(0);
                                                    board[desired_position as usize] = BLACK_ROOK;
                                                    black_rooks.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                'B' => { // promotion to bishop
                                                    black_column_g.remove(0);
                                                    board[desired_position as usize] = BLACK_BISHOP;
                                                    black_bishops.insert(0, desired_position);
                                                    prom_loop = false;
                                                },
                                                _ => ()
                                            }
                                        },
                                        _ => ()
                                    };
                                }else{ // queen promotion (Enter key doesnt add a character to the vector, so it's lenght stays 0)
                                    match san_move[2] {
                                        'a' => {
                                            black_column_a.remove(0);
                                            board[desired_position as usize] = BLACK_QUEEN;
                                            black_queens.insert(0, desired_position);
                                            prom_loop = false;
                                        },
                                        'b' => {
                                            black_column_b.remove(0);
                                            board[desired_position as usize] = BLACK_QUEEN;
                                            black_queens.insert(0, desired_position);
                                            prom_loop = false;
                                        },
                                        'c' => {
                                            black_column_c.remove(0);
                                            board[desired_position as usize] = BLACK_QUEEN;
                                            black_queens.insert(0, desired_position);
                                            prom_loop = false;
                                        },
                                        'd' => {
                                            black_column_d.remove(0);
                                            board[desired_position as usize] = BLACK_QUEEN;
                                            black_queens.insert(0, desired_position);
                                            prom_loop = false;
                                        },
                                        'e' => {
                                            black_column_e.remove(0);
                                            board[desired_position as usize] = BLACK_QUEEN;
                                            black_queens.insert(0, desired_position);
                                            prom_loop = false;
                                        },
                                        'f' => {
                                            black_column_f.remove(0);
                                            board[desired_position as usize] = BLACK_QUEEN;
                                            black_queens.insert(0, desired_position);
                                            prom_loop = false;
                                        },
                                        'g' => {
                                            black_column_g.remove(0);
                                            board[desired_position as usize] = BLACK_QUEEN;
                                            black_queens.insert(0, desired_position);
                                            prom_loop = false;
                                        },
                                        _ => ()
                                    };
                                }
                                if prom_loop == true {
                                    println!("Not a possible promotion, try again!\n");
                                }
                            }
                        }
                    }
                }
            }

            if try_again == true{
                println!("Not a possible move, try again!\n");
            }
        }

        white_column_a_enpassant = false;
        white_column_b_enpassant = false;
        white_column_c_enpassant = false;
        white_column_d_enpassant = false;
        white_column_e_enpassant = false;
        white_column_f_enpassant = false;
        white_column_g_enpassant = false;
        white_column_h_enpassant = false;
    
    } // loop end
} 

fn show_board(board: [char; 64]) { //print the board
    for i in 0..board.len() { //print the board
        //for every element in the board until 0, print the current element

        // i+1 = the actual position of the item in the array (instead of its index)
        // if the position of the item -isn't- divisible by 8 (remainder is NOT equal to 0), print the board normally
            if (i+1)%8 != 0{
                print!(" {} ", board[i]);
            }else{
            //if -it is- divisible by 8 (remainder IS equal to 0), add a line break
                println!(" {} ", board[i]);
            };
        };
}

// piece/color checks
fn is_piece(piece: char) -> bool {
    match piece {
        'N' |
        'B' |
        'R' |
        'Q' |
        'K' => true,
        _ => false
    }
}

fn is_white(piece: char) -> bool {
    match piece {
        'i' |
        'R' |
        'N' |
        'B' |
        'Q' |
        'K' => true,
        _ => false
    }
}

fn is_black(piece: char) -> bool {
    match piece {
        'j' |
        'r' |
        'n' |
        'b' |
        'q' |
        'k' => true,
        _ => false
    }
}


// diagonal checks
fn upper_right_diagonal(b: i8, i: i8) -> bool {
    // if the index is one of the following,
    // return true

    match b - i*7 {
        // a bishop cannot trace a path after it hits the board's "walls" or corners
        56 | 48 | 40 | 32 | 24 | 16 | 8 | 0 |
        // positions where subtracting to the index would exceed the array:
        -1 | -2 | -3 | -4 | -5 | -6 | -7 => true,
        _ => false
    }
}

fn upper_left_diagonal(b: i8, i: i8) -> bool {
    match b - i*9 {
        // a bishop cannot trace a path after it hits the board's "walls" or corners
        47 | 39 | 31 | 23 | 15 | 7 |
        // positions where subtracting to the index would exceed the array:
        -1 | -2 | -3 | -4 | -5 | -6 | -7 | -8 | -9 => true,
        _ => false
    }
}

fn inferior_right_diagonal(b: i8, i: i8) -> bool {
    match b + i*9 {
        // a bishop cannot trace a path after it hits the board's "walls" or corners
        16 | 24 | 32 | 40 | 48 | 50 | 51 | 52 | 56 |
        // positions where adding to the index would exceed the array:
        64..=72  => true,
        _ => false
    }
}

fn inferior_left_diagonal(b: i8, i: i8) -> bool {
    match b + i*7 {
        // a bishop cannot trace a path after it hits the board's "walls" or corners
        7 | 15 | 23 | 31 | 39 | 47 | 55 |
        // positions where adding to the index would exceed the array:
        64..=70 => true,
        _ => false
    }
}


// rook-like movement checks
fn rook_right(b: i8, i: i8) -> bool {
    match b + i {
        8 | 16 | 24 | 32 | 40 | 48 | 56 | 64 => true,
        _ => false
    }
}

fn rook_left(b: i8, i: i8) -> bool {
    match b - i {
        55 | 47 | 39 | 31 | 23 | 15 | 7 | -1 => true,
        _ => false
    }
}

fn rook_down(b: i8, i: i8) -> bool {
    match b + i*8 {
        64..= 71 => true,
        _ => false
    }
}

fn rook_up(b: i8, i: i8) -> bool {
    match b - i*8 {
        -1 | -2 | -3 | -4 | -5 | -6 | -7 | -8 => true,
        _ => false
    }
}

fn get_line(piece: i8) -> i8 {
    match piece {
        0..=7 => 8,
        8..=15 => 7,
        16..=23 => 6,
        24..=31 => 5,
        32..=39 => 4,
        40..=47 => 3,
        48..=55 => 2,
        56..=63 => 1,
        _ => 0
    }
}


fn test_multiple_rooks(pieces: &mut Vec<i8>, position: i8) -> bool {
    // this tests every rook or piece with rook-like 
    // movement inside a vector to see if 
    // more than one of them can reach the same square

    // true = multiple pieces may go to the desired square
    let mut counter = 0;

    if pieces.len() >= 2 {
        for i in 0..pieces.len() {
            if get_line(pieces[i]) == get_line(position) || (position - pieces[i])%8 == 0 {
                counter += 1
            }
            if counter >= 2 {
                return true
            }
        }
    }

    return false
}

fn test_multiple_bishops(pieces: &mut Vec<i8> , position: i8) -> bool {
    // this tests every bishop inside a vector to see if  
    // more than one of them can reach the same square

    // return true = multiple pieces may go to the desired square
    // return false = only one piece may go to the desired square
    if pieces.len() == 2 {
        if match pieces[0] { 
            // true = light-squared bishop
            // false = dark-squared bishop
            0 | 2 | 4 | 6 | 9 | 11 | 13 |
            15 | 16 | 18 | 20 | 22 | 25 |
            27 | 29 | 31 | 32 | 34 | 36 |
            38 | 41 | 43 | 45 | 47 | 48 |
            50 | 52 | 54 | 57 | 59 | 61 |
            63 => true,
            _ => false
        } == match pieces[1] {
            0 | 2 | 4 | 6 | 9 | 11 | 13 |
            15 | 16 | 18 | 20 | 22 | 25 |
            27 | 29 | 31 | 32 | 34 | 36 |
            38 | 41 | 43 | 45 | 47 | 48 |
            50 | 52 | 54 | 57 | 59 | 61 |
            63 => true,
            _ => false
        } { // if both bishops are the same color
            if ((position - pieces[0])%7 == 0 || (position - pieces[0])%9 == 0) 
            && ((position - pieces[1])%7 == 0 || (position - pieces[1])%9 == 0) { 
                // and both may reach the same square
                return true
            }
        }
    } else if pieces.len() > 2 {
        // if there are more than 2 bishops, there must be more than one dark-squared or light-squared bishop
        // therefore, they probably can go to the same square
        // this is an extremely rare situation by itself (no one ever promotes to bishops in real games)
        return true
    }

    // if there is only one bishop
    return false
}

fn test_multiple_knights(pieces: &mut Vec<i8>, position: i8) -> bool {
    // this tests every knight
    // inside a vector to see if
    // more than one of them can reach the same square

    // true = multiple pieces may go to the desired square
    let mut counter = 0;
    
    if pieces.len() >= 2 {
        for i in 0..pieces.len() {
            match position - pieces[i]{
                -17 | -15 | -10 | -6 | 6 | 10 | 15 | 17 => {
                    counter += 1;
                },
                _ => ()
            }

            if counter >= 2 {
                return true
            }
        }
    }

    return false
}

fn test_multiple_queens(pieces: &mut Vec<i8>, position: i8) -> bool {
    let mut counter = 0;
    if pieces.len() >= 2 {
        for i in 0..pieces.len() {
            if (position - pieces[i])%7 == 0 || (position - pieces[i])%9 == 0 // diagonal movement
            || get_line(pieces[i]) == get_line(position) || (position - pieces[i])%8 == 0 { // rook-like movement 
                counter += 1;
            }
            if counter >= 2 {
                return true
            }
        }
    }

    return false
}

fn get_pieces_checking_the_white_king(kings_position: i8, board: &[char;64]) -> Vec<i8> {
    // check every possible cardinal direction of the king to see if there are pieces checking it
    let mut pieces_checking_the_king: Vec<i8> = vec![];

    //left side of the king
    for left_square in 1..8 {
        if !rook_left(kings_position, left_square) 
        && (board[(kings_position-left_square) as usize] == BLACK_ROOK 
        || board[(kings_position-left_square) as usize] == BLACK_QUEEN) {
            pieces_checking_the_king.insert(0, kings_position-left_square);
            break;
        }else if rook_left(kings_position, left_square)
        || is_white(board[(kings_position-left_square) as usize]) 
        || is_black(board[(kings_position-left_square) as usize]) {
            // if it is black but not a rook or a queen, 
            // its not aiming directly at the king
            break;
        }
    }

    //right side of the king
    for right_square in 1..8 {
        if !rook_right(kings_position, right_square)
        && (board[(kings_position+right_square) as usize] == BLACK_ROOK 
        || board[(kings_position+right_square) as usize] == BLACK_QUEEN) {
            pieces_checking_the_king.insert(0, kings_position+right_square);
            break;
        }else if rook_right(kings_position, right_square)
        || is_white(board[(kings_position+right_square) as usize]) 
        || is_black(board[(kings_position+right_square) as usize]) {
            break;
        }
    }

    for down_square in 1..8 {
        if !rook_down(kings_position, down_square)
        && (board[(kings_position+down_square*8) as usize] == BLACK_ROOK 
        || board[(kings_position+down_square*8) as usize] == BLACK_QUEEN) {
            pieces_checking_the_king.insert(0, kings_position+down_square*8);
            break;
        }else if rook_down(kings_position, down_square)
        || is_white(board[(kings_position+down_square*8) as usize]) 
        || is_black(board[(kings_position+down_square*8) as usize]) {
            break;
        }
    }

    for up_square in 1..8 {
        if !rook_up(kings_position, up_square)
        && (board[(kings_position-up_square*8) as usize] == BLACK_ROOK 
        || board[(kings_position-up_square*8) as usize] == BLACK_QUEEN) {
            pieces_checking_the_king.insert(0, kings_position-up_square*8);
            break;
        }else if rook_up(kings_position, up_square)
        || is_white(board[(kings_position-up_square*8) as usize]) 
        || is_black(board[(kings_position-up_square*8) as usize]) {
            break;
        }
    }

    //upper left diagonal
    for diagonal in 1..8 {
        if !upper_left_diagonal(kings_position, diagonal)
        && (board[(kings_position-diagonal*9) as usize] == BLACK_BISHOP 
        || board[(kings_position-diagonal*9) as usize] == BLACK_QUEEN 
        || board[(kings_position-9) as usize] == BLACK_PAWN) {
            pieces_checking_the_king.insert(0, kings_position-diagonal*9);
            break;
        }else if upper_left_diagonal(kings_position, diagonal)
        || is_white(board[(kings_position-diagonal*9) as usize]) 
        || is_black(board[(kings_position-diagonal*9) as usize]) {
            break;
        }
    }
    
    for diagonal in 1..8 {
        if !upper_right_diagonal(kings_position, diagonal)
        && (board[(kings_position-diagonal*7) as usize] == BLACK_BISHOP 
        || board[(kings_position-diagonal*7) as usize] == BLACK_QUEEN
        || board[(kings_position-7) as usize] == BLACK_PAWN) {
            pieces_checking_the_king.insert(0, kings_position-diagonal*7);
            break;
        }else if upper_right_diagonal(kings_position, diagonal)
        || is_white(board[(kings_position-diagonal*7) as usize]) 
        || is_black(board[(kings_position-diagonal*7) as usize]) {
            break;
        }
    }

    for diagonal in 1..8 {
        if !inferior_left_diagonal(kings_position, diagonal)
        && (board[(kings_position+diagonal*7) as usize] == BLACK_BISHOP 
        || board[(kings_position+diagonal*7) as usize] == BLACK_QUEEN) {
            pieces_checking_the_king.insert(0, kings_position+diagonal*7);
            break;
        }else if inferior_left_diagonal(kings_position, diagonal)
        || is_white(board[(kings_position+diagonal*7) as usize]) 
        || is_black(board[(kings_position+diagonal*7) as usize]) {
            break;
        }
    }
    
    for diagonal in 1..8 {
        if !inferior_right_diagonal(kings_position, diagonal)
        && (board[(kings_position+diagonal*9) as usize] == BLACK_BISHOP 
        || board[(kings_position+diagonal*9) as usize] == BLACK_QUEEN) {
            pieces_checking_the_king.insert(0, kings_position+diagonal*9);
            break;
        }else if inferior_right_diagonal(kings_position, diagonal)
        || is_white(board[(kings_position+diagonal*9) as usize]) 
        || is_black(board[(kings_position+diagonal*9) as usize]) {
            break;
        }
    }
    
    if (kings_position + 17) <= 63 {
        if board[(kings_position+17) as usize] == BLACK_KNIGHT {
            pieces_checking_the_king.insert(0, kings_position+17);
        }
    }
    if (kings_position + 15) <= 63 {
        if board[(kings_position+15) as usize] == BLACK_KNIGHT {
            pieces_checking_the_king.insert(0, kings_position+15);
        }
    }
    if (kings_position + 10) <= 63 {
        if board[(kings_position+10) as usize] == BLACK_KNIGHT {
            pieces_checking_the_king.insert(0, kings_position+10);
        }
    }
    if (kings_position + 6) <= 63 {
        if board[(kings_position+6) as usize] == BLACK_KNIGHT {
            pieces_checking_the_king.insert(0, kings_position+6);
        }
    }
    if (kings_position - 6) >= 0 {
        if board[(kings_position-6) as usize] == BLACK_KNIGHT {
            pieces_checking_the_king.insert(0, kings_position-6);
        }
    }
    if (kings_position - 10) >= 0 {
        if board[(kings_position-10) as usize] == BLACK_KNIGHT {
            pieces_checking_the_king.insert(0, kings_position-10);
        }
    }
    if (kings_position - 15) >= 0 {
        if board[(kings_position-15) as usize] == BLACK_KNIGHT {
            pieces_checking_the_king.insert(0, kings_position-15);
        }
    }
    if (kings_position - 17) >= 0 {
        if board[(kings_position-17) as usize] == BLACK_KNIGHT {
            pieces_checking_the_king.insert(0, kings_position-17);
        }
    }

    return pieces_checking_the_king

}

fn get_pieces_checking_the_black_king(kings_position: i8, board: &[char;64]) -> Vec<i8> {
    // check every possible cardinal direction of the king to see if there are pieces checking it
    let mut pieces_checking_the_king: Vec<i8> = vec![];

    //left side of the king
    for left_square in 1..8 {
        if !rook_left(kings_position, left_square) 
        && (board[(kings_position-left_square) as usize] == WHITE_ROOK 
        || board[(kings_position-left_square) as usize] == WHITE_QUEEN) {
            pieces_checking_the_king.insert(0, kings_position-left_square);
            break;
        }else if rook_left(kings_position, left_square)
        || is_white(board[(kings_position-left_square) as usize]) 
        || is_black(board[(kings_position-left_square) as usize]) {
            // if it is white but not a rook or a queen, 
            // its not aiming directly at the king
            break;
        }
    }

    //right side of the king
    for right_square in 1..8 {
        if !rook_right(kings_position, right_square)
        && (board[(kings_position+right_square) as usize] == WHITE_ROOK 
        || board[(kings_position+right_square) as usize] == WHITE_QUEEN) {
            pieces_checking_the_king.insert(0, kings_position+right_square);
            break;
        }else if rook_right(kings_position, right_square)
        || is_white(board[(kings_position+right_square) as usize]) 
        || is_black(board[(kings_position+right_square) as usize]) {
            break;
        }
    }

    for down_square in 1..8 {
        if !rook_down(kings_position, down_square)
        && (board[(kings_position+down_square*8) as usize] == WHITE_ROOK 
        || board[(kings_position+down_square*8) as usize] == WHITE_QUEEN) {
            pieces_checking_the_king.insert(0, kings_position+down_square*8);
            break;
        }else if rook_down(kings_position, down_square)
        || is_white(board[(kings_position+down_square*8) as usize]) 
        || is_black(board[(kings_position+down_square*8) as usize]) {
            break;
        }
    }

    for up_square in 1..8 {
        if !rook_up(kings_position, up_square)
        && (board[(kings_position-up_square*8) as usize] == WHITE_ROOK 
        || board[(kings_position-up_square*8) as usize] == WHITE_QUEEN) {
            pieces_checking_the_king.insert(0, kings_position-up_square*8);
            break;
        }else if rook_up(kings_position, up_square)
        || is_white(board[(kings_position-up_square*8) as usize]) 
        || is_black(board[(kings_position-up_square*8) as usize]) {
            break;
        }
    }

    //upper left diagonal
    for diagonal in 1..8 {
        if !upper_left_diagonal(kings_position, diagonal)
        && (board[(kings_position-diagonal*9) as usize] == WHITE_BISHOP 
        || board[(kings_position-diagonal*9) as usize] == WHITE_QUEEN) {
            pieces_checking_the_king.insert(0, kings_position-diagonal*9);
            break;
        }else if upper_left_diagonal(kings_position, diagonal)
        || is_white(board[(kings_position-diagonal*9) as usize]) 
        || is_black(board[(kings_position-diagonal*9) as usize]) {
            break;
        }
    }

    for diagonal in 1..8 {
        if !upper_right_diagonal(kings_position, diagonal)
        && (board[(kings_position-diagonal*7) as usize] == WHITE_BISHOP 
        || board[(kings_position-diagonal*7) as usize] == WHITE_QUEEN) {
            pieces_checking_the_king.insert(0, kings_position-diagonal*7);
            break;
        }else if upper_right_diagonal(kings_position, diagonal)
        || is_white(board[(kings_position-diagonal*7) as usize]) 
        || is_black(board[(kings_position-diagonal*7) as usize]) {
            break;
        }
    }

    for diagonal in 1..8 {
        if !inferior_left_diagonal(kings_position, diagonal)
        && (board[(kings_position+diagonal*7) as usize] == WHITE_BISHOP 
        || board[(kings_position+diagonal*7) as usize] == WHITE_QUEEN
        || board[(kings_position+7) as usize] == WHITE_PAWN) {
            pieces_checking_the_king.insert(0, kings_position+diagonal*7);
            break;
        }else if inferior_left_diagonal(kings_position, diagonal)
        || is_white(board[(kings_position+diagonal*7) as usize]) 
        || is_black(board[(kings_position+diagonal*7) as usize]) {
            break;
        }
    }
    
    for diagonal in 1..8 {
        if !inferior_right_diagonal(kings_position, diagonal)
        && (board[(kings_position+diagonal*9) as usize] == WHITE_BISHOP 
        || board[(kings_position+diagonal*9) as usize] == WHITE_QUEEN
        || board[(kings_position+9) as usize] == WHITE_PAWN) {
            pieces_checking_the_king.insert(0, kings_position+diagonal*9);
            break;
        }else if inferior_right_diagonal(kings_position, diagonal)
        || is_white(board[(kings_position+diagonal*9) as usize]) 
        || is_black(board[(kings_position+diagonal*9) as usize]) {
            break;
        }
    }
    

    if (kings_position + 17) <= 63 {
        if board[(kings_position+17) as usize] == WHITE_KNIGHT {
            pieces_checking_the_king.insert(0, kings_position+17);
        }
    }
    if (kings_position + 15) <= 63 {
        if board[(kings_position+15) as usize] == WHITE_KNIGHT {
            pieces_checking_the_king.insert(0, kings_position+15);
        }
    }
    if (kings_position + 10) <= 63 {
        if board[(kings_position+10) as usize] == WHITE_KNIGHT {
            pieces_checking_the_king.insert(0, kings_position+10);
        }
    }
    if (kings_position + 6) <= 63 {
        if board[(kings_position+6) as usize] == WHITE_KNIGHT {
            pieces_checking_the_king.insert(0, kings_position+6);
        }
    }
    if (kings_position - 6) >= 0 {
        if board[(kings_position-6) as usize] == WHITE_KNIGHT {
            pieces_checking_the_king.insert(0, kings_position-6);
        }
    }
    if (kings_position - 10) >= 0 {
        if board[(kings_position-10) as usize] == WHITE_KNIGHT {
            pieces_checking_the_king.insert(0, kings_position-10);
        }
    }
    if (kings_position - 15) >= 0 {
        if board[(kings_position-15) as usize] == WHITE_KNIGHT {
            pieces_checking_the_king.insert(0, kings_position-15);
        }
    }
    if (kings_position - 17) >= 0 {
        if board[(kings_position-17) as usize] == WHITE_KNIGHT {
            pieces_checking_the_king.insert(0, kings_position-17);
        }
    }

    return pieces_checking_the_king

}