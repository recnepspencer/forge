use crate::scenario::S7BlobHarnessScenarioMetadata;
use crate::{
    CheckpointInterlockObservation, CompactionInterlockObservation, IndependentVerifierObservation,
    ObservedPhysicalTrace, ObserverKind, PhysicalScenarioCanonicalIdentity, PhysicalSimulationPlan,
    PhysicalSimulationPlanIdentity, PhysicalSimulationScenarioFamily, RecoveryOutcomeObservation,
    S6IoPressureOracleObservation, S7BlobHarnessOracleObservation, ShortcutRejectionObservation,
    ShortcutRejectionObservationKind,
};

use super::OracleDenial;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OracleVerdictBasis {
    scenario_identity: PhysicalScenarioCanonicalIdentity,
    scenario_family: PhysicalSimulationScenarioFamily,
    plan_identity: PhysicalSimulationPlanIdentity,
    observer: ObserverKind,
    runtime_trace_present: bool,
    independent_verifier: Option<IndependentVerifierObservation>,
    recovery_outcome: Option<RecoveryOutcomeObservation>,
    checkpoint_interlock: Option<CheckpointInterlockObservation>,
    compaction_interlock: Option<CompactionInterlockObservation>,
    s6_io_pressure: Option<S6IoPressureOracleObservation>,
    s7_blob_harness_metadata: Option<S7BlobHarnessScenarioMetadata>,
    s7_blob_harness_observation: Option<S7BlobHarnessOracleObservation>,
    shortcut_rejections: Vec<ShortcutRejectionObservation>,
}

impl OracleVerdictBasis {
    pub(crate) fn from_plan_and_trace(
        plan: &PhysicalSimulationPlan,
        trace: &ObservedPhysicalTrace,
    ) -> Result<Self, OracleDenial> {
        if trace.scenario_identity() != plan.scenario_identity()
            || trace.plan_identity() != plan.identity()
        {
            return Err(OracleDenial::PlanTraceIdentityMismatch);
        }
        Ok(Self {
            scenario_identity: plan.scenario_identity().clone(),
            scenario_family: plan.scenario_family(),
            plan_identity: plan.identity().clone(),
            observer: trace.observer(),
            runtime_trace_present: true,
            independent_verifier: trace.independent_verifier().cloned(),
            recovery_outcome: trace.recovery_outcome().cloned(),
            checkpoint_interlock: trace.checkpoint_interlock(),
            compaction_interlock: trace.compaction_interlock(),
            s6_io_pressure: trace.s6_io_pressure_observation(),
            s7_blob_harness_metadata: plan.s7_blob_harness_metadata(),
            s7_blob_harness_observation: trace.s7_blob_harness_observation(),
            shortcut_rejections: trace.shortcut_rejections().to_vec(),
        })
    }

    pub const fn scenario_identity(&self) -> &PhysicalScenarioCanonicalIdentity {
        &self.scenario_identity
    }

    pub const fn scenario_family(&self) -> PhysicalSimulationScenarioFamily {
        self.scenario_family
    }

    pub const fn plan_identity(&self) -> &PhysicalSimulationPlanIdentity {
        &self.plan_identity
    }

    pub const fn observer(&self) -> ObserverKind {
        self.observer
    }

    pub const fn runtime_trace_present(&self) -> bool {
        self.runtime_trace_present
    }

    pub fn independent_verifier(&self) -> Option<&IndependentVerifierObservation> {
        self.independent_verifier.as_ref()
    }

    pub fn independent_verifier_present(&self) -> bool {
        self.independent_verifier.is_some()
    }

    pub fn recovery_outcome(&self) -> Option<&RecoveryOutcomeObservation> {
        self.recovery_outcome.as_ref()
    }

    pub const fn checkpoint_interlock_present(&self) -> bool {
        self.checkpoint_interlock.is_some()
    }

    pub const fn checkpoint_interlock(&self) -> Option<CheckpointInterlockObservation> {
        self.checkpoint_interlock
    }

    pub const fn compaction_interlock_present(&self) -> bool {
        self.compaction_interlock.is_some()
    }

    pub const fn compaction_interlock(&self) -> Option<CompactionInterlockObservation> {
        self.compaction_interlock
    }

    pub const fn s6_io_pressure(&self) -> Option<S6IoPressureOracleObservation> {
        self.s6_io_pressure
    }

    pub const fn s7_blob_harness_metadata(&self) -> Option<S7BlobHarnessScenarioMetadata> {
        self.s7_blob_harness_metadata
    }

    pub const fn s7_blob_harness_observation(&self) -> Option<S7BlobHarnessOracleObservation> {
        self.s7_blob_harness_observation
    }

    pub fn has_shortcut_rejection(&self, kind: ShortcutRejectionObservationKind) -> bool {
        self.shortcut_rejections
            .iter()
            .any(|observation| observation.kind() == kind)
    }

    pub fn shortcut_rejections(&self) -> &[ShortcutRejectionObservation] {
        &self.shortcut_rejections
    }
}
