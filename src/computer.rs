use rand::{RngExt, rngs::ThreadRng};

pub struct Computer {
    rng: ThreadRng,
}

impl Computer {
    pub fn pick_card(&mut self, len: i32) -> i32 {
        self.rng.random_range(..len as usize) as i32
    }
}

impl Default for Computer {
    fn default() -> Self {
        Self { rng: rand::rng() }
    }
}
