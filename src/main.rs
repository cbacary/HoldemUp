pub mod card;
pub mod hand;
pub mod equity;

use std::{io, vec};

use hand::{Hand, HandValue};
use card::card_flags::*;
use equity::equity::{calculate_odds, get_current_hand_value};

fn run_game_loop() {
    println!("Directions\n------------------------------------------\nCards 2 through 9 are denoted as numbers.\nCase Sensitive\nTEN = T\nJACK = J\nQUEEN = Q\nKING = K\nACE = A\nh = Heart\ns = spade\nc = club\nd = diamond\n------------------------------------------\n\n");
    loop {
        let mut your_hand = vec![];
        let mut board: Vec<u32> = vec![];
        let mut cards_to_draw = "".to_owned();
        println!("\n\nCards to draw: ");
        io::stdin().read_line(&mut cards_to_draw).expect("WRONG");
        let cards_to_draw: u32 = cards_to_draw[0..cards_to_draw.len() - 1].parse().unwrap();

        println!("\nYour hand: \r");
        let mut player_hand = String::new();
        io::stdin().read_line(&mut player_hand)
                   .expect("WRONG!");
        
        println!("\nBoard hand: \r");
        let mut board_hand = String::new();
        io::stdin().read_line(&mut board_hand)
                   .expect("fffff");

        if (player_hand.len() - 1 )% 2 != 0 || (board_hand.len() - 1) % 2 == 1 {
            println!("Incorrect length. {} {}", player_hand.len(), board_hand.len());
            continue;
        }

        let mut player_hand_chars = player_hand.chars();
        for _ in 0..(player_hand.len() / 2) {
            let mut s = String::new();
            s.push(player_hand_chars.next().unwrap());
            s.push(player_hand_chars.next().unwrap());
            let c = str_to_card(s);
            your_hand.push(c);
        }
        
        let mut board_hand_chars = board_hand.chars();
        for _ in 0..(board_hand.len() / 2) {
            let mut s = String::new();
            s.push(board_hand_chars.next().unwrap());
            s.push(board_hand_chars.next().unwrap());
            let c = str_to_card(s);
            board.push(c);
        }

        let deck = &mut Hand::construct_deck();

        let mut indicies_to_remove = vec![];

        for i in 0..your_hand.len() {
            for j in 0..deck.len() {
                if deck[j] == your_hand[i] {
                    indicies_to_remove.push(j);
                }
            }
        }

        indicies_to_remove.sort();
        indicies_to_remove.reverse();

        for i in indicies_to_remove {
            deck.remove(i);
        }

        let hand_value = get_current_hand_value(&mut your_hand.to_vec(), &mut board);
        
        let odds_map = calculate_odds(
            &mut your_hand, &mut board, deck, hand_value, 
            cards_to_draw, cards_to_draw, 1.);

        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        for (hand, chance) in odds_map {
            let h = HandValue::from_index(hand);
            println!("{:#?} has a chance {}", h, chance * 100.);
        }

    }
}

fn main() {
    run_game_loop();
}
