use std::num::NonZeroU64;

use worth_store_physical_format::RecordFrameCoordinate;

/// Store-owned request to warm one exact physical frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalPrefetchIntent {
    coordinate: RecordFrameCoordinate,
}

/// Store-owned ordered request to warm multiple exact physical frames.
///
/// The coordinate slice is borrowed so constructing an intent cannot allocate
/// before pool scope and kind admission.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalReadAheadIntent<'coordinates> {
    coordinates: &'coordinates [RecordFrameCoordinate],
    total_bytes: NonZeroU64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalReadAheadIntentDenial {
    EmptyCoordinateSet,
    ByteDemandOverflow,
}

impl PhysicalPrefetchIntent {
    pub const fn new(coordinate: RecordFrameCoordinate) -> Self {
        Self { coordinate }
    }

    pub const fn coordinate(self) -> RecordFrameCoordinate {
        self.coordinate
    }

    pub fn bytes(self) -> NonZeroU64 {
        NonZeroU64::new(u64::from(self.coordinate.length()))
            .expect("an admitted physical frame coordinate has nonzero length")
    }
}

impl<'coordinates> PhysicalReadAheadIntent<'coordinates> {
    pub fn new(
        coordinates: &'coordinates [RecordFrameCoordinate],
    ) -> Result<Self, PhysicalReadAheadIntentDenial> {
        let (first, remaining) = coordinates
            .split_first()
            .ok_or(PhysicalReadAheadIntentDenial::EmptyCoordinateSet)?;
        let total_bytes =
            remaining
                .iter()
                .try_fold(u64::from(first.length()), |total, coordinate| {
                    total
                        .checked_add(u64::from(coordinate.length()))
                        .ok_or(PhysicalReadAheadIntentDenial::ByteDemandOverflow)
                })?;
        Ok(Self {
            coordinates,
            total_bytes: NonZeroU64::new(total_bytes)
                .expect("a nonempty set of admitted coordinates has nonzero byte demand"),
        })
    }

    pub const fn coordinates(self) -> &'coordinates [RecordFrameCoordinate] {
        self.coordinates
    }

    pub const fn total_bytes(self) -> NonZeroU64 {
        self.total_bytes
    }
}
