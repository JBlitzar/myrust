use std::hash::DefaultHasher;
use fastrand::Rng;


struct Xorshift32 {
    state: u32,
    og_state: u32,
}

impl Xorshift32 {
    fn new(seed: u32) -> Self {
        let valid_seed = if seed == 0 { 1 } else { seed };
        Self { state: valid_seed, og_state: valid_seed }
    }

    fn next(&mut self) -> u32 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.state = x;
        x
    }

    fn rst_state(&mut self) {
        self.state = self.og_state;
    }
}


struct BloomFilter {
    filter: Vec<bool>,
    size: usize,
    k: usize,
    rng: Xorshift32,
}


impl BloomFilter {
    fn new(size: usize, k: usize) -> Self
   {
        BloomFilter {
            filter: vec![false; size],
            size,
            k,
            rng: Xorshift32::new(fastrand::u32(..)),
        }
    }
    fn hash(&mut self, item: &usize, kidx: usize) -> usize {
        // wrapping mul's slightly better than xor
        let hash = item.wrapping_mul(self.rng.next() as usize);
        hash ^ (hash >> 16)
    }

    fn add(&mut self, item: &usize) {
        self.rng.rst_state();

        for i in 0..self.k {
            let hash = self.hash(item, i) % self.size;
            self.filter[hash] = true;
        }
    }

    fn contains(&mut self, item: &usize) -> bool {
        self.rng.rst_state();

        for i in 0..self.k {
            let hash = self.hash(item, i) % self.size;
            if !self.filter[hash] {
                return false;
            }
        }
        true
    }

}


fn main() {
    println!("Hello, world!");
    let mut filter = BloomFilter::new(1000, 3);
    filter.add(&42);
    filter.add(&99);
    println!("Contains 42? {}", filter.contains(&42));
    println!("Contains 43? {}", filter.contains(&43));
}
