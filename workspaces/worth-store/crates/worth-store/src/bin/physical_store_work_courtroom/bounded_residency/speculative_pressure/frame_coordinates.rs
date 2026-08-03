use worth_store::physical_runtime::{
    PhysicalRecordId, RecordByteLimit, RecordReadLimits, ServingPhysicalRuntime,
};
use worth_store_physical_format::RecordFrameCoordinate;

use super::super::configuration::BoundedResidencyConfiguration;

const SPECULATIVE_COORDINATE_COUNT: usize = 8;

pub(super) fn discover(
    serving: &ServingPhysicalRuntime,
    records: &[PhysicalRecordId],
    configuration: BoundedResidencyConfiguration,
) -> Result<[RecordFrameCoordinate; SPECULATIVE_COORDINATE_COUNT], String> {
    let first = configuration.first_extent_ordinal();
    let selected = records
        .get(first..first + SPECULATIVE_COORDINATE_COUNT)
        .ok_or_else(|| "bounded-residency speculative coordinate inventory is short".to_owned())?;
    let mut coordinates = Vec::with_capacity(SPECULATIVE_COORDINATE_COUNT);
    for (offset, record) in selected.iter().copied().enumerate() {
        let ordinal = first + offset;
        let maximum = configuration
            .record_bytes(ordinal)
            .and_then(|bytes| u32::try_from(bytes).ok())
            .and_then(RecordByteLimit::new)
            .ok_or_else(|| "bounded-residency speculative record limit is invalid".to_owned())?;
        let mut session = serving
            .records()
            .open(record, RecordReadLimits::new(maximum))
            .map_err(|failure| {
                format!("bounded-residency speculative coordinate open failed: {failure:?}")
            })?;
        let coordinate = session
            .next_chunk()
            .map_err(|failure| {
                format!("bounded-residency speculative coordinate read failed: {failure:?}")
            })?
            .ok_or_else(|| {
                "bounded-residency speculative coordinate record had no payload".to_owned()
            })?
            .basis()
            .frame_coordinate();
        if coordinates.contains(&coordinate) {
            return Err("bounded-residency speculative coordinates were not distinct".to_owned());
        }
        coordinates.push(coordinate);
    }
    coordinates
        .try_into()
        .map_err(|_| "bounded-residency speculative coordinate width drifted".to_owned())
}
