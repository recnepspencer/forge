use crate::{AllocationSentinel, LargeRecordStreamPressure, LargeStorePressureFixture};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemoryPressureDriverInput {
    fixture: LargeStorePressureFixture,
    allocation_sentinel: AllocationSentinel,
    stream_pressure: LargeRecordStreamPressure,
}

impl MemoryPressureDriverInput {
    pub const fn from_fixture(fixture: LargeStorePressureFixture) -> Self {
        Self {
            fixture,
            allocation_sentinel: AllocationSentinel::no_shortcuts(),
            stream_pressure: LargeRecordStreamPressure::from_fixture(fixture),
        }
    }

    pub const fn fixture(&self) -> LargeStorePressureFixture {
        self.fixture
    }

    pub const fn allocation_sentinel(&self) -> AllocationSentinel {
        self.allocation_sentinel
    }

    pub const fn stream_pressure(&self) -> LargeRecordStreamPressure {
        self.stream_pressure
    }
}
