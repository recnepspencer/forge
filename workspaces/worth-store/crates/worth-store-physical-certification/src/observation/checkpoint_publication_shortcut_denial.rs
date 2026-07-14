use crate::{
    ForbiddenShortcutKind, ObservationDenial, PhysicalInterleavingSchedule,
    PhysicalIsolationCheckpointPublicationLaneBinding, PhysicalScenarioActorRole,
    ShortcutRejectionBoundary, ShortcutRejectionObservation, SyntheticHarnessShortcutDenialReceipt,
};
use worth_store_physical_isolation::{
    CheckpointInterlockEvidenceOrigin, CheckpointInterlockFoundationalEvidence,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalIsolationCheckpointPublicationShortcutDenialLaneOutput {
    plan_identity: [u8; 32],
    schedule_identity: [u8; 32],
    checkpoint_origin: CheckpointInterlockEvidenceOrigin,
    shortcut_actor_step_index: usize,
    observation: ShortcutRejectionObservation,
}

impl PhysicalIsolationCheckpointPublicationShortcutDenialLaneOutput {
    pub fn from_denial_receipt(
        binding: &PhysicalIsolationCheckpointPublicationLaneBinding,
        schedule: &PhysicalInterleavingSchedule,
        shortcut_actor_step_index: usize,
        expected_origin: &CheckpointInterlockEvidenceOrigin,
        evidence: CheckpointInterlockFoundationalEvidence,
        receipt: SyntheticHarnessShortcutDenialReceipt,
    ) -> Result<Self, ObservationDenial> {
        schedule
            .actor_steps()
            .get(shortcut_actor_step_index)
            .filter(|step| step.actor_role() == PhysicalScenarioActorRole::ShortcutRejectionProbe)
            .ok_or(ObservationDenial::CheckpointPublicationShortcutLaneScheduleMismatch)?;
        if schedule.identity().digest_bytes() != binding.schedule_identity_digest() {
            return Err(ObservationDenial::CheckpointPublicationShortcutLaneScheduleMismatch);
        }
        if expected_origin != evidence.origin() {
            return Err(ObservationDenial::CheckpointPublicationEvidenceOriginMismatch);
        }
        if receipt.shortcut() != ForbiddenShortcutKind::SameRunSelfComparison
            || !matches!(
                receipt.boundary(),
                ShortcutRejectionBoundary::EvidenceSameRunSelfComparison
                    | ShortcutRejectionBoundary::EvidenceSameRunTranscript
                    | ShortcutRejectionBoundary::HarnessBoundarySameRunSelfComparison
            )
        {
            return Err(ObservationDenial::CheckpointPublicationShortcutBoundaryMismatch);
        }
        Ok(Self {
            plan_identity: *binding.plan_identity_digest(),
            schedule_identity: *binding.schedule_identity_digest(),
            checkpoint_origin: expected_origin.clone(),
            shortcut_actor_step_index,
            observation: ShortcutRejectionObservation::same_run_self_comparison_denied(),
        })
    }

    pub(crate) const fn plan_identity(&self) -> &[u8; 32] {
        &self.plan_identity
    }

    pub(crate) const fn schedule_identity(&self) -> &[u8; 32] {
        &self.schedule_identity
    }

    pub(crate) const fn checkpoint_origin(&self) -> &CheckpointInterlockEvidenceOrigin {
        &self.checkpoint_origin
    }

    pub(crate) const fn shortcut_actor_step_index(&self) -> usize {
        self.shortcut_actor_step_index
    }

    pub(crate) const fn observation(&self) -> ShortcutRejectionObservation {
        self.observation
    }
}
