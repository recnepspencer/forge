use crate::s4_recovery_harness::{
    RecoveryPhysicsCounterExpectation, RecoveryPhysicsCrashLane, RecoveryPhysicsOracleJudgment,
    RecoveryPhysicsScenarioPlan,
};
use worth_store_test_support::StorageBoundaryEvent;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryPhysicsEvidenceBundle {
    id: String,
    lane: RecoveryPhysicsCrashLane,
    backend_profile: &'static str,
}

impl RecoveryPhysicsEvidenceBundle {
    pub fn from_plan(plan: &RecoveryPhysicsScenarioPlan) -> Self {
        Self {
            id: format!(
                "s4-recovery:{}:{}:{}",
                plan.lane().as_str(),
                plan.seed(),
                plan.backend_profile()
            ),
            lane: plan.lane(),
            backend_profile: plan.backend_profile(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub const fn lane(&self) -> RecoveryPhysicsCrashLane {
        self.lane
    }

    pub const fn backend_profile(&self) -> &'static str {
        self.backend_profile
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryPhysicsTranscript {
    lane: RecoveryPhysicsCrashLane,
    driver_name: &'static str,
    observer_names: Vec<&'static str>,
    oracle_judgments: Vec<RecoveryPhysicsOracleJudgment>,
    evidence_bundle: RecoveryPhysicsEvidenceBundle,
    boundary_event: StorageBoundaryEvent,
    seed: u64,
    backend_profile: &'static str,
    counter_expectations: Vec<RecoveryPhysicsCounterExpectation>,
}

impl RecoveryPhysicsTranscript {
    pub fn from_plan(
        plan: &RecoveryPhysicsScenarioPlan,
        oracle_judgments: Vec<RecoveryPhysicsOracleJudgment>,
    ) -> Self {
        Self {
            lane: plan.lane(),
            driver_name: "fresh-runtime-storage-boundary-fault-scheduler",
            observer_names: vec![
                "crash-lane",
                "storage-boundary",
                "fresh-runtime",
                "oracle-evidence",
                "counter-evidence",
                "evidence-bundle",
                "transcript",
            ],
            oracle_judgments,
            evidence_bundle: RecoveryPhysicsEvidenceBundle::from_plan(plan),
            boundary_event: plan.definition().boundary_event().clone(),
            seed: plan.seed(),
            backend_profile: plan.backend_profile(),
            counter_expectations: plan.definition().counter_expectations().to_vec(),
        }
    }

    pub const fn lane(&self) -> RecoveryPhysicsCrashLane {
        self.lane
    }

    pub const fn driver_name(&self) -> &'static str {
        self.driver_name
    }

    pub fn observer_names(&self) -> &[&'static str] {
        &self.observer_names
    }

    pub fn oracle_judgments(&self) -> &[RecoveryPhysicsOracleJudgment] {
        &self.oracle_judgments
    }

    pub const fn evidence_bundle(&self) -> &RecoveryPhysicsEvidenceBundle {
        &self.evidence_bundle
    }

    pub const fn boundary_event(&self) -> &StorageBoundaryEvent {
        &self.boundary_event
    }

    pub const fn seed(&self) -> u64 {
        self.seed
    }

    pub const fn backend_profile(&self) -> &'static str {
        self.backend_profile
    }

    pub fn counter_expectations(&self) -> &[RecoveryPhysicsCounterExpectation] {
        &self.counter_expectations
    }
}
