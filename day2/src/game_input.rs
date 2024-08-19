use std::fmt;
use std::str::FromStr;

pub struct Game {
    pub id: u32,
    pub draws: Vec<GameDraw>,
}

pub struct GameDraw {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl FromStr for GameDraw {
    type Err = GameParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Split by commas and loop.
        // For each one, trim, find the space and split into first and second piece
        // - First piece is the digit
        // - Second piece is the color

        let mut value = Self {
            red: 0,
            green: 0,
            blue: 0
        };

        for pair in s.split(',') {
            let pair = pair.trim();
            let pieces: Vec<&str> = pair.split(' ').collect();
            if pieces.len() != 2 {
                return Err(GameParsingError::new(format!("Cannot parse {:?} into a 'GameDraw'", pair)));
            }

            let digit: u32 = pieces[0]
                .trim()    
                .parse()
                .map_err(|_e| GameParsingError::new(format!("Cannot parse {:?} into a 'GameDraw'", pair)))
                ?;

            match pieces[1] {
                "red" => value.red = digit,
                "green" => value.green = digit,
                "blue" => value.blue = digit,
                color => { return Err(GameParsingError::new(format!("Invalid color {:?}", color) )); }
            }
        }

        Ok(value)
    }
}

#[derive(Debug)]
pub struct GameParsingError {
    message: String
}

impl GameParsingError {
    fn new(message: String) -> Self {
        Self { message }
    }

    fn from(message: &str) -> Self {
        Self {
            message: String::from(message)
        }
    }
}

impl fmt::Display for GameParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.message)
    }
}
