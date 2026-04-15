use crate::failure::{StoreError, StoreErrorKind};
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub enum DurabilityBarrierClass {
    ProcessBufferOnly,
    KernelBufferResident,
    FileContentDurable,
    FileAndRequiredMetadataDurable,
    RenameOrPublicationMarkerDurable,
    DirectoryEntryDurable,
    TransactionalCommitDurable,
}

#[derive(Debug, Clone)]
pub(crate) struct BarrierClassifiedDurableRecord {
    record: crate::media::IntegrityValidatedDurableRecord,
    barrier_class: DurabilityBarrierClass,
}

impl BarrierClassifiedDurableRecord {
    pub(crate) fn classify(
        record: crate::media::IntegrityValidatedDurableRecord,
        barrier_class: DurabilityBarrierClass,
    ) -> Self {
        Self {
            record,
            barrier_class,
        }
    }

    pub(crate) fn record(&self) -> &crate::media::IntegrityValidatedDurableRecord {
        &self.record
    }

    pub(crate) fn barrier_class(&self) -> DurabilityBarrierClass {
        self.barrier_class
    }
}

pub(crate) fn validate_barrier_satisfies_requirement(
    observed: DurabilityBarrierClass,
    required: DurabilityBarrierClass,
) -> Result<(), StoreError> {
    let ordered_barriers = [
        DurabilityBarrierClass::ProcessBufferOnly,
        DurabilityBarrierClass::KernelBufferResident,
        DurabilityBarrierClass::FileContentDurable,
        DurabilityBarrierClass::FileAndRequiredMetadataDurable,
        DurabilityBarrierClass::RenameOrPublicationMarkerDurable,
        DurabilityBarrierClass::DirectoryEntryDurable,
        DurabilityBarrierClass::TransactionalCommitDurable,
    ];
    let observed_rank = ordered_barriers
        .iter()
        .position(|candidate| *candidate == observed)
        .expect("observed barrier class should be ordered");
    let required_rank = ordered_barriers
        .iter()
        .position(|candidate| *candidate == required)
        .expect("required barrier class should be ordered");

    if observed_rank < required_rank {
        return Err(StoreError::new(
            StoreErrorKind::DurableBarrierContractViolation,
            format!(
                "observed durability barrier {observed:?} does not satisfy required barrier {required:?}"
            ),
        ));
    }
    Ok(())
}
