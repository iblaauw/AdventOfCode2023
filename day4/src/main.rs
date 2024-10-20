mod scratch_card;
mod card_registrar;

fn main() {
    println!("Hello, world!");

    let helper = advent_utils::Helper::new(4);
    helper.print_header();
    let solution = solve(helper.open_file());
    helper.print_solution(solution);

    let solution2 = solve2(helper.open_file());
    helper.print_solution(solution2);
}

fn solve(helper: advent_utils::FileHelper) -> u32 {
    helper.into_lines()
        .map(|s| s.parse::<scratch_card::ScratchCard>().expect("Invalid card"))
        .map(|c| c.get_value())
        .sum()
}

fn solve2(helper: advent_utils::FileHelper) -> u32 {
    let mut card_copies = card_registrar::CardCopyRegistrar::new();
    for line in helper.into_lines() {
        let card : scratch_card::ScratchCard = line.parse().expect("Invalid card??");
        let id = card.get_id();
        card_copies.register(id, card.get_num_matches());
    }
    card_copies.get_total_copies()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given() {
        let helper = advent_utils::Helper::new(4);
        let fh = helper.open_file_with_name("given_test");
        let solution = solve(fh);
        assert_eq!(solution, 13);
    }

    #[test]
    fn test_given2() {
        let helper = advent_utils::Helper::new(4);
        let fh = helper.open_file_with_name("given_test");
        let solution = solve2(fh);
        assert_eq!(solution, 30);
    }
}
