use worth_store_physical_certification::{
    SecurityScopeHarnessScenario, SecurityScopeHarnessSchedule, SecurityScopePhysicalReplayDenial,
    SecurityScopePhysicalReplayEvidence, SecurityScopePhysicalScheduleBinding,
    SecurityScopeReplayMutationKind,
};

use crate::s5_interleaving_harness_support;

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

pub(crate) fn physical_replay_for_scenario(
    scenario: SecurityScopeHarnessScenario,
) -> SecurityScopePhysicalReplayEvidence {
    let binding = scenario.schedule().physical_replay_binding();
    physical_replay_for_scenario_with_binding(scenario, binding)
        .expect("security-scope scenario must bind to its physical replay lane")
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

pub(crate) fn physical_lane_for_binding(
    binding: SecurityScopePhysicalScheduleBinding,
) -> worth_store_certification::PhysicalIsolationHarnessLane {
    worth_store_certification::physical_isolation_lanes()
        .into_iter()
        .find(|lane| {
            lane.name() == binding.physical_isolation_lane_name()
                && lane.scenario().definition().family()
                    == binding.physical_isolation_scenario_family()
        })
        .expect("security-scope binding must map to a physical harness lane")
}
