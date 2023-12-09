use std::cmp;
use std::cmp::Ordering;
use std::collections::HashMap;

static CARD_STRENGTHS: once_cell::sync::Lazy<HashMap<char, u16>> = once_cell::sync::Lazy::new(|| {
    let mut card_strengths = HashMap::new();
    card_strengths.insert('J', 1);
    card_strengths.insert('2', 2);
    card_strengths.insert('3', 3);
    card_strengths.insert('4', 4);
    card_strengths.insert('5', 5);
    card_strengths.insert('6', 6);
    card_strengths.insert('7', 7);
    card_strengths.insert('8', 8);
    card_strengths.insert('9', 9);
    card_strengths.insert('T', 10);
    card_strengths.insert('Q', 12);
    card_strengths.insert('K', 13);
    card_strengths.insert('A', 14);
    card_strengths
});

#[derive(Debug)]
pub struct CardHand2 {
    pub value: String,
}

impl Ord for CardHand2 {
    fn cmp(&self, other: &Self) -> Ordering {
        Self::score_cards(self, other)
    }
}

impl PartialOrd for CardHand2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for CardHand2 {}

impl PartialEq for CardHand2 {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl CardHand2 {
    pub fn new(value: String) -> Self {
        CardHand2 { value }
    }

    fn score_cards(&self, other: &Self) -> Ordering {
        let self_hand = self.value.clone();
        let other_hand = other.value.clone();

        let self_card_occurrences = Self::count_hand(self_hand.as_str());
        let other_card_occurrences = Self::count_hand(other_hand.as_str());

        let self_js: Vec<_> = self_card_occurrences.iter().filter(|co| co.0 == 'J').collect();
        let other_js: Vec<_> = other_card_occurrences.iter().filter(|co| co.0 == 'J').collect();

        let self_non_js: Vec<_> = self_card_occurrences.iter().filter(|co| co.0 != 'J').collect();
        let other_non_js: Vec<_> = other_card_occurrences.iter().filter(|co| co.0 != 'J').collect();

        let self_js_count = if !self_js.is_empty() { self_js[0].1 } else { 0 };
        let other_js_count = if !other_js.is_empty() { other_js[0].1 } else { 0 };

        //let first_self_card_is_j = self_card_occurrences[0].0 == 'J';
        //let first_other_card_is_j = other_card_occurrences[0].0 == 'J';

        if self_js_count == 5 && other_non_js[0].1 < 5 {
            return Ordering::Less
        }

        if other_js_count == 5 && self_non_js[0].1 < 5 {
            return Ordering::Greater
        }

        if self_js_count == 5 && other_non_js[0].1 == 5 {
            return Ordering::Greater
        }

        if other_js_count == 5 && self_non_js[0].1 == 5 {
            return Ordering::Less
        }


        let min_length = cmp::min(self_non_js.len(), other_non_js.len());
        // Score if card types are different
        for index in 0..min_length{

            let self_card_occurrence = self_non_js[index];
            let other_card_occurrence = other_non_js[index];

            //let self_is_j = self_card_occurrence.0 == 'J';
            //let other_is_j = other_card_occurrence.0 == 'J';

            let self_card_count = self_card_occurrence.1 + self_js_count;
            let other_card_count = other_card_occurrence.1 + other_js_count;
            // Reverse ordering, to get Min-heap
            if self_card_count > other_card_count {
                //println!("1 -> {self_hand} higher than {other_hand} -> {self_card_count} vs {other_card_count} at {index}");
                return Ordering::Less
            }
            else if self_card_count < other_card_count {
                //println!("2 -> {other_hand} higher than {self_hand} -> {other_card_count} vs {self_card_count} at {index}");
                return Ordering::Greater
            }
        }

        // Score if card types are the same, base score on card strengths
        let self_cards: Vec<_> = self_hand.chars().collect();
        let other_cards: Vec<_> = other_hand.chars().collect();
        for index in 0..self_hand.len() {
            let self_card = self_cards[index];
            let other_card = other_cards[index];
            //println!("{} vs {} -> self ({}) {} other {}", self.value, other.value, index, self_card, other_card);
            let ordering = Self::compare_cards(self_card, other_card);

            // Reverse ordering, to get Min-heap
            if ordering.is_gt() {
                //println!("3 -> {self_hand} higher than {other_hand}");
                return Ordering::Less
            }
            else if ordering.is_lt() {
                //println!("4 -> {other_hand} higher than {self_hand}");
                return Ordering::Greater
            }
        }

        panic!("Something terrible happened when scoring the cards :/")
    }

    fn compare_cards(card_1: char, card_2: char) -> Ordering {
        let strength_of_card_1 = *CARD_STRENGTHS.get(&card_1).unwrap();
        let strength_of_card_2 = *CARD_STRENGTHS.get(&card_2).unwrap();

        strength_of_card_1.cmp(&strength_of_card_2)
    }

    fn count_hand(hand: &str) -> Vec<(char, usize)> {
        let mut card_counts = HashMap::with_capacity(5);

        for card in hand.chars() {
            let count = card_counts.entry(card).or_insert(0);
            *count += 1;
        }

        let mut card_counts_vec: Vec<(char, usize)> = card_counts.into_iter().collect();
        // Sort by count in descending order
        card_counts_vec.sort_by(|a, b| b.1.cmp(&a.1));

        card_counts_vec
    }
}