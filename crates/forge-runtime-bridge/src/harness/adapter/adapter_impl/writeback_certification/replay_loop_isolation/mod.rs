use crate::facade::{
    BridgeWritebackError, BridgeWritebackFeedbackContext, BridgeWritebackIdempotenceBasis,
    BridgeWritebackLoopPreventionReport, BridgeWritebackReplayBundle,
};
use crate::writeback::{
    BridgeDerivedWritebackEffect, BridgeWritebackExecutionRecord, BridgeWritebackReplayRecord,
};

mod changed_causality;
mod cross_family_replay;
mod family_evidence;
mod feedback_isolation;
mod same_family_equivalence;

pub(in crate::harness::adapter::adapter_impl) use changed_causality::ReplayLoopChangedCausalityIsolation;
pub(in crate::harness::adapter::adapter_impl) use cross_family_replay::ReplayLoopCrossFamilyIsolation;
pub(in crate::harness::adapter::adapter_impl) use family_evidence::ReplayLoopFamilyEvidence;
pub(in crate::harness::adapter::adapter_impl) use feedback_isolation::ReplayLoopFeedbackIsolation;
pub(in crate::harness::adapter::adapter_impl) use same_family_equivalence::ReplayLoopSameFamilyEquivalence;

pub(in crate::harness::adapter::adapter_impl) struct WritebackReplayLoopIsolationMatrixEvidence<'a>
{
    pub projected_effect: &'a BridgeDerivedWritebackEffect,
    pub aspect_effect: &'a BridgeDerivedWritebackEffect,
    pub projected_idempotence: &'a BridgeWritebackIdempotenceBasis,
    pub aspect_idempotence: &'a BridgeWritebackIdempotenceBasis,
    pub projected_bundle: &'a BridgeWritebackReplayBundle,
    pub aspect_bundle: &'a BridgeWritebackReplayBundle,
    pub cross_family_replay_error: &'a BridgeWritebackError,
    pub cross_family_replay_record: &'a BridgeWritebackReplayRecord,
    pub rebuilt_projected_effect: &'a BridgeDerivedWritebackEffect,
    pub rebuilt_projected_bundle: &'a BridgeWritebackReplayBundle,
    pub rebuilt_execution_record: &'a BridgeWritebackExecutionRecord,
    pub changed_projected_bundle: &'a BridgeWritebackReplayBundle,
    pub same_family_drift_error: &'a BridgeWritebackError,
    pub same_family_drift_replay_record: &'a BridgeWritebackReplayRecord,
    pub projected_feedback_context: &'a BridgeWritebackFeedbackContext,
    pub cross_family_loop_prevention: &'a BridgeWritebackLoopPreventionReport,
}

pub(in crate::harness::adapter::adapter_impl) struct WritebackReplayLoopIsolationMatrix {
    projected_family: ReplayLoopFamilyEvidence,
    aspect_family: ReplayLoopFamilyEvidence,
    cross_family_replay_isolation: ReplayLoopCrossFamilyIsolation,
    same_family_equivalence: ReplayLoopSameFamilyEquivalence,
    same_family_changed_causality: ReplayLoopChangedCausalityIsolation,
    cross_family_loop_isolation: ReplayLoopFeedbackIsolation,
}

impl WritebackReplayLoopIsolationMatrix {
    pub(in crate::harness::adapter::adapter_impl) fn from_replay_loop_evidence(
        evidence: WritebackReplayLoopIsolationMatrixEvidence<'_>,
    ) -> Self {
        Self {
            projected_family: ReplayLoopFamilyEvidence::from_family_evidence(
                evidence.projected_effect,
                evidence.projected_idempotence,
                evidence.projected_bundle,
            ),
            aspect_family: ReplayLoopFamilyEvidence::from_family_evidence(
                evidence.aspect_effect,
                evidence.aspect_idempotence,
                evidence.aspect_bundle,
            ),
            cross_family_replay_isolation: ReplayLoopCrossFamilyIsolation::from_replay_error(
                evidence.projected_bundle,
                evidence.aspect_bundle,
                evidence.cross_family_replay_error,
                evidence.cross_family_replay_record,
            ),
            same_family_equivalence: ReplayLoopSameFamilyEquivalence::from_rebuilt_family(
                evidence.projected_effect,
                evidence.projected_bundle,
                evidence.rebuilt_projected_effect,
                evidence.rebuilt_projected_bundle,
                evidence.rebuilt_execution_record,
            ),
            same_family_changed_causality:
                ReplayLoopChangedCausalityIsolation::from_changed_causality(
                    evidence.projected_bundle,
                    evidence.changed_projected_bundle,
                    evidence.same_family_drift_error,
                    evidence.same_family_drift_replay_record,
                ),
            cross_family_loop_isolation: ReplayLoopFeedbackIsolation::from_loop_prevention(
                evidence.projected_feedback_context,
                evidence.cross_family_loop_prevention,
            ),
        }
    }

    pub(in crate::harness::adapter::adapter_impl) fn projected_family(
        &self,
    ) -> &ReplayLoopFamilyEvidence {
        &self.projected_family
    }

    pub(in crate::harness::adapter::adapter_impl) fn aspect_family(
        &self,
    ) -> &ReplayLoopFamilyEvidence {
        &self.aspect_family
    }

    pub(in crate::harness::adapter::adapter_impl) fn cross_family_replay_isolation(
        &self,
    ) -> &ReplayLoopCrossFamilyIsolation {
        &self.cross_family_replay_isolation
    }

    pub(in crate::harness::adapter::adapter_impl) fn same_family_equivalence(
        &self,
    ) -> &ReplayLoopSameFamilyEquivalence {
        &self.same_family_equivalence
    }

    pub(in crate::harness::adapter::adapter_impl) fn same_family_changed_causality(
        &self,
    ) -> &ReplayLoopChangedCausalityIsolation {
        &self.same_family_changed_causality
    }

    pub(in crate::harness::adapter::adapter_impl) fn cross_family_loop_isolation(
        &self,
    ) -> &ReplayLoopFeedbackIsolation {
        &self.cross_family_loop_isolation
    }
}
