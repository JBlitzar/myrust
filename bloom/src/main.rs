use std::hash::DefaultHasher;
use fastrand::Rng;
use std::hash::{Hash, Hasher};

struct BloomFilter {
    filter: Vec<bool>,
    size: usize,
    k: usize,
}


impl BloomFilter {
    fn new(size: usize, k: usize) -> Self
   {
        BloomFilter {
            filter: vec![false; size],
            size,
            k,
        }
    }

    // Declaration: I asked AI (chatgpt.com) about best utilization of DefaultHasher and used this
    // there's also a sketch version in an earlier commit that uses homemade xorshift32, but this is stronger. 
    // If I really wanted to be enterprise I'd somehow make this implementation-agnostic, but this is a toy example. 
    // Anyways idk this could be cool to find collisions.
    fn hash<T: Hash>(&self, item: &T, i: usize) -> usize {
        let mut h = DefaultHasher::new();
        i.hash(&mut h);
        item.hash(&mut h);
        (h.finish() as usize) % self.size
    }


    fn add(&mut self, item: &usize) {

        for i in 0..self.k {
            let hash = self.hash(item, i) % self.size;
            self.filter[hash] = true;
        }
    }

    fn contains(&self, item: &usize) -> bool {
        for i in 0..self.k {
            let hash = self.hash(item, i) % self.size;
            if !self.filter[hash] {
                return false;
            }
        }
        true
    }

}

fn evaluate_collisions(m: usize,k: usize,fill: usize, iterations: usize){
    let mut filter = BloomFilter::new(m, k);
    let mut rng = fastrand::Rng::new();
    let mut false_positives = 0;
    for _ in 0..fill {
        let item = rng.usize(..); // the probability of usize collisions is negligible. Otherwise we could, idk, use another bloom filter to check if its already added or something lol
        // 2^64 is really big. As long as we do an amount of iterations less than 10^19, we're proably fine. 10^6 isn't even close. 
        filter.add(&item);
        
    }

    for _ in 0..iterations {
        let item = rng.usize(..);
        if filter.contains(&item) {
            false_positives += 1;
        }
    }
    print!("{},",false_positives);
}


fn main() {

    // let mut filter = BloomFilter::new(10, 3);
    // filter.add(&42);
    // filter.add(&99);
    // println!("Contains 42? {}", filter.contains(&42));
    // println!("Contains 43? {}", filter.contains(&43));

    for m in 1..10_000 {
        // for k in 1..10 {
        let k = 3;
            // println!("Evaluating Bloom Filter with m = {}, k = {}", m, k);
            evaluate_collisions(m, k, 1000, 1000);
        // }
    }
}
