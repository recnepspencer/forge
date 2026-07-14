use crate::resident_pressure_fixtures::LargeStorePressureFixture;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LargeRecordStreamPressure {
    extent_count: u64,
    window_bytes: u64,
}

impl LargeRecordStreamPressure {
    pub const fn from_fixture(fixture: LargeStorePressureFixture) -> Self {
        Self {
            extent_count: fixture.fragment_count(),
            window_bytes: fixture.streaming_window_bytes(),
        }
    }

    pub const fn extent_count(&self) -> u64 {
        self.extent_count
    }

    pub const fn window_bytes(&self) -> u64 {
        self.window_bytes
    }
}
