use std::collections::HashMap;


pub struct CardCopyRegistrar {
    copies: HashMap<u32, u32>,
    total: u32,
}

impl CardCopyRegistrar {
    pub fn new() -> Self {
        CardCopyRegistrar {
            copies: HashMap::new(),
            total: 0,
        }
    }

    pub fn register(&mut self, card_id: u32, num_matches: usize) {
        // The idea is that at this point "card_id" has a certain number of copies already in our hash map.
        // We add 1 to it (for the original).
        // We know the number of matches turns into a list of ids. We multiply this by the number of copies that we have and add them to the hashmap.
        // We add the number of card_id to our total.

        // Deref needed because unwrap_or provides a ref to the underlying value.
        let current_copies = *(self.copies.get(&card_id).unwrap_or(&0));
        let total_copies = current_copies + 1;
        self.total += total_copies;
        
        // Early bail on all the work here if it's unneeded.
        if num_matches == 0 {
            return;
        }

        // Convert from usize to u32
        let real_matches: u32 = num_matches.try_into().expect("Overflow - too many matches");

        // For each of the next X cards, add Y copies, where:
        //   X = real_matches
        //   Y = total_copies
        let new_copies_range = (card_id+1)..(card_id+real_matches+1);
        for new_copy_id in new_copies_range {
            let value: &mut u32 = self.copies.entry(new_copy_id).or_insert(0);
            *value += total_copies;
        }
    }

    pub fn get_total_copies(&self) -> u32 {
        self.total
    }
}