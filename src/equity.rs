
pub mod Equity {
    use crate::hand::{HandValue, Hand};
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
            deck.remove(card_num);
            let map = calculate_odds(&mut your_hand.to_vec(), &mut board.to_vec(), deck, original_hand, original_cards_left_to_draw, cards_left_to_draw - 1, chance);
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
        all_cards.append(&mut your_hand.to_vec());
        all_cards.append(&mut board.to_vec());
        
        let mut suits = get_only_suits(all_cards.to_vec());
        let mut ranks = get_removed_suits(all_cards.to_vec());
        let hand_ranks = get_removed_suits(your_hand.to_vec());
        let board_ranks = get_removed_suits(board.to_vec());

        suits.sort();
        ranks.sort();

        hand_values.push(find_pair(hand_ranks.to_vec(), board_ranks.to_vec()));        
        hand_values.push(find_straight(&ranks.to_vec()));
        hand_values.push(find_flush(suits.to_vec()));

        hand_values.sort();

        return hand_values[hand_values.len() - 1];

    }

    pub fn find_pair(cards: Vec<u32>, board: Vec<u32>)  -> HandValue {
        
        // let mut cards = cards.to_vec();
        // cards.sort();
        // board.sort();


        let mut hand_map: HashMap<u32, u32> = HashMap::new();
        let mut board_map: HashMap<u32, u32> = HashMap::new();
        
        for i in cards {
            hand_map.entry(i).and_modify(|counter| *counter += 1).or_insert(1);
        }

       
        for i in board {
            board_map.entry(i).and_modify(|counter| *counter += 1).or_insert(1);
        }
        
        // Check if board has pair
        // let mut board_pair = 0;
        // for val in board_map.values() {
        //     if *val >= 2 && *val > board_pair {
        //         board_pair = *val;
        //     }
        // }

        // // Check if we have a pair
        // let mut our_pair = 0;
        // for val in hand_map.values() {
        //     if *val >= 2 {
        //         our_pair = *val;
        //     }
        // }

        // let mut full_h = HandValue::HighCard;
        // let mut two_p = HandValue::HighCard;
        // if our_pair == 2 {
        //     // Check if full house
        //     if board_pair >= 3 {
        //         full_h = HandValue::FullHouse;
        //     } else if board_pair  == 2 {
        //         two_p = HandValue::TwoPair;
        //     }
        // }
        
        // Check if pair exists between board and player.
        let mut pocket = HandValue::HighCard;
        for i in hand_map.values() {
            if i == &2 {
                pocket = HandValue::OnePair;
            }
        }

        let mut larger_pair = 0;
        let mut smaller_pair = 0;
        let mut board_pair = 0;
        for (key, val) in board_map {
            if let Some(v) = hand_map.get(&key) {
                let pair_size = val + *v;
                if pair_size > larger_pair {
                    smaller_pair = larger_pair;
                    larger_pair = pair_size;                    
                } else if pair_size > smaller_pair {
                    smaller_pair = pair_size;
                }
            }
            if val >= 2 && val > board_pair {
                board_pair = val;
            }
        }
       
        let first_hand_val = match larger_pair {
            2 => HandValue::OnePair,
            3 => HandValue::ThreeKind,
            4 => HandValue::FourKind,
            _ => HandValue::HighCard
        };
        let second_hand_val = match smaller_pair {
            2 => HandValue::OnePair,
            3 => HandValue::ThreeKind,
            4 => HandValue::FourKind,
            _ => HandValue::HighCard
        };
        let final_hand_val =  match larger_pair + smaller_pair {
            4 => HandValue::TwoPair,
            5 => HandValue::FullHouse,
            _ => HandValue::HighCard
        };
        if pocket == HandValue::OnePair && board_pair >= 2 {
            if board_pair == 2 {
                pocket = HandValue::TwoPair;
            }
        }

        let mut v = vec![first_hand_val, second_hand_val, final_hand_val, pocket];
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

    pub fn get_removed_suits(cards: Vec<u32>) -> Vec<u32> {
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

#[cfg(test)]
mod tests {
    use super::Equity::{get_current_hand_value};
    use crate::hand::HandValue;
    use rand::Rng;
    use crate::card::CardFlags::*;
    #[test]
    fn get_current_hand_value_test() {
        let mut card: u32 = 1;
        let mut under_card: u32 = 4;
        let suits = vec![Heart, Diamond, Spade, Club];
        for i in 1..12 {
            let suit1 = suits[rand::thread_rng().gen_range(0..suits.len())];
            let suit2 = suits[rand::thread_rng().gen_range(0..suits.len())];
            let mut hand = vec![card | suit1, (card) | suit2, under_card | suit2];
            let mut board: Vec<u32> = vec![];
            let hand_val = get_current_hand_value(&mut hand.to_vec(), &mut board);

            assert_eq!(hand_val, HandValue::OnePair);
            under_card = 1;
            card *= 2;
            
        }
    }
}