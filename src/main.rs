use std::io;
use std::string::String;
//use strum::IntoEnumIterator; 
//use strum_macros::EnumIter;

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

    //movement stuff
    let pawn_movement: [usize; 2] = [8, 16];
    let knight_movement: [i8;8] = [-17, -15, -10, -6, 6, 10, 15, 17];
    
//LAST POSITION OF EACH PIECE AND PAWN:

    //white pawns and pieces
    let mut white_pawn_a = 48;
    let mut white_pawn_b = 49;
    let mut white_pawn_c = 50;
    let mut white_pawn_d = 51;
    let mut white_pawn_e = 52;
    let mut white_pawn_f = 53;
    let mut white_pawn_g = 54;
    let mut white_pawn_h = 55;
    let mut white_pawns: [usize;8] = [white_pawn_a, white_pawn_b, white_pawn_c, white_pawn_d, white_pawn_e, white_pawn_f, white_pawn_g, white_pawn_h];
    
    let mut white_rook1 = 56;
    let mut white_rook2 = 63;
    
    let mut white_knight1 = 57;
    let mut white_knight2 = 62;
    
    let mut white_bishop1 = 58;
    let mut white_bishop2 = 61;
    
    let mut white_queen = 59;
    let mut white_king = 60;
    
    //black pawns and pieces
    let mut black_pawn_a = 8;
    let mut black_pawn_b = 9;
    let mut black_pawn_c = 10;
    let mut black_pawn_d = 11;
    let mut black_pawn_e = 12;
    let mut black_pawn_f = 13;
    let mut black_pawn_g = 14;
    let mut black_pawn_h = 15;
    let mut black_pawns: [usize;8] = [black_pawn_a, black_pawn_b, black_pawn_c, black_pawn_d, black_pawn_e, black_pawn_f, black_pawn_g, black_pawn_h];
    
    let mut black_rook1 = 0;
    let mut black_rook2 = 7;
    
    let mut black_knight1 = 1;
    let mut black_knight2 = 6;
    
    let mut black_bishop1 = 2;
    let mut black_bishop2 = 5;
    
    let mut black_queen = 3;
    let mut black_king = 4;

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
    //show/update board 
    //for every element in the board until 0, print the current element
    for i in 0..board.len() {
        // i+1 = the actual position of the item in the array (instead of its index)
        // if the position of the item -isn't- divisible by 8 (remainder is NOT equal to 0), print the board normally
        if (i+1)%8 != 0{
            print!(" {} ", board[i]);
        }else{
        //if -it is- divisible by 8 (remainder IS equal to 0), add a line break
            println!(" {} ", board[i]);
        };
    };

    let mut player_move = String::new();
    let mut san_move: Vec<char> = Vec::new();

    //"column" also stands for the piece to be moved, since it's tied to the first letter in the SAN notation
    let mut column: usize;
    let mut line: usize;

    let mut try_again: bool = true;

    while try_again{
        println!("White moves");
            
        io::stdin()
            .read_line(&mut player_move)
            .expect("Read error");

        //san = short algebraic notation
        let mut san_move: Vec<char> = player_move.trim().chars().collect();

        if is_piece(san_move[0]) == true {
            //if the first letter indicates a piece, the move has to be described in the two next letters
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

            match san_move[0] {
                'N' => for possible_moves in knight_movement.iter() {
                    if ((column + line) as i8) == (white_knight1 as i8) + *possible_moves {
                        //last position is freed
                        board[white_knight1] = NOTHING;

                        //piece is moved to new position
                        board[column+line] = WHITE_KNIGHT;

                        //current position is updated
                        white_knight1 = column+line;

                        try_again = false;
                        break;
                    }else if ((column + line) as i8) == (white_knight2 as i8) + *possible_moves {
                        board[white_knight2] = NOTHING;
                        board[column+line] = WHITE_KNIGHT;
                        white_knight2 = column+line;

                        try_again = false;
                        break;
                    }
                },
                // 'B' => 
                _ => (),
            }

        }else{

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

            //for each pawn in white's possession
            //i = last position of the pawn
            for i in white_pawns.iter_mut() {
                //if the pawn is in it's starting position
                if *i <= 55 {
                    //get every move possible for a pawn
                    //j = one of the possible moves
                    for j in pawn_movement.iter() {
                        //while in the starting position, pawns can move up to 2 pieces
                        if *i >= *j{
                        //and check if any of the pawns can go to the specified move
                            if *i-*j == column+line {
                                //last position is freed
                                board[*i] = NOTHING;

                                //pawn is moved to the new position
                                board[column+line] = WHITE_PAWN;

                                //current position is updated
                                *i = column+line;

                                try_again = false;
                                break;
                            }
                        }
                    };
                }else if *i >= 55{
                    if *i-8 == column+line{
                        //last position is freed
                        board[*i] = NOTHING;

                        //pawn is moved to the new position
                        board[column+line] = WHITE_PAWN;

                        //current position is updated
                        *i = column+line;

                        try_again = false;
                        break;
                    }
                }
            };
        }
        if try_again == true{
            println!("Not a possible move, try again!\n");
        }

        //has to be cleared, otherwise read_line would mut just append the string to the last move registered in player_move
        player_move.clear();
        san_move.clear();
    }

    for i in 0..board.len() {
        // i+1 = the actual position of the item in the array (instead of its index)
        // if the position of the item -isn't- divisible by 8 (remainder is NOT equal to 0), print the board normally
        if (i+1)%8 != 0{
            print!(" {} ", board[i]);
        }else{
        //if -it is- divisible by 8 (remainder IS equal to 0), add a line break
            println!(" {} ", board[i]);
        };
    };

    try_again = true;

    while try_again{
        println!("Black moves");

        io::stdin()
            .read_line(&mut player_move)
            .expect("Read error");

        san_move = player_move.trim().chars().collect();

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

            match san_move[0] {
                'N' => for possible_moves in knight_movement.iter() {
                    if ((column + line) as i8) == (black_knight1 as i8) + *possible_moves {
                        //last position is freed
                        board[black_knight1] = NOTHING;

                        //piece is moved to new position
                        board[column+line] = BLACK_KNIGHT;

                        //current position is updated
                        black_knight1 = column+line;

                        try_again = false;
                        break;
                    } else if ((column + line) as i8) == (black_knight2 as i8) + *possible_moves {
                        board[black_knight2] = NOTHING;
                        board[column+line] = BLACK_KNIGHT;
                        black_knight2 = column+line;

                        try_again = false;
                        break;
                    }
                },
                _ => (),
            }

        }else{
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

            //for each pawn in black's possession
            //i = last position of the pawn
            for i in black_pawns.iter_mut() {
                if 8 <= *i && *i <= 15 {
                    //get every move possible for a pawn
                    //j = one of the possible moves
                    for j in pawn_movement.iter() {
                    //if *i >= *j isnt needed, the black pawn loop doesnt subtract from the index
                        //and check if any of the pawns can go to the specified move
                        //(last position + possible move)
                        if *i+*j == column+line {
                            //last position is freed
                            board[*i] = NOTHING;

                            //pawn is moved to the new position
                            board[column+line] = BLACK_PAWN;

                            //current position is updated
                            *i = column+line;

                            try_again = false;
                            break;
                        }
                    };
                }else if *i+8 == column+line{
                    //last position is freed
                    board[*i] = NOTHING;

                    //pawn is moved to the new position
                    board[column+line] = BLACK_PAWN;

                    //current position is updated
                    *i = column+line;

                    try_again = false;
                    break;
                }
                
            };
        }

        if try_again == true{
            println!("Not a possible move, try again!\n");
        }
        player_move.clear();
        san_move.clear();
    }

        /*    
        println!("Would you like to play default Chess or Fischer Random? (d/f)");

        let mut answer = String::new();

        io::stdin()
            .read_line(&mut answer)
            .expect("That was not a string");
        
        if answer.trim() == "d"{
            board_default();
        }else if answer.trim() == "f"{
            //board_chess960(Pieces);
        }else{
            println!("That was not a possible answer.");
        };*/

    }
}


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

// fn usize_to_i8(v: usize) -> i8 {
//     i8::from(v)
// }