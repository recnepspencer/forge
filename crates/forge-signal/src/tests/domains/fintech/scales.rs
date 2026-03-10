#[derive(Clone, Copy, Debug)]
pub(super) struct FintechScale {
    pub instruments: usize,
    pub scenarios: usize,
    pub buckets: usize,
}

impl FintechScale {
    pub(super) fn smoke() -> Self {
        Self {
            instruments: 24,
            scenarios: 4,
            buckets: 3,
        }
    }

    pub(super) fn stress_10k() -> Self {
        Self {
            instruments: 500,
            scenarios: 20,
            buckets: 5,
        }
    }
}
