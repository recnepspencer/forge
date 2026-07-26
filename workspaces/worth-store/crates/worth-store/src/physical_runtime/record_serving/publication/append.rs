use worth_store_physical_backend::ArtifactTreeFailure;

use super::super::{
    IndeterminateRecordPublication, RecordStreamFailure, UnpublishedRecordBatchFailure,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordPlacementClass {
    InlinePage,
    ExtentBacked,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecordAppendDenial {
    EmptyBatch,
    BatchRecordLimitExceeded,
    BatchByteLimitExceeded,
    RecordTooLarge,
    InlinePageFull,
    RootGenerationExhausted,
    RecordIdentityExhausted,
    PhysicalIdentityExhausted,
    IdentityEntropyUnavailable,
    BackendUnavailable(ArtifactTreeFailure),
    ServingRequiresInspection,
    PublicationAuthorityReleased,
    PublicationAdmissionStopped,
    PhysicalWorkUnavailable(Box<super::super::PhysicalRecordMutationFailureEvidence>),
    PhysicalReadWorkUnavailable(super::super::RecordReadWorkDenial),
    CatalogReplacementEligibilityMismatch,
    PlacementFormatMismatch,
    ManifestCapacityMigrationRequired,
    PublishedLayoutDamaged,
    PhysicalPressure,
    ResidencyUnavailable(super::super::PhysicalRecordResidencyFailure),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::physical_runtime::record_serving) enum ManifestCapacityTransition {
    PreserveCurrent,
    ReconstructToRequested,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecordAppendError {
    Denied(RecordAppendDenial),
    PhysicalPressure {
        evidence: super::super::PhysicalRecordPressureEvidence,
    },
    StreamFailed(RecordStreamFailure),
    Unpublished(UnpublishedRecordBatchFailure),
    Indeterminate(IndeterminateRecordPublication),
}

impl RecordAppendError {
    pub const fn pressure(&self) -> Option<super::super::PhysicalRecordPressureEvidence> {
        match self {
            Self::PhysicalPressure { evidence } => Some(*evidence),
            _ => None,
        }
    }

    pub const fn pressure_denial(&self) -> Option<RecordAppendDenial> {
        match self {
            Self::PhysicalPressure { .. } => Some(RecordAppendDenial::PhysicalPressure),
            _ => None,
        }
    }
}

impl RecordAppendDenial {
    pub(in crate::physical_runtime::record_serving) fn from_residency(
        denial: worth_store_buffer_pool::PhysicalResidencyDenial,
    ) -> Self {
        Self::ResidencyUnavailable(denial.into())
    }
}

pub(in crate::physical_runtime::record_serving) fn next_nonzero_random(
) -> Result<u64, RecordAppendError> {
    let mut bytes = [0_u8; 8];
    getrandom::fill(&mut bytes)
        .map_err(|_| RecordAppendError::Denied(RecordAppendDenial::IdentityEntropyUnavailable))?;
    let value = u64::from_le_bytes(bytes);
    if value == 0 {
        return Err(RecordAppendError::Denied(
            RecordAppendDenial::IdentityEntropyUnavailable,
        ));
    }
    Ok(value)
}
