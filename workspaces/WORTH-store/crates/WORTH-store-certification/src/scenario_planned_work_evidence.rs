use worth_store_contracts::StableDigest;
use worth_store_physical_integrity::{
    FoundationalBoundaryRoleMapping, StorePlannedWorkBoundaryKind,
};

use crate::{PhysicalScenarioPlan, PhysicalScenarioPlanIdentity};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalScenarioPlannedWorkBoundaryReport {
    kind: StorePlannedWorkBoundaryKind,
    mapping: FoundationalBoundaryRoleMapping,
    basis: StableDigest,
    plan_identity: PhysicalScenarioPlanIdentity,
    planned_step_count: u64,
    required_oracle_count: u64,
    expected_counter_count: u64,
}

impl PhysicalScenarioPlannedWorkBoundaryReport {
    pub fn from_scenario_plan(plan: &PhysicalScenarioPlan) -> Self {
        Self {
            kind: StorePlannedWorkBoundaryKind::ScenarioPlan,
            mapping: FoundationalBoundaryRoleMapping::store_planned_work(),
            basis: scenario_plan_digest(plan),
            plan_identity: plan.identity().clone(),
            planned_step_count: plan.story_steps().len() as u64,
            required_oracle_count: plan.required_oracles().len() as u64,
            expected_counter_count: plan.expected_counters().len() as u64,
        }
    }

    pub const fn kind(&self) -> StorePlannedWorkBoundaryKind {
        self.kind
    }

    pub const fn mapping(&self) -> &FoundationalBoundaryRoleMapping {
        &self.mapping
    }

    pub fn basis(&self) -> &StableDigest {
        &self.basis
    }

    pub const fn plan_identity(&self) -> &PhysicalScenarioPlanIdentity {
        &self.plan_identity
    }

    pub const fn planned_step_count(&self) -> u64 {
        self.planned_step_count
    }

    pub const fn required_oracle_count(&self) -> u64 {
        self.required_oracle_count
    }

    pub const fn expected_counter_count(&self) -> u64 {
        self.expected_counter_count
    }
}

fn scenario_plan_digest(plan: &PhysicalScenarioPlan) -> StableDigest {
    StableDigest::new(format!(
        "s3-scenario-planned-work:{:?}:{}:{}:{}",
        plan.identity(),
        plan.story_steps().len(),
        plan.required_oracles().len(),
        plan.expected_counters().len()
    ))
    .expect("S.3 scenario planned-work digest basis is non-empty")
}
