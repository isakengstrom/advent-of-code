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
        let min_length = cmp::min(self_card_occurrences.len(), other_card_occurrences.len());

        // Score if card types are different
        for index in 0..min_length{
            let self_card_occurrence = self_card_occurrences[index];
            let other_card_occurrence = other_card_occurrences[index];

            // Reverse ordering, to get Min-heap
            if self_card_occurrence.1 > other_card_occurrence.1 {
                return Ordering::Less
            }
            else if self_card_occurrence.1 < other_card_occurrence.1 {
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
                return Ordering::Less
            }
            else if ordering.is_lt() {
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
        // The only time a hand with J's is counted as a hand without J, is when all cards are J.
        // Other times, J's are removed from the hand, but the number of J's are added to the count
        // of the non-J card that occurs the most in the hand.
        if hand == "JJJJJ" {
            return Vec::from([('J', 5)])
        }

        let mut card_counts = HashMap::with_capacity(5);

        for card in hand.chars() {
            // Skip adding J's
            if card == 'J' {
                continue
            }

            let count = card_counts.entry(card).or_insert(0);
            *count += 1;
        }

        let mut card_counts_vec: Vec<(char, usize)> = card_counts.into_iter().collect();
        // Sort by count in descending order
        card_counts_vec.sort_by(|a, b| b.1.cmp(&a.1));

        // Add the number of J's to the count of the non-J card that occurs the most in the hand.
        let j_count = hand.chars().filter(|c| *c == 'J').count();
        card_counts_vec[0].1 += j_count;

        card_counts_vec
    }
}