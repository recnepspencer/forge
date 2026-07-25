#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiHostMeasurementDeadline {
    tick: u64,
}

impl UiHostMeasurementDeadline {
    pub const fn at_tick(tick: u64) -> Self {
        Self { tick }
    }

    pub const fn tick(self) -> u64 {
        self.tick
    }

    pub const fn expired_at(self, now: u64) -> bool {
        now >= self.tick
    }
}
