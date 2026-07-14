use super::DegradedScanLoweringBasis;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhysicalDegradedExecutionDenial {
    StoreAuthorityMismatch {
        expected: forge_store_authority::StoreCurrentAuthorityIdentity,
        actual: forge_store_authority::StoreCurrentAuthorityIdentity,
    },
    Admission(forge_store_physical_format::PlatformPhysicalOperationAdmissionDenial),
    Physical(Box<forge_store_physical_format::PhysicalStoreRuntimeDenial>),
    CounterDomainOverflow {
        observed_rows: u64,
    },
    CounterEnvelope(crate::CounterEnvelopeViolation),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DegradedScanAdmissionDenied {
    ReplacementAuthorityMismatch {
        basis: DegradedScanLoweringBasis,
        expected: crate::AdmittedPhysicalArtifactFamily,
        actual: crate::AdmittedPhysicalArtifactFamily,
    },
    ReplacementRequestMismatch {
        basis: DegradedScanLoweringBasis,
        expected: Box<crate::keyspace::AdmittedPhysicalAccessIdentity>,
        actual: Box<crate::keyspace::AdmittedPhysicalAccessIdentity>,
    },
    ReplacementIntentMismatch {
        basis: DegradedScanLoweringBasis,
        expected: crate::AdmittedAccessIntent,
        actual: crate::AdmittedAccessIntent,
    },
    ReplacementFrontierMismatch {
        basis: DegradedScanLoweringBasis,
        expected: Box<crate::LayoutMaterializationSourceIdentity>,
        actual: Box<crate::LayoutMaterializationSourceIdentity>,
    },
    RebindAdmissionMismatch {
        basis: DegradedScanLoweringBasis,
        expected_replacement: crate::AccessPlanIdentity,
        admitted_replacement: crate::AccessPlanIdentity,
    },
}

impl DegradedScanAdmissionDenied {
    pub const fn basis(&self) -> &DegradedScanLoweringBasis {
        match self {
            Self::ReplacementAuthorityMismatch { basis, .. }
            | Self::ReplacementRequestMismatch { basis, .. }
            | Self::ReplacementIntentMismatch { basis, .. }
            | Self::ReplacementFrontierMismatch { basis, .. }
            | Self::RebindAdmissionMismatch { basis, .. } => basis,
        }
    }
}
