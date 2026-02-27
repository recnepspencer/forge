//! Deterministic pseudo-random number generator.
//!
//! A simple xorshift64 PRNG seeded from a `u64`. Used for deterministic
//! tie-breaking and fuzzer seeding (Doctrine D1). Zero external dependencies.
//!
//! # Determinism Guarantee
//!
//! Given the same seed, the sequence of outputs is identical across
//! all platforms. This is critical for replay (Milestone 0.4).

/// A deterministic PRNG using the xorshift64 algorithm.
///
/// Produces a repeatable sequence of pseudo-random `u64` values
/// given an initial seed. The same seed always produces the same
/// sequence, regardless of platform.
#[derive(Debug, Clone)]
pub struct DeterministicRng {
    state: u64,
}

impl DeterministicRng {
    /// Create a new RNG from a seed.
    ///
    /// A seed of `0` is silently replaced with `1` to avoid the
    /// degenerate all-zeros fixed point of xorshift.
    pub fn new(seed: u64) -> Self {
        let state = if seed == 0 { 1 } else { seed };
        Self { state }
    }

    /// Generate the next pseudo-random `u64`.
    pub fn next_u64(&mut self) -> u64 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;
        self.state
    }

    /// Generate a pseudo-random `f64` in `[0, 1)`.
    pub fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / ((1u64 << 53) as f64)
    }

    /// The current internal state (for snapshot / replay).
    pub fn state(&self) -> u64 {
        self.state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_seed_produces_same_sequence() {
        let mut a = DeterministicRng::new(42);
        let mut b = DeterministicRng::new(42);

        let seq_a: Vec<u64> = (0..100).map(|_| a.next_u64()).collect();
        let seq_b: Vec<u64> = (0..100).map(|_| b.next_u64()).collect();
        assert_eq!(seq_a, seq_b);
    }

    #[test]
    fn different_seeds_produce_different_sequences() {
        let mut a = DeterministicRng::new(1);
        let mut b = DeterministicRng::new(2);
        assert_ne!(a.next_u64(), b.next_u64());
    }

    #[test]
    fn zero_seed_is_safe() {
        let mut rng = DeterministicRng::new(0);
        let val = rng.next_u64();
        assert_ne!(val, 0);
    }

    #[test]
    fn next_f64_in_unit_range() {
        let mut rng = DeterministicRng::new(12345);
        for _ in 0..1000 {
            let val = rng.next_f64();
            assert!((0.0..1.0).contains(&val));
        }
    }

    #[test]
    fn state_snapshot_enables_replay() {
        let mut rng = DeterministicRng::new(99);
        for _ in 0..50 {
            rng.next_u64();
        }

        let snapshot = rng.state();
        let mut replayed_from_snapshot = DeterministicRng::new(snapshot);

        let mut advanced_from_same_seed = DeterministicRng::new(99);
        for _ in 0..50 {
            advanced_from_same_seed.next_u64();
        }

        assert_eq!(
            advanced_from_same_seed.next_u64(),
            replayed_from_snapshot.next_u64()
        );
    }
}
