mod pieces;
use std::vec;
use pieces::*;

const BOARD_SIZE: usize = 8; // max board size: 26

fn main() {
    const FREE_SQUARE_SYMBOL: char = '.';
    let mut board = [[FREE_SQUARE_SYMBOL; BOARD_SIZE]; BOARD_SIZE];

    let white_pawns = Piece {
        positions: vec![],
        color: true,
        piece_type: PieceType::Pawn,
        symbol: 'i'
    };

    println!("White moves:");
    let a = get_player_move();

    show_board(&board);

}

fn show_board(board: &[[char;BOARD_SIZE];BOARD_SIZE]) {
    for row in (0..BOARD_SIZE).rev() {
        for column in 0..BOARD_SIZE {
            if column == 0 {
                if row+1 >= 10 {
                    print!("{} {} ", (row+1), board[column][row]);
                }else {
                    print!("{}  {} ", (row+1), board[column][row]);
                }
            }else if column == BOARD_SIZE - 1 {
                println!(" {}", board[column][row]);
            }else {
                print!(" {} ", board[column][row]);
            }


        }
    }

    print!("  ");
    for letter in 0..BOARD_SIZE {
        if letter == BOARD_SIZE - 1 {
            println!(" {}\n", BOARD_LETTERS[letter]);
        }else {
            print!(" {} ", BOARD_LETTERS[letter]);
        }
    }
}

const BOARD_LETTERS: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', '#', 'y', 'z']; 
// notice how '#' was used instead of 'x'
