use crate::state::Edition;

#[derive(Debug)]
pub enum Jokers {
    Joker = 0,
}

#[derive(Debug)]
pub struct StaticJokerInfo {
    pub name: &'static str,
    pub blueprint_compat: bool,
    pub perishable_compat: bool,
    pub eternal_compat: bool,
    /// rarity 1 is common, 2 is uncommon, 3 is rare and 4 is legendary
    pub rarity: u8,
    pub buy_cost: u8,
}

// All 150 jokers' static info will be stored in here
pub const STATIC_JOKERS: [StaticJokerInfo; 1] = [
    StaticJokerInfo {
        name: "Joker",
        blueprint_compat: true,
        perishable_compat: true,
        eternal_compat: true,
        rarity: 1,
        buy_cost: 4,
    },
];

#[derive(Debug)]
pub struct Joker {
    // we index into the StaticJokers array to get joker info
    pub index_into_static_jokers: Jokers,
    pub sell_cost: u8,
    pub edition: Option<Edition>,
    pub is_eternal: bool,
    pub is_perishable: bool,
    pub perishable_rounds_left: Option<u8>,
}

