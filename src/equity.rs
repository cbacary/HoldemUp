
#![feature(hash_drain_filter)]
pub mod Equity {
    use crate::hand::{Hand, HandValue};
    use crate::card::CardFlags::*;  

    use std::collections::HashMap;
    use std::vec;
 
    // possibly a recursive thingy here?
    pub fn calculate_odds(
            your_hand: &mut Vec<u32>, board: &mut Vec<u32>,  deck: &mut Vec<u32>, 
            original_hand: HandValue, original_cards_left_to_draw: u32, 
            cards_left_to_draw: u32, current_chance: f64)  -> HashMap<u32, f64>
    {
        let current_hand_value = get_current_hand_value(&mut your_hand.to_vec(), &mut board.to_vec());
        if cards_left_to_draw == 0 {
            let mut h: HashMap<u32, f64> = HashMap::new();
            h.insert(current_hand_value.to_index() as u32, current_chance);
            return h;
        }
        let mut hand_list: HashMap<u32, f64> = HashMap::new();
        for card_num in 0..deck.len() {
            let org_card = deck[card_num];
            let chance = (1. as f64) / (deck.len() as f64 - 1. ) * current_chance;
            board.push(org_card);
            // let hand_val = get_current_hand_value(&mut your_hand.to_vec(), &mut board.to_vec());
            deck.remove(card_num);
            let map = calculate_odds(your_hand, board, deck, original_hand, original_cards_left_to_draw, cards_left_to_draw - 1, chance);
            deck.insert(card_num, org_card);
            board.remove(board.len() - 1);
            for (key, val) in map.iter() {
                if *key > original_hand.to_index() as u32 {
                    hand_list.entry(*key).and_modify(|counter| *counter += val).or_insert(*val);
                }
            }
        }
        return hand_list;
    }

    pub fn get_current_hand_value(your_hand: &mut Vec<u32>, board: &mut Vec<u32> ) -> HandValue {
        let mut hand_values: Vec<HandValue> = vec![];

        let mut all_cards: Vec<u32>= vec![];
        all_cards.append(your_hand);
        all_cards.append(board);
        
        let mut suits = get_only_suits(all_cards.to_vec());
        let mut ranks = get_removed_suits(all_cards.to_vec());

        suits.sort();
        ranks.sort();

        hand_values.push(find_pair(&mut ranks.to_vec()));        
        hand_values.push(find_straight(&ranks));
        hand_values.push(find_flush(suits));

        hand_values.sort();

        return hand_values[hand_values.len() - 1];

    }

    pub fn find_pair(cards: &mut Vec<u32>)  -> HandValue {
        
        cards.sort();

        let mut card_map: HashMap<u32, u32> = HashMap::new();

        for i in 0..cards.len() {
            card_map.entry(cards[i]).and_modify(|counter| *counter += 1).or_insert(1);
            // if current_pair_card != cards[i] {
            //     last_checked_index = i;
            //     break;
            // }
            // last_checked_index = i;
            // first_pair_count += 1;
        }
        let mut greatest_one = 0;
        let mut greatest_two = 0;

        for (key, val) in card_map.iter() {
            if val > &greatest_one {
                greatest_one = *val;
            } else if val > &greatest_two {
                greatest_two = *val;
            }
        }

        // let old_first_pair_val = first_pair_count;
        // if first_pair_count < 2 {
        //     first_pair_count = 0;
        // }

        // let next_card_pair_set = cards[last_checked_index];

        // for i in (last_checked_index + 1)..cards.len() {
        //     if next_card_pair_set != cards[i] {
        //         break;
        //     }
        //     second_pair_count += 1;
        // }
        // let old_second_pair_count = second_pair_count;
        // if second_pair_count < 2 {
        //     second_pair_count = 0;
        // }

        // Do equity calculations
        if greatest_one < 2 {
            greatest_one = 0;
        }
        if greatest_two < 2 {
            greatest_two = 0;
        }
        let first_hand_val = match greatest_one {
            2 => HandValue::OnePair,
            3 => HandValue::ThreeKind,
            4 => HandValue::FourKind,
            _ => HandValue::HighCard
        };
        let second_hand_val = match greatest_two {
            2 => HandValue::OnePair,
            3 => HandValue::ThreeKind,
            4 => HandValue::FourKind,
            _ => HandValue::HighCard
        };
        let final_hand_val =  match greatest_one + greatest_two {
            4 => HandValue::TwoPair,
            5 => HandValue::FullHouse,
            _ => HandValue::HighCard
        };

        let mut v = vec![first_hand_val, second_hand_val, final_hand_val];
        v.sort();

        return v[v.len() - 1];
    }

    /// 'cards' must be sorted and suits must be removed.
    fn find_straight(cards: &Vec<u32>) -> HandValue {
        // You could also do a bitshift left here but they 
        // are the same thing and I think this is easeir to 
        // understand.
        let mut next_value = cards[0] * 2;
        let mut cards_in_a_row = 1;
        for i in 1..cards.len() {
            
            if cards[i] == next_value {
                cards_in_a_row += 1;
                next_value *= 2;
            } else {
                cards_in_a_row = 1;
                next_value = cards[i] * 2;
            }
            if cards_in_a_row == 5 {
                if cards[i] == Ace {
                    return HandValue::RoyalStraight;
                }
                return HandValue::Straight;
            }
        }
        return HandValue::HighCard;
    }

    /// 'cards' must be sorted by suit.
    fn find_flush(cards: Vec<u32>) -> HandValue {
        if cards.len() < 5 {
            return HandValue::HighCard;
        }
        let mut same_suit_count = 1;
        let mut current_suit = cards[0];
        for i in 1..cards.len() {
            if current_suit == cards[i] {
                same_suit_count += 1;
            } else {
                same_suit_count = 1;
                current_suit = cards[i];
            }
            if same_suit_count == 5 {
                return HandValue::Flush;
            }
        }
        return HandValue::HighCard;
    }

    fn get_removed_suits(cards: Vec<u32>) -> Vec<u32> {
        let mut cards_no_suits: Vec<u32> = vec![];
        for i in cards {
            // let c = *i  (Heart | Diamond | Spade | Club);
            let c = i & (Two | Three | Four | Five | Six | Seven | Eight | Nine | Ten | Jack | King | Queen | Ace);

            // let c  = c ^ Heart ^ Diamond ^ Spade ^ Club;
            cards_no_suits.push(c);
        }
        return cards_no_suits;
    }

    fn get_only_suits(cards: Vec<u32>) -> Vec<u32> {
        let mut cards_suits: Vec<u32> = vec![];
        for i in cards {
            let c = i | Two | Three | Four | Five | Six | Seven | Eight | Nine | Ten | Jack | King | Queen | Ace;
            let c = c ^ Two ^ Three ^ Four ^ Five ^ Six ^ Seven ^ Eight ^ Nine ^ Ten ^ Jack ^ King ^ Queen ^ Ace;
            cards_suits.push(c);
        }
        return cards_suits;
    }

}