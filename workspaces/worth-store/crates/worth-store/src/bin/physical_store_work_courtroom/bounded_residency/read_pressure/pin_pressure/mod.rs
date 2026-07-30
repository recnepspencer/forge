mod eviction;
mod saturation;

use worth_store::physical_runtime::{PhysicalRecordId, ServingPhysicalRuntime};

use super::super::configuration::BoundedResidencyConfiguration;

pub(in crate::bounded_residency) use eviction::PinnedEvictionEvidence;
pub(in crate::bounded_residency) use saturation::PinSaturationEvidence;

pub(in crate::bounded_residency) struct PinnedFramePressureEvidence {
    pub(in crate::bounded_residency) saturation: PinSaturationEvidence,
    pub(in crate::bounded_residency) eviction: PinnedEvictionEvidence,
}

pub(in crate::bounded_residency) fn prove_pins(
    serving: &ServingPhysicalRuntime,
    records: &[PhysicalRecordId],
    configuration: BoundedResidencyConfiguration,
) -> Result<PinnedFramePressureEvidence, String> {
    let saturation = saturation::prove(serving, records, configuration)?;
    let eviction = eviction::prove(serving, records, configuration)?;
    Ok(PinnedFramePressureEvidence {
        saturation,
        eviction,
    })
}
