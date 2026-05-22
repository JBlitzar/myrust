use crate::types::U512;
use crate::types::csprng_u256;
use crate::types::fastrand_u256;

use rand::SeedableRng;
use rand::rng;
use rand_chacha::ChaCha20Rng;


fn pow(mut x: U512, mut n: U512, m: U512) -> U512 {
    // I got AI to refactor my fast modular expo function because I had integer overflowing problems. It's also slightly more optimized because it uses bit level hacking instead of recursion
    // a truly optimized version of this would use montgomery multiplication. honestly out of scope for me for now
    
    let mut res = U512::from(1);
    x = x % m;

    while n > U512::from(0) {
        if n % 2 == U512::from(1) {
            res = res.overflowing_mul(x).0 % m;
        }
        n = n >> 1;
        x = x.overflowing_mul(x).0 % m;
    }
    res
}

pub fn check(n: U512, rounds: usize) -> bool {
    const SMALL_PRIMES: [u64; 11] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31];
    for p in SMALL_PRIMES {
        if n % p == U512::from(0) {
            return false;
        }
    }

    // println!("Checking {}", n);
    if n == U512::from(2) || n == U512::from(3) {
        return true;
    }
    if n <= U512::from(1){
        return false;
    }

    let mut cur = n - 1;
    let mut q: usize = 0;
    while cur % 2 == U512::from(0) {
        q += 1;
        cur = cur >> 1;
    }

    let k = q;
    let m = cur;

    for _ in 0..rounds {
        let mut a = fastrand_u256();

        while a == U512::from(1) || a >= n - 1 {
            a = fastrand_u256();
        }

        // println!("a, m,n,k = {},{},{},{}", a, m, n, k);

        let b0 = pow(a, m, n);
        let mut bb = b0;
        if b0 == U512::from(1) || b0 == U512::from(n - 1) {
            // return true;
        } else {
            let mut broken = false;
            if k > 0 {
                for _ in 0..(k - 1) {
                    bb = bb.overflowing_mul(bb).0 % n;
                    if bb == U512::from(1) {
                        return false;
                    } else if bb == U512::from(n - 1) {
                        // return true;
                        broken = true;
                        break;
                    } else {
                        // have to loop again
                    }
                }
            }
            if !broken {
                return false;
            }
        }
    }

    true
}

pub fn get_prime() -> U512 {
    let mut i = 0;
    loop {
        let mut rng = ChaCha20Rng::from_rng(&mut rng());
        let p = csprng_u256(&mut rng) | U512::from(1) | (U512::from(1) << 255);
        if check(p, 10) {
            return p;
        }
        i += 1;
        // if i % 100 == 0 {
        //     println!("{i} rounds");
        // }
        if i > 10_000 {
            println!("10000 rounds, this should never happen");
            break;
        }
    }
    return U512::from(0); // should never happen, just here to appease the borrow checkers
}
