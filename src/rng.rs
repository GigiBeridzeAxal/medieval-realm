//! Tiny deterministic RNG (xorshift64*). Deterministic = reproducible bugs and tests.

use bevy::prelude::Resource;

#[derive(Resource, Debug, Clone)]
pub struct GameRng(u64);

impl GameRng {
    pub fn new(seed: u64) -> Self {
        Self(seed.max(1))
    }

    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    /// Uniform float in [0, 1).
    pub fn next_f32(&mut self) -> f32 {
        (self.next_u64() >> 40) as f32 / (1u64 << 24) as f32
    }

    pub fn range(&mut self, min: f32, max: f32) -> f32 {
        min + (max - min) * self.next_f32()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic() {
        let (mut a, mut b) = (GameRng::new(42), GameRng::new(42));
        for _ in 0..100 {
            assert_eq!(a.next_u64(), b.next_u64());
        }
    }

    #[test]
    fn range_bounds() {
        let mut r = GameRng::new(7);
        for _ in 0..10_000 {
            let v = r.range(-2.0, 3.0);
            assert!((-2.0..3.0).contains(&v));
        }
    }

    #[test]
    fn zero_seed_is_safe() {
        let mut r = GameRng::new(0);
        assert_ne!(r.next_u64(), 0);
    }
}
