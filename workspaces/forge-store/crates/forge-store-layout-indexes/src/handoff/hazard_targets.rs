use super::{
    S8CompileFailProofTarget, S8HazardDetection, S8HazardEvidenceRequirement, S8HazardProofLane,
    S8HazardProofTarget, S8LayoutHazard, S8RuntimeProofTarget, S9FormalModelTarget,
    S9LayoutMachineState,
};

pub(crate) const fn proof_target_for(hazard: S8LayoutHazard) -> S8HazardProofTarget {
    use S8CompileFailProofTarget::*;
    use S8HazardProofTarget::*;
    match hazard {
        S8LayoutHazard::HiddenBroadScan => {
            Runtime(S8RuntimeProofTarget::HiddenScanDeniedWithOwnerCounters)
        }
        S8LayoutHazard::StaleIndexExactness => {
            Runtime(S8RuntimeProofTarget::StaleIndexCannotReachExactAccess)
        }
        S8LayoutHazard::PartialIndexFalseAbsence => {
            Runtime(S8RuntimeProofTarget::PartialCoverageCannotProveAbsence)
        }
        S8LayoutHazard::DerivedAsAuthority => CompileFail(DerivedAuthorityAdmission),
        S8LayoutHazard::CrossScopeIndex => {
            Runtime(S8RuntimeProofTarget::CrossScopeIndexDeniedByScopeAdmission)
        }
        S8LayoutHazard::CorruptionAsEmpty => {
            Runtime(S8RuntimeProofTarget::CorruptionQuarantinesBeforeRebuild)
        }
        S8LayoutHazard::DerivedRollbackAuthority => CompileFail(DerivedRollbackAuthority),
        S8LayoutHazard::BTreeSeparatorMisroute => {
            Runtime(S8RuntimeProofTarget::BTreeSeparatorCorruptionDenied)
        }
        S8LayoutHazard::LsmTombstoneLoss => FormalModel(S9FormalModelTarget::CompactionCutover),
        S8LayoutHazard::BootstrapCatalogMisdiscovery => {
            Runtime(S8RuntimeProofTarget::BootstrapMisdiscoveryDenied)
        }
        S8LayoutHazard::CacheAdmissionBypass => CompileFail(CacheAdmission),
        S8LayoutHazard::LegacyReadyPlan => CompileFail(LegacyReadyPlan),
        S8LayoutHazard::CopiedCounters => CompileFail(ExecutedCounterAdmission),
        S8LayoutHazard::DegradedExactScanWithoutBudget => {
            Runtime(S8RuntimeProofTarget::UnbudgetedDegradedScanDenied)
        }
    }
}

pub(crate) const fn edge_states_for(
    hazard: S8LayoutHazard,
) -> (S9LayoutMachineState, S9LayoutMachineState) {
    use S8LayoutHazard::*;
    use S9LayoutMachineState::*;
    match hazard {
        HiddenBroadScan => (SelectionRequested, Denied),
        StaleIndexExactness => (Stale, Denied),
        PartialIndexFalseAbsence => (CoveragePartial, Denied),
        DerivedAsAuthority => (Admitted, Denied),
        CrossScopeIndex => (Declared, Denied),
        CorruptionAsEmpty => (Unclassified, Quarantined),
        DerivedRollbackAuthority => (Declared, Denied),
        BTreeSeparatorMisroute => (CanonicalKeysAdmitted, Denied),
        LsmTombstoneLoss => (
            CompactionRewriteLowered,
            CompactionTombstoneRetentionAdmitted,
        ),
        BootstrapCatalogMisdiscovery => (CatalogDiscovered, Denied),
        CacheAdmissionBypass => (Lowered, Ready),
        LegacyReadyPlan => (Ready, Denied),
        CopiedCounters => (Ready, Denied),
        DegradedExactScanWithoutBudget => (SelectionRequested, Denied),
    }
}

pub(crate) const fn requirement_for(
    hazard: S8LayoutHazard,
    lane: S8HazardProofLane,
) -> S8HazardEvidenceRequirement {
    if matches!(hazard, S8LayoutHazard::HiddenBroadScan) {
        return S8HazardEvidenceRequirement::OwnerBoundExactCounters;
    }
    if matches!(lane, S8HazardProofLane::Runtime) {
        return S8HazardEvidenceRequirement::OwnerRuntimeOutcome;
    }
    match lane {
        S8HazardProofLane::CompileFail => S8HazardEvidenceRequirement::CompileFailFence,
        S8HazardProofLane::Runtime => S8HazardEvidenceRequirement::OwnerBoundExactCounters,
        S8HazardProofLane::Simulation => S8HazardEvidenceRequirement::SimulationOracle,
        S8HazardProofLane::FormalModel => S8HazardEvidenceRequirement::FormalInvariant,
    }
}

pub(crate) const fn detection_for(lane: S8HazardProofLane) -> S8HazardDetection {
    match lane {
        S8HazardProofLane::CompileFail => S8HazardDetection::TypeBoundary,
        S8HazardProofLane::Runtime => S8HazardDetection::RuntimeOutcome,
        S8HazardProofLane::Simulation => S8HazardDetection::SimulationOracle,
        S8HazardProofLane::FormalModel => S8HazardDetection::FormalInvariant,
    }
}
