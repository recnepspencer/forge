#[derive(Debug, Clone, Copy)]
pub(super) struct FintechScale {
    pub(super) accounts: usize,
    pub(super) trades: usize,
    pub(super) market_points: usize,
}

impl FintechScale {
    pub(super) const fn smoke() -> Self {
        Self {
            accounts: 4,
            trades: 4,
            market_points: 4,
        }
    }
}
