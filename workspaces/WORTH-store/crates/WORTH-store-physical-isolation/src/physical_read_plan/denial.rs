use crate::{
    GenerationCountedReferenceDenial, LatchAcquisitionDenial, PhysicalEpochVectorDenial,
    PhysicalReferenceGenerationMismatch, StalePhysicalReadPlanDenial,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalReadPlanAdmissionDenial {
    MissingReleaseSemantics,
    AuthorityRootMismatch {
        expected_root: u64,
        observed_root: u64,
        expected_manifest: u64,
        observed_manifest: u64,
    },
    EmptyProtectedFootprint,
    UnboundedProtectedFootprint {
        requested: usize,
        capacity: usize,
    },
    StaleGeneration(PhysicalReferenceGenerationMismatch),
    WrongPhysicalReferenceKind,
    ExecutionTimeReferenceDiscovery,
    PostProtectionObservationHazardMismatch {
        expected_protected_references: u64,
        observed_protected_references: u64,
    },
    LatchPlan(LatchAcquisitionDenial),
    EpochVector(PhysicalEpochVectorDenial),
    StalePlan(StalePhysicalReadPlanDenial),
}

impl From<LatchAcquisitionDenial> for PhysicalReadPlanAdmissionDenial {
    fn from(value: LatchAcquisitionDenial) -> Self {
        Self::LatchPlan(value)
    }
}

impl From<PhysicalEpochVectorDenial> for PhysicalReadPlanAdmissionDenial {
    fn from(value: PhysicalEpochVectorDenial) -> Self {
        Self::EpochVector(value)
    }
}

impl From<StalePhysicalReadPlanDenial> for PhysicalReadPlanAdmissionDenial {
    fn from(value: StalePhysicalReadPlanDenial) -> Self {
        Self::StalePlan(value)
    }
}

impl From<GenerationCountedReferenceDenial> for PhysicalReadPlanAdmissionDenial {
    fn from(value: GenerationCountedReferenceDenial) -> Self {
        match value {
            GenerationCountedReferenceDenial::ReferenceGenerationMismatch(mismatch) => {
                Self::StaleGeneration(mismatch)
            }
            GenerationCountedReferenceDenial::WrongPhysicalReferenceKind
            | GenerationCountedReferenceDenial::FutureChunkLifecycleNotOwnedByS5 => {
                Self::WrongPhysicalReferenceKind
            }
        }
    }
}
