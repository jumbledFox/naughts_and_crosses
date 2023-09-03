use std::{io, process};
use naughts_and_crosses::board;

fn main() {
    while true {
        // Make a new game
        let mut b = board::Board::new();
        // If there are still moves to make
        while b.check_win() == 0 || b.places.contains(&0) {
            // Get user input
            let mut input_text = String::new();
            io::stdin()
                .read_line(&mut input_text)
                .expect("failed to read from stdin");

            let trimmed = input_text.trim();
            match trimmed.parse::<u32>() {
                Ok(i) => println!("your integer input: {}", i),
                Err(..) => println!("Input not valid!"),
            };
            b.places[0][2] = 2;
            b.places[1][1] = 1;
            // Draw the board
            b.draw_board();
        }
        // Exit
        std::process::exit(1);
    }
    
}
