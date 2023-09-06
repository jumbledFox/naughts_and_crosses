use std::io;
use naughts_and_crosses::board;

fn main() {
    'program: loop {
        println!("SUPER DUPER NAUGHTS AND CROSSES !!!!!!!");
        // Make a new game
        let mut b = board::Board::new();
        let mut current_player = 1;
        // If there are still moves to make
        'game: while b.check_win() == 0 && b.spaces_empty() {
            // Draw the board
            let player_name = match current_player {
                1 => "Cross",
                _ => "Naught"
            };
            println!("{}'s go!!", player_name);
            b.draw_board();

            // Get user input
            let mut input_text = String::new();
            io::stdin()
                .read_line(&mut input_text)
                .expect("failed to read from stdin");

            let trimmed = input_text.trim();
            let input_space: u32; 
            match trimmed.parse::<u32>() {
                Ok(i) => input_space = i,
                Err(..) => { println!("Input not valid!"); continue 'game; },
            };
            if input_space > 9 || input_space < 1 {
                println!("Input not valid!");
                continue 'game;
            }
            if !b.space_empty(input_space as usize) {
                println!("Space already taken!!");
                continue 'game;
            }
            b.set_space(input_space as usize, current_player);
            current_player = match current_player {
                1 => 2,
                _ => 1,
            };
        }
        // Announce winner (or draw!!)
        println!("{}", match b.check_win() {
            1 => "Crosses win!!!!!!",
            2 => "Naughts win!!!!!!",
            _ => "Nobody wins!!!!!!",
        });
        b.draw_board();
        loop {
            // Ask if the user wants to play again
            println!("Do you want to play again? (y/n)");
            // Get user input
            let mut input_text = String::new();
            io::stdin()
                .read_line(&mut input_text)
                .expect("failed to read from stdin");
            let trimmed = input_text.trim();

            match trimmed {
                "y" => continue 'program,
                "n" => std::process::exit(1),
                _ => println!("Please type y or n please !!!!"),
            };
        }
    }
    
}
