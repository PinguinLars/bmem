use rand::{RngExt, rngs::ThreadRng};

use crate::cxxqt_object::Card;

pub struct Computer {
    mem: Vec<(Card, i32)>,
    rng: ThreadRng,
}

impl Computer {
    pub fn pick_cards(&mut self, len: i32) -> (i32, i32) {
        let len = len as usize;
        (self.rng.random_range(..len) as i32, self.rng.random_range(..len) as i32)
    }
}

impl Default for Computer {
    fn default() -> Self {
        Self { mem: Vec::new(), rng: rand::rng()}
    }
}
