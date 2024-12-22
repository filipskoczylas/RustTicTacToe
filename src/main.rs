mod enums;
mod TicTacToe;

fn main() {
    let mut game = TicTacToe::CTicTacToe::new();
    game.play();
}
