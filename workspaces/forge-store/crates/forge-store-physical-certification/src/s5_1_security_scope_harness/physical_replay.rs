use crate::SimulationReplayBundle;

use super::{
    S51SecurityScopeHarnessEvidence, S51SecurityScopeHarnessScenario,
    S51SecurityScopePhysicalScheduleBinding,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S51SecurityScopePhysicalReplayEvidence {
    replay_bundle: SimulationReplayBundle,
    scenario: S51SecurityScopeHarnessScenario,
    binding: S51SecurityScopePhysicalScheduleBinding,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S51SecurityScopePhysicalReplayDenial {
    PhysicalScheduleIdentityMismatch,
    PhysicalReplayFamilyMismatch,
    PhysicalReplayScenarioMismatch,
    BaselineScenarioMismatch,
    ReplayScenarioMismatch,
    MissingBaselineAdmission,
    MissingReplayDenialBeforeLogicalDecode,
}

impl S51SecurityScopePhysicalReplayEvidence {
    pub fn try_from_replay_bundle(
        replay_bundle: SimulationReplayBundle,
        scenario: S51SecurityScopeHarnessScenario,
        binding: S51SecurityScopePhysicalScheduleBinding,
    ) -> Result<Self, S51SecurityScopePhysicalReplayDenial> {
        if binding.schedule() != scenario.schedule() {
            return Err(S51SecurityScopePhysicalReplayDenial::PhysicalReplayScenarioMismatch);
        }
        if replay_bundle.plan().scenario_family() != binding.s5_scenario_family() {
            return Err(S51SecurityScopePhysicalReplayDenial::PhysicalReplayFamilyMismatch);
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

    pub const fn scenario(&self) -> S51SecurityScopeHarnessScenario {
        self.scenario
    }

    pub const fn binding(&self) -> S51SecurityScopePhysicalScheduleBinding {
        self.binding
    }

    pub fn same_physical_schedule_identity_as(&self, other: &Self) -> bool {
        self.replay_bundle.schedule().identity() == other.replay_bundle.schedule().identity()
    }
}

pub(super) fn require_replay_physical_binding(
    baseline_physical_replay: &S51SecurityScopePhysicalReplayEvidence,
    replay_physical_replay: &S51SecurityScopePhysicalReplayEvidence,
    baseline_evidence: S51SecurityScopeHarnessEvidence,
    replay_evidence: S51SecurityScopeHarnessEvidence,
) -> Result<(), S51SecurityScopePhysicalReplayDenial> {
    if baseline_physical_replay.scenario()
        != S51SecurityScopeHarnessScenario::metadata_preserved(
            baseline_evidence.scenario().schedule(),
        )
    {
        return Err(S51SecurityScopePhysicalReplayDenial::BaselineScenarioMismatch);
    }
    if replay_physical_replay.scenario() != replay_evidence.scenario() {
        return Err(S51SecurityScopePhysicalReplayDenial::ReplayScenarioMismatch);
    }
    if !baseline_physical_replay.same_physical_schedule_identity_as(replay_physical_replay) {
        return Err(S51SecurityScopePhysicalReplayDenial::PhysicalScheduleIdentityMismatch);
    }
    if baseline_evidence.counters().readiness_acceptances() != 1 {
        return Err(S51SecurityScopePhysicalReplayDenial::MissingBaselineAdmission);
    }
    if replay_evidence.counters().denied_before_logical_decode() != 1 {
        return Err(S51SecurityScopePhysicalReplayDenial::MissingReplayDenialBeforeLogicalDecode);
    }
    Ok(())
}
