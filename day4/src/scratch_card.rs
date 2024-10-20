use advent_utils::generic_error::GenericError;
use std::collections::HashSet;
use std::str::FromStr;

pub struct ScratchCard {
    id: u32,
    winners: HashSet<u32>,
    values: HashSet<u32>,
}

impl ScratchCard {
    pub fn get_value(&self) -> u32 {
        let correct_count = self.get_num_matches();
        if correct_count == 0 {
            0
        } else {
            // 1 correct -> 1
            // 2 correct -> 2
            // 3 correct -> 4
            // 4 correct -> 8...
            1 << (correct_count - 1)
        }
    }

    pub fn get_num_matches(&self) -> usize {
        let intersect = self.winners.intersection(&self.values);
        intersect.count()
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
}

fn hashset_from_numbers(numbers: &str) -> Result<HashSet<u32>, GenericError> {
    let result: Result<HashSet<u32>, std::num::ParseIntError> = numbers.split_whitespace()
        .map(|s| s.parse::<u32>())
        .collect();
    let value = result?; // Take advantage of automatic error conversion
    Ok(value)
}

impl FromStr for ScratchCard {
    type Err = GenericError;

    fn from_str(line: &str) -> Result<Self, GenericError> {
        let mut card = ScratchCard {
            id: 0,
            winners: HashSet::new(),
            values: HashSet::new(),
        };

        // Strip the "card" prefix
        let line = line.strip_prefix("Card ")
            .ok_or_else(
                || GenericError::new("Invalid line - did not start with 'Card '".to_string())
            )?;

        let split = line.split_once(':');
        if split.is_none() {
            return GenericError::as_err("Invalid line - found no ':' symbol".to_string());
        }

        let (id_piece, remaining) = split.unwrap();

        card.id = id_piece.trim().parse()?;

        let split2 = remaining.split_once('|');
        if split2.is_none() {
            return GenericError::as_err("Invalid line - found no '|' symbol".to_string());
        }

        let (winners_piece, values_piece) = split2.unwrap();
        let winners_piece = winners_piece.trim();
        let values_piece = values_piece.trim();

        card.winners = hashset_from_numbers(winners_piece)?;
        card.values = hashset_from_numbers(values_piece)?;

        Ok(card)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() -> Result<(), GenericError> {
        let card : ScratchCard = "Card 5: 7 12 15 9 | 10 40 42 9".parse()?;
        assert_eq!(card.id, 5);
        assert_eq!(card.winners, HashSet::from([7, 12, 15, 9]));
        assert_eq!(card.values, HashSet::from([10, 40, 42, 9]));
        assert_eq!(card.get_value(), 1);
        Ok(())
    }

    #[test]
    fn funky_spacing() {
        let card: ScratchCard = "Card  7:  1 12  3 15 |  7  9 12  3".parse().expect("Invalid card");
        assert_eq!(card.id, 7);
        assert_eq!(card.winners, HashSet::from([1, 12, 3, 15]));
        assert_eq!(card.values, HashSet::from([7, 9, 12, 3]));
        assert_eq!(card.get_value(), 2);
    }
}