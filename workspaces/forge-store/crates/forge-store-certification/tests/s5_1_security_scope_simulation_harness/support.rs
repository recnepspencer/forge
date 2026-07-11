use forge_store_physical_certification::{
    SecurityScopeHarnessEvidence, SecurityScopeHarnessOutcomeKind, SecurityScopeHarnessScenario,
    SecurityScopeHarnessSchedule, SecurityScopePhysicalReplayDenial,
    SecurityScopePhysicalReplayEvidence, SecurityScopePhysicalScheduleBinding,
    SecurityScopeReplayMutationKind,
};
use forge_store_security::StoreSecurityScopeAdmissionCounterSnapshot;

use crate::s5_interleaving_harness_support;

const WITNESSES_PER_SECURITY_SCOPE_ADMISSION: u64 = 4;

#[derive(Clone, Copy)]
pub(crate) struct ExpectedTypedCounters {
    stale_key_posture: u64,
    rebind_required: u64,
    physical_scope_drift: u64,
    wrong_tenant_scope: u64,
    missing_authenticity_requirement: u64,
    replayed_custody_posture: u64,
}

impl ExpectedTypedCounters {
    pub(crate) const fn admitted() -> Self {
        Self::none()
    }

    pub(crate) const fn physical_scope_drift() -> Self {
        Self {
            physical_scope_drift: 1,
            ..Self::none()
        }
    }

    pub(crate) const fn stale_key_posture() -> Self {
        Self {
            stale_key_posture: 1,
            ..Self::none()
        }
    }

    pub(crate) const fn wrong_tenant_scope() -> Self {
        Self {
            wrong_tenant_scope: 1,
            ..Self::none()
        }
    }

    pub(crate) const fn missing_authenticity_requirement() -> Self {
        Self {
            missing_authenticity_requirement: 1,
            ..Self::none()
        }
    }

    pub(crate) const fn replayed_custody_posture() -> Self {
        Self {
            replayed_custody_posture: 1,
            ..Self::none()
        }
    }

    const fn none() -> Self {
        Self {
            stale_key_posture: 0,
            rebind_required: 0,
            physical_scope_drift: 0,
            wrong_tenant_scope: 0,
            missing_authenticity_requirement: 0,
            replayed_custody_posture: 0,
        }
    }
}

pub(crate) fn assert_security_scope_harness_evidence(
    evidence: SecurityScopeHarnessEvidence,
    expected_outcome: SecurityScopeHarnessOutcomeKind,
    expected_readiness_acceptances: u64,
    expected_denied_before_decode: u64,
) {
    assert_eq!(evidence.observation().outcome(), expected_outcome);
    assert_eq!(evidence.oracle().outcome(), expected_outcome);
    assert!(evidence.oracle().satisfied());
    assert!(evidence.oracle().no_s11_claim());
    assert_eq!(evidence.counters().scenarios_executed(), 1);
    assert_eq!(evidence.counters().scope_admission_attempts(), 1);
    assert_eq!(
        evidence.counters().readiness_acceptances(),
        expected_readiness_acceptances
    );
    assert_eq!(
        evidence.counters().denied_before_logical_decode(),
        expected_denied_before_decode
    );
}

pub(crate) fn assert_security_scope_typed_counters(
    evidence: SecurityScopeHarnessEvidence,
    expected: ExpectedTypedCounters,
) {
    let counters = evidence.counters();
    assert_eq!(counters.stale_key_posture(), expected.stale_key_posture);
    assert_eq!(counters.rebind_required(), expected.rebind_required);
    assert_eq!(
        counters.physical_scope_drift(),
        expected.physical_scope_drift
    );
    assert_eq!(counters.wrong_tenant_scope(), expected.wrong_tenant_scope);
    assert_eq!(
        counters.missing_authenticity_requirement(),
        expected.missing_authenticity_requirement
    );
    assert_eq!(
        counters.replayed_custody_posture(),
        expected.replayed_custody_posture
    );
}

pub(crate) fn assert_lower_store_counter_crosscheck(
    evidence: SecurityScopeHarnessEvidence,
    expected: ExpectedTypedCounters,
) {
    let lower = evidence.lower_store_admission_counters();
    assert_eq!(lower.requests(), 1);
    assert_eq!(lower.current_authority_checks(), 1);
    assert_eq!(
        lower.witnesses_issued(),
        evidence.counters().readiness_acceptances() * WITNESSES_PER_SECURITY_SCOPE_ADMISSION
    );
    assert_eq!(
        evidence.counters().physical_scope_drift(),
        lower.wrong_physical_scopes()
    );
    assert_eq!(
        evidence.counters().wrong_tenant_scope(),
        lower.wrong_tenant_scopes()
    );
    assert_eq!(
        evidence.counters().missing_authenticity_requirement(),
        lower.missing_authenticity_requirements()
    );
    assert_eq!(
        evidence.counters().stale_key_posture(),
        lower.stale_key_postures()
    );
    assert_eq!(
        evidence.counters().rebind_required(),
        lower.rebind_required_key_postures()
    );
    assert_eq!(
        evidence.counters().replayed_custody_posture(),
        lower.replayed_admission_evidence() + lower.readmission_required()
    );
    assert_eq!(lower.denials(), expected.denial_count());
    assert_lower_counter_family_checks(lower, expected);
}

pub(crate) fn replay_scenario(
    schedule: SecurityScopeHarnessSchedule,
    mutation: SecurityScopeReplayMutationKind,
) -> SecurityScopeHarnessScenario {
    match mutation {
        SecurityScopeReplayMutationKind::ChangedTenantScope => {
            SecurityScopeHarnessScenario::wrong_tenant_scope(schedule)
        }
        SecurityScopeReplayMutationKind::ChangedKeyVersionPosture => {
            SecurityScopeHarnessScenario::stale_key_posture(schedule)
        }
        SecurityScopeReplayMutationKind::ChangedAuthenticityRequirement => {
            SecurityScopeHarnessScenario::missing_authenticity_requirement(schedule)
        }
    }
}

pub(crate) fn expected_counters_for_mutation(
    mutation: SecurityScopeReplayMutationKind,
) -> ExpectedTypedCounters {
    match mutation {
        SecurityScopeReplayMutationKind::ChangedTenantScope => {
            ExpectedTypedCounters::wrong_tenant_scope()
        }
        SecurityScopeReplayMutationKind::ChangedKeyVersionPosture => {
            ExpectedTypedCounters::stale_key_posture()
        }
        SecurityScopeReplayMutationKind::ChangedAuthenticityRequirement => {
            ExpectedTypedCounters::missing_authenticity_requirement()
        }
    }
}

pub(crate) fn physical_replay_for_scenario(
    scenario: SecurityScopeHarnessScenario,
) -> SecurityScopePhysicalReplayEvidence {
    let binding = scenario.schedule().physical_replay_binding();
    physical_replay_for_scenario_with_binding(scenario, binding)
        .expect("Phase 10 scenario must bind to its S5 replay lane")
}

pub(crate) fn physical_replay_for_scenario_with_binding(
    scenario: SecurityScopeHarnessScenario,
    binding: SecurityScopePhysicalScheduleBinding,
) -> Result<SecurityScopePhysicalReplayEvidence, SecurityScopePhysicalReplayDenial> {
    let lane = physical_lane_for_binding(binding);
    let plan = s5_interleaving_harness_support::lower_lane(&lane);
    let replay = s5_interleaving_harness_support::replay_bundle(&plan, lane.expected_fault());
    SecurityScopePhysicalReplayEvidence::try_from_replay_bundle(replay, scenario, binding)
}

pub(crate) fn physical_replay_for_scenario_with_replay_binding(
    scenario: SecurityScopeHarnessScenario,
    scenario_binding: SecurityScopePhysicalScheduleBinding,
    replay_binding: SecurityScopePhysicalScheduleBinding,
) -> Result<SecurityScopePhysicalReplayEvidence, SecurityScopePhysicalReplayDenial> {
    let lane = physical_lane_for_binding(replay_binding);
    let plan = s5_interleaving_harness_support::lower_lane(&lane);
    let replay = s5_interleaving_harness_support::replay_bundle(&plan, lane.expected_fault());
    SecurityScopePhysicalReplayEvidence::try_from_replay_bundle(replay, scenario, scenario_binding)
}

pub(crate) fn physical_lane_for_binding(
    binding: SecurityScopePhysicalScheduleBinding,
) -> forge_store_certification::PhysicalIsolationHarnessLane {
    forge_store_certification::physical_isolation_lanes()
        .into_iter()
        .find(|lane| {
            lane.name() == binding.physical_isolation_lane_name()
                && lane.scenario().definition().family()
                    == binding.physical_isolation_scenario_family()
        })
        .expect("Phase 10 binding must map to an S5 physical harness lane")
}

pub(crate) fn assert_physical_binding_matches_replay(
    physical_replay: &SecurityScopePhysicalReplayEvidence,
) {
    let binding = physical_replay.binding();
    assert_eq!(physical_replay.scenario().schedule(), binding.schedule());
    assert_eq!(
        physical_replay.replay_bundle().plan().scenario_family(),
        binding.physical_isolation_scenario_family()
    );
    assert_eq!(
        physical_replay
            .replay_bundle()
            .schedule()
            .replay_identity_matches_plan(physical_replay.replay_bundle().plan()),
        true
    );
}

fn assert_lower_counter_family_checks(
    lower: StoreSecurityScopeAdmissionCounterSnapshot,
    expected: ExpectedTypedCounters,
) {
    assert_eq!(lower.wrong_physical_scopes(), expected.physical_scope_drift);
    assert_eq!(lower.wrong_tenant_scopes(), expected.wrong_tenant_scope);
    assert_eq!(
        lower.missing_authenticity_requirements(),
        expected.missing_authenticity_requirement
    );
    assert_eq!(lower.stale_key_postures(), expected.stale_key_posture);
    assert_eq!(
        lower.replayed_admission_evidence() + lower.readmission_required(),
        expected.replayed_custody_posture
    );
}

impl ExpectedTypedCounters {
    const fn denial_count(self) -> u64 {
        self.physical_scope_drift
            + self.wrong_tenant_scope
            + self.missing_authenticity_requirement
            + self.replayed_custody_posture
    }
}
