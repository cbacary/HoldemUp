
pub mod card_flags {
    pub const TWO: u32 = 1;
    pub const THREE: u32 = 2;  
    pub const FOUR: u32 = 4;
    pub const FIVE: u32 = 8;
    pub const SIX: u32 = 16;
    pub const SEVEN: u32 = 32;
    pub const EIGHT: u32 = 64;
    pub const NINE: u32 = 128;
    pub const TEN: u32 = 256;
    pub const JACK: u32 = 512;
    pub const QUEEN: u32 = 1024;
    pub const KING: u32 = 2048;
    pub const ACE: u32 = 4096;
    pub const HEART: u32 = 8192;
    pub const DIAMOND: u32 = 16384;
    pub const CLUB: u32 = 32768;
    pub const SPADE: u32 = 65536;

    pub fn deck_to_str(deck: Vec<u32>) -> String {
        let mut s = "".to_owned();
        for i in deck {
            let rank = i | HEART | DIAMOND | CLUB | SPADE;
            let rank = rank ^ HEART ^ DIAMOND ^ CLUB ^ SPADE;
            let suit = i | TWO | THREE | FOUR | FIVE | SIX | SEVEN | EIGHT | NINE | TEN | JACK | KING | QUEEN | ACE;
            let suit = suit ^ TWO ^ THREE ^ FOUR ^ FIVE ^ SIX ^ SEVEN ^ EIGHT ^ NINE ^ TEN ^ JACK ^ KING ^ QUEEN ^ ACE;
            s.push_str(
                match rank {
                    TWO => "TWO",
                    THREE => "THREE",  
                    FOUR => "FOUR",
                    FIVE => "FIVE",
                    SIX => "SIX",
                    SEVEN => "SEVEN",
                    EIGHT => "EIGHT",
                    NINE => "NINE",
                    TEN => "TEN",
                    JACK => "JACK",
                    QUEEN => "QUEEN",
                    KING => "KING",
                    ACE => "ACE",
                    HEART => "HEART",
                    DIAMOND => "DIAMOND",
                    CLUB => "CLUB",
                    SPADE => "SPADE",
                    _ => ("")
            });
            s.push_str(
                match suit {
                    HEART => " HEART",
                    DIAMOND => " DIAMOND",
                    CLUB => " CLUB",
                    SPADE => " SPADE",
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
            '2' => TWO,
            '3' => THREE,
            '4' => FOUR,
            '5' => FIVE,
            '6' => SIX,
            '7' => SEVEN, 
            '8' => EIGHT,
            '9' => NINE,
            'T' => TEN,
            'J' => JACK,
            'Q' => QUEEN,
            'K' => KING,
            'A' => ACE,
            _ => 0
        };
        if rank == 0 {
            return 0;
        }
        let suit = match chr_1 {
            'h' => HEART,
            'd' => DIAMOND,
            'c' => CLUB,
            's' => SPADE,
            _ => 0
        };
        if suit == 0 {
            return 0;
        }
        return rank | suit;
    }

    pub fn get_rank(card: u32) -> u32 {
        let c = card & (TWO | THREE | FOUR | FIVE | SIX | SEVEN | EIGHT | NINE | TEN | JACK | KING | QUEEN | ACE);
        return c;
    }

    pub fn get_suit(card: u32) -> u32 {
        let c = card & (HEART | DIAMOND | SPADE | CLUB);
        return c;
    }
}
