use crate::random::pseudohash;
mod random;
mod state;
mod deck;
mod jokers;

fn main() {
    let test_seed: &str = "FOOBAR";

    println!("{}", pseudohash(test_seed.to_string()));
}
