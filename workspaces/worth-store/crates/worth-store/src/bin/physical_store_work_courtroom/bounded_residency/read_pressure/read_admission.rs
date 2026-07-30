use worth_store::physical_runtime::{RecordByteLimit, RecordReadLimits};

use super::super::configuration::BoundedResidencyConfiguration;

pub(in crate::bounded_residency) fn read_limits(
    configuration: BoundedResidencyConfiguration,
    ordinal: usize,
) -> Result<RecordReadLimits, String> {
    Ok(RecordReadLimits::new(
        RecordByteLimit::new(
            configuration
                .record_bytes(ordinal)
                .ok_or_else(|| "bounded-residency record ordinal is out of range".to_owned())?
                as u32,
        )
        .expect("validated bounded-residency record bytes are nonzero"),
    ))
}
