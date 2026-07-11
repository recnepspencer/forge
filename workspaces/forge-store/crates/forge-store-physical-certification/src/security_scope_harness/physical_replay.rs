use crate::SimulationReplayBundle;

use super::{
    SecurityScopeHarnessEvidence, SecurityScopeHarnessScenario,
    SecurityScopePhysicalScheduleBinding,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecurityScopePhysicalReplayEvidence {
    replay_bundle: SimulationReplayBundle,
    scenario: SecurityScopeHarnessScenario,
    binding: SecurityScopePhysicalScheduleBinding,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityScopePhysicalReplayDenial {
    PhysicalScheduleIdentityMismatch,
    PhysicalReplayFamilyMismatch,
    PhysicalReplayScenarioMismatch,
    BaselineScenarioMismatch,
    ReplayScenarioMismatch,
    MissingBaselineAdmission,
    MissingReplayDenialBeforeLogicalDecode,
}

impl SecurityScopePhysicalReplayEvidence {
    pub fn try_from_replay_bundle(
        replay_bundle: SimulationReplayBundle,
        scenario: SecurityScopeHarnessScenario,
        binding: SecurityScopePhysicalScheduleBinding,
    ) -> Result<Self, SecurityScopePhysicalReplayDenial> {
        if binding.schedule() != scenario.schedule() {
            return Err(SecurityScopePhysicalReplayDenial::PhysicalReplayScenarioMismatch);
        }
        if replay_bundle.plan().scenario_family() != binding.physical_isolation_scenario_family() {
            return Err(SecurityScopePhysicalReplayDenial::PhysicalReplayFamilyMismatch);
        }
        Ok(Self {
            replay_bundle,
            scenario,
            binding,
        })
    }

    pub const fn replay_bundle(&self) -> &SimulationReplayBundle {
        &self.replay_bundle
    }

    pub const fn scenario(&self) -> SecurityScopeHarnessScenario {
        self.scenario
    }

    pub const fn binding(&self) -> SecurityScopePhysicalScheduleBinding {
        self.binding
    }

    pub fn same_physical_schedule_identity_as(&self, other: &Self) -> bool {
        self.replay_bundle.schedule().identity() == other.replay_bundle.schedule().identity()
    }
}

pub(super) fn require_replay_physical_binding(
    baseline_physical_replay: &SecurityScopePhysicalReplayEvidence,
    replay_physical_replay: &SecurityScopePhysicalReplayEvidence,
    baseline_evidence: SecurityScopeHarnessEvidence,
    replay_evidence: SecurityScopeHarnessEvidence,
) -> Result<(), SecurityScopePhysicalReplayDenial> {
    if baseline_physical_replay.scenario()
        != SecurityScopeHarnessScenario::metadata_preserved(baseline_evidence.scenario().schedule())
    {
        return Err(SecurityScopePhysicalReplayDenial::BaselineScenarioMismatch);
    }
    if replay_physical_replay.scenario() != replay_evidence.scenario() {
        return Err(SecurityScopePhysicalReplayDenial::ReplayScenarioMismatch);
    }
    if !baseline_physical_replay.same_physical_schedule_identity_as(replay_physical_replay) {
        return Err(SecurityScopePhysicalReplayDenial::PhysicalScheduleIdentityMismatch);
    }
    if baseline_evidence.counters().readiness_acceptances() != 1 {
        return Err(SecurityScopePhysicalReplayDenial::MissingBaselineAdmission);
    }
    if replay_evidence.counters().denied_before_logical_decode() != 1 {
        return Err(SecurityScopePhysicalReplayDenial::MissingReplayDenialBeforeLogicalDecode);
    }
    Ok(())
}
