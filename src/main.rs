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

    let mut white_column_a = vec![48];
    let mut white_column_b = vec![49];
    let mut white_column_c = vec![50];
    let mut white_column_d = vec![51];
    let mut white_column_e = vec![52];
    let mut white_column_f = vec![53];
    let mut white_column_g = vec![54];
    let mut white_column_h = vec![55];

    //black pawns
    
    let mut black_column_a = vec![8];
    let mut black_column_b = vec![9];
    let mut black_column_c = vec![10];
    let mut black_column_d = vec![11];
    let mut black_column_e = vec![12];
    let mut black_column_f = vec![13];
    let mut black_column_g = vec![14];
    let mut black_column_h = vec![15];

    //white pieces

    let mut white_rooks: Vec<i8> = vec![56, 63];
    
    let mut white_knights = vec![57, 62];
    
    let mut white_bishops = vec![58, 61];
    
    let mut white_queens = vec![59];

    let mut white_king: i8 = 60;
    
    //black pieces

    let mut black_rooks: Vec<i8> = vec![0, 7];
    
    let mut black_knights = vec![1, 6];
    
    let mut black_bishops = vec![2, 5];
    
    let mut black_queens = vec![3];

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
                                
                                for w_bishop in white_bishops.iter_mut() {
                                    if bishop_line + bishop_column == *w_bishop {
                                            // if the desired square is "above" the initial position
                                        if *w_bishop > desired_position {
                                            // and if the distance is divisible by 7
                                            if (*w_bishop - desired_position)%7 == 0 {
                                                for diagonal in 1..8 {
                                                    // count each possible diagonal (until the maximum of 7 diagonals)
                                                    if *w_bishop - diagonal*7 == desired_position && !upper_right_diagonal(*w_bishop, diagonal) {
                                                        // check if any of the squares in the bishop's diagonal is the desired square,
                                                        // check if any of the calculated diagonals are forbidden (done using the 'upper_right_diagonal' function),
                                                        // and finally, check if the desired square has no white pieces that may block the movement
                                                        // if all of those checks are true, the bishop may be moved
                                                        board[*w_bishop as usize] = NOTHING;
                                                        board[desired_position as usize] = WHITE_BISHOP;
                                                        *w_bishop = desired_position;
    
                                                        try_again = false;
                                                        break;
                                                    }else if is_white(board[(*w_bishop - diagonal*7) as usize]) || is_black(board[((*w_bishop - diagonal*7) as usize)]) {
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
                                                    }else if is_white(board[(*w_bishop - diagonal*9) as usize]) || is_black(board[((*w_bishop - diagonal*9) as usize)]){
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
                                                    }else if is_white(board[(*w_bishop + 7*diagonal) as usize]) || is_black(board[(*w_bishop + 7*diagonal) as usize]){
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
                                                    }else if is_white(board[(*w_bishop + 9*diagonal) as usize]) || is_black(board[(*w_bishop + 9*diagonal) as usize]) {
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                };
                            }else{
                                for w_bishop in white_bishops.iter_mut() {
                                        // if the desired square is "above" the initial position
                                    if *w_bishop > desired_position {
                                        // and if the distance is divisible by 7
                                        if (*w_bishop - desired_position)%7 == 0 {
                                            for diagonal in 1..8 {
                                                // count each possible diagonal (until the maximum of 7 diagonals)
                                                if *w_bishop - diagonal*7 == desired_position && !upper_right_diagonal(*w_bishop, diagonal) {
                                                    // check if any of the squares in the bishop's diagonal is the desired square,
                                                    // check if any of the calculated diagonals are forbidden (done using the 'upper_right_diagonal' function),
                                                    // and finally, check if the desired square has no white pieces that may block the movement
                                                    // if all of those checks are true, the bishop may be moved
                                                    board[*w_bishop as usize] = NOTHING;
                                                    board[desired_position as usize] = WHITE_BISHOP;
                                                    *w_bishop = desired_position;

                                                    try_again = false;
                                                    break;
                                                }else if is_white(board[(*w_bishop - diagonal*7) as usize]) || is_black(board[((*w_bishop - diagonal*7) as usize)]) {
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
                                                }else if is_white(board[(*w_bishop - diagonal*9) as usize]) || is_black(board[((*w_bishop - diagonal*9) as usize)]){
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
                                                }else if is_white(board[(*w_bishop + 7*diagonal) as usize]) || is_black(board[(*w_bishop + 7*diagonal) as usize]){
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
                                                }else if is_white(board[(*w_bishop + 9*diagonal) as usize]) || is_black(board[(*w_bishop + 9*diagonal) as usize]) {
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
                                                        *w_rook = desired_position;
                
                                                        try_again = false;
                                                        break;
                                                    }else if is_white(board[(*w_rook - square) as usize]) || is_black(board[(*w_rook - square) as usize]) {
                                                        break;
                                                    }
                                                }
                                            }else if (*w_rook - desired_position)%8 == 0 {
                                                // otherwise, test if it is on the same file
                                                for square in 1..8 {
                                                    if *w_rook - square*8 == desired_position && !rook_up(*w_rook, square) {
                                                        board[*w_rook as usize] = NOTHING;
                                                        board[desired_position as usize] = WHITE_ROOK;
                                                        *w_rook = desired_position;
                
                                                        try_again = false;
                                                        break;
                                                    }else if is_white(board[(*w_rook - square*8) as usize]) || is_black(board[(*w_rook - square*8) as usize]) {
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
                                                        *w_rook = desired_position;
                
                                                        try_again = false;
                                                        break;
                                                    }else if is_white(board[(*w_rook + square) as usize]) || is_black(board[(*w_rook + square) as usize]) {
                                                        break;
                                                    }
                                                }
                                            }else if (desired_position - *w_rook)%8 == 0 {
                                                for square in 1..8 {
                                                    if *w_rook + square*8 == desired_position && !rook_down(*w_rook, square) {
                                                        board[*w_rook as usize] = NOTHING;
                                                        board[desired_position as usize] = WHITE_ROOK;
                                                        *w_rook = desired_position;
                
                                                        try_again = false;
                                                        break;
                                                    }else if is_white(board[(*w_rook + square*8) as usize]) || is_black(board[(*w_rook + square*8) as usize]) {
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
                                        // if the desired square is on the same rank as the initial position
                                        if get_line(*w_rook) == get_line(desired_position) {
                                            for square in 1..8 {
                                                if *w_rook - square == desired_position && !rook_left(*w_rook, square) {
                                                    board[*w_rook as usize] = NOTHING;
                                                    board[desired_position as usize] = WHITE_ROOK;
                                                    *w_rook = desired_position;
            
                                                    try_again = false;
                                                    break;
                                                }else if is_white(board[(*w_rook - square) as usize]) || is_black(board[(*w_rook - square) as usize]) {
                                                    break;
                                                }
                                            }
                                        }else if (*w_rook - desired_position)%8 == 0 {
                                            // otherwise, test if it is on the same file
                                            for square in 1..8 {
                                                if *w_rook - square*8 == desired_position && !rook_up(*w_rook, square) {
                                                    board[*w_rook as usize] = NOTHING;
                                                    board[desired_position as usize] = WHITE_ROOK;
                                                    *w_rook = desired_position;
            
                                                    try_again = false;
                                                    break;
                                                }else if is_white(board[(*w_rook - square*8) as usize]) || is_black(board[(*w_rook - square*8) as usize]) {
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
                                                    *w_rook = desired_position;
            
                                                    try_again = false;
                                                    break;
                                                }else if is_white(board[(*w_rook + square) as usize]) || is_black(board[(*w_rook + square) as usize]) {
                                                    break;
                                                }
                                            }
                                        }else if (desired_position - *w_rook)%8 == 0 {
                                            for square in 1..8 {
                                                if *w_rook + square*8 == desired_position && !rook_down(*w_rook, square) {
                                                    board[*w_rook as usize] = NOTHING;
                                                    board[desired_position as usize] = WHITE_ROOK;
                                                    *w_rook = desired_position;
            
                                                    try_again = false;
                                                    break;
                                                }else if is_white(board[(*w_rook + square*8) as usize]) || is_black(board[(*w_rook + square*8) as usize]) {
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
                        },
                        'Q' => {
                            if test_multiple_rooks(&mut white_queens, desired_position) == true 
                            || test_multiple_bishops(&mut white_queens, desired_position) == true {
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
                                                // otherwise, test if it is on the same file
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
                                            // otherwise, test if it is on the same file
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

                    if board[desired_position as usize] == NOTHING {

                        match san_move[0] {
                            'a' => {
                                // for every pawn in the column
                                for pawn in &mut white_column_a.iter_mut() {
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
                            },
                            'b' => {
                                // for every pawn in the column
                                for pawn in &mut white_column_b.iter_mut() {
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
                            },
                            'c' => {
                                // for every pawn in the column
                                for pawn in &mut white_column_c.iter_mut() {
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
                            },
                            'd' => {
                                // for every pawn in the column
                                for pawn in &mut white_column_d.iter_mut() {
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
                            },
                            'e' => {
                                // for every pawn in the column
                                for pawn in &mut white_column_e.iter_mut() {
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
                            },
                            'f' => {
                                // for every pawn in the column
                                for pawn in &mut white_column_f.iter_mut() {
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
                            },
                            'g' => {
                                // for every pawn in the column
                                for pawn in &mut white_column_g.iter_mut() {
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
                            },
                            'h' => {
                                // for every pawn in the column
                                for pawn in &mut white_column_h.iter_mut() {
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
                            },
                            _ => ()
                        };

                        if line == 0 && try_again == false{ // if the pawn moved to the other side of the board successfully
                            println!("Indicate the pawn promotion: (Press Enter to promote it to a Queen)");
                            println!("'N'=Knight,'R'=Rook,'B'=Bishop");
                            let mut promotion_move = String::new();
                            let mut prom_loop: bool = true;

                            while prom_loop {
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

                    if is_black(board[desired_position as usize]) {
                        let captured_piece = board[desired_position as usize];

                        match san_move[0] {
                            'a' => {
                                let mut pawn_index: usize = 0;
                                // for every *pawn in the column
                                for pawn in &mut white_column_a.iter_mut() {
                                    if desired_position - *pawn == -7 && !upper_right_diagonal(*pawn, 1) {
                                        board[*pawn as usize] = NOTHING;
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
                                    if (desired_position - *pawn == -7 && !upper_right_diagonal(*pawn, 1)) || (desired_position  - *pawn == -9 && !upper_left_diagonal(*pawn, 1)) {
                                        board[*pawn as usize] = NOTHING;
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
                                    if (desired_position - *pawn == -7 && !upper_right_diagonal(*pawn, 1)) || (desired_position  - *pawn == -9 && !upper_left_diagonal(*pawn, 1)) {
                                        board[*pawn as usize] = NOTHING;
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
                                    if (desired_position - *pawn == -7 && !upper_right_diagonal(*pawn, 1)) || (desired_position - *pawn == -9 && !upper_left_diagonal(*pawn, 1)) {
                                        board[*pawn as usize] = NOTHING;
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
                                    if (desired_position - *pawn == -7 && !upper_right_diagonal(*pawn, 1)) || (desired_position - *pawn == -9 && !upper_left_diagonal(*pawn, 1)) {
                                        board[*pawn as usize] = NOTHING;
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
                                    if (desired_position - *pawn == -7 && !upper_right_diagonal(*pawn, 1)) || (desired_position  - *pawn == -9 && !upper_left_diagonal(*pawn, 1)) {
                                        board[*pawn as usize] = NOTHING;
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
                                    if (desired_position - *pawn == -7 && !upper_right_diagonal(*pawn, 1)) || (desired_position  - *pawn == -9 && !upper_left_diagonal(*pawn, 1)) {
                                        board[*pawn as usize] = NOTHING;
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

                        if line == 0 && try_again == false{ // if the pawn moved to the other side of the board successfully
                            println!("Indicate the pawn promotion: (Press Enter to promote it to a Queen)");
                            println!("'N'=Knight,'R'=Rook,'B'=Bishop");
                            let mut promotion_move = String::new();
                            let mut prom_loop: bool = true;

                            while prom_loop {
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
                }
            }

            if try_again == true{
                println!("Not a possible move, try again!\n");
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

                if column >= 100 || line >= 100 {
                    println!("Not a possible move, try again!\n");
                    continue;
                }

                desired_position = column + line;

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

                                for b_knight in black_knights.iter_mut() {
                                    if knight_column + knight_line == *b_knight {
                                        match *b_knight - desired_position {
                                            -17 | -15 | -10 | -6 | 6 | 10 | 15 | 17 => {
                                                    //last position is freed
                                                    board[*b_knight as usize] = NOTHING;

                                                    //piece is moved to new position
                                                    board[desired_position as usize] = BLACK_KNIGHT;

                                                    //current position is updated
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
                                                //last position is freed
                                                board[*b_knight as usize] = NOTHING;

                                                //piece is moved to new position
                                                board[desired_position as usize] = BLACK_KNIGHT;

                                                //current position is updated
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
                                
                                for b_bishop in black_bishops.iter_mut() {
                                    if bishop_line + bishop_column == *b_bishop {
                                            // if the desired square is "above" the initial position
                                        if *b_bishop > desired_position {
                                            // and if the distance is divisible by 7
                                            if (*b_bishop - desired_position)%7 == 0 {
                                                for diagonal in 1..8 {
                                                    // count each possible diagonal (until the maximum of 7 diagonals)
                                                    if *b_bishop - diagonal*7 == desired_position && !upper_right_diagonal(*b_bishop, diagonal) {
                                                        // check if any of the squares in the bishop's diagonal is the desired square,
                                                        // check if any of the calculated diagonals are forbidden (done using the 'upper_right_diagonal' function),
                                                        // and finally, check if the desired square has no white pieces that may block the movement
                                                        // if all of those checks are true, the bishop may be moved
                                                        board[*b_bishop as usize] = NOTHING;
                                                        board[desired_position as usize] = BLACK_BISHOP;
                                                        *b_bishop = desired_position;
    
                                                        try_again = false;
                                                        break;
                                                    }else if is_white(board[(*b_bishop - diagonal*7) as usize]) || is_black(board[((*b_bishop - diagonal*7) as usize)]) {
                                                        // otherwise, if there are any white/black pieces on the way, the square is unreachable
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
                                                    }else if is_white(board[(*b_bishop - diagonal*9) as usize]) || is_black(board[((*b_bishop - diagonal*9) as usize)]){
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
                                                    }else if is_white(board[(*b_bishop + 7*diagonal) as usize]) || is_black(board[(*b_bishop + 7*diagonal) as usize]){
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
                                                    }else if is_white(board[(*b_bishop + 9*diagonal) as usize]) || is_black(board[(*b_bishop + 9*diagonal) as usize]) {
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                };
                            }else{
                                for b_bishop in black_bishops.iter_mut() {
                                        // if the desired square is "above" the initial position
                                    if *b_bishop > desired_position {
                                        // and if the distance is divisible by 7
                                        if (*b_bishop - desired_position)%7 == 0 {
                                            for diagonal in 1..8 {
                                                // count each possible diagonal (until the maximum of 7 diagonals)
                                                if *b_bishop - diagonal*7 == desired_position && !upper_right_diagonal(*b_bishop, diagonal) {
                                                    // check if any of the squares in the bishop's diagonal is the desired square,
                                                    // check if any of the calculated diagonals are forbidden (done using the 'upper_right_diagonal' function),
                                                    // and finally, check if the desired square has no white pieces that may block the movement
                                                    // if all of those checks are true, the bishop may be moved
                                                    board[*b_bishop as usize] = NOTHING;
                                                    board[desired_position as usize] = BLACK_BISHOP;
                                                    *b_bishop = desired_position;

                                                    try_again = false;
                                                    break;
                                                }else if is_white(board[(*b_bishop - diagonal*7) as usize]) || is_black(board[((*b_bishop - diagonal*7) as usize)]) {
                                                    // otherwise, if there are any white/black pieces on the way, the square is unreachable
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
                                                }else if is_white(board[(*b_bishop - diagonal*9) as usize]) || is_black(board[((*b_bishop - diagonal*9) as usize)]){
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
                                                }else if is_white(board[(*b_bishop + 7*diagonal) as usize]) || is_black(board[(*b_bishop + 7*diagonal) as usize]){
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
                                                }else if is_white(board[(*b_bishop + 9*diagonal) as usize]) || is_black(board[(*b_bishop + 9*diagonal) as usize]) {
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
                                for b_rook in &mut black_rooks.iter_mut() {
                                    if rook_column + rook_line == *b_rook {
                                        if *b_rook > desired_position {
                                            // if the desired square is on the same rank as the initial position
                                            if get_line(*b_rook) == get_line(desired_position) {
                                                for square in 1..8 {
                                                    if *b_rook - square == desired_position && !rook_left(*b_rook, square) {
                                                        board[*b_rook as usize] = NOTHING;
                                                        board[desired_position as usize] = BLACK_ROOK;
                                                        *b_rook = desired_position;
                
                                                        try_again = false;
                                                        break;
                                                    }else if is_white(board[(*b_rook - square) as usize]) || is_black(board[(*b_rook - square) as usize]) {
                                                        break;
                                                    }
                                                }
                                            }else if (*b_rook - desired_position)%8 == 0 {
                                                // otherwise, test if it is on the same file
                                                for square in 1..8 {
                                                    if *b_rook - square*8 == desired_position && !rook_up(*b_rook, square) {
                                                        board[*b_rook as usize] = NOTHING;
                                                        board[desired_position as usize] = BLACK_ROOK;
                                                        *b_rook = desired_position;
                
                                                        try_again = false;
                                                        break;
                                                    }else if is_white(board[(*b_rook - square*8) as usize]) || is_black(board[(*b_rook - square*8) as usize]) {
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
                                                        *b_rook = desired_position;
                
                                                        try_again = false;
                                                        break;
                                                    }else if is_white(board[(*b_rook + square) as usize]) || is_black(board[(*b_rook + square) as usize]) {
                                                        break;
                                                    }
                                                }
                                            }else if (desired_position - *b_rook)%8 == 0 {
                                                for square in 1..8 {
                                                    if *b_rook + square*8 == desired_position && !rook_down(*b_rook, square) {
                                                        board[*b_rook as usize] = NOTHING;
                                                        board[desired_position as usize] = BLACK_ROOK;
                                                        *b_rook = desired_position;
                
                                                        try_again = false;
                                                        break;
                                                    }else if is_white(board[(*b_rook + square*8) as usize]) || is_black(board[(*b_rook + square*8) as usize]) {
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
                                        // if the desired square is on the same rank as the initial position
                                        if get_line(*b_rook) == get_line(desired_position) {
                                            for square in 1..8 {
                                                if *b_rook - square == desired_position && !rook_left(*b_rook, square) {
                                                    board[*b_rook as usize] = NOTHING;
                                                    board[desired_position as usize] = BLACK_ROOK;
                                                    *b_rook = desired_position;
            
                                                    try_again = false;
                                                    break;
                                                }else if is_white(board[(*b_rook - square) as usize]) || is_black(board[(*b_rook - square) as usize]) {
                                                    break;
                                                }
                                            }
                                        }else if (*b_rook - desired_position)%8 == 0 {
                                            // otherwise, test if it is on the same file
                                            for square in 1..8 {
                                                if *b_rook - square*8 == desired_position && !rook_up(*b_rook, square) {
                                                    board[*b_rook as usize] = NOTHING;
                                                    board[desired_position as usize] = BLACK_ROOK;
                                                    *b_rook = desired_position;
            
                                                    try_again = false;
                                                    break;
                                                }else if is_white(board[(*b_rook - square*8) as usize]) || is_black(board[(*b_rook - square*8) as usize]) {
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
                                                    *b_rook = desired_position;
            
                                                    try_again = false;
                                                    break;
                                                }else if is_white(board[(*b_rook + square) as usize]) || is_black(board[(*b_rook + square) as usize]) {
                                                    break;
                                                }
                                            }
                                        }else if (desired_position - *b_rook)%8 == 0 {
                                            for square in 1..8 {
                                                if *b_rook + square*8 == desired_position && !rook_down(*b_rook, square) {
                                                    board[*b_rook as usize] = NOTHING;
                                                    board[desired_position as usize] = BLACK_ROOK;
                                                    *b_rook = desired_position;
            
                                                    try_again = false;
                                                    break;
                                                }else if is_white(board[(*b_rook + square*8) as usize]) || is_black(board[(*b_rook + square*8) as usize]) {
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
                        },
                        'Q' => {
                            if test_multiple_rooks(&mut black_queens, desired_position) == true 
                            || test_multiple_bishops(&mut black_queens, desired_position) == true {
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
                                                // otherwise, test if it is on the same file
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
                                            // otherwise, test if it is on the same file
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

                    if board[desired_position as usize] == NOTHING {
                        match san_move[0] {
                            'a' => {
                                // for every pawn in the column
                                for pawn in &mut black_column_a.iter_mut() {
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
                            },
                            'b' => {
                                // for every pawn in the column
                                for pawn in &mut black_column_b.iter_mut() {
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
                            },
                            'c' => {
                                // for every pawn in the column
                                for pawn in &mut black_column_c.iter_mut() {
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
                            },
                            'd' => {
                                // for every pawn in the column
                                for pawn in &mut black_column_d.iter_mut() {
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
                            },
                            'e' => {
                                // for every pawn in the column
                                for pawn in &mut black_column_e.iter_mut() {
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
                            },
                            'f' => {
                                // for every pawn in the column
                                for pawn in &mut black_column_f.iter_mut() {
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
                            },
                            'g' => {
                                // for every pawn in the column
                                for pawn in &mut black_column_g.iter_mut() {
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
                            },
                            'h' => {
                                // for every pawn in the column
                                for pawn in &mut black_column_h.iter_mut() {
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
                            },
                            _ => ()
                        };

                        if line == 56 && try_again == false{ // if the pawn moved to the other side of the board successfully
                            println!("Indicate the pawn promotion: (Press Enter to promote it to a Queen)");
                            println!("'N'=Knight,'R'=Rook,'B'=Bishop");
                            let mut promotion_move = String::new();
                            let mut prom_loop: bool = true;

                            while prom_loop {
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

                    if is_white(board[desired_position as usize]) {
                        let captured_piece = board[desired_position as usize]; 

                        match san_move[0] {
                            'a' => {
                                let mut pawn_index: usize = 0;
                                // for every *pawn in the column
                                for pawn in &mut black_column_a.iter_mut() {
                                    if desired_position - *pawn == 9 && !inferior_right_diagonal(*pawn, 1) {
                                        board[*pawn as usize] = NOTHING;
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
                                // for every *pawn in the column
                                for pawn in &mut black_column_b.iter_mut() {
                                    if (desired_position - *pawn == 9 && !inferior_right_diagonal(*pawn, 1)) || (desired_position - *pawn == 7 && !inferior_left_diagonal(*pawn, 1)) {
                                        board[*pawn as usize] = NOTHING;
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
                                // for every pawn in the column
                                for pawn in &mut black_column_c.iter_mut() {
                                    if (desired_position - *pawn == 9 && !inferior_right_diagonal(*pawn, 1)) || (desired_position - *pawn == 7 && !inferior_left_diagonal(*pawn, 1)) {
                                        board[*pawn as usize] = NOTHING;
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
                                // for every *pawn in the column
                                for pawn in &mut black_column_d.iter_mut() {
                                    if (desired_position - *pawn == 9 && !inferior_right_diagonal(*pawn, 1)) || (desired_position - *pawn == 7 && !inferior_left_diagonal(*pawn, 1)) {
                                        board[*pawn as usize] = NOTHING;
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
                                // for every *pawn in the column
                                for pawn in &mut black_column_e.iter_mut() {
                                    if (desired_position - *pawn == 9 && !inferior_right_diagonal(*pawn, 1)) || (desired_position - *pawn == 7 && !inferior_left_diagonal(*pawn, 1)) {
                                        board[*pawn as usize] = NOTHING;
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
                                // for every *pawn in the column
                                for pawn in &mut black_column_f.iter_mut() {
                                    if (desired_position - *pawn == 9 && !inferior_right_diagonal(*pawn, 1)) || (desired_position - *pawn == 7 && !inferior_left_diagonal(*pawn, 1)) {
                                        board[*pawn as usize] = NOTHING;
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
                                // for every *pawn in the column
                                for pawn in &mut black_column_g.iter_mut() {
                                    if (desired_position - *pawn == 9 && !inferior_right_diagonal(*pawn, 1)) || (desired_position - *pawn == 7 && !inferior_left_diagonal(*pawn, 1)) {
                                        board[*pawn as usize] = NOTHING;
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
                                // for every *pawn in the column
                                for pawn in &mut black_column_h.iter_mut() {
                                    if desired_position - *pawn == 7 && !inferior_left_diagonal(*pawn, 1) {
                                        board[*pawn as usize] = NOTHING;
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
                    
                        if line == 56 && try_again == false{ // if the pawn moved to the other side of the board successfully
                            println!("Indicate the pawn promotion: (Press Enter to promote it to a Queen)");
                            println!("'N'=Knight,'R'=Rook,'B'=Bishop");
                            let mut promotion_move = String::new();
                            let mut prom_loop: bool = true;

                            while prom_loop {
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

    match b - i*7 {
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

fn get_line(piece: i8) -> i8 {
    match piece {
        0..=7 => 1,
        8..=15 => 2,
        16..=23 => 3,
        24..=31 => 4,
        32..=39 => 5,
        40..=47 => 6,
        48..=55 => 7,
        56..=63 => 8,
        _ => 0
    }
}


fn test_multiple_rooks(pieces: &mut Vec<i8>, position: i8) -> bool {
    // this tests every rook or piece with rook-like 
    // movement inside a vector to see if more than one
    // of them can reach the same square

    // true = multiple pieces may go to the desired square
    let mut counter = 0;

    for i in 0..pieces.len() {
        if get_line(pieces[i]) == get_line(position) || (position - pieces[i])%8 == 0 {
            counter += 1
        }
        if counter >= 2 {
            return true
        }
    }

    return false
}

fn test_multiple_bishops(pieces: &mut Vec<i8>, position: i8) -> bool {
    // this tests every rook or piece with rook-like 
    // movement inside a vector to see if more than one
    // of them can reach the same square

    // true = multiple pieces may go to the desired square
    let mut counter = 0;

    for i in 0..pieces.len() {
        if (position - pieces[i])%7 == 0 || (position - pieces[i])%9 == 0 {
            counter += 1
        }
        if counter >= 2 {
            return true
        }
    }

    return false
}

fn test_multiple_knights(pieces: &mut Vec<i8>, position: i8) -> bool {
    // this tests every rook or piece with rook-like 
    // movement inside a vector to see if more than one
    // of them can reach the same square

    // true = multiple pieces may go to the desired square
    let mut counter = 0;

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

    return false
}