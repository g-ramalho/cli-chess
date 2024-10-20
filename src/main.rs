use board::Board;

mod board;
mod pieces;

fn main() {
    let mut board = Board::default();

    println!("{}", board);

    if let Some(n) = board[7][6] {
        n.move_to(5, 5, &mut board);
    }

    println!("{}", board);
}
