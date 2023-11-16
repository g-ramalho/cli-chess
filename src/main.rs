#![allow(dead_code)]
use std::io;
use std::string::String;

fn main() {
    
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

//LAST POSITION OF EACH PIECE AND PAWN:
    //white pawns
    let white_pawn_a: i8 = 48;
    let mut white_column_a = vec![white_pawn_a];

    let white_pawn_b: i8 = 49;
    let mut white_column_b = vec![white_pawn_b];

    let white_pawn_c: i8 = 50;
    let mut white_column_c = vec![white_pawn_c];

    let white_pawn_d: i8 = 51;
    let mut white_column_d = vec![white_pawn_d];

    let white_pawn_e: i8 = 52;
    let mut white_column_e = vec![white_pawn_e];

    let white_pawn_f: i8 = 53;
    let mut white_column_f = vec![white_pawn_f];

    let white_pawn_g: i8 = 54;
    let mut white_column_g = vec![white_pawn_g];

    let white_pawn_h: i8 = 55;
    let mut white_column_h = vec![white_pawn_h];

    //black pawns
    let black_pawn_a: i8 = 8;
    let mut black_column_a = vec![black_pawn_a];

    let black_pawn_b: i8 = 9;
    let mut black_column_b = vec![black_pawn_b];

    let black_pawn_c: i8 = 10;
    let mut black_column_c = vec![black_pawn_c];

    let black_pawn_d: i8 = 11;
    let mut black_column_d = vec![black_pawn_d];

    let black_pawn_e: i8 = 12;
    let mut black_column_e = vec![black_pawn_e];

    let black_pawn_f: i8 = 13;
    let mut black_column_f = vec![black_pawn_f];

    let black_pawn_g: i8 = 14;
    let mut black_column_g = vec![black_pawn_g];

    let black_pawn_h: i8 = 15;
    let mut black_column_h = vec![black_pawn_h];

    //white pieces
    let mut white_rook1: i8 = 56;
    let mut white_rook2: i8 = 63;
    
    let mut white_knight1: i8 = 57;
    let mut white_knight2: i8 = 62;
    
    let mut white_bishop1: i8 = 58;
    let mut white_bishop2: i8 = 61;
    
    //let mut white_queen: i8 = 59;
    let mut white_king: i8 = 60;
    
    //black pieces
    let mut black_rook1: i8 = 0;
    let mut black_rook2: i8 = 7;
    
    let mut black_knight1: i8 = 1;
    let mut black_knight2: i8 = 6;
    
    let mut black_bishop1: i8 = 2;
    let mut black_bishop2: i8 = 5;
    
    //let mut black_queen: i8 = 3;
    let mut black_king: i8 = 4;

//create the board
    let mut board = [NOTHING; 64];

    // pawns setup
    // number ranges are half-open (a <= x < b), so we +1 to the last number
    for i in 0..8 {
        board[i+8] = BLACK_PAWN;
        board[i+48] = WHITE_PAWN;
    };

    //rook setup 0 7 56 63
    board[0] = BLACK_ROOK;
    board[7] = BLACK_ROOK;
    board[56] = WHITE_ROOK;
    board[63] = WHITE_ROOK;

    //knight setup 1 6 57 62
    board[1] = BLACK_KNIGHT;
    board[6] = BLACK_KNIGHT;
    board[57] = WHITE_KNIGHT;
    board[62] = WHITE_KNIGHT;

    //bishop setup 2 5 58 61
    board[2] = BLACK_BISHOP;
    board[5] = BLACK_BISHOP;
    board[58] = WHITE_BISHOP;
    board[61] = WHITE_BISHOP;

    //queen setup 3 59
    board[3] = BLACK_QUEEN;
    board[59] = WHITE_QUEEN;

    //king setup 4 60
    board[4] = BLACK_KING;
    board[60] = WHITE_KING;

    loop {
    
        let mut player_move = String::new();
        let mut san_move: Vec<char>;

        let mut column: i8;
        let mut line: i8;
        let mut desired_position: i8;

        let mut try_again: bool;

        // WHITE'S TURN:
        try_again = true;
        show_board(board); //print the board

        while try_again {
            //has to be cleared, otherwise read_line would just append the string to the last move registered in player_move
            player_move.clear();

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
                //if the first letter indicates a piece, the move has to be described in the next two letters
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

                desired_position = column + line;
                if desired_position >= 100 {
                    println!("Not a possible move, try again!\n");
                    continue;
                }

                //pieces' movement checks
                match san_move[0] {
                    'N' => {
                        if !is_white(board[desired_position as usize]) {   
                            match desired_position - white_knight1 {
                                -17 | -15 | -10 | -6 | 6 | 10 | 15 | 17 => {
                                        //last position is freed
                                        board[white_knight1 as usize] = NOTHING;

                                        //piece is moved to new position
                                        board[desired_position as usize] = WHITE_KNIGHT;

                                        //current position is updated
                                        white_knight1 = desired_position;
                                        break;
                                },
                                _ => ()
                            }
                            match desired_position - white_knight2 {
                                -17 | -15 | -10 | -6 | 6 | 10 | 15 | 17 => {
                                        board[white_knight2 as usize] = NOTHING;
                                        board[desired_position as usize] = WHITE_KNIGHT;
                                        white_knight2 = desired_position;
                                        break;
                                },
                                _ => ()
                            }
                        }
                    },
                    'B' => {
                        // this tests the dark squares bishop
                        // if the desired square is "above" the initial position
                        if white_bishop1 > desired_position && !is_white(board[desired_position as usize]) {
                            // and if the distance is divisible by 7
                            if (white_bishop1 - desired_position)%7 == 0 {
                                for diagonal in 1..8 {
                                    // count each possible diagonal (until the maximum of 7 diagonals)
                                    if white_bishop1 - diagonal*7 == desired_position && !upper_right_diagonal(white_bishop1, diagonal) {
                                        // check if any of the squares in the bishop's diagonal is the desired square,
                                        // check if any of the calculated diagonals are forbidden (done using the 'upper_right_diagonal' function),
                                        // and finally, check if the desired square has no white pieces that may block the movement
                                        // if all of those checks are true, the bishop may be moved
                                        board[white_bishop1 as usize] = NOTHING;
                                        board[desired_position as usize] = WHITE_BISHOP;
                                        white_bishop1 = desired_position;

                                        try_again = false;
                                        break;
                                    }else if is_white(board[(white_bishop1 - diagonal*7) as usize]) || is_black(board[((white_bishop1 - diagonal*7) as usize)]) {
                                        // otherwise, if there are any white/black pieces on the way, the square is unreachable
                                        break;
                                    }
                                }
                            }else if (white_bishop1 - desired_position)%9 == 0 {
                                for diagonal in 1..8 {
                                    if white_bishop1 - diagonal*9 == desired_position && !upper_left_diagonal(white_bishop1, diagonal) {
                                        board[white_bishop1 as usize] = NOTHING;
                                        board[desired_position as usize] = WHITE_BISHOP;
                                        white_bishop1 = desired_position;

                                        try_again = false;
                                        break;
                                    }else if is_white(board[(white_bishop1 - diagonal*9) as usize]) || is_black(board[((white_bishop1 - diagonal*9) as usize)]){
                                        break;
                                    }
                                }
                            }
                        }else if white_bishop1 < desired_position  && (((desired_position - white_bishop1)%7 == 0) || ((desired_position - white_bishop1)%9 == 0)) {
                            if (white_bishop1 - desired_position)%7 == 0 {
                                for diagonal in 1..8 {
                                    if white_bishop1 + diagonal*7 == desired_position && !inferior_left_diagonal(white_bishop1, diagonal) {
                                        board[white_bishop1 as usize] = NOTHING;
                                        board[desired_position as usize] = WHITE_BISHOP;
                                        white_bishop1 = desired_position;

                                        try_again = false;
                                        break;
                                    }else if is_white(board[(white_bishop1 + 7*diagonal) as usize]) || is_black(board[(white_bishop1 + 7*diagonal) as usize]){
                                        break;
                                    }
                                }
                            }else if (white_bishop1 - desired_position)%9 == 0 {
                                for diagonal in 1..8 {
                                    if white_bishop1 + diagonal*9 == desired_position && !inferior_right_diagonal(white_bishop1, diagonal) {
                                        board[white_bishop1 as usize] = NOTHING;
                                        board[desired_position as usize] = WHITE_BISHOP;
                                        white_bishop1 = desired_position;

                                        try_again = false;
                                        break;
                                    }else if is_white(board[(white_bishop1 + 9*diagonal) as usize]) || is_black(board[(white_bishop1 + 9*diagonal) as usize]) {
                                        break;
                                    }
                                }
                            }
                        }
                        // this tests the light squares bishop
                        if white_bishop2 > desired_position && !is_white(board[desired_position as usize]) {
                            if (white_bishop2 - desired_position)%7 == 0 {
                                for diagonal in 1..8 {
                                    if white_bishop2 - diagonal*7 == desired_position && !upper_right_diagonal(white_bishop2, diagonal) {
                                        board[white_bishop2 as usize] = NOTHING;
                                        board[desired_position as usize] = WHITE_BISHOP;
                                        white_bishop2 = desired_position;

                                        try_again = false;
                                        break;
                                    }else if is_white(board[(white_bishop2 - diagonal*7) as usize]) || is_black(board[(white_bishop2 - diagonal*7) as usize]){
                                        break;
                                    }
                                }
                            }else if (white_bishop2 - desired_position)%9 == 0 {
                                for diagonal in 1..8 {
                                    if white_bishop2 - diagonal*9 == desired_position && !upper_left_diagonal(white_bishop2, diagonal) {
                                        board[white_bishop2 as usize] = NOTHING;
                                        board[desired_position as usize] = WHITE_BISHOP;
                                        white_bishop2 = desired_position;

                                        try_again = false;
                                        break;
                                    }else if is_white(board[(white_bishop2 - diagonal*9) as usize]) || is_black(board[(white_bishop2 - diagonal*9) as usize]){
                                        break;
                                    }
                                }
                            }
                        }else if white_bishop2 < desired_position && (((desired_position - white_bishop2)%7 == 0) || ((desired_position - white_bishop2)%9 == 0)) {
                            if (white_bishop2 - desired_position)%7 == 0 {
                                for diagonal in 1..8 {
                                    if white_bishop2 + diagonal*7 == desired_position && !inferior_left_diagonal(white_bishop2, diagonal) {
                                        board[white_bishop2 as usize] = NOTHING;
                                        board[desired_position as usize] = WHITE_BISHOP;
                                        white_bishop2 = desired_position;

                                        try_again = false;
                                        break;
                                    }else if is_white(board[(white_bishop2 + 7*diagonal) as usize]) || is_black(board[(white_bishop2 + 7*diagonal) as usize]){
                                        break;
                                    }
                                }
                            }else if (white_bishop2 - desired_position)%9 == 0 {
                                for diagonal in 1..8 {
                                    if white_bishop2 + diagonal*9 == desired_position && !inferior_right_diagonal(white_bishop2, diagonal) {
                                        board[white_bishop2 as usize] = NOTHING;
                                        board[desired_position as usize] = WHITE_BISHOP;
                                        white_bishop2 = desired_position;

                                        try_again = false;
                                        break;
                                    }else if is_white(board[(white_bishop2 + 9*diagonal) as usize]) || is_black(board[(white_bishop2 + 9*diagonal) as usize]) {
                                        break;
                                    }
                                }
                            }
                        }
                    },
                    'R' => {
                        // check if both rooks can reach the desired square
                            // if the rooks are in the same column
                        if ((white_rook1 - desired_position)%8 == 0 && (white_rook2 - desired_position)%8 == 0)
                            // or in the same line 
                        || ((white_rook1 - desired_position) <= 7 && (white_rook1 - desired_position) >= 1 && (white_rook2 - desired_position) >= -7 && (white_rook2 - desired_position) <= -1)
                        || ((white_rook1 - desired_position) >= -7 && (white_rook1 - desired_position) <= -1 && (white_rook2 - desired_position) <= 7 && (white_rook2 - desired_position) >= 1)
                            // or in different columns, but both are still able to reach the square
                        || ((white_rook1 - desired_position)%8 == 0 && (white_rook2 - desired_position) <= 7 && (white_rook2 - desired_position) >= -7) 
                        || ((white_rook1 - desired_position) <= 7 && (white_rook1 - desired_position) >= -7 && (white_rook2 - desired_position)%8 == 0) {
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
                            
                            if rook_column + rook_line == white_rook1 {
                                if white_rook1 > desired_position && !is_white(board[desired_position as usize]) {
                                    // if the desired square is on the same rank as the initial position
                                    if white_rook1 - desired_position <= 7 {
                                        for square in 1..8 {
                                            if white_rook1 - square == desired_position && !rook_left(white_rook1, square) {
                                                board[white_rook1 as usize] = NOTHING;
                                                board[desired_position as usize] = WHITE_ROOK;
                                                white_rook1 = desired_position;
        
                                                try_again = false;
                                                break;
                                            }else if is_white(board[(white_rook1 - square) as usize]) || is_black(board[(white_rook1 - square) as usize]) {
                                                break;
                                            }
                                        }
                                    }else if (white_rook1 - desired_position)%8 == 0 {
                                        // otherwise, test if it is on the same file
                                        for square in 1..8 {
                                            if white_rook1 - square*8 == desired_position && !rook_up(white_rook1, square) {
                                                board[white_rook1 as usize] = NOTHING;
                                                board[desired_position as usize] = WHITE_ROOK;
                                                white_rook1 = desired_position;
        
                                                try_again = false;
                                                break;
                                            }else if is_white(board[(white_rook1 - square*8) as usize]) || is_black(board[(white_rook1 - square*8) as usize]) {
                                                break;
                                            }
                                        }
                                    }
                                }else if white_rook1 < desired_position && !is_white(board[desired_position as usize]) {
                                    if desired_position - white_rook1 <= 7 {
                                        for square in 1..8 {
                                            if white_rook1 + square == desired_position && !rook_right(white_rook1, square) {
                                                board[white_rook1 as usize] = NOTHING;
                                                board[desired_position as usize] = WHITE_ROOK;
                                                white_rook1 = desired_position;
        
                                                try_again = false;
                                                break;
                                            }else if is_white(board[(white_rook1 + square) as usize]) || is_black(board[(white_rook1 + square) as usize]) {
                                                break;
                                            }
                                        }
                                    }else if (desired_position - white_rook1)%8 == 0 {
                                        for square in 1..8 {
                                            if white_rook1 + square*8 == desired_position && !rook_down(white_rook1, square) {
                                                board[white_rook1 as usize] = NOTHING;
                                                board[desired_position as usize] = WHITE_ROOK;
                                                white_rook1 = desired_position;
        
                                                try_again = false;
                                                break;
                                            }else if is_white(board[(white_rook1 + square*8) as usize]) || is_black(board[(white_rook1 + square*8) as usize]) {
                                                break;
                                            }
                                        }
                                    }
                                }
                            }else if rook_column + rook_line == white_rook2 {
                                if white_rook2 > desired_position && !is_white(board[desired_position as usize]) {
                                    // if the desired square is on the same rank as the initial position
                                    if white_rook2 - desired_position <= 7 {
                                        for square in 1..8 {
                                            if white_rook2 - square == desired_position && !rook_left(white_rook2, square) {
                                                board[white_rook2 as usize] = NOTHING;
                                                board[desired_position as usize] = WHITE_ROOK;
                                                white_rook2 = desired_position;
        
                                                try_again = false;
                                                break;
                                            }else if is_white(board[(white_rook2 - square) as usize]) || is_black(board[(white_rook2 - square) as usize]) {
                                                break;
                                            }
                                        }
                                    }else if (white_rook2 - desired_position)%8 == 0 {
                                        // otherwise, test if it is on the same file
                                        for square in 1..8 {
                                            if white_rook2 - square*8 == desired_position && !rook_up(white_rook2, square) {
                                                board[white_rook2 as usize] = NOTHING;
                                                board[desired_position as usize] = WHITE_ROOK;
                                                white_rook2 = desired_position;
        
                                                try_again = false;
                                                break;
                                            }else if is_white(board[(white_rook2 - square*8) as usize]) || is_black(board[(white_rook2 - square*8) as usize]) {
                                                break;
                                            }
                                        }
                                    }
                                }else if white_rook2 < desired_position && !is_white(board[desired_position as usize]) {
                                    if desired_position - white_rook2 <= 7 {
                                        for square in 1..8 {
                                            if white_rook2 + square == desired_position && !rook_right(white_rook2, square) {
                                                board[white_rook2 as usize] = NOTHING;
                                                board[desired_position as usize] = WHITE_ROOK;
                                                white_rook2 = desired_position;
        
                                                try_again = false;
                                                break;
                                            }else if is_white(board[(white_rook2 + square) as usize]) || is_black(board[(white_rook2 + square) as usize]) {
                                                break;
                                            }
                                        }
                                    }else if (desired_position - white_rook2)%8 == 0 {
                                        for square in 1..8 {
                                            if white_rook2 + square*8 == desired_position && !rook_down(white_rook2, square) {
                                                board[white_rook2 as usize] = NOTHING;
                                                board[desired_position as usize] = WHITE_ROOK;
                                                white_rook2 = desired_position;
        
                                                try_again = false;
                                                break;
                                            }else if is_white(board[(white_rook2 + square*8) as usize]) || is_black(board[(white_rook2 + square*8) as usize]) {
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                        }else{ // only one rook may reach the desired square
                            if white_rook1 > desired_position && !is_white(board[desired_position as usize]) {
                                // if the desired square is on the same rank as the initial position
                                if white_rook1 - desired_position <= 7 {
                                    for square in 1..8 {
                                        if white_rook1 - square == desired_position && !rook_left(white_rook1, square) {
                                            board[white_rook1 as usize] = NOTHING;
                                            board[desired_position as usize] = WHITE_ROOK;
                                            white_rook1 = desired_position;
    
                                            try_again = false;
                                            break;
                                        }else if is_white(board[(white_rook1 - square) as usize]) || is_black(board[(white_rook1 - square) as usize]) {
                                            break;
                                        }
                                    }
                                }else if (white_rook1 - desired_position)%8 == 0 {
                                    // otherwise, test if it is on the same file
                                    for square in 1..8 {
                                        if white_rook1 - square*8 == desired_position && !rook_up(white_rook1, square) {
                                            board[white_rook1 as usize] = NOTHING;
                                            board[desired_position as usize] = WHITE_ROOK;
                                            white_rook1 = desired_position;
    
                                            try_again = false;
                                            break;
                                        }else if is_white(board[(white_rook1 - square*8) as usize]) || is_black(board[(white_rook1 - square*8) as usize]) {
                                            break;
                                        }
                                    }
                                }
                            }else if white_rook1 < desired_position && !is_white(board[desired_position as usize]) {
                                if desired_position - white_rook1 <= 7 {
                                    for square in 1..8 {
                                        if white_rook1 + square == desired_position && !rook_right(white_rook1, square) {
                                            board[white_rook1 as usize] = NOTHING;
                                            board[desired_position as usize] = WHITE_ROOK;
                                            white_rook1 = desired_position;
    
                                            try_again = false;
                                            break;
                                        }else if is_white(board[(white_rook1 + square) as usize]) || is_black(board[(white_rook1 + square) as usize]) {
                                            break;
                                        }
                                    }
                                }else if (desired_position - white_rook1)%8 == 0 {
                                    for square in 1..8 {
                                        if white_rook1 + square*8 == desired_position && !rook_down(white_rook1, square) {
                                            board[white_rook1 as usize] = NOTHING;
                                            board[desired_position as usize] = WHITE_ROOK;
                                            white_rook1 = desired_position;
    
                                            try_again = false;
                                            break;
                                        }else if is_white(board[(white_rook1 + square*8) as usize]) || is_black(board[(white_rook1 + square*8) as usize]) {
                                            break;
                                        }
                                    }
                                }
                            }
                            if white_rook2 > desired_position && !is_white(board[desired_position as usize]) {
                                // if the desired square is on the same rank as the initial position
                                if white_rook2 - desired_position <= 7 {
                                    for square in 1..8 {
                                        if white_rook2 - square == desired_position && !rook_left(white_rook2, square) {
                                            board[white_rook2 as usize] = NOTHING;
                                            board[desired_position as usize] = WHITE_ROOK;
                                            white_rook2 = desired_position;
    
                                            try_again = false;
                                            break;
                                        }else if is_white(board[(white_rook2 - square) as usize]) || is_black(board[(white_rook2 - square) as usize]) {
                                            break;
                                        }
                                    }
                                }else if (white_rook2 - desired_position)%8 == 0 {
                                    // otherwise, test if it is on the same file
                                    for square in 1..8 {
                                        if white_rook2 - square*8 == desired_position && !rook_up(white_rook2, square) {
                                            board[white_rook2 as usize] = NOTHING;
                                            board[desired_position as usize] = WHITE_ROOK;
                                            white_rook2 = desired_position;
    
                                            try_again = false;
                                            break;
                                        }else if is_white(board[(white_rook2 - square*8) as usize]) || is_black(board[(white_rook2 - square*8) as usize]) {
                                            break;
                                        }
                                    }
                                }
                            }else if white_rook2 < desired_position && !is_white(board[desired_position as usize]) {
                                if desired_position - white_rook2 <= 7 {
                                    for square in 1..8 {
                                        if white_rook2 + square == desired_position && !rook_right(white_rook2, square) {
                                            board[white_rook2 as usize] = NOTHING;
                                            board[desired_position as usize] = WHITE_ROOK;
                                            white_rook2 = desired_position;
    
                                            try_again = false;
                                            break;
                                        }else if is_white(board[(white_rook2 + square) as usize]) || is_black(board[(white_rook2 + square) as usize]) {
                                            break;
                                        }
                                    }
                                }else if (desired_position - white_rook2)%8 == 0 {
                                    for square in 1..8 {
                                        if white_rook2 + square*8 == desired_position && !rook_down(white_rook2, square) {
                                            board[white_rook2 as usize] = NOTHING;
                                            board[desired_position as usize] = WHITE_ROOK;
                                            white_rook2 = desired_position;
    
                                            try_again = false;
                                            break;
                                        }else if is_white(board[(white_rook2 + square*8) as usize]) || is_black(board[(white_rook2 + square*8) as usize]) {
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    },
                    'K' => {
                        if !is_white(board[desired_position as usize]) {
                            match desired_position - white_king {
                                -9 => {
                                    if !upper_left_diagonal(white_king, 1) {
                                        board[white_king as usize] = NOTHING;
                                        board[desired_position as usize] = WHITE_KING;
                                        white_king = desired_position;
                                        try_again = false;
                                        }
                                    },
                                -8 => {
                                    if !rook_up(white_king, 1) {
                                        board[white_king as usize] = NOTHING;
                                        board[desired_position as usize] = WHITE_KING;
                                        white_king = desired_position;
                                        try_again = false;
                                        }
                                    },
                                -7 => {
                                    if !upper_right_diagonal(white_king, 1) {
                                        board[white_king as usize] = NOTHING;
                                        board[desired_position as usize] = WHITE_KING;
                                        white_king = desired_position;
                                        try_again = false;
                                        }
                                    },
                                -1 => {
                                    if !rook_left(white_king, 1) {
                                        board[white_king as usize] = NOTHING;
                                        board[desired_position as usize] = WHITE_KING;
                                        white_king = desired_position;
                                        try_again = false;
                                        }
                                    },
                                1 => {
                                    if !rook_right(white_king, 1) {
                                        board[white_king as usize] = NOTHING;
                                        board[desired_position as usize] = WHITE_KING;
                                        white_king = desired_position;
                                        try_again = false;
                                        }
                                    },
                                7 => {
                                    if !inferior_left_diagonal(white_king, 1) {
                                        board[white_king as usize] = NOTHING;
                                        board[desired_position as usize] = WHITE_KING;
                                        white_king = desired_position;
                                        try_again = false;
                                        }
                                    },
                                8 => {
                                    if !rook_down(white_king, 1) {
                                        board[white_king as usize] = NOTHING;
                                        board[desired_position as usize] = WHITE_KING;
                                        white_king = desired_position;
                                        try_again = false;
                                        }
                                    },
                                9 => {
                                    if !inferior_right_diagonal(white_king, 1) {
                                        board[white_king as usize] = NOTHING;
                                        board[desired_position as usize] = WHITE_KING;
                                        white_king = desired_position;
                                        try_again = false;
                                        }
                                    },
                                _ => ()
                            }
                        }
                    },
                    _ => ()
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

                    desired_position = column + line;

                    match san_move[0] {
                        'a' => {
                            if board[desired_position as usize] == NOTHING {
                                // for every pawn in the column
                                for pawn in &mut white_column_a {
                                    // if the pawn is in it's starting position
                                    if *pawn >= 48 {
                                        if desired_position - *pawn == -16 || desired_position - *pawn == -8 {
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
                            }
                        },
                        'b' => {
                            if board[desired_position as usize] == NOTHING {
                                // for every pawn in the column
                                for pawn in &mut white_column_b {
                                    // if the pawn is in it's starting position
                                    if *pawn >= 48 {
                                        if desired_position - *pawn == -16 || desired_position - *pawn == -8 {
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
                            }
                        },
                        'c' => {
                            if board[desired_position as usize] == NOTHING {
                                // for every pawn in the column
                                for pawn in &mut white_column_c {
                                    // if the pawn is in it's starting position
                                    if *pawn >= 48 {
                                        if desired_position - *pawn == -16 || desired_position - *pawn == -8 {
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
                            }
                        },
                        'd' => {
                            if board[desired_position as usize] == NOTHING {
                                // for every pawn in the column
                                for pawn in &mut white_column_d {
                                    // if the pawn is in it's starting position
                                    if *pawn >= 48 {
                                        if desired_position - *pawn == -16 || desired_position - *pawn == -8 {
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
                            }
                        },
                        'e' => {
                            if board[desired_position as usize] == NOTHING {
                                // for every pawn in the column
                                for pawn in &mut white_column_e {
                                    // if the pawn is in it's starting position
                                    if *pawn >= 48 {
                                        if desired_position - *pawn == -16 || desired_position - *pawn == -8 {
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
                            }
                        },
                        'f' => {
                            if board[desired_position as usize] == NOTHING {
                                // for every pawn in the column
                                for pawn in &mut white_column_f {
                                    // if the pawn is in it's starting position
                                    if *pawn >= 48 {
                                        if desired_position - *pawn == -16 || desired_position - *pawn == -8 {
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
                            }
                        },
                        'g' => {
                            if board[desired_position as usize] == NOTHING {
                                // for every pawn in the column
                                for pawn in &mut white_column_g {
                                    // if the pawn is in it's starting position
                                    if *pawn >= 48 {
                                        if desired_position - *pawn == -16 || desired_position - *pawn == -8 {
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
                            }
                        },
                        'h' => {
                            if board[desired_position as usize] == NOTHING {
                                // for every pawn in the column
                                for pawn in &mut white_column_h {
                                    // if the pawn is in it's starting position
                                    if *pawn >= 48 {
                                        if desired_position - *pawn == -16 || desired_position - *pawn == -8 {
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
                            }
                        },
                        _ => ()
                    }

                }else{ // if the second letter in the SAN notation move is 'x' (which means a capture):
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

                    desired_position = column + line;

                    match san_move[0] {
                        'a' => {
                            if !is_white(board[desired_position as usize]) {
                                let mut pawn_index: usize = 0;
                                // for every *pawn in the column
                                for pawn in &mut white_column_a {
                                    if desired_position - *pawn == -7 && !upper_right_diagonal(*pawn, 1) {
                                        board[*pawn as usize] = NOTHING;
                                        board[desired_position as usize] = WHITE_PAWN;
                                        *pawn = desired_position;

                                        white_column_b.insert(0, *pawn);
                                        
                                        try_again = false;
                                        break;
                                    }
                                    pawn_index += 1;
                                };
                                white_column_a.remove(pawn_index);
                            }
                        },
                        'b' => {
                            if !is_white(board[desired_position as usize]) {
                                let mut pawn_index: usize = 0;
                                // for every *pawn in the column
                                for pawn in &mut white_column_b {
                                    if (desired_position - *pawn == -7 && !upper_right_diagonal(*pawn, 1)) || (desired_position  - *pawn == -9 && !upper_left_diagonal(*pawn, 1)) {
                                        board[*pawn as usize] = NOTHING;
                                        board[desired_position as usize] = WHITE_PAWN;
                                        *pawn = desired_position;

                                        match san_move[2]{
                                            'a' => white_column_a.insert(0, *pawn),
                                            'c' => white_column_c.insert(0, *pawn),
                                            _ => ()
                                        }
                                        
                                        try_again = false;
                                        break;
                                    }
                                    pawn_index += 1;
                                };
                                white_column_b.remove(pawn_index);
                            }
                        },
                        'c' => {
                            if !is_white(board[desired_position as usize]) {
                                let mut pawn_index: usize = 0;
                                // for every pawn in the column
                                for pawn in &mut white_column_c {
                                    if (desired_position - *pawn == -7 && !upper_right_diagonal(*pawn, 1)) || (desired_position  - *pawn == -9 && !upper_left_diagonal(*pawn, 1)) {
                                        board[*pawn as usize] = NOTHING;
                                        board[desired_position as usize] = WHITE_PAWN;
                                        *pawn = desired_position;
                                        
                                        match san_move[2]{
                                            'b' => white_column_b.insert(0, *pawn),
                                            'd' => white_column_d.insert(0, *pawn),
                                            _ => ()
                                        }
                                        
                                        try_again = false;
                                        break;
                                    }
                                    pawn_index += 1;
                                };
                                white_column_c.remove(pawn_index);
                            }
                        },
                        'd' => {
                            if !is_white(board[desired_position as usize]) {
                                let mut pawn_index: usize = 0;
                                // for every *pawn in the column
                                for pawn in &mut white_column_d {
                                    if (desired_position - *pawn == -7 && !upper_right_diagonal(*pawn, 1)) || (desired_position - *pawn == -9 && !upper_left_diagonal(*pawn, 1)) {
                                        board[*pawn as usize] = NOTHING;
                                        board[desired_position as usize] = WHITE_PAWN;
                                        *pawn = desired_position;
                                        
                                        match san_move[2]{
                                            'c' => white_column_c.insert(0, *pawn),
                                            'e' => white_column_e.insert(0, *pawn),
                                            _ => ()
                                        }

                                        try_again = false;
                                        break;
                                    }
                                    pawn_index += 1;
                                };
                                white_column_d.remove(pawn_index);
                            }
                        },
                        'e' => {
                            if !is_white(board[desired_position as usize]) {
                                let mut pawn_index: usize = 0;
                                // for every *pawn in the column
                                for pawn in &mut white_column_e {
                                    if (desired_position - *pawn == -7 && !upper_right_diagonal(*pawn, 1)) || (desired_position - *pawn == -9 && !upper_left_diagonal(*pawn, 1)) {
                                        board[*pawn as usize] = NOTHING;
                                        board[desired_position as usize] = WHITE_PAWN;
                                        *pawn = desired_position;
                                        
                                        match san_move[2]{
                                            'd' => white_column_d.insert(0, *pawn),
                                            'f' => white_column_f.insert(0, *pawn),
                                            _ => ()
                                        }
                                        
                                        try_again = false;
                                        break;
                                    }
                                    pawn_index += 1;
                                };
                                white_column_e.remove(pawn_index);
                            }
                        },
                        'f' => {
                            if !is_white(board[desired_position as usize]) {
                                let mut pawn_index: usize = 0;
                                // for every *pawn in the column
                                for pawn in &mut white_column_f {
                                    if (desired_position - *pawn == -7 && !upper_right_diagonal(*pawn, 1)) || (desired_position  - *pawn == -9 && !upper_left_diagonal(*pawn, 1)) {
                                        board[*pawn as usize] = NOTHING;
                                        board[desired_position as usize] = WHITE_PAWN;
                                        *pawn = desired_position;
                                        
                                        match san_move[2]{
                                            'e' => white_column_e.insert(0, *pawn),
                                            'g' => white_column_g.insert(0, *pawn),
                                            _ => ()
                                        }
                                        
                                        try_again = false;
                                        break;
                                    }
                                    pawn_index += 1;
                                };
                                white_column_f.remove(pawn_index);
                            }
                        },
                        'g' => {
                            if !is_white(board[desired_position as usize]) {
                                let mut pawn_index: usize = 0;
                                // for every *pawn in the column
                                for pawn in &mut white_column_g {
                                    if (desired_position - *pawn == -7 && !upper_right_diagonal(*pawn, 1)) || (desired_position  - *pawn == -9 && !upper_left_diagonal(*pawn, 1)) {
                                        board[*pawn as usize] = NOTHING;
                                        board[desired_position as usize] = WHITE_PAWN;
                                        *pawn = desired_position;
                                        
                                        match san_move[2]{
                                            'f' => white_column_f.insert(0, *pawn),
                                            'h' => white_column_h.insert(0, *pawn),
                                            _ => ()
                                        }

                                        try_again = false;
                                        break;
                                    }
                                    pawn_index += 1;
                                };
                                white_column_g.remove(pawn_index);
                            }
                        },
                        'h' => {
                            if !is_white(board[desired_position as usize]) {
                                let mut pawn_index: usize = 0;
                                // for every *pawn in the column
                                for pawn in &mut white_column_h {
                                    if desired_position - *pawn == -9 && !upper_left_diagonal(*pawn, 1) {
                                        board[*pawn as usize] = NOTHING;
                                        board[desired_position as usize] = WHITE_PAWN;
                                        *pawn = desired_position;
                                        
                                        white_column_g.insert(0, *pawn);
                                        
                                        try_again = false;
                                        break;
                                    }
                                    pawn_index += 1;
                                };
                                white_column_h.remove(pawn_index);
                            }
                        },
                        _ => ()
                    }
                }

                if try_again == true{
                    println!("Not a possible move, try again!\n");
                }
            }
        }


        // BLACK'S TURN:
        try_again = true;
        show_board(board); //print the board

        while try_again{
            player_move.clear();

            println!("Black moves");

            io::stdin()
                .read_line(&mut player_move)
                .expect("Read error");

            san_move = player_move.trim().chars().collect();

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

                desired_position = column + line;
                if desired_position >= 100 {
                    println!("Not a possible move, try again!\n");
                    continue;
                }

                match san_move[0] {
                    'N' => {
                        if !is_black(board[desired_position as usize]) {
                            match desired_position - black_knight1 {
                                -17 | -15 | -10 | -6 | 6 | 10 | 15 | 17 => {
                                        //last position is freed
                                        board[black_knight1 as usize] = NOTHING;

                                        //piece is moved to new position
                                        board[desired_position as usize] = BLACK_KNIGHT;

                                        //current position is updated
                                        black_knight1 = desired_position;
                                        break;
                                },
                                _ => ()
                            }
                            match desired_position - black_knight2 {
                                -17 | -15 | -10 | -6 | 6 | 10 | 15 | 17 => {
                                        board[black_knight2 as usize] = NOTHING;
                                        board[desired_position as usize] = BLACK_KNIGHT;
                                        black_knight2 = desired_position;
                                        break;
                                },
                                _ => ()
                            }
                        }
                    },
                    'B' => {
                        // this tests the dark squares bishop
                        // if the desired square is "above" the initial position
                        if black_bishop1 > desired_position && !is_black(board[desired_position as usize]) {
                            // and if the distance is divisible by 7
                            if (black_bishop1 - desired_position)%7 == 0 {
                                for diagonal in 1..8 {
                                    // count each possible diagonal (until the maximum of 7 diagonals)
                                    if black_bishop1 - diagonal*7 == desired_position && !upper_right_diagonal(black_bishop1, diagonal) {
                                        // check if any of the squares in the bishop's diagonal is the desired square,
                                        // check if any of the calculated diagonals are forbidden (done using the 'upper_right_diagonal' function),
                                        // and finally, check if the desired square has no black pieces that may block the movement
                                        // if all of those checks are true, the bishop may be moved
                                        board[black_bishop1 as usize] = NOTHING;
                                        board[desired_position as usize] = BLACK_BISHOP;
                                        black_bishop1 = desired_position;

                                        try_again = false;
                                        break;
                                    }else if is_black(board[(black_bishop1 - diagonal*7) as usize]) || is_white(board[((black_bishop1 - diagonal*7) as usize)]) {
                                        // otherwise, if there are any black/black pieces on the way, the square is unreachable
                                        break;
                                    }
                                }
                            }else if (black_bishop1 - desired_position)%9 == 0 {
                                for diagonal in 1..8 {
                                    if black_bishop1 - diagonal*9 == desired_position && !upper_left_diagonal(black_bishop1, diagonal) {
                                        board[black_bishop1 as usize] = NOTHING;
                                        board[desired_position as usize] = BLACK_BISHOP;
                                        black_bishop1 = desired_position;

                                        try_again = false;
                                        break;
                                    }else if is_black(board[(black_bishop1 - diagonal*9) as usize]) || is_white(board[((black_bishop1 - diagonal*9) as usize)]){
                                        break;
                                    }
                                }
                            }
                        }else if black_bishop1 < desired_position  && (((desired_position - black_bishop1)%7 == 0) || ((desired_position - black_bishop1)%9 == 0)) {
                            if (black_bishop1 - desired_position)%7 == 0 {
                                for diagonal in 1..8 {
                                    if black_bishop1 + diagonal*7 == desired_position && !inferior_left_diagonal(black_bishop1, diagonal) {
                                        board[black_bishop1 as usize] = NOTHING;
                                        board[desired_position as usize] = BLACK_BISHOP;
                                        black_bishop1 = desired_position;

                                        try_again = false;
                                        break;
                                    }else if is_black(board[(black_bishop1 + 7*diagonal) as usize]) || is_white(board[(black_bishop1 + 7*diagonal) as usize]){
                                        break;
                                    }
                                }
                            }else if (black_bishop1 - desired_position)%9 == 0 {
                                for diagonal in 1..8 {
                                    if black_bishop1 + diagonal*9 == desired_position && !inferior_right_diagonal(black_bishop1, diagonal) {
                                        board[black_bishop1 as usize] = NOTHING;
                                        board[desired_position as usize] = BLACK_BISHOP;
                                        black_bishop1 = desired_position;

                                        try_again = false;
                                        break;
                                    }else if is_black(board[(black_bishop1 + 9*diagonal) as usize]) || is_white(board[(black_bishop1 + 9*diagonal) as usize]) {
                                        break;
                                    }
                                }
                            }
                        }
                        // this tests the light squares bishop
                        if black_bishop2 > desired_position && !is_black(board[desired_position as usize]) {
                            if (black_bishop2 - desired_position)%7 == 0 {
                                for diagonal in 1..8 {
                                    if black_bishop2 - diagonal*7 == desired_position && !upper_right_diagonal(black_bishop2, diagonal) {
                                        board[black_bishop2 as usize] = NOTHING;
                                        board[desired_position as usize] = BLACK_BISHOP;
                                        black_bishop2 = desired_position;

                                        try_again = false;
                                        break;
                                    }else if is_black(board[(black_bishop2 - diagonal*7) as usize]) || is_white(board[(black_bishop2 - diagonal*7) as usize]){
                                        break;
                                    }
                                }
                            }else if (black_bishop2 - desired_position)%9 == 0 {
                                for diagonal in 1..8 {
                                    if black_bishop2 - diagonal*9 == desired_position && !upper_left_diagonal(black_bishop2, diagonal) {
                                        board[black_bishop2 as usize] = NOTHING;
                                        board[desired_position as usize] = BLACK_BISHOP;
                                        black_bishop2 = desired_position;

                                        try_again = false;
                                        break;
                                    }else if is_black(board[(black_bishop2 - diagonal*9) as usize]) || is_white(board[(black_bishop2 - diagonal*9) as usize]){
                                        break;
                                    }
                                }
                            }
                        }else if black_bishop2 < desired_position && (((desired_position - black_bishop2)%7 == 0) || ((desired_position - black_bishop2)%9 == 0)) {
                            if (black_bishop2 - desired_position)%7 == 0 {
                                for diagonal in 1..8 {
                                    if black_bishop2 + diagonal*7 == desired_position && !inferior_left_diagonal(black_bishop2, diagonal) {
                                        board[black_bishop2 as usize] = NOTHING;
                                        board[desired_position as usize] = BLACK_BISHOP;
                                        black_bishop2 = desired_position;

                                        try_again = false;
                                        break;
                                    }else if is_black(board[(black_bishop2 + 7*diagonal) as usize]) || is_white(board[(black_bishop2 + 7*diagonal) as usize]){
                                        break;
                                    }
                                }
                            }else if (black_bishop2 - desired_position)%9 == 0 {
                                for diagonal in 1..8 {
                                    if black_bishop2 + diagonal*9 == desired_position && !inferior_right_diagonal(black_bishop2, diagonal) {
                                        board[black_bishop2 as usize] = NOTHING;
                                        board[desired_position as usize] = BLACK_BISHOP;
                                        black_bishop2 = desired_position;

                                        try_again = false;
                                        break;
                                    }else if is_black(board[(black_bishop2 + 9*diagonal) as usize]) || is_white(board[(black_bishop2 + 9*diagonal) as usize]) {
                                        break;
                                    }
                                }
                            }
                        }
                    },
                    'R' => {
                        // check if both rooks can reach the desired square
                            // if the rooks are in the same column
                        if ((black_rook1 - desired_position)%8 == 0 && (black_rook2 - desired_position)%8 == 0)
                            // or in the same line 
                        || ((black_rook1 - desired_position) <= 7 && (black_rook1 - desired_position) >= 1 && (black_rook2 - desired_position) >= -7 && (black_rook2 - desired_position) <= -1)
                        || ((black_rook1 - desired_position) >= -7 && (black_rook1 - desired_position) <= -1 && (black_rook2 - desired_position) <= 7 && (black_rook2 - desired_position) >= 1)
                            // or in different columns, but both are still able to reach the square
                        || ((black_rook1 - desired_position)%8 == 0 && (black_rook2 - desired_position) <= 7 && (black_rook2 - desired_position) >= -7) 
                        || ((black_rook1 - desired_position) <= 7 && (black_rook1 - desired_position) >= -7 && (black_rook2 - desired_position)%8 == 0) {
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
                                
                            //must be in reverse because we view the board as black
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
                            
                            if rook_column + rook_line == black_rook1 {
                                if black_rook1 > desired_position && !is_black(board[desired_position as usize]) {
                                    // if the desired square is on the same rank as the initial position
                                    if black_rook1 - desired_position <= 7 {
                                        for square in 1..8 {
                                            if black_rook1 - square == desired_position && !rook_left(black_rook1, square) {
                                                board[black_rook1 as usize] = NOTHING;
                                                board[desired_position as usize] = BLACK_ROOK;
                                                black_rook1 = desired_position;
        
                                                try_again = false;
                                                break;
                                            }else if is_black(board[(black_rook1 - square) as usize]) || is_black(board[(black_rook1 - square) as usize]) {
                                                break;
                                            }
                                        }
                                    }else if (black_rook1 - desired_position)%8 == 0 {
                                        // otherwise, test if it is on the same file
                                        for square in 1..8 {
                                            if black_rook1 - square*8 == desired_position && !rook_up(black_rook1, square) {
                                                board[black_rook1 as usize] = NOTHING;
                                                board[desired_position as usize] = BLACK_ROOK;
                                                black_rook1 = desired_position;
        
                                                try_again = false;
                                                break;
                                            }else if is_black(board[(black_rook1 - square*8) as usize]) || is_black(board[(black_rook1 - square*8) as usize]) {
                                                break;
                                            }
                                        }
                                    }
                                }else if black_rook1 < desired_position && !is_black(board[desired_position as usize]) {
                                    if desired_position - black_rook1 <= 7 {
                                        for square in 1..8 {
                                            if black_rook1 + square == desired_position && !rook_right(black_rook1, square) {
                                                board[black_rook1 as usize] = NOTHING;
                                                board[desired_position as usize] = BLACK_ROOK;
                                                black_rook1 = desired_position;
        
                                                try_again = false;
                                                break;
                                            }else if is_black(board[(black_rook1 + square) as usize]) || is_black(board[(black_rook1 + square) as usize]) {
                                                break;
                                            }
                                        }
                                    }else if (desired_position - black_rook1)%8 == 0 {
                                        for square in 1..8 {
                                            if black_rook1 + square*8 == desired_position && !rook_down(black_rook1, square) {
                                                board[black_rook1 as usize] = NOTHING;
                                                board[desired_position as usize] = BLACK_ROOK;
                                                black_rook1 = desired_position;
        
                                                try_again = false;
                                                break;
                                            }else if is_black(board[(black_rook1 + square*8) as usize]) || is_black(board[(black_rook1 + square*8) as usize]) {
                                                break;
                                            }
                                        }
                                    }
                                }
                            }else if rook_column + rook_line == black_rook2 {
                                if black_rook2 > desired_position && !is_black(board[desired_position as usize]) {
                                    // if the desired square is on the same rank as the initial position
                                    if black_rook2 - desired_position <= 7 {
                                        for square in 1..8 {
                                            if black_rook2 - square == desired_position && !rook_left(black_rook2, square) {
                                                board[black_rook2 as usize] = NOTHING;
                                                board[desired_position as usize] = BLACK_ROOK;
                                                black_rook2 = desired_position;
        
                                                try_again = false;
                                                break;
                                            }else if is_black(board[(black_rook2 - square) as usize]) || is_black(board[(black_rook2 - square) as usize]) {
                                                break;
                                            }
                                        }
                                    }else if (black_rook2 - desired_position)%8 == 0 {
                                        // otherwise, test if it is on the same file
                                        for square in 1..8 {
                                            if black_rook2 - square*8 == desired_position && !rook_up(black_rook2, square) {
                                                board[black_rook2 as usize] = NOTHING;
                                                board[desired_position as usize] = BLACK_ROOK;
                                                black_rook2 = desired_position;
        
                                                try_again = false;
                                                break;
                                            }else if is_black(board[(black_rook2 - square*8) as usize]) || is_black(board[(black_rook2 - square*8) as usize]) {
                                                break;
                                            }
                                        }
                                    }
                                }else if black_rook2 < desired_position && !is_black(board[desired_position as usize]) {
                                    if desired_position - black_rook2 <= 7 {
                                        for square in 1..8 {
                                            if black_rook2 + square == desired_position && !rook_right(black_rook2, square) {
                                                board[black_rook2 as usize] = NOTHING;
                                                board[desired_position as usize] = BLACK_ROOK;
                                                black_rook2 = desired_position;
        
                                                try_again = false;
                                                break;
                                            }else if is_black(board[(black_rook2 + square) as usize]) || is_black(board[(black_rook2 + square) as usize]) {
                                                break;
                                            }
                                        }
                                    }else if (desired_position - black_rook2)%8 == 0 {
                                        for square in 1..8 {
                                            if black_rook2 + square*8 == desired_position && !rook_down(black_rook2, square) {
                                                board[black_rook2 as usize] = NOTHING;
                                                board[desired_position as usize] = BLACK_ROOK;
                                                black_rook2 = desired_position;
        
                                                try_again = false;
                                                break;
                                            }else if is_black(board[(black_rook2 + square*8) as usize]) || is_black(board[(black_rook2 + square*8) as usize]) {
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                        }else{ // only one rook may reach the desired square
                            if black_rook1 > desired_position && !is_black(board[desired_position as usize]) {
                                // if the desired square is on the same rank as the initial position
                                if black_rook1 - desired_position <= 7 {
                                    for square in 1..8 {
                                        if black_rook1 - square == desired_position && !rook_left(black_rook1, square) {
                                            board[black_rook1 as usize] = NOTHING;
                                            board[desired_position as usize] = BLACK_ROOK;
                                            black_rook1 = desired_position;
    
                                            try_again = false;
                                            break;
                                        }else if is_black(board[(black_rook1 - square) as usize]) || is_black(board[(black_rook1 - square) as usize]) {
                                            break;
                                        }
                                    }
                                }else if (black_rook1 - desired_position)%8 == 0 {
                                    // otherwise, test if it is on the same file
                                    for square in 1..8 {
                                        if black_rook1 - square*8 == desired_position && !rook_up(black_rook1, square) {
                                            board[black_rook1 as usize] = NOTHING;
                                            board[desired_position as usize] = BLACK_ROOK;
                                            black_rook1 = desired_position;
    
                                            try_again = false;
                                            break;
                                        }else if is_black(board[(black_rook1 - square*8) as usize]) || is_black(board[(black_rook1 - square*8) as usize]) {
                                            break;
                                        }
                                    }
                                }
                            }else if black_rook1 < desired_position && !is_black(board[desired_position as usize]) {
                                if desired_position - black_rook1 <= 7 {
                                    for square in 1..8 {
                                        if black_rook1 + square == desired_position && !rook_right(black_rook1, square) {
                                            board[black_rook1 as usize] = NOTHING;
                                            board[desired_position as usize] = BLACK_ROOK;
                                            black_rook1 = desired_position;
    
                                            try_again = false;
                                            break;
                                        }else if is_black(board[(black_rook1 + square) as usize]) || is_black(board[(black_rook1 + square) as usize]) {
                                            break;
                                        }
                                    }
                                }else if (desired_position - black_rook1)%8 == 0 {
                                    for square in 1..8 {
                                        if black_rook1 + square*8 == desired_position && !rook_down(black_rook1, square) {
                                            board[black_rook1 as usize] = NOTHING;
                                            board[desired_position as usize] = BLACK_ROOK;
                                            black_rook1 = desired_position;
    
                                            try_again = false;
                                            break;
                                        }else if is_black(board[(black_rook1 + square*8) as usize]) || is_black(board[(black_rook1 + square*8) as usize]) {
                                            break;
                                        }
                                    }
                                }
                            }
                            if black_rook2 > desired_position && !is_black(board[desired_position as usize]) {
                                // if the desired square is on the same rank as the initial position
                                if black_rook2 - desired_position <= 7 {
                                    for square in 1..8 {
                                        if black_rook2 - square == desired_position && !rook_left(black_rook2, square) {
                                            board[black_rook2 as usize] = NOTHING;
                                            board[desired_position as usize] = BLACK_ROOK;
                                            black_rook2 = desired_position;
    
                                            try_again = false;
                                            break;
                                        }else if is_black(board[(black_rook2 - square) as usize]) || is_black(board[(black_rook2 - square) as usize]) {
                                            break;
                                        }
                                    }
                                }else if (black_rook2 - desired_position)%8 == 0 {
                                    // otherwise, test if it is on the same file
                                    for square in 1..8 {
                                        if black_rook2 - square*8 == desired_position && !rook_up(black_rook2, square) {
                                            board[black_rook2 as usize] = NOTHING;
                                            board[desired_position as usize] = BLACK_ROOK;
                                            black_rook2 = desired_position;
    
                                            try_again = false;
                                            break;
                                        }else if is_black(board[(black_rook2 - square*8) as usize]) || is_black(board[(black_rook2 - square*8) as usize]) {
                                            break;
                                        }
                                    }
                                }
                            }else if black_rook2 < desired_position && !is_black(board[desired_position as usize]) {
                                if desired_position - black_rook2 <= 7 {
                                    for square in 1..8 {
                                        if black_rook2 + square == desired_position && !rook_right(black_rook2, square) {
                                            board[black_rook2 as usize] = NOTHING;
                                            board[desired_position as usize] = BLACK_ROOK;
                                            black_rook2 = desired_position;
    
                                            try_again = false;
                                            break;
                                        }else if is_black(board[(black_rook2 + square) as usize]) || is_black(board[(black_rook2 + square) as usize]) {
                                            break;
                                        }
                                    }
                                }else if (desired_position - black_rook2)%8 == 0 {
                                    for square in 1..8 {
                                        if black_rook2 + square*8 == desired_position && !rook_down(black_rook2, square) {
                                            board[black_rook2 as usize] = NOTHING;
                                            board[desired_position as usize] = BLACK_ROOK;
                                            black_rook2 = desired_position;
    
                                            try_again = false;
                                            break;
                                        }else if is_black(board[(black_rook2 + square*8) as usize]) || is_black(board[(black_rook2 + square*8) as usize]) {
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    },
                    'K' => {
                        if !is_black(board[desired_position as usize]) {
                            match desired_position - black_king {
                                -9 => {
                                    if !upper_left_diagonal(black_king, 1) {
                                        board[black_king as usize] = NOTHING;
                                        board[desired_position as usize] = BLACK_KING;
                                        black_king = desired_position;
                                        try_again = false;
                                        }
                                    },
                                -8 => {
                                    if !rook_up(black_king, 1) {
                                        board[black_king as usize] = NOTHING;
                                        board[desired_position as usize] = BLACK_KING;
                                        black_king = desired_position;
                                        try_again = false;
                                        }
                                    },
                                -7 => {
                                    if !upper_right_diagonal(black_king, 1) {
                                        board[black_king as usize] = NOTHING;
                                        board[desired_position as usize] = BLACK_KING;
                                        black_king = desired_position;
                                        try_again = false;
                                        }
                                    },
                                -1 => {
                                    if !rook_left(black_king, 1) {
                                        board[black_king as usize] = NOTHING;
                                        board[desired_position as usize] = BLACK_KING;
                                        black_king = desired_position;
                                        try_again = false;
                                        }
                                    },
                                1 => {
                                    if !rook_right(black_king, 1) {
                                        board[black_king as usize] = NOTHING;
                                        board[desired_position as usize] = BLACK_KING;
                                        black_king = desired_position;
                                        try_again = false;
                                        }
                                    },
                                7 => {
                                    if !inferior_left_diagonal(black_king, 1) {
                                        board[black_king as usize] = NOTHING;
                                        board[desired_position as usize] = BLACK_KING;
                                        black_king = desired_position;
                                        try_again = false;
                                        }
                                    },
                                8 => {
                                    if !rook_down(black_king, 1) {
                                        board[black_king as usize] = NOTHING;
                                        board[desired_position as usize] = BLACK_KING;
                                        black_king = desired_position;
                                        try_again = false;
                                        }
                                    },
                                9 => {
                                    if !inferior_right_diagonal(black_king, 1) {
                                        board[black_king as usize] = NOTHING;
                                        board[desired_position as usize] = BLACK_KING;
                                        black_king = desired_position;
                                        try_again = false;
                                        }
                                    },
                                _ => ()
                            }
                        }
                    },
                    _ => ()
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

                    desired_position = column + line;

                    match san_move[0] {
                        'a' => {
                            if board[desired_position as usize] == NOTHING {
                                // for every pawn in the column
                                for pawn in &mut black_column_a {
                                    // if the pawn is in it's starting position
                                    if *pawn <= 15 {
                                        if desired_position - *pawn == 16 || desired_position - *pawn == 8 {
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
                            }
                        },
                        'b' => {
                            if board[desired_position as usize] == NOTHING {
                                // for every pawn in the column
                                for pawn in &mut black_column_b {
                                    // if the pawn is in it's starting position
                                    if *pawn <= 15 {
                                        if desired_position - *pawn == 16 || desired_position - *pawn == 8 {
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
                            }
                        },
                        'c' => {
                            if board[desired_position as usize] == NOTHING {
                                // for every pawn in the column
                                for pawn in &mut black_column_c {
                                    // if the pawn is in it's starting position
                                    if *pawn <= 15 {
                                        if desired_position - *pawn == 16 || desired_position - *pawn == 8 {
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
                            }
                        },
                        'd' => {
                            if board[desired_position as usize] == NOTHING {
                                // for every pawn in the column
                                for pawn in &mut black_column_d {
                                    // if the pawn is in it's starting position
                                    if *pawn <= 15 {
                                        if desired_position - *pawn == 16 || desired_position - *pawn == 8 {
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
                            }
                        },
                        'e' => {
                            if board[desired_position as usize] == NOTHING {
                                // for every pawn in the column
                                for pawn in &mut black_column_e {
                                    // if the pawn is in it's starting position
                                    if *pawn <= 15 {
                                        if desired_position - *pawn == 16 || desired_position - *pawn == 8 {
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
                            }
                        },
                        'f' => {
                            if board[desired_position as usize] == NOTHING {
                                // for every pawn in the column
                                for pawn in &mut black_column_f {
                                    // if the pawn is in it's starting position
                                    if *pawn <= 15 {
                                        if desired_position - *pawn == 16 || desired_position - *pawn == 8 {
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
                            }
                        },
                        'g' => {
                            if board[desired_position as usize] == NOTHING {
                                // for every pawn in the column
                                for pawn in &mut black_column_g {
                                    // if the pawn is in it's starting position
                                    if *pawn <= 15 {
                                        if desired_position - *pawn == 16 || desired_position - *pawn == 8 {
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
                            }
                        },
                        'h' => {
                            if board[desired_position as usize] == NOTHING {
                                // for every pawn in the column
                                for pawn in &mut black_column_h {
                                    // if the pawn is in it's starting position
                                    if *pawn <= 15 {
                                        if desired_position - *pawn == 16 || desired_position - *pawn == 8 {
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
                            }
                        },
                        _ => ()
                    }

                }else{
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

                    desired_position = column + line;

                    match san_move[0] {
                        'a' => {
                            if !is_black(board[desired_position as usize]) {
                                let mut pawn_index: usize = 0;
                                // for every *pawn in the column
                                for pawn in &mut black_column_a {
                                    if desired_position - *pawn == 9 && !inferior_right_diagonal(*pawn, 1) {
                                        board[*pawn as usize] = NOTHING;
                                        board[desired_position as usize] = BLACK_PAWN;
                                        *pawn = desired_position;

                                        black_column_b.insert(0, *pawn);
                                        
                                        try_again = false;
                                        break;
                                    }
                                    pawn_index += 1;
                                };
                                black_column_a.remove(pawn_index);
                            }
                        },
                        'b' => {
                            if !is_black(board[desired_position as usize]) {
                                let mut pawn_index: usize = 0;
                                // for every *pawn in the column
                                for pawn in &mut black_column_b {
                                    if (desired_position - *pawn == 9 && !inferior_right_diagonal(*pawn, 1)) || (desired_position - *pawn == 7 && !inferior_left_diagonal(*pawn, 1)) {
                                        board[*pawn as usize] = NOTHING;
                                        board[desired_position as usize] = BLACK_PAWN;
                                        *pawn = desired_position;

                                        match san_move[2]{
                                            'a' => black_column_a.insert(0, *pawn),
                                            'c' => black_column_c.insert(0, *pawn),
                                            _ => ()
                                        }
                                        
                                        try_again = false;
                                        break;
                                    }
                                    pawn_index += 1;
                                };
                                black_column_b.remove(pawn_index);
                            }
                        },
                        'c' => {
                            if !is_black(board[desired_position as usize]) {
                                let mut pawn_index: usize = 0;
                                // for every pawn in the column
                                for pawn in &mut black_column_c {
                                    if (desired_position - *pawn == 9 && !inferior_right_diagonal(*pawn, 1)) || (desired_position - *pawn == 7 && !inferior_left_diagonal(*pawn, 1)) {
                                        board[*pawn as usize] = NOTHING;
                                        board[desired_position as usize] = BLACK_PAWN;
                                        *pawn = desired_position;
                                        
                                        match san_move[2]{
                                            'b' => black_column_b.insert(0, *pawn),
                                            'd' => black_column_d.insert(0, *pawn),
                                            _ => ()
                                        }
                                        
                                        try_again = false;
                                        break;
                                    }
                                    pawn_index += 1;
                                };
                                black_column_c.remove(pawn_index);
                            }
                        },
                        'd' => {
                            if !is_black(board[desired_position as usize]) {
                                let mut pawn_index: usize = 0;
                                // for every *pawn in the column
                                for pawn in &mut black_column_d {
                                    if (desired_position - *pawn == 9 && !inferior_right_diagonal(*pawn, 1)) || (desired_position - *pawn == 7 && !inferior_left_diagonal(*pawn, 1)) {
                                        board[*pawn as usize] = NOTHING;
                                        board[desired_position as usize] = BLACK_PAWN;
                                        *pawn = desired_position;
                                        
                                        match san_move[2]{
                                            'c' => black_column_c.insert(0, *pawn),
                                            'e' => black_column_e.insert(0, *pawn),
                                            _ => ()
                                        }

                                        try_again = false;
                                        break;
                                    }
                                    pawn_index += 1;
                                };
                                black_column_d.remove(pawn_index);
                            }
                        },
                        'e' => {
                            if !is_black(board[desired_position as usize]) {
                                let mut pawn_index: usize = 0;
                                // for every *pawn in the column
                                for pawn in &mut black_column_e {
                                    if (desired_position - *pawn == 9 && !inferior_right_diagonal(*pawn, 1)) || (desired_position - *pawn == 7 && !inferior_left_diagonal(*pawn, 1)) {
                                        board[*pawn as usize] = NOTHING;
                                        board[desired_position as usize] = BLACK_PAWN;
                                        *pawn = desired_position;
                                        
                                        match san_move[2]{
                                            'd' => black_column_d.insert(0, *pawn),
                                            'f' => black_column_f.insert(0, *pawn),
                                            _ => ()
                                        }
                                        
                                        try_again = false;
                                        break;
                                    }
                                    pawn_index += 1;
                                };
                                black_column_e.remove(pawn_index);
                            }
                        },
                        'f' => {
                            if !is_black(board[desired_position as usize]) {
                                let mut pawn_index: usize = 0;
                                // for every *pawn in the column
                                for pawn in &mut black_column_f {
                                    if (desired_position - *pawn == 9 && !inferior_right_diagonal(*pawn, 1)) || (desired_position - *pawn == 7 && !inferior_left_diagonal(*pawn, 1)) {
                                        board[*pawn as usize] = NOTHING;
                                        board[desired_position as usize] = BLACK_PAWN;
                                        *pawn = desired_position;
                                        
                                        match san_move[2]{
                                            'e' => black_column_e.insert(0, *pawn),
                                            'g' => black_column_g.insert(0, *pawn),
                                            _ => ()
                                        }
                                        
                                        try_again = false;
                                        break;
                                    }
                                    pawn_index += 1;
                                };
                                black_column_f.remove(pawn_index);
                            }
                        },
                        'g' => {
                            if !is_black(board[desired_position as usize]) {
                                let mut pawn_index: usize = 0;
                                // for every *pawn in the column
                                for pawn in &mut black_column_g {
                                    if (desired_position - *pawn == 9 && !inferior_right_diagonal(*pawn, 1)) || (desired_position - *pawn == 7 && !inferior_left_diagonal(*pawn, 1)) {
                                        board[*pawn as usize] = NOTHING;
                                        board[desired_position as usize] = BLACK_PAWN;
                                        *pawn = desired_position;
                                        
                                        match san_move[2]{
                                            'f' => black_column_f.insert(0, *pawn),
                                            'h' => black_column_h.insert(0, *pawn),
                                            _ => ()
                                        }

                                        try_again = false;
                                        break;
                                    }
                                    pawn_index += 1;
                                };
                                black_column_g.remove(pawn_index);
                            }
                        },
                        'h' => {
                            if !is_black(board[desired_position as usize]) {
                                let mut pawn_index: usize = 0;
                                // for every *pawn in the column
                                for pawn in &mut black_column_h {
                                    if desired_position - *pawn == 7 && !inferior_left_diagonal(*pawn, 1) {
                                        board[*pawn as usize] = NOTHING;
                                        board[desired_position as usize] = BLACK_PAWN;
                                        *pawn = desired_position;
                                        
                                        black_column_g.insert(0, *pawn);
                                        
                                        try_again = false;
                                        break;
                                    }
                                    pawn_index += 1;
                                };
                                black_column_h.remove(pawn_index);
                            }
                        },
                        _ => ()
                    }
                }
            }

            if try_again == true{
                println!("Not a possible move, try again!\n");
            }
        }
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

    match b  - i*7 {
        // a bishop cannot trace a path after it hits the board's "walls" or corners
        56 |
        48 |
        40 |
        32 |
        24 |
        16 |
        8 |
        0 |
        // positions where subtracting to the index would exceed the array:
        -1 |
        -2 |
        -3 |
        -4 |
        -5 |
        -6 |
        -7 => true,
        _ => false
    }
}

fn upper_left_diagonal(b: i8, i: i8) -> bool {
    match b - i*9 {
        // a bishop cannot trace a path after it hits the board's "walls" or corners
        47 |
        39 |
        31 |
        23 |
        15 |
        7 |
        // positions where subtracting to the index would exceed the array:
        -1 |
        -2 |
        -3 |
        -4 |
        -5 |
        -6 |
        -7 |
        -8 |
        -9 => true,
        _ => false
    }
}

fn inferior_right_diagonal(b: i8, i: i8) -> bool {
    match b + i*9 {
        // a bishop cannot trace a path after it hits the board's "walls" or corners
        16 |
        24 |
        32 |
        40 |
        48 |
        56 |
        // positions where adding to the index would exceed the array:
        64 |
        65 |
        66 |
        67 |
        68 |
        69 |
        50 |
        51 |
        52 => true,
        _ => false
    }
}

fn inferior_left_diagonal(b: i8, i: i8) -> bool {
    match b + i*7 {
        // a bishop cannot trace a path after it hits the board's "walls" or corners
        7 |
        15 |
        23 |
        31 |
        39 |
        47 |
        55 |
        // positions where adding to the index would exceed the array:
        64 |
        65 |
        66 |
        67 |
        68 |
        69 |
        70 => true,
        _ => false
    }
}


// rook-like movement checks
fn rook_right(b: i8, i: i8) -> bool {
    match b + i {
        8 |
        16 |
        24 |
        32 |
        40 |
        48 |
        56 |
        64 => true,
        _ => false
    }
}

fn rook_left(b: i8, i: i8) -> bool {
    match b - i {
        55 |
        47 |
        39 |
        31 |
        23 |
        15 |
        7 |
        -1 => true,
        _ => false
    }
}

fn rook_down(b: i8, i: i8) -> bool {
    match b + i*8 {
        64 |
        65 |
        66 |
        67 |
        68 |
        69 |
        70 |
        71 => true,
        _ => false
    }
}

fn rook_up(b: i8, i: i8) -> bool {
    match b - i*8 {
        -1 |
        -2 |
        -3 |
        -4 |
        -5 |
        -6 |
        -7 |
        -8 => true,
        _ => false
    }
}
