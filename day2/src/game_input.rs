use std::fmt;
use std::str::FromStr;
use std::cmp::max;

pub struct Game {
    pub id: u32,
    pub draws: Vec<GameDraw>,
}

impl Game {
    pub fn get_power(&self) -> u32 {
        let (r, g, b) = self.get_min_use();
        r * g * b
    }

    fn get_min_use(&self) -> (u32, u32, u32) {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        for d in &self.draws {
            r = max(r, d.red);
            g = max(g, d.green);
            b = max(b, d.blue);
        }
        (r,g,b)
    }
}

impl FromStr for Game {
    type Err = GameParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const PREFIX: &'static str = "Game ";
        if !s.starts_with(PREFIX) {
            return GameParsingError::as_err(format!("Invalid input - expected line to start with '{:?}'", PREFIX));
        }

        let s = s[(PREFIX.len())..].trim();
        let colon_index = s.find(':')
            .ok_or_else(|| GameParsingError::from("Expected line to have a ':'"))
            ?;

        let digit_piece = s[..colon_index].trim();
        let draw_piece = s[(colon_index)+1..].trim();

        let id: u32 = digit_piece.parse()
            .map_err(|_e| GameParsingError::new(format!("Could not parse id '{:?}'", digit_piece)))
            ?;

        let mut game = Self {
            id,
            draws: Vec::new()
        };

        for draw_str in draw_piece.split(';') {
            let draw: GameDraw = draw_str.trim()
                .parse()
                ?;
            
            game.draws.push(draw);
        }

        Ok(game)
    }
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
                color => {
                    return GameParsingError::as_err(format!("Invalid color {:?}", color) );
                }
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

    fn as_err<T>(message: String) -> Result<T, Self> {
        Err(Self::new(message))
    }
}

impl fmt::Display for GameParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_parse_gamedraw() {
        let try1: GameDraw = "3 blue, 6 red, 5 green".parse().unwrap();
        assert!(try1.red == 6);
        assert!(try1.blue == 3);
        assert!(try1.green == 5);

        let try2: GameDraw = "79 green, 17 red".parse().unwrap();
        assert!(try2.red == 17);
        assert!(try2.blue == 0);
        assert!(try2.green == 79);
    }

    #[test]
    fn basic_parse_game() {
        let try1: Game = "Game 17: 5 blue, 3 red; 4 red, 16 blue, 9 green".parse().unwrap();
        assert!(try1.id == 17);
        assert!(try1.draws.len() == 2);
        assert!(try1.draws[0].red == 3);
        assert!(try1.draws[0].green == 0);
        assert!(try1.draws[0].blue == 5);
        assert!(try1.draws[1].red == 4);
        assert!(try1.draws[1].green == 9);
        assert!(try1.draws[1].blue == 16);
    }

    #[test]
    fn test_basic_minuse() {
        let game: Game = "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".parse().unwrap();
        let (r,g,b) = game.get_min_use();
        assert!(r == 6);
        assert!(g == 3);
        assert!(b == 2);
    }

    #[test]
    fn test_basic_power() {
        let game: Game = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".parse().unwrap();
        let p = game.get_power();
        assert!(p == 1560);
    }
}
