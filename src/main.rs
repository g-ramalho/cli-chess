use board::Board;

mod board;
mod pieces;

fn main() {
    let board = Board::default();

    println!("{}", board);
}
