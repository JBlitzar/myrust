use fastrand;
use rand::Rng;
use rand::RngExt;

use uint::construct_uint;
construct_uint! {
    pub struct U512(8);
}

pub fn fastrand_u256() -> U512 {
    U512([
        fastrand::u64(..),
        fastrand::u64(..),
        fastrand::u64(..),
        fastrand::u64(..),
        0,
        0,
        0,
        0,
    ])
}

pub fn fastrand_u512() -> U512 {
    U512([
        fastrand::u64(..),
        fastrand::u64(..),
        fastrand::u64(..),
        fastrand::u64(..),
        fastrand::u64(..),
        fastrand::u64(..),
        fastrand::u64(..),
        fastrand::u64(..),
    ])
}

pub fn csprng_u256(rng: &mut impl Rng) -> U512 {
    U512([
        rng.random(),
        rng.random(),
        rng.random(),
        rng.random(),
        0,
        0,
        0,
        0,
    ])
}
