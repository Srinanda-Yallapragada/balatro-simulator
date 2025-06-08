// game state machine will need to track
// jokers
// number of hands played, skipped blinds, etc for the jokers
// planet cards used for strength of hand played
// track if the special hands like flush 5 are played to start showing eris planet card
// deck of 52 cards and their editions
// track dollaroos
//

// NOTE: We assume all jokers are unlocked.
// NOTE: We assume that our simulation ends at ante 8
// below is the game struct from the lua code of Balatro

use std::fmt::Display;
use crate::deck;
use crate::jokers::*;


// won = false,
// round_scores = {
// furthest_ante = {label = 'Ante', amt = 0},
// furthest_round = {label = 'Round', amt = 0},
// hand = {label = 'Best Hand', amt = 0},
// poker_hand = {label = 'Most Played Hand', amt = 0},
// new_collection = {label = 'New Discoveries', amt = 0},
// cards_played = {label = 'Cards Played', amt = 0},
// cards_discarded = {label = 'Cards Discarded', amt = 0},
// times_rerolled = {label = 'Times Rerolled', amt = 0},
// cards_purchased = {label = 'Cards Purchased', amt = 0},
// },
//
// = {},
// consumeable_usage = {},
// hand_usage = {},
// last_tarot_planet = nil,
// win_ante = 8,
// stake = 1,
// modifiers = {},
// starting_params = get_starting_params(),
// banned_keys = {},
// round = 0,
// probabilities = {
// normal = 1,
// },
// bosses_used = bosses_used,
// pseudorandom = {},
// starting_deck_size = 52,
// ecto_minus = 1,
// pack_size = 2,
// skips = 0,
// STOP_USE = 0,
// edition_rate = 1,
// joker_rate = 20,
// tarot_rate = 4,
// planet_rate = 4,
// spectral_rate = 0,
// playing_card_rate = 0,
// consumeable_buffer = 0,
// joker_buffer = 0,
// discount_percent = 0,
// interest_cap = 25,
// interest_amount = 1,
// inflation = 0,
// hands_played = 0,
// unused_discards = 0,
// perishable_rounds = 5,
// rental_rate = 3,
// blind =  nil,
// chips = 0,
// chips_text = '0',
// voucher_text = '',
// dollars = 0,
// max_jokers = 0,
// bankrupt_at = 0,
// current_boss_streak = 0,
// base_reroll_cost = 5,
// blind_on_deck = nil,
// sort = 'desc',
// previous_round = {
// dollars = 4
// },
// tags = {},
// tag_tally = 0,
// pool_flags = {},
// used_jokers = {},
// used_vouchers = {},
// current_round = {
// current_hand = {
// chips = 0,
// chip_text = '0',
// mult = 0,
// mult_text = '0',
// chip_total = 0,
// chip_total_text = '',
// handname = "",
// hand_level = ''
// },
// used_packs = {},
// cards_flipped = 0,
// round_text = 'Round ',
// idol_card = {suit = 'Spades', rank = 'Ace'},
// mail_card = {rank = 'Ace'},
// ancient_card = {suit = 'Spades'},
// castle_card = {suit = 'Spades'},
// hands_left = 0,
// hands_played = 0,
// discards_left = 0,
// discards_used = 0,
// dollars = 0,
// reroll_cost = 5,
// reroll_cost_increase = 0,
// jokers_purchased = 0,
// free_rerolls = 0,
// round_dollars = 0,
// dollars_to_be_earned = '!!!',
// most_played_poker_hand = 'High Card',
// },
// round_resets = {
// hands = 1,
// discards = 1,
// reroll_cost = 1,
// temp_reroll_cost = nil,
// temp_handsize = nil,
// ante = 1,
// blind_ante = 1,
// blind_states = {Small = 'Select', Big = 'Upcoming', Boss = 'Upcoming'},
// loc_blind_states = {Small = '', Big = '', Boss = ''},
// blind_choices = {Small = 'bl_small', Big = 'bl_big'},
// boss_rerolled = false,
// },
// round_bonus = {
// next_hands = 0,
// discards = 0,
// },
// shop = {
// joker_max = 2,
// },
// cards_played = {
// ['Ace'] = {suits = {}, total = 0},
// ['2'] = {suits = {}, total = 0},
// ['3'] = {suits = {}, total = 0},
// ['4'] = {suits = {}, total = 0},
// ['5'] = {suits = {}, total = 0},
// ['6'] = {suits = {}, total = 0},
// ['7'] = {suits = {}, total = 0},
// ['8'] = {suits = {}, total = 0},
// ['9'] = {suits = {}, total = 0},
// ['10'] = {suits = {}, total = 0},
// ['Jack'] = {suits = {}, total = 0},
// ['Queen'] = {suits = {}, total = 0},
// ['King'] = {suits = {}, total = 0},
// },
// hands = {
// ["Flush Five"] =        {visible = false,   order = 1, mult = 16,  chips = 160, s_mult = 16,  s_chips = 160, level = 1, l_mult = 3, l_chips = 50, played = 0, played_this_round = 0, example = {{'S_A', true},{'S_A', true},{'S_A', true},{'S_A', true},{'S_A', true}}},
// ["Flush House"] =       {visible = false,   order = 2, mult = 14,  chips = 140, s_mult = 14,  s_chips = 140, level = 1, l_mult = 4, l_chips = 40, played = 0, played_this_round = 0, example = {{'D_7', true},{'D_7', true},{'D_7', true},{'D_4', true},{'D_4', true}}},
// ["Five of a Kind"] =    {visible = false,   order = 3, mult = 12,  chips = 120, s_mult = 12,  s_chips = 120, level = 1, l_mult = 3, l_chips = 35, played = 0, played_this_round = 0, example = {{'S_A', true},{'H_A', true},{'H_A', true},{'C_A', true},{'D_A', true}}},
// ["Straight Flush"] =    {visible = true,    order = 4, mult = 8,   chips = 100, s_mult = 8,   s_chips = 100, level = 1, l_mult = 4, l_chips = 40, played = 0, played_this_round = 0, example = {{'S_Q', true},{'S_J', true},{'S_T', true},{'S_9', true},{'S_8', true}}},
// ["Four of a Kind"] =    {visible = true,    order = 5, mult = 7,   chips = 60,  s_mult = 7,   s_chips = 60,  level = 1, l_mult = 3, l_chips = 30, played = 0, played_this_round = 0, example = {{'S_J', true},{'H_J', true},{'C_J', true},{'D_J', true},{'C_3', false}}},
// ["Full House"] =        {visible = true,    order = 6, mult = 4,   chips = 40,  s_mult = 4,   s_chips = 40,  level = 1, l_mult = 2, l_chips = 25, played = 0, played_this_round = 0, example = {{'H_K', true},{'C_K', true},{'D_K', true},{'S_2', true},{'D_2', true}}},
// ["Flush"] =             {visible = true,    order = 7, mult = 4,   chips = 35,  s_mult = 4,   s_chips = 35,  level = 1, l_mult = 2, l_chips = 15, played = 0, played_this_round = 0, example = {{'H_A', true},{'H_K', true},{'H_T', true},{'H_5', true},{'H_4', true}}},
// ["Straight"] =          {visible = true,    order = 8, mult = 4,   chips = 30,  s_mult = 4,   s_chips = 30,  level = 1, l_mult = 3, l_chips = 30, played = 0, played_this_round = 0, example = {{'D_J', true},{'C_T', true},{'C_9', true},{'S_8', true},{'H_7', true}}},
// ["Three of a Kind"] =   {visible = true,    order = 9, mult = 3,   chips = 30,  s_mult = 3,   s_chips = 30,  level = 1, l_mult = 2, l_chips = 20, played = 0, played_this_round = 0, example = {{'S_T', true},{'C_T', true},{'D_T', true},{'H_6', false},{'D_5', false}}},
// ["Two Pair"] =          {visible = true,    order = 10,mult = 2,   chips = 20,  s_mult = 2,   s_chips = 20,  level = 1, l_mult = 1, l_chips = 20, played = 0, played_this_round = 0, example = {{'H_A', true},{'D_A', true},{'C_Q', false},{'H_4', true},{'C_4', true}}},
// ["Pair"] =              {visible = true,    order = 11,mult = 2,   chips = 10,  s_mult = 2,   s_chips = 10,  level = 1, l_mult = 1, l_chips = 15, played = 0, played_this_round = 0, example = {{'S_K', false},{'S_9', true},{'D_9', true},{'H_6', false},{'D_3', false}}},
// ["High Card"] =         {visible = true,    order = 12,mult = 1,   chips = 5,   s_mult = 1,   s_chips = 5,   level = 1, l_mult = 1, l_chips = 10, played = 0, played_this_round = 0, example = {{'S_A', true},{'D_Q', false},{'D_9', false},{'C_4', false},{'D_3', false}}},
// }
pub static WIN_ANTE: u8 = 8;

/// Chips added for each hand type per use of planet card
/// See `HandType`.
pub static L_CHIPS: [u8; 12] = [10, 15, 20, 20, 30, 15, 25, 30, 40, 35, 40, 50];
/// Mult added for each hand type per use of planet card
pub static L_MULT: [u8; 12] = [1, 1, 1, 2, 3, 2, 2, 3, 4, 3, 4, 3];

#[derive(Eq, PartialEq, Hash)]
pub enum HandType {
    HighCard = 0,
    Pair = 1,
    TwoPair = 2,
    ThreeOfAKind = 3,
    Straight = 4,
    Flush = 5,
    FullHouse = 6,
    FourOfAKind = 7,
    StraightFlush = 8,
    FiveOfAKind = 9,
    FLushHouse = 10,
    FlushFive = 11,
}

//
// enum Blinds {
//     SmallBlind(String),
//     BigBlind(String),
//     TheHook(String),
//     TheOx(String),
//     TheHouse(String),
//     TheWall(String),
//     TheWheel(String),
//     TheArm(String),
//     TheClub(String),
//     TheFish(String),
//     ThePsychic(String),
//     TheGoad(String),
//     }
//
//
// struct Blind {
//     pub blind_type: Blinds,
//
// }

#[derive(Debug)]
pub enum Stake {
    White = 1,
    Red = 2,
    Green = 3,
    Black = 4,
    Blue = 5,
    Purple = 6,
}

#[derive(Debug)]
pub struct HandInfo {
    /// Set visible to true if hidden hands are played.
    ///
    /// Hidden hands are flush house, five of a kind, and flush five.
    /// The player must first play these hands naturally before those planet cards are shown in the
    /// shop
    pub visible: bool,
    pub mult: u64,
    pub chips: u64,

    /// number of hands played of this type
    pub total_played: u8,
    /// number of hands played this round of this type
    pub played_this_round: u8,
}
#[derive(Debug)]
pub struct RoundScores {
    pub furthest_ante: u8,
    pub furthest_round: u8,
    pub best_hand: u64,
    pub cards_played: u16,
    pub cards_discarded: u16,
    pub cards_purchased: u16,
    pub times_rerolled: u16,
}

#[derive(Debug)]
pub struct CurrentRound {
    pub chips: u64, // hand
    pub mult: u64,
    pub chip_total: u64, // cumulative
    pub hands_played: u8,
    pub hands_left: u8,
    pub discards_left: u8,
    pub discards_used: u8,
    pub chip_target: u64,
}

#[derive(Debug)]
struct HandLevels {
    high_card: HandInfo,
    pair: HandInfo,
    two_pair: HandInfo,
    three_of_a_kind: HandInfo,
    straight: HandInfo,
    flush: HandInfo,
    full_house: HandInfo,
    four_of_a_kind: HandInfo,
    straight_flush: HandInfo,
    five_of_a_kind: HandInfo,
    flush_house: HandInfo,
    flush_five: HandInfo,
}
#[derive(Debug, Clone)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

#[derive(Debug, Clone)]
pub enum Seal {
    Red,
    Blue,
    Purple,
    Gold,
}
#[derive(Debug, Clone)]
pub enum Edition {
    Foil,
    Holographic,
    Polychrome,
    Negative,
}
#[derive(Debug, Clone)]
pub enum Enhancement {
    Bonus,
    Mult,
    Wild,
    Glass,
    Steel,
    Stone,
    Gold,
    Lucky,
}

/// Card backs
#[derive(Debug)]
pub enum DeckType {
    Red,
    Blue,
    Yellow,
    Green,
    Black,
    Magic,
    Nebula,
    Ghost,
    Abandoned,
    Checkered,
    Zodiac,
    Painted,
    Anaglyph,
    Plasma,
    // Erratic
}

#[derive(Debug, Clone)]
pub struct Card {
    pub suit: Suit,
    pub rank: u8,
    pub chip_val: u16,
    pub edition: Option<Edition>,
    pub enhancements: Option<Enhancement>,
    pub seal: Option<Seal>,
}

#[derive(Debug)]
pub struct GameState {
    pub won: bool,
    pub round_scores: RoundScores,
    pub stake: Stake,
    pub round: u8,
    pub rounds_skipped: u8,
    pub ante: u8,
    pub hands_played: u16,
    pub ununsed_discards: u16,
    pub dollars: u16,
    pub current_round: CurrentRound,
    pub hand_info: HandLevels,
    pub deck: Vec<Card>,
    pub deck_type: DeckType,
    pub jokers: Vec<Joker>,
}



impl GameState {
    const fn init_hands() -> HandLevels {
        HandLevels {
            high_card: HandInfo {
                visible: true,
                mult: 1,
                chips: 5,
                total_played: 0,
                played_this_round: 0,
            },

            pair: HandInfo {
                visible: true,
                mult: 2,
                chips: 10,
                total_played: 0,
                played_this_round: 0,
            },
            two_pair: HandInfo {
                visible: true,
                mult: 2,
                chips: 20,
                total_played: 0,
                played_this_round: 0,
            },
            three_of_a_kind: HandInfo {
                visible: true,
                mult: 3,
                chips: 30,
                total_played: 0,
                played_this_round: 0,
            },
            straight: HandInfo {
                visible: true,
                mult: 4,
                chips: 30,
                total_played: 0,
                played_this_round: 0,
            },
            flush: HandInfo {
                visible: true,
                mult: 4,
                chips: 35,
                total_played: 0,
                played_this_round: 0,
            },
            full_house: HandInfo {
                visible: true,
                mult: 4,
                chips: 40,
                total_played: 0,
                played_this_round: 0,
            },
            four_of_a_kind: HandInfo {
                visible: true,
                mult: 7,
                chips: 60,
                total_played: 0,
                played_this_round: 0,
            },
            straight_flush: HandInfo {
                visible: true,
                mult: 8,
                chips: 100,
                total_played: 0,
                played_this_round: 0,
            },
            five_of_a_kind: HandInfo {
                visible: false,
                mult: 12,
                chips: 120,
                total_played: 0,
                played_this_round: 0,
            },
            flush_house: HandInfo {
                visible: false,
                mult: 14,
                chips: 140,
                total_played: 0,
                played_this_round: 0,
            },
            flush_five: HandInfo {
                visible: false,
                mult: 16,
                chips: 160,
                total_played: 0,
                played_this_round: 0,
            },
        }
    }

    pub fn new(stake: Stake, deck_type: DeckType) -> Self {
        Self {
            won: false,
            round_scores: RoundScores {
                furthest_ante: 1,
                furthest_round: 1,
                best_hand: 0,
                cards_played: 0,
                cards_discarded: 0,
                cards_purchased: 0,
                times_rerolled: 0,
            },
            stake,
            round: 1,
            ante: 1,
            rounds_skipped: 0,
            hands_played: 0,
            ununsed_discards: 0,
            dollars: match deck_type {
                DeckType::Yellow => 14,
                _ => 4,
            },
            current_round: CurrentRound {
                chips: 0,
                mult: 0,
                chip_total: 0,
                hands_played: 0,
                hands_left: 0,
                discards_left: 0,
                discards_used: 0,
                chip_target: 0,
            },
            hand_info: Self::init_hands(),
            deck: deck::init_deck().to_vec(),
            deck_type,
            jokers: Vec::with_capacity(5),
        }
    }
}
