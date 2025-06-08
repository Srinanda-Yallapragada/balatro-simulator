//self.GAME.pseudorandom.hashed_seed = pseudohash(self.GAME.pseudorandom.seed)

//
// function pseudohash(str)
// if true then
// local num = 1
// for i=#str, 1, -1 do
// num = ((1.1239285023/num)*string.byte(str, i)*math.pi + math.pi*i)%1
// end
// return num

use std::f64::consts;
use rand_xoshiro::rand_core::{RngCore, SeedableRng};
use rand_xoshiro::Xoshiro256StarStar;

// pub static PI: f64 = 3.141592653589793238462643383279502884;
pub fn pseudohash(string: String) -> f64 {
    let mut num = 1.;

    for i in (1..=string.len()).rev() {
        num = ((1.1239285023 / num) * (string.as_bytes()[i - 1] as f64) * consts::PI
            + consts::PI * i as f64)
            % 1.0;
    }
    num
} // 0.9140354836243887
// 0.91403548362439

// function pseudoseed(key, predict_seed)
// if key == 'seed' then return math.random() end
//
// if predict_seed then
// local _pseed = pseudohash(key..(predict_seed or ''))
// _pseed = math.abs(tonumber(string.format("%.13f", (2.134453429141+_pseed*1.72431234)%1)))
// return (_pseed + (pseudohash(predict_seed) or 0))/2
// end
//
// if not G.GAME.pseudorandom[key] then
// G.GAME.pseudorandom[key] = pseudohash(key..(G.GAME.pseudorandom.seed or ''))
// end
//
// G.GAME.pseudorandom[key] = math.abs(tonumber(string.format("%.13f", (2.134453429141+G.GAME.pseudorandom[key]*1.72431234)%1)))
// return (G.GAME.pseudorandom[key] + (G.GAME.pseudorandom.hashed_seed or 0))/2
// end

pub fn psuedoseed(key: String) {
    let seed = pseudohash(key);
    let mut x256 = Xoshiro256StarStar::seed_from_u64(seed as u64);
    x256.next_u64();
}
