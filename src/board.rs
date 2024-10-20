use crate::pieces::{Piece, PieceColor, PieceType};

const BOARD_SIZE: usize = 8;

pub struct Board([[Option<Piece>; BOARD_SIZE]; BOARD_SIZE]);

impl Board {
    pub fn new() -> Self {
        Self {
            0: [[None; BOARD_SIZE]; BOARD_SIZE],
        }
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let biggest_row_label_digits = BOARD_SIZE.to_string().len();

        let enumerated_rows_as_str = self
            .0
            .into_iter()
            .map(|row| {
                row.map(|op| match op {
                    Some(piece) => format!(" {} ", piece),
                    None => format!(" . "),
                })
                .concat()
            })
            .zip(1..=BOARD_SIZE)
            .fold(String::new(), |acc, (column_as_str, row_number)| {
                acc + &format!(
                    "{}[{}] {}\n",
                    " ".repeat(biggest_row_label_digits - row_number.to_string().len()),
                    row_number,
                    column_as_str
                )
            });

        let column_labels =
            ('a' as usize..('a' as usize + BOARD_SIZE)).fold(String::new(), |acc, char_bytes| {
                acc + &format!(
                    "[{}]",
                    char::from_u32(char_bytes as u32).unwrap_or_else(|| '#')
                )
            });

        write!(
            f,
            "\n{}{}{}",
            enumerated_rows_as_str,
            " ".repeat(biggest_row_label_digits + 3),
            column_labels
        )
    }
}

fn load_piece_using_idx(idx: usize, color: PieceColor) -> Option<Piece> {
    match idx {
        0 => Some(Piece::new(PieceType::Rook, color)),
        1 => Some(Piece::new(PieceType::Knight, color)),
        2 => Some(Piece::new(PieceType::Bishop, color)),
        3 => Some(Piece::new(PieceType::Queen, color)),
        4 => Some(Piece::new(PieceType::King, color)),
        5 => Some(Piece::new(PieceType::Bishop, color)),
        6 => Some(Piece::new(PieceType::Knight, color)),
        7 => Some(Piece::new(PieceType::Rook, color)),
        _ => None,
    }
}

impl Default for Board {
    fn default() -> Self {
        let mut board = Board::new().0;

        board[0].into_iter().enumerate().for_each(|(i, _)| {
            board[0][i] = load_piece_using_idx(i, PieceColor::Black);
        });

        board[1] = [Some(Piece::new(PieceType::Pawn, PieceColor::Black)); BOARD_SIZE];
        board[BOARD_SIZE - 2] = [Some(Piece::new(PieceType::Pawn, PieceColor::White)); BOARD_SIZE];

        board[BOARD_SIZE - 1]
            .into_iter()
            .enumerate()
            .for_each(|(i, _)| {
                board[BOARD_SIZE - 1][i] = load_piece_using_idx(i, PieceColor::White);
            });

        Self { 0: board }
    }
}

impl std::ops::Index<usize> for Board {
    type Output = [Option<Piece>];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
