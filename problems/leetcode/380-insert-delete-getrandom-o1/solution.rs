use rand::{prelude::IteratorRandom, Rng};
use std::collections::HashSet;

struct RandomizedSet {
    set: HashSet<i32>,
}

impl RandomizedSet {
    fn new() -> Self {
        RandomizedSet {
            set: HashSet::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        self.set.insert(val)
    }

    fn remove(&mut self, val: i32) -> bool {
        self.set.remove(&val)
    }

    fn get_random(&self) -> i32 {
        let mut rng = rand::thread_rng();
        *self.set.iter().choose(&mut rng).unwrap()
    }
}
