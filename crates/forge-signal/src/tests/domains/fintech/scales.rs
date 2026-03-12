#[derive(Clone, Copy, Debug)]
pub(crate) struct FintechScale {
    pub instruments: usize,
    pub scenarios: usize,
    pub buckets: usize,
    pub books: usize,
    pub desks: usize,
}

impl FintechScale {
    pub(crate) fn smoke() -> Self {
        Self {
            instruments: 24,
            scenarios: 4,
            buckets: 3,
            books: 6,
            desks: 2,
        }
    }

    pub(crate) fn stress_10k() -> Self {
        Self {
            instruments: 500,
            scenarios: 20,
            buckets: 5,
            books: 25,
            desks: 5,
        }
    }

    pub(crate) fn fanout() -> Self {
        Self {
            instruments: 160,
            scenarios: 8,
            buckets: 5,
            books: 12,
            desks: 4,
        }
    }
}
