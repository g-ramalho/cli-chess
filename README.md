# cli-chess
CLI Chessboard built in Rust.
Features:
- En passant
- Castling
- Pawn promotion
- Checkmate
- Stalemate
- Draw by insufficient material
- Pinned pieces system

## Setup instructions

1. **Clone the repository**:
   ```
   git clone https://github.com/g-ramalho/chess
   cd chess
   ```

2. [Install Cargo (the Rust package manager)](https://www.rust-lang.org/tools/install)

3. **Build and run**:
   ```
   cargo run
   ```

## The game

The game is viewed as whites, the moves are inputted using SAN (Short Algebraic Notation) \
*i.e. "Nf3" to move a knight to f3* \
*or "bxa3" to take a piece/pawn on a3 with a pawn on the b column*

The simbols on the "board" have the following meaning:
- " . " counts as a square with no pieces or pawns
- " K " or " k " = king
- " Q " or " q " = queen
- " R " or " r " = rook
- " B " or " b " = bishop
- " N " or " n " = knight
- " i " or " j " = white and black pawn, respectively

Uppercase letters indicate white pieces while lowercase letters indicate black pieces

### Move input info
- In case of ambiguity (more than one piece can go to the same square), indicate the move normally (i.e. "Rae8").

- Castling is also possible. **Kingside** castles are done by inputting **"0-0" or "O-O"** while *Queenside* castles are done by inputting *"0-0-0" or "O-O-O"*.

- Pawn promotions are inputted as normal movements ("e8" or "gxh8", for example). The game will then ask for the input of a character indicating the piece to promote to. Pressing Enter is an option to promote to a Queen instantly.

- In case of check or checkmate moves, there is no need to input "+" or "#" in the moves. The game will only use the position and the piece you inputted for the move.

### Game preview:
#### Scholar's Checkmate
![Scholar's Checkmate](https://github.com/g-ramalho/chess/assets/62910017/5e342310-1e4b-4669-a4dd-ec2697f911cd)

#### Draw by insufficient material
![Draw by insufficient material](https://github.com/g-ramalho/chess/assets/62910017/e2655001-3d96-4a6b-942a-61a00c613821)

#### Shortest stalemate possible
![Shortest stalemate possible](https://github.com/g-ramalho/chess/assets/62910017/8b3bdade-ee90-43ea-858d-3c27c86edd2e)


