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

    fn contains(&mut self, item: &usize) -> bool {
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
