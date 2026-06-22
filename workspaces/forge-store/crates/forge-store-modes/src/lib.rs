#![forbid(unsafe_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreOperatingMode {
    Durable,
    Embedded,
    Absent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OperatingModeContract {
    mode: StoreOperatingMode,
}

impl OperatingModeContract {
    pub const fn new(mode: StoreOperatingMode) -> Self {
        Self { mode }
    }

    pub const fn mode(&self) -> StoreOperatingMode {
        self.mode
    }
}
