use std::vec;

use crate::card::{CardFlags::*,};

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandValue {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    Straight,
    Flush,
    FullHouse,
    FourKind,
    StraightFlush,
    RoyalFlush,
    RoyalStraight, // It's not a real thing but makes it easier to check if royal flush
}

#[derive(Debug)]
pub struct Hand {
    cards: Vec<u32>,
    hand_value: HandValue,
}

impl Hand {

    pub fn new() -> Hand {
        Hand { cards: vec![] , hand_value: HandValue::HighCard }
    }
    
    pub fn construct_deck() -> Vec<u32> {
        let mut deck = vec![];
        let mut current_card = 1;
        for i in 0..13 {
            // Starting suit is heart
            let mut current_suit = 8192; 
            for j in 0..4 {
                deck.push(current_card | current_suit);
                current_suit *= 2;
            }
            current_card *= 2;
        }
        return deck;
    }

    /// 'cards' must be sorted and suits must be removed.
    fn find_pair(&mut self, cards: &Vec<u32> ) -> HandValue {
        let mut first_pair_count = 1;
        let mut second_pair_count = 1;

        let current_pair_card = cards[0];
        let mut last_checked_index = 0;
        for i in 1..cards.len() {
            if current_pair_card != cards[i] {
                last_checked_index = i;
                break;
            }
            last_checked_index = i;
            first_pair_count += 1;
        }
        let old_first_pair_val = first_pair_count;
        if first_pair_count < 2 {
            first_pair_count = 0;
        }

        let next_card_pair_set = cards[last_checked_index];

        for i in (last_checked_index + 1)..cards.len() {
            if next_card_pair_set != cards[i] {
                break;
            }
            second_pair_count += 1;
        }
        let old_second_pair_count = second_pair_count;
        if second_pair_count < 2 {
            second_pair_count = 0;
        }

        // Do equity calculations
        

        let total_pair_count = first_pair_count + second_pair_count;
        let first_hand_val = match first_pair_count {
            2 => HandValue::OnePair,
            3 => HandValue::ThreeKind,
            4 => HandValue::FourKind,
            _ => HandValue::HighCard
        };
        let second_hand_val = match second_pair_count {
            2 => HandValue::OnePair,
            3 => HandValue::ThreeKind,
            4 => HandValue::FourKind,
            _ => HandValue::HighCard
        };
        let final_hand_val =  match first_pair_count + second_pair_count {
            4 => HandValue::TwoPair,
            5 => HandValue::FullHouse,
            _ => HandValue::HighCard
        };

        let mut v = vec![first_hand_val, second_hand_val, final_hand_val];
        v.sort();

        return v[v.len() - 1];
    }

     
}

impl HandValue {
    pub fn to_index(self,) -> usize {
        self as usize
    }

    pub fn from_index(index: u32) -> HandValue {
        match index {
            0 => HandValue::HighCard,
            1 => HandValue::OnePair,
            2 => HandValue::TwoPair,
            3 => HandValue::ThreeKind,
            4 => HandValue::Straight,
            5 => HandValue::Flush,
            6 => HandValue::FullHouse,
            7 => HandValue::FourKind,
            8 => HandValue::StraightFlush,
            9 => HandValue::RoyalFlush,
            10 => HandValue::RoyalStraight,
            _ => HandValue::HighCard,
        }
    } 
}