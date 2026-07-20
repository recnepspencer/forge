use std::collections::{BTreeMap, BTreeSet};

use sha2::{Digest, Sha256};
use worth_store_operations::{CurrentReplicaPromotion, SelectedOperationalControlState};
use worth_store_physical_certification::OperationalRecoveryYieldpoint;

use super::{
    OperationalRecoveryCapabilityMatrix, S10HostileProgramRequirement,
    S10OperationalScenarioEvidence, S10OperationalScenarioKind, S10Phase,
    S10ProofFoundationalAdoptionMatrix, S10ScaleComparisonDenial, S10ScaleComparisonMatrix,
    S11StructuredAuditHardeningHandoff, S12PhysicalQualificationHandoff, ScenarioScaleProfile,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum S10ScenarioSuiteDenial {
    WrongProfile,
    DuplicateScenario,
    MissingScenario(S10OperationalScenarioKind),
    PhaseCoveredByFewerThanTwoScenarios(S10Phase),
}

#[derive(Debug, Clone)]
pub struct S10ScenarioSuiteEvidence {
    profile: ScenarioScaleProfile,
    scenarios: BTreeMap<S10OperationalScenarioKind, S10OperationalScenarioEvidence>,
    suite_identity: [u8; 32],
}

impl S10ScenarioSuiteEvidence {
    pub fn join(
        profile: ScenarioScaleProfile,
        scenarios: impl IntoIterator<Item = S10OperationalScenarioEvidence>,
    ) -> Result<Self, S10ScenarioSuiteDenial> {
        let mut by_kind = BTreeMap::new();
        for scenario in scenarios {
            if scenario.program().profile() != profile {
                return Err(S10ScenarioSuiteDenial::WrongProfile);
            }
            if by_kind
                .insert(scenario.program().kind(), scenario)
                .is_some()
            {
                return Err(S10ScenarioSuiteDenial::DuplicateScenario);
            }
        }
        for kind in [
            S10OperationalScenarioKind::BurningPrimary,
            S10OperationalScenarioKind::SplitBrainPromotion,
            S10OperationalScenarioKind::AuthorityRepairRollback,
        ] {
            if !by_kind.contains_key(&kind) {
                return Err(S10ScenarioSuiteDenial::MissingScenario(kind));
            }
        }
        for phase in S10Phase::scenario_phases() {
            let count = by_kind
                .values()
                .filter(|scenario| {
                    scenario
                        .phase_invocations()
                        .iter()
                        .any(|invocation| invocation.phase() == phase)
                })
                .count();
            if count < 2 {
                return Err(S10ScenarioSuiteDenial::PhaseCoveredByFewerThanTwoScenarios(
                    phase,
                ));
            }
        }
        let mut digest = Sha256::new();
        digest.update(b"worth-store-s10-scenario-suite-v1");
        digest.update([profile as u8]);
        for scenario in by_kind.values() {
            digest.update(scenario.evidence_identity());
        }
        Ok(Self {
            profile,
            scenarios: by_kind,
            suite_identity: digest.finalize().into(),
        })
    }

    pub const fn profile(&self) -> ScenarioScaleProfile {
        self.profile
    }
    pub const fn suite_identity(&self) -> [u8; 32] {
        self.suite_identity
    }
    pub fn scenarios(&self) -> impl Iterator<Item = &S10OperationalScenarioEvidence> {
        self.scenarios.values()
    }

    pub(super) fn scenario(
        &self,
        kind: S10OperationalScenarioKind,
    ) -> &S10OperationalScenarioEvidence {
        self.scenarios
            .get(&kind)
            .expect("joined suites contain every required scenario")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PromotionRemoteExclusionEvidence {
    promotion_receipt_identity: [u8; 32],
    fence_identity: [u8; 32],
    publication_identity: [u8; 32],
    serve_lease_identity: [u8; 32],
    promoted_epoch: u64,
    serving_epoch: u64,
}

impl PromotionRemoteExclusionEvidence {
    pub fn from_current_promotion(
        current: &CurrentReplicaPromotion,
    ) -> Result<Self, S10CloseoutDenial> {
        let receipt = current.promotion_receipt();
        let lease = current.serve_lease();
        let publication_identity = current.publication().publication_identity();
        if receipt.fence_identity() == [0; 32]
            || publication_identity == [0; 32]
            || lease.epoch() < receipt.promoted_epoch().get()
        {
            return Err(S10CloseoutDenial::RemoteExclusionNotProven);
        }
        Ok(Self {
            promotion_receipt_identity: receipt.receipt_identity(),
            fence_identity: receipt.fence_identity(),
            publication_identity,
            serve_lease_identity: lease.lease_identity(),
            promoted_epoch: receipt.promoted_epoch().get(),
            serving_epoch: lease.epoch(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum S10CloseoutDenial {
    CiSuiteRequired,
    ReleaseSuiteRequired,
    ReusedScenarioEvidenceAcrossProfiles,
    ControlStoreHasNoDurableHistory,
    RemoteExclusionNotProven,
    MissingFreshProcessCrashCoverage {
        profile: ScenarioScaleProfile,
        scenario: S10OperationalScenarioKind,
        yieldpoint: OperationalRecoveryYieldpoint,
    },
    IncompleteHostileProgram {
        profile: ScenarioScaleProfile,
        scenario: S10OperationalScenarioKind,
        requirement: S10HostileProgramRequirement,
    },
    PhaseDefectSuite(super::S10PhaseDefectSuiteDenial),
    ScaleComparison(S10ScaleComparisonDenial),
}

#[derive(Debug, Clone)]
pub struct S10CertificationCloseout {
    closeout_identity: [u8; 32],
    ci_suite_identity: [u8; 32],
    release_suite_identity: [u8; 32],
    phase_defect_suite_identity: [u8; 32],
    selected_control_generation: u64,
    adoption: S10ProofFoundationalAdoptionMatrix,
    capabilities: OperationalRecoveryCapabilityMatrix,
    scale_comparisons: S10ScaleComparisonMatrix,
    s11: S11StructuredAuditHardeningHandoff,
    s12: S12PhysicalQualificationHandoff,
}

pub fn close_s10_certification(
    ci: S10ScenarioSuiteEvidence,
    release: S10ScenarioSuiteEvidence,
    phase_defects: super::S10PhaseDefectSuite,
    control: &SelectedOperationalControlState,
    exclusion: PromotionRemoteExclusionEvidence,
) -> Result<S10CertificationCloseout, S10CloseoutDenial> {
    if ci.profile != ScenarioScaleProfile::Ci {
        return Err(S10CloseoutDenial::CiSuiteRequired);
    }
    if release.profile != ScenarioScaleProfile::Release {
        return Err(S10CloseoutDenial::ReleaseSuiteRequired);
    }
    let ci_ids = ci
        .scenarios()
        .map(|scenario| scenario.evidence_identity())
        .collect::<BTreeSet<_>>();
    if release
        .scenarios()
        .any(|scenario| ci_ids.contains(&scenario.evidence_identity()))
    {
        return Err(S10CloseoutDenial::ReusedScenarioEvidenceAcrossProfiles);
    }
    if control.history_summary().record_count() == 0 {
        return Err(S10CloseoutDenial::ControlStoreHasNoDurableHistory);
    }
    if exclusion.serving_epoch < exclusion.promoted_epoch {
        return Err(S10CloseoutDenial::RemoteExclusionNotProven);
    }
    require_fresh_process_crash_coverage(&ci)?;
    require_fresh_process_crash_coverage(&release)?;
    require_complete_hostile_program(&ci)?;
    require_complete_hostile_program(&release)?;
    let scenario_evidence_identities = six_scenario_identities(&ci, &release);
    let scenario_identity_set = scenario_evidence_identities.into_iter().collect();
    phase_defects
        .require_scenario_membership(&scenario_identity_set)
        .map_err(S10CloseoutDenial::PhaseDefectSuite)?;
    let scale_comparisons = S10ScaleComparisonMatrix::from_suites(&ci, &release)
        .map_err(S10CloseoutDenial::ScaleComparison)?;
    let mut digest = Sha256::new();
    digest.update(b"worth-store-s10-certification-closeout-v1");
    digest.update(ci.suite_identity);
    digest.update(release.suite_identity);
    digest.update(
        control
            .selected_generation()
            .generation()
            .get()
            .to_be_bytes(),
    );
    digest.update(exclusion.promotion_receipt_identity);
    digest.update(exclusion.fence_identity);
    digest.update(exclusion.publication_identity);
    digest.update(exclusion.serve_lease_identity);
    digest.update(phase_defects.suite_identity());
    digest.update(scale_comparisons.matrix_identity());
    let closeout_identity = digest.finalize().into();
    Ok(S10CertificationCloseout {
        closeout_identity,
        ci_suite_identity: ci.suite_identity,
        release_suite_identity: release.suite_identity,
        phase_defect_suite_identity: phase_defects.suite_identity(),
        selected_control_generation: control.selected_generation().generation().get(),
        adoption: S10ProofFoundationalAdoptionMatrix::canonical(),
        capabilities: OperationalRecoveryCapabilityMatrix::from_closed_suites(&ci, &release),
        scale_comparisons,
        s11: S11StructuredAuditHardeningHandoff::from_closeout(
            closeout_identity,
            scenario_evidence_identities,
        ),
        s12: S12PhysicalQualificationHandoff::from_closeout(
            closeout_identity,
            scenario_evidence_identities,
        ),
    })
}

fn require_fresh_process_crash_coverage(
    suite: &S10ScenarioSuiteEvidence,
) -> Result<(), S10CloseoutDenial> {
    for scenario in suite.scenarios() {
        if let Some(yieldpoint) = scenario.missing_crash_reopen_yieldpoint() {
            return Err(S10CloseoutDenial::MissingFreshProcessCrashCoverage {
                profile: suite.profile(),
                scenario: scenario.program().kind(),
                yieldpoint,
            });
        }
    }
    Ok(())
}

fn require_complete_hostile_program(
    suite: &S10ScenarioSuiteEvidence,
) -> Result<(), S10CloseoutDenial> {
    for scenario in suite.scenarios() {
        if let Some(requirement) = scenario.hostile_program().missing_requirement() {
            return Err(S10CloseoutDenial::IncompleteHostileProgram {
                profile: suite.profile(),
                scenario: scenario.program().kind(),
                requirement,
            });
        }
    }
    Ok(())
}

impl S10CertificationCloseout {
    pub const fn closeout_identity(&self) -> [u8; 32] {
        self.closeout_identity
    }
    pub const fn ci_suite_identity(&self) -> [u8; 32] {
        self.ci_suite_identity
    }
    pub const fn release_suite_identity(&self) -> [u8; 32] {
        self.release_suite_identity
    }
    pub const fn phase_defect_suite_identity(&self) -> [u8; 32] {
        self.phase_defect_suite_identity
    }
    pub const fn selected_control_generation(&self) -> u64 {
        self.selected_control_generation
    }
    pub const fn adoption_matrix(&self) -> &S10ProofFoundationalAdoptionMatrix {
        &self.adoption
    }
    pub const fn capability_matrix(&self) -> &OperationalRecoveryCapabilityMatrix {
        &self.capabilities
    }
    pub const fn scale_comparisons(&self) -> &S10ScaleComparisonMatrix {
        &self.scale_comparisons
    }
    pub const fn s11_handoff(&self) -> &S11StructuredAuditHardeningHandoff {
        &self.s11
    }
    pub const fn s12_handoff(&self) -> &S12PhysicalQualificationHandoff {
        &self.s12
    }
}

fn six_scenario_identities(
    ci: &S10ScenarioSuiteEvidence,
    release: &S10ScenarioSuiteEvidence,
) -> [[u8; 32]; 6] {
    let mut identities = [[0; 32]; 6];
    for (slot, identity) in identities.iter_mut().zip(
        ci.scenarios()
            .chain(release.scenarios())
            .map(|scenario| scenario.evidence_identity()),
    ) {
        *slot = identity;
    }
    identities
}
