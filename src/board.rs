use colored::Colorize;

pub struct Board {
    // 0 - Empty cell
    // 1 - X
    // 2 - O
    pub places: [[u8; 3]; 3]
}

impl Board {
    pub fn new() -> Self {
        Board {places: [[0; 3]; 3]}
    }

    pub fn check_win(&self) -> i8 {
        let p = self.places;
        // Vector of results
        // Loop through each row, column, and diagonal, multiplying the three cells that make them up together, push this to r
        // If we find 1^2 or 2^2 in 'r', that means player 1 or player 2 have won.
        let mut r = vec![];
        // Diagonals
        r.push(p[0][0] * p[1][1] * p[2][2]);
        r.push(p[2][0] * p[1][1] * p[0][2]);
        // Rows and columns
        for i in 0..3 {
            r.push(p[i][0] * p[i][1] * p[i][2]);
            r.push(p[0][i] * p[1][i] * p[2][i]);
        }
        // Check 'r' to see if it contains any winning places
        if r.contains(&1) {
            return 1
        } else if r.contains(&2) {
            return 2
        } else {
            return 0
        }
    }

    pub fn draw_board(&self) {
        let mut c: [colored::ColoredString; 9] = Default::default();
        for i in 0..9 {
            let cell = self.places[i / 3][i % 3];
            c[i] = match cell {
                1 => "X".truecolor(253,102,0).bold(),
                2 => "O".truecolor(0,120,255).bold(),
                _ => (i+1).to_string().bright_black(),
            }
        }
        
        let sv = "│".white();
        println!(" {} {} {} {} {} ", c[0], sv, c[1], sv, c[2]);
        println!("{}", "───┼───┼───".white());
        println!(" {} {} {} {} {} ", c[3], sv, c[4], sv, c[5]);
        println!("{}", "───┼───┼───".white());
        println!(" {} {} {} {} {} ", c[6], sv, c[7], sv, c[8]);
    }
}