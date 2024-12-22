use crate::enums;
const BOARD_SIZE: usize = 3;
const BOARD_REFRESH_SIZE : u64 = 333;
struct CCursor{
    x: u8,
    y: u8
}
pub struct CTicTacToe{
    board: [[enums::ESymbol; BOARD_SIZE]; BOARD_SIZE],
    cursor: CCursor,
    cursor_state: bool,
    current_player: enums::EPlayer,
}

impl CTicTacToe {
    pub fn new() -> CTicTacToe {
        CTicTacToe{
            board: [[enums::ESymbol::None; 3]; 3],
            cursor: CCursor{x: 0, y: 0},
            cursor_state: true,
            current_player: enums::EPlayer::Cross,
        }
    }

    pub fn play(&mut self) {
        while (true){
            self.print_board();
            std::thread::sleep(std::time::Duration::from_millis(BOARD_REFRESH_SIZE));
        }
    }

    fn print_hello(&self) {
        println!("Hello, this is TicTacToe game implemented in Rust!");
        println!("To change cursor position use arrow keys or WASD.");
        println!("To accept your move press enter.");
        println!("Press enter to play.");
    }

    fn print_have_fun(&self){
        println!("Have fun!");
    }

    fn print_play_again(&self){
        println!("Press enter to play again.");
        println!("Press esc to quit.");
    }

    fn print_board(&mut self) {
        self.clear_console();
        println!("Current player: {}", self.current_player);
        for row_idx in 0..BOARD_SIZE {
            for col_idx in 0..BOARD_SIZE {
                let symbol = match self.board[row_idx][col_idx] {
                    enums::ESymbol::None => "-",
                    enums::ESymbol::Cross => "X",
                    enums::ESymbol::Circle => "O",
                };

                // If the cursor is at this position, print only if it is time to blink
                if self.cursor.x == col_idx as u8 && self.cursor.y == row_idx as u8 {
                    if self.cursor_state {
                        print!("{} ", symbol);
                    }
                    else {
                        print!("  ");
                    }
                }
                else {
                    print!("{} ", symbol);
                }
            }
            println!(); // Newline after each row
        }
        self.cursor_state = !self.cursor_state;
    }

    fn clear_console(&self) {
        print!("\x1B[2J\x1B[1;1H");
    }

    fn next_player(&mut self){
        match self.current_player{
            enums::EPlayer::Cross => {
                self.current_player = enums::EPlayer::Circle;
            }
            enums::EPlayer::Circle => {
                self.current_player = enums::EPlayer::Cross;
            }
        }
    }
}