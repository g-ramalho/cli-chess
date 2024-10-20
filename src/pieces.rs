#[derive(Clone, Copy, Debug)]
pub enum PieceType {
    Queen,
    King,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Clone, Copy, Debug)]
pub enum PieceColor {
    Black,
    White,
}

#[derive(Clone, Copy, Debug)]
pub struct Piece {
    p_type: PieceType,
    color: PieceColor,
}

impl Piece {
    pub fn new(p_type: PieceType, color: PieceColor) -> Self {
        Self { p_type, color }
    }
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let representation = match self.p_type {
            PieceType::Queen => 'Q',
            PieceType::King => 'K',
            PieceType::Rook => 'R',
            PieceType::Bishop => 'B',
            PieceType::Knight => 'N',
            PieceType::Pawn => {
                return write!(
                    f,
                    "{}",
                    match self.color {
                        PieceColor::Black => 'j',
                        PieceColor::White => 'i',
                    }
                )
            }
        };

        write!(
            f,
            "{}",
            match self.color {
                PieceColor::White => representation,
                PieceColor::Black => representation.to_ascii_lowercase(),
            }
        )
    }
}
