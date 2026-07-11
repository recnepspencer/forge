use super::hazard_catalog::CANONICAL_HAZARDS;
use super::hazard_targets::{detection_for, edge_states_for, proof_target_for, requirement_for};
use super::{
    S8HazardProofLane, S9FormalModelTarget, S9LayoutMachineState, S9LayoutMachineTransition,
    S9LayoutStateMachine,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum S8LayoutHazard {
    HiddenBroadScan,
    StaleIndexExactness,
    PartialIndexFalseAbsence,
    DerivedAsAuthority,
    CrossScopeIndex,
    CorruptionAsEmpty,
    DerivedRollbackAuthority,
    BTreeSeparatorMisroute,
    LsmTombstoneLoss,
    BootstrapCatalogMisdiscovery,
    CacheAdmissionBypass,
    LegacyReadyPlan,
    CopiedCounters,
    DegradedExactScanWithoutBudget,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8HazardContainment {
    Deny,
    Quarantine,
    Rebuild,
    Rebind,
    Rollback,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8HazardRecovery {
    None,
    RebuildDerived,
    RebindCurrentAuthority,
    RollbackMigration,
    RepairAuthoritativeSource,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8HazardDetection {
    TypeBoundary,
    ExecutedCounterBoundary,
    RuntimeOutcome,
    SimulationOracle,
    FormalInvariant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8HazardResidualRisk {
    None,
    ExplicitlyContained {
        explanation: &'static str,
        why_not_ordinary_completion_work: &'static str,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8HazardEvidenceRequirement {
    CompileFailFence,
    SimulationOracle,
    FormalInvariant,
    OwnerBoundExactCounters,
    OwnerRuntimeOutcome,
}

/// The concrete proof surface that must remain live for a hazard row.  This
/// is deliberately more specific than a lane label: the S.9 handoff carries
/// the exact boundary or model obligation whose loss would reopen the hazard.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8HazardProofTarget {
    CompileFail(S8CompileFailProofTarget),
    Runtime(S8RuntimeProofTarget),
    FormalModel(S9FormalModelTarget),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8CompileFailProofTarget {
    DerivedAuthorityAdmission,
    CacheAdmission,
    LegacyReadyPlan,
    ExecutedCounterAdmission,
    DerivedRollbackAuthority,
}

impl S8CompileFailProofTarget {
    pub const fn harness(self) -> S8CompileFailHarness {
        match self {
            Self::LegacyReadyPlan => S8CompileFailHarness::Phase29,
            _ => S8CompileFailHarness::Phase0,
        }
    }

    /// Repository-relative source compiled by the real S.8 UI harness.
    pub const fn fixture(self) -> &'static str {
        match self {
            Self::DerivedAuthorityAdmission => {
                "tests/ui/phase0/derived_projection_cannot_satisfy_exact_access_admission.rs"
            }
            Self::CacheAdmission => {
                "tests/ui/phase0/cache_hit_cannot_satisfy_execution_readiness.rs"
            }
            Self::LegacyReadyPlan => {
                "tests/ui/phase29/legacy_layout_execution_decision_is_not_reexported_by_workspace_facade.rs"
            }
            Self::ExecutedCounterAdmission => {
                "tests/ui/phase0/copied_counter_rows_cannot_satisfy_planned_vs_observed.rs"
            }
            Self::DerivedRollbackAuthority => {
                "tests/ui/phase0/derived_projection_cannot_bind_rollback_authority.rs"
            }
        }
    }

    pub const fn expected_stderr(self) -> &'static [&'static str] {
        match self {
            Self::DerivedAuthorityAdmission => {
                &["S8LayoutCoverageWitness", "S8DerivedIndexParityWitness"]
            }
            Self::CacheAdmission => &["S8LoweredAccessReceipt", "ResidentFrameAdmission"],
            Self::LegacyReadyPlan => &["no `AspectLayoutReadExecutionDecision` in the root"],
            Self::ExecutedCounterAdmission => &[
                "S8PlannedVsObservedCounterReceipt",
                "FoundationalPerformanceCounterRow",
            ],
            Self::DerivedRollbackAuthority => &[
                "StoreCurrentAuthorityWitness",
                "S8DerivedIndexParityWitness",
            ],
        }
    }

    pub const fn extern_crates(self) -> &'static [&'static str] {
        match self {
            Self::CacheAdmission => &["forge_store_buffer_pool"],
            Self::ExecutedCounterAdmission => &["forge_foundational"],
            Self::LegacyReadyPlan => &["forge_store"],
            _ => &[],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8RuntimeProofTarget {
    HiddenScanDeniedWithOwnerCounters,
    StaleIndexCannotReachExactAccess,
    CrossScopeIndexDeniedByScopeAdmission,
    UnbudgetedDegradedScanDenied,
    PartialCoverageCannotProveAbsence,
    CorruptionQuarantinesBeforeRebuild,
    BTreeSeparatorCorruptionDenied,
    BootstrapMisdiscoveryDenied,
}

impl S8RuntimeProofTarget {
    pub const fn owner(self) -> S8RuntimeProofOwner {
        match self {
            Self::HiddenScanDeniedWithOwnerCounters => S8RuntimeProofOwner::PhysicalFormat,
            _ => S8RuntimeProofOwner::LayoutIndexes,
        }
    }

    pub const fn operation(self) -> S8RuntimeProofOperation {
        match self {
            Self::HiddenScanDeniedWithOwnerCounters => {
                S8RuntimeProofOperation::RejectHiddenBroadScan
            }
            Self::StaleIndexCannotReachExactAccess => {
                S8RuntimeProofOperation::DenyStaleIndexExactness
            }
            Self::CrossScopeIndexDeniedByScopeAdmission => {
                S8RuntimeProofOperation::DenyCrossScopeLayoutAdmission
            }
            Self::UnbudgetedDegradedScanDenied => {
                S8RuntimeProofOperation::DenyUnbudgetedDegradedExactScan
            }
            Self::PartialCoverageCannotProveAbsence => S8RuntimeProofOperation::DenyPartialAbsence,
            Self::CorruptionQuarantinesBeforeRebuild => {
                S8RuntimeProofOperation::QuarantineCorruption
            }
            Self::BTreeSeparatorCorruptionDenied => {
                S8RuntimeProofOperation::DenyBTreeSeparatorCorruption
            }
            Self::BootstrapMisdiscoveryDenied => S8RuntimeProofOperation::DenyBootstrapMisdiscovery,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8CompileFailHarness {
    Phase0,
    Phase29,
}

impl S8CompileFailHarness {
    pub const fn ui_dir(self) -> &'static str {
        match self {
            Self::Phase0 => "phase0",
            Self::Phase29 => "phase29",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8RuntimeProofOwner {
    PhysicalFormat,
    LayoutIndexes,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8RuntimeProofOperation {
    RejectHiddenBroadScan,
    DenyStaleIndexExactness,
    DenyCrossScopeLayoutAdmission,
    DenyUnbudgetedDegradedExactScan,
    DenyPartialAbsence,
    QuarantineCorruption,
    DenyBTreeSeparatorCorruption,
    DenyBootstrapMisdiscovery,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8LayoutHazardRow {
    hazard: S8LayoutHazard,
    machine: S9LayoutStateMachine,
    proof_lane: S8HazardProofLane,
    transition: S9LayoutMachineTransition,
    transition_from: S9LayoutMachineState,
    transition_to: S9LayoutMachineState,
    evidence_requirement: S8HazardEvidenceRequirement,
    proof_target: S8HazardProofTarget,
    containment: S8HazardContainment,
    recovery: S8HazardRecovery,
    detection: S8HazardDetection,
    residual_risk: S8HazardResidualRisk,
}

impl S8LayoutHazardRow {
    pub(crate) const fn new(
        hazard: S8LayoutHazard,
        machine: S9LayoutStateMachine,
        proof_lane: S8HazardProofLane,
        containment: S8HazardContainment,
        recovery: S8HazardRecovery,
        transition: S9LayoutMachineTransition,
    ) -> Self {
        let (transition_from, transition_to) = edge_states_for(hazard);
        Self {
            hazard,
            machine,
            proof_lane,
            transition,
            transition_from,
            transition_to,
            evidence_requirement: requirement_for(hazard, proof_lane),
            proof_target: proof_target_for(hazard),
            containment,
            recovery,
            detection: if matches!(hazard, S8LayoutHazard::HiddenBroadScan) {
                S8HazardDetection::ExecutedCounterBoundary
            } else {
                detection_for(proof_lane)
            },
            residual_risk: residual_risk_for(hazard),
        }
    }
    pub const fn hazard(self) -> S8LayoutHazard {
        self.hazard
    }
    pub const fn machine(self) -> S9LayoutStateMachine {
        self.machine
    }
    pub const fn proof_lane(self) -> S8HazardProofLane {
        self.proof_lane
    }
    pub const fn containment(self) -> S8HazardContainment {
        self.containment
    }
    pub const fn recovery(self) -> S8HazardRecovery {
        self.recovery
    }
    pub const fn detection(self) -> S8HazardDetection {
        self.detection
    }
    pub const fn residual_risk(self) -> S8HazardResidualRisk {
        self.residual_risk
    }
    /// The exact transition S.9 must model for this hazard.  A machine name
    /// or proof-lane label alone is deliberately insufficient.
    pub const fn transition(self) -> S9LayoutMachineTransition {
        self.transition
    }
    pub const fn transition_from(self) -> S9LayoutMachineState {
        self.transition_from
    }
    pub const fn transition_to(self) -> S9LayoutMachineState {
        self.transition_to
    }
    pub const fn evidence_requirement(self) -> S8HazardEvidenceRequirement {
        self.evidence_requirement
    }
    pub const fn proof_target(self) -> S8HazardProofTarget {
        self.proof_target
    }
}

const fn residual_risk_for(hazard: S8LayoutHazard) -> S8HazardResidualRisk {
    match hazard {
        S8LayoutHazard::LsmTombstoneLoss => S8HazardResidualRisk::ExplicitlyContained {
            explanation: "S.8 contains the pending formal risk by requiring a WAL-owner compaction receipt with canonical run ordering, retained tombstone identity, manifest/replay binding, and exact counters before physical-isolation can publish the rewritten root.",
            why_not_ordinary_completion_work: "S.9 must still model hostile crash/concurrency schedules around the now-enforced WAL-retention-to-physical-publication transition; S.8 supplies the real code-state mapping without claiming the model has run.",
        },
        _ => S8HazardResidualRisk::None,
    }
}

/// Fixed Store-law inventory. It describes required hazards; it does not
/// replace the family-owned runtime evidence cited by its proof lanes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8LayoutHazardInventory {
    rows: &'static [S8LayoutHazardRow],
}

impl S8LayoutHazardInventory {
    pub(crate) const fn canonical() -> Self {
        Self {
            rows: CANONICAL_HAZARDS,
        }
    }
    pub const fn rows(self) -> &'static [S8LayoutHazardRow] {
        self.rows
    }
    pub fn is_complete(self) -> bool {
        self.rows == CANONICAL_HAZARDS
    }
}
