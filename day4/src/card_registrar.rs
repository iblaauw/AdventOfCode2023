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

    pub fn register(&self, card_id: u32, num_matches: u32) {
        // The idea is that at this point "card_id" has a certain number of copies already in our hash map.
        // We add 1 to it (for the original).
        // We know the number of matches turns into a list of ids. We multiply this by the number of copies that we have and add them to the hashmap.
        // We add the number of card_id to our total.
        panic!("Not Impl");
    }

    pub fn get_total_copies(&self) -> u32 {
        panic!("Not Impl");
    }
}