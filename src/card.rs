
pub mod CardFlags {
    pub const Two: u32 = 1;
    pub const Three: u32 = 2;  
    pub const Four: u32 = 4;
    pub const Five: u32 = 8;
    pub const Six: u32 = 16;
    pub const Seven: u32 = 32;
    pub const Eight: u32 = 64;
    pub const Nine: u32 = 128;
    pub const Ten: u32 = 256;
    pub const Jack: u32 = 512;
    pub const Queen: u32 = 1024;
    pub const King: u32 = 2048;
    pub const Ace: u32 = 4096;
    pub const Heart: u32 = 8192;
    pub const Diamond: u32 = 16384;
    pub const Club: u32 = 32768;
    pub const Spade: u32 = 65536;

    pub fn deck_to_str(deck: Vec<u32>) -> String {
        let mut s = "".to_owned();
        for i in deck {
            let rank = i | Heart | Diamond | Club | Spade;
            let rank = rank ^ Heart ^ Diamond ^ Club ^ Spade;
            let suit = i | Two | Three | Four | Five | Six | Seven | Eight | Nine | Ten | Jack | King | Queen | Ace;
            let suit = suit ^ Two ^ Three ^ Four ^ Five ^ Six ^ Seven ^ Eight ^ Nine ^ Ten ^ Jack ^ King ^ Queen ^ Ace;
            s.push_str(
                match rank {
                    Two => "Two",
                    Three => "Three",  
                    Four => "Four",
                    Five => "Five",
                    Six => "Six",
                    Seven => "Seven",
                    Eight => "Eight",
                    Nine => "Nine",
                    Ten => "Ten",
                    Jack => "Jack",
                    Queen => "Queen",
                    King => "King",
                    Ace => "Ace",
                    Heart => "Heart",
                    Diamond => "Diamond",
                    Club => "Club",
                    Spade => "Spade",
                    _ => ("")
            });
            s.push_str(
                match suit {
                    Heart => " Heart",
                    Diamond => " Diamond",
                    Club => " Club",
                    Spade => " Spade",
                    _ => ("")
                }
            );
            s.push_str("\n");
        }
        return s;
    }

    pub fn str_to_card(card: String) -> u32 {
        if card.len() != 2 {
            return 0;
        }

        let chr_0 = card.as_bytes()[0] as char;
        let chr_1 = card.as_bytes()[1] as char;

        let rank = match chr_0 {
            '2' => Two,
            '3' => Three,
            '4' => Four,
            '5' => Five,
            '6' => Six,
            '7' => Seven, 
            '8' => Eight,
            '9' => Nine,
            'T' => Ten,
            'J' => Jack,
            'Q' => Queen,
            'K' => King,
            'A' => Ace,
            _ => 0
        };
        if rank == 0 {
            return 0;
        }
        let suit = match chr_1 {
            'h' => Heart,
            'd' => Diamond,
            'c' => Club,
            's' => Spade,
            _ => 0
        };
        if suit == 0 {
            return 0;
        }
        return rank | suit;
    }

    pub fn get_rank(card: u32) -> u32 {
        let c = card & (Two | Three | Four | Five | Six | Seven | Eight | Nine | Ten | Jack | King | Queen | Ace);
        return c;
    }

    pub fn get_suit(card: u32) -> u32 {
        let c = card & (Heart | Diamond | Spade | Club);
        return c;
    }
}


// #[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]

// pub enum Card {
//     Two = 0,
//     Three = 1,
//     Four = 2,
//     Five = 3,
//     Six = 4,
//     Seven = 5,
//     Eight = 6,
//     Nine = 7,
//     Ten = 8,
//     Jack = 9,
//     Queen = 10,
//     King = 11,
//     Ace = 12,
//     Heart = 13,
//     Diamond = 14,
//     Club = 15,
//     Spade = 16,

// }

// pub enum Suit {
//     Heart,
//     Diamond,
//     Club,
//     Spade,
// }

// impl Card {
//     pub fn to_index(&self, ) -> usize {
//         *self as usize
//     }

// }