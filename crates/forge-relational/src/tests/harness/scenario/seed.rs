#[derive(Debug, Clone, Copy)]
pub(super) struct DeterministicGenerator {
    state: u64,
}

impl DeterministicGenerator {
    pub(super) fn new(seed: u64) -> Self {
        Self {
            state: seed.wrapping_mul(0x9E3779B97F4A7C15).wrapping_add(1),
        }
    }

    pub(super) fn next_u64(&mut self) -> u64 {
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.state
    }
}
