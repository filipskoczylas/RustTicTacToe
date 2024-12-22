use std::cmp::PartialEq;
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
    stdout: Term,
}

impl CTicTacToe {
    pub fn new() -> CTicTacToe {
        CTicTacToe{
            board: [[enums::ESymbol::None; 3]; 3],
            cursor: CCursor{x: 0, y: 0},
            current_player: enums::EPlayer::Cross,
            run_game: false,
            stdout: Term::buffered_stdout(),
        }
    }

    pub fn play(&mut self) {
        self.print_hello();
        self.stdout.read_key().unwrap();
        self.run_game = true;
        self.play_board();
    }

    fn play_board(&mut self) {
        while self.run_game {
            // Print board
            self.print_board();

            // Player decides his move
            self.scan_player_input();
        }
    }
    fn end_board(&mut self) {
        self.run_game = false;
        self.print_end_screen();
        if let Ok(character) = self.stdout.read_key() {
            match character {
                Key::Enter => self.reset_board(),
                _ => { }
            }
        }
    }

    fn reset_board (&mut self) {
        self.cursor.x = 0;
        self.cursor.y = 0;
        self.current_player = enums::EPlayer::Cross;
        for row_idx in 0..BOARD_SIZE {
            for col_idx in 0..BOARD_SIZE {
                self.board[row_idx][col_idx] = enums::ESymbol::None;
            }
        }
        self.run_game = true;

        // Play board again
        self.play_board();
    }

    fn print_hello(&self) {
        println!("Hello, this is TicTacToe game implemented in Rust!");
        println!("To change cursor position use arrow keys or WASD.");
        println!("To accept your move press enter.");
        println!("Press any key to play.");

    }

    fn print_end_screen (&self){
        println!("");
        println!("{} won !", self.current_player);
        println!("Press enter to play again.");
        println!("Press any other key to quit.");
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

                // Mark current cursor
                if self.cursor.x == col_idx as u8 && self.cursor.y == row_idx as u8 {
                    print!("[{}]", symbol);
                }
                else {
                    print!(" {} ", symbol);
                }
            }
            println!();
        }
    }

    fn scan_player_input(& mut self){
        if let Ok(character) = self.stdout.read_key() {
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
        if self.board[self.cursor.y as usize][self.cursor.x as usize] == enums::ESymbol::None {
            if self.current_player == enums::EPlayer::Cross {
                self.board[self.cursor.y as usize][self.cursor.x as usize] = enums::ESymbol::Cross;
            }
            else{
                self.board[self.cursor.y as usize][self.cursor.x as usize] = enums::ESymbol::Circle;
            }

            if self.determine_win() {
                self.end_board();
            }
            else{
                self.next_player();
            }
        }
    }

    fn determine_win (&mut self) -> bool {
        let player: enums::ESymbol = if self.current_player == enums::EPlayer::Circle{
            enums::ESymbol::Circle
        }else{
            enums::ESymbol::Cross
        };
        let mut column_sum: u8 = 0;
        let mut row_sum: u8 = 0;
        let mut diagonal_sum: u8 = 0;
        let mut antidiagonal_sum: u8 = 0;
        for idx in 0..BOARD_SIZE {
            if self.board[idx][self.cursor.x as usize] == player {
                column_sum += 1;
            }
            if self.board[self.cursor.y as usize][idx] == player {
                row_sum += 1;
            }
            if self.board[idx][idx] == player {
                diagonal_sum += 1;
            }
            if self.board[idx][BOARD_SIZE - (idx + 1)] == player {
                antidiagonal_sum += 1;
            }
        }
        column_sum == BOARD_SIZE as u8 ||   // Return
                row_sum == BOARD_SIZE as u8 ||
                diagonal_sum == BOARD_SIZE as u8 ||
                antidiagonal_sum == BOARD_SIZE as u8
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