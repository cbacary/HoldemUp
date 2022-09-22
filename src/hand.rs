use std::vec;


#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandValue {
    HighCard,
    OnePair,
    TWOPair,
    THREEKind,
    Straight,
    Flush,
    FullHouse,
    FOURKind,
    StraightFlush,
    RoyalFlush,
    RoyalStraight, // It's not a real thing but makes it easier to check if royal flush
}

#[derive(Debug)]
#[allow(dead_code)]
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
        for _ in 0..13 {
            // Starting suit is heart
            let mut current_suit = 8192; 
            for _ in 0..4 {
                deck.push(current_card | current_suit);
                current_suit *= 2;
            }
            current_card *= 2;
        }
        return deck;
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
            2 => HandValue::TWOPair,
            3 => HandValue::THREEKind,
            4 => HandValue::Straight,
            5 => HandValue::Flush,
            6 => HandValue::FullHouse,
            7 => HandValue::FOURKind,
            8 => HandValue::StraightFlush,
            9 => HandValue::RoyalFlush,
            10 => HandValue::RoyalStraight,
            _ => HandValue::HighCard,
        }
    } 
}