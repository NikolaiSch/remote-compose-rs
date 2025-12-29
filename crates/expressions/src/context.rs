use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::collections::HashMap;

pub struct ExpressionContext<'a> {
    pub vars: &'a HashMap<u32, f32>,
    pub registers: [f32; 4],
    pub rng: ChaCha8Rng,
}

impl<'a> ExpressionContext<'a> {
    pub fn new(vars: &'a HashMap<u32, f32>) -> Self {
        Self {
            vars,
            registers: [0.0; 4],
            rng: ChaCha8Rng::seed_from_u64(0),
        }
    }
}
