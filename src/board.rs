use crate::pieces::{Piece, PieceColor, PieceType};

const BOARD_SIZE: usize = 8;

pub struct Board(pub [[Option<Piece>; BOARD_SIZE]; BOARD_SIZE]);

impl Board {
    pub fn new() -> Self {
        Self([[None; BOARD_SIZE]; BOARD_SIZE])
    }

    fn load_default_pieces_row(&mut self, color: PieceColor) {
        let row_op = match color {
            PieceColor::Black => self.0.first_mut(),
            PieceColor::White => self.0.last_mut(),
        };

        if let Some(row) = row_op {
            row.iter_mut().enumerate().for_each(|(column, p)| {
                *p = match column {
                    0 => Some(Piece::new(PieceType::Rook, color)),
                    1 => Some(Piece::new(PieceType::Knight, color)),
                    2 => Some(Piece::new(PieceType::Bishop, color)),
                    3 => Some(Piece::new(PieceType::Queen, color)),
                    4 => Some(Piece::new(PieceType::King, color)),
                    5 => Some(Piece::new(PieceType::Bishop, color)),
                    6 => Some(Piece::new(PieceType::Knight, color)),
                    7 => Some(Piece::new(PieceType::Rook, color)),
                    _ => None,
                };
            });
        }
    }

    fn load_default_pawns_row(&mut self, color: PieceColor) {
        let row_op = match color {
            PieceColor::Black => self.0.get_mut(1),
            PieceColor::White => self.0.iter_mut().nth_back(1),
        };

        if let Some(row) = row_op {
            row.fill(Some(Piece::new(PieceType::Pawn, color)));
        }
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let biggest_row_label_digits = BOARD_SIZE.to_string().len();

        let enumerated_rows_as_str =
            self.0
                .into_iter()
                .zip(1..=BOARD_SIZE)
                .fold(String::new(), |str, (row, row_number)| {
                    format!(
                        "{str}{}[{}] {}\n",
                        " ".repeat(biggest_row_label_digits - row_number.to_string().len()),
                        row_number,
                        row.iter().fold(String::new(), |acc, opt_p| {
                            format!(
                                "{acc} {} ",
                                match opt_p {
                                    Some(piece) => piece.to_string(),
                                    None => '.'.to_string(),
                                }
                            )
                        })
                    )
                });

        let column_labels =
            ('a' as usize..('a' as usize + BOARD_SIZE)).fold(String::new(), |acc, char_bytes| {
                acc + &format!("[{}]", char::from_u32(char_bytes as u32).unwrap_or('#'))
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

impl Default for Board {
    fn default() -> Self {
        let mut board = Board::new();

        board.load_default_pieces_row(PieceColor::Black);
        board.load_default_pieces_row(PieceColor::White);
        board.load_default_pawns_row(PieceColor::Black);
        board.load_default_pawns_row(PieceColor::White);

        board
    }
}

impl std::ops::Index<usize> for Board {
    type Output = [Option<Piece>];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
