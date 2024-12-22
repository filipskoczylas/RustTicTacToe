use console::{Key, Term};
use crate::enums;
const BOARD_SIZE: usize = 3;
struct CCursor{
    x: u8,
    y: u8
}
pub struct CTicTacToe{
    board: [[enums::ESymbol; BOARD_SIZE]; BOARD_SIZE],
    cursor: CCursor,
    current_player: enums::EPlayer,
    run_game: bool,
}

impl CTicTacToe {
    pub fn new() -> CTicTacToe {
        CTicTacToe{
            board: [[enums::ESymbol::None; 3]; 3],
            cursor: CCursor{x: 0, y: 0},
            current_player: enums::EPlayer::Cross,
            run_game: true,
        }
    }

    pub fn play(&mut self) {
        let stdout = Term::buffered_stdout();
        while (self.run_game){
            self.scan_player_input(&stdout);
            self.print_board();
        }
    }

    fn print_hello(&self) {
        println!("Hello, this is TicTacToe game implemented in Rust!");
        println!("To change cursor position use arrow keys or WASD.");
        println!("To accept your move press enter.");
        println!("Press enter to play.");
    }

    fn print_play_again(&self){
        println!("Press enter to play again.");
        println!("Press X to quit.");
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
                    print!("[{}]", symbol);
                }
                else {
                    print!(" {} ", symbol);
                }
            }
            println!(); // Newline after each row
        }
    }

    fn scan_player_input(& mut self, stdout: &Term){
        if let Ok(character) = stdout.read_key() {
            match character {
                Key::Char('w') => self.move_up(),
                Key::Char('a') => self.move_left(),
                Key::Char('s') => self.move_down(),
                Key::Char('d') => self.move_right(),
                Key::Char('x') => self.run_game = false,
                Key::ArrowUp => self.move_up(),
                Key::ArrowLeft => self.move_left(),
                Key::ArrowDown => self.move_down(),
                Key::ArrowRight => self.move_right(),
                Key::Enter => self.place_mark(),
                _ => { }
            }
        }
    }

    fn place_mark(&mut self){

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

    fn move_up(&mut self){
        if self.cursor.y > 0{
            self.cursor.y -= 1;
        }
    }

    fn move_down(&mut self){
        if self.cursor.y < (BOARD_SIZE - 1) as u8 {
            self.cursor.y += 1;
        }
    }

    fn move_left(&mut self){
        if self.cursor.x > 0 {
            self.cursor.x -= 1;
        }
    }

    fn move_right(&mut self){
        if self.cursor.x < (BOARD_SIZE - 1) as u8 {
            self.cursor.x += 1;
        }
    }
}