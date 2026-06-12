use super::super::super::{
    ForgeQueryAuthorityLane, ForgeQueryEffectPolicy, ForgeQueryPreviewCloseoutEvidence,
    ForgeQueryPreviewCloseoutKind, ForgeQueryPreviewOutcome,
};
use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::memory_workspace::ForgeQuerySnapshotIdentity;
use crate::session_label::ForgeQuerySessionLabel;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryPreviewOutcomeInspection {
    label: ForgeQuerySessionLabel,
    closeout_kind: ForgeQueryPreviewCloseoutKind,
    effect_policy: ForgeQueryEffectPolicy,
    promoted: bool,
    discarded: bool,
    write_count: usize,
    preview_binding_count: usize,
    live_binding_count: usize,
    computed_binding_count: usize,
    effect_binding_count: usize,
    subscription_residue_count: usize,
    derived_runtime_residue_count: usize,
    effect_delivery_residue_count: usize,
    pending_write_intent_residue_count: usize,
    preview_write_staging_count: usize,
    promoted_write_count: usize,
    temporal_wake_residue_count: usize,
    async_result_residue_count: usize,
    mixed_cause_residue_count: usize,
    crossed_authoritative_residue_count: usize,
    authoritative_residue_count: usize,
    source_lane: ForgeQueryAuthorityLane,
    target_lane: ForgeQueryAuthorityLane,
    basis_evidence: Vec<String>,
    basis_identity: ForgeQueryEvidenceIdentity,
    preview_basis_snapshot_identity: ForgeQuerySnapshotIdentity,
    target_basis_snapshot_identity: ForgeQuerySnapshotIdentity,
    closeout_identity: ForgeQueryEvidenceIdentity,
    residue_identity: ForgeQueryEvidenceIdentity,
    rebinding_identity: Option<ForgeQueryEvidenceIdentity>,
    inspection_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryPreviewOutcomeInspection {
    pub(in crate::runtime) fn from_outcome(outcome: &ForgeQueryPreviewOutcome) -> Self {
        Self::from_closeout(
            outcome,
            outcome.closeout_evidence(),
            outcome.promoted(),
            outcome.discarded(),
            outcome.write_count(),
            outcome.preview_binding_count(),
            outcome.closeout_evidence().live_binding_count(),
            outcome.closeout_evidence().computed_binding_count(),
            outcome.effect_binding_count(),
            outcome.closeout_evidence().subscription_residue_count(),
            outcome.closeout_evidence().derived_runtime_residue_count(),
            outcome.effect_delivery_residue_count(),
            outcome.pending_write_intent_residue_count(),
            outcome.closeout_evidence().preview_write_staging_count(),
            outcome.closeout_evidence().promoted_write_count(),
            outcome.closeout_evidence().temporal_wake_residue_count(),
            outcome.closeout_evidence().async_result_residue_count(),
            outcome.closeout_evidence().mixed_cause_residue_count(),
            outcome
                .closeout_evidence()
                .crossed_authoritative_residue_count(),
            outcome.authoritative_residue_count(),
            outcome.source_lane(),
            outcome.target_lane(),
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn from_closeout(
        outcome: &ForgeQueryPreviewOutcome,
        closeout: &ForgeQueryPreviewCloseoutEvidence,
        promoted: bool,
        discarded: bool,
        write_count: usize,
        preview_binding_count: usize,
        live_binding_count: usize,
        computed_binding_count: usize,
        effect_binding_count: usize,
        subscription_residue_count: usize,
        derived_runtime_residue_count: usize,
        effect_delivery_residue_count: usize,
        pending_write_intent_residue_count: usize,
        preview_write_staging_count: usize,
        promoted_write_count: usize,
        temporal_wake_residue_count: usize,
        async_result_residue_count: usize,
        mixed_cause_residue_count: usize,
        crossed_authoritative_residue_count: usize,
        authoritative_residue_count: usize,
        source_lane: ForgeQueryAuthorityLane,
        target_lane: ForgeQueryAuthorityLane,
    ) -> Self {
        let basis_evidence = closeout.basis_evidence().to_vec();
        let basis_identity = forge_query_evidence_identity(
            ForgeQueryEvidenceScope::PreviewOutcomeInspectionArtifact,
        )
        .field_shape(ForgeQueryEvidenceTag::new("artifact_kind"), "basis")
        .field_identity(
            ForgeQueryEvidenceTag::new("session_label_identity"),
            outcome.session_label().identity_digest().as_str(),
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("basis_evidence"),
            basis_evidence.iter().map(String::as_str),
        )
        .seal();
        let residue_identity = forge_query_evidence_identity(
            ForgeQueryEvidenceScope::PreviewOutcomeInspectionArtifact,
        )
        .field_shape(ForgeQueryEvidenceTag::new("artifact_kind"), "residue")
        .field_identity(
            ForgeQueryEvidenceTag::new("session_label_identity"),
            outcome.session_label().identity_digest().as_str(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("subscription_residue_count"),
            subscription_residue_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("derived_runtime_residue_count"),
            derived_runtime_residue_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("effect_delivery_residue_count"),
            effect_delivery_residue_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("pending_write_intent_residue_count"),
            pending_write_intent_residue_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("preview_write_staging_count"),
            preview_write_staging_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("promoted_write_count"),
            promoted_write_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("temporal_wake_residue_count"),
            temporal_wake_residue_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("async_result_residue_count"),
            async_result_residue_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("mixed_cause_residue_count"),
            mixed_cause_residue_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("crossed_authoritative_residue_count"),
            crossed_authoritative_residue_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("authoritative_residue_count"),
            authoritative_residue_count,
        )
        .seal();
        let inspection_identity = forge_query_evidence_identity(
            ForgeQueryEvidenceScope::PreviewOutcomeInspectionArtifact,
        )
        .field_shape(ForgeQueryEvidenceTag::new("artifact_kind"), "inspection")
        .field_identity(
            ForgeQueryEvidenceTag::new("session_label_identity"),
            outcome.session_label().identity_digest().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("closeout_kind"),
            closeout.kind().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("effect_policy"),
            outcome.effect_policy().as_str(),
        )
        .field_bool(ForgeQueryEvidenceTag::new("promoted"), promoted)
        .field_bool(ForgeQueryEvidenceTag::new("discarded"), discarded)
        .field_usize(ForgeQueryEvidenceTag::new("write_count"), write_count)
        .field_usize(
            ForgeQueryEvidenceTag::new("preview_binding_count"),
            preview_binding_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("live_binding_count"),
            live_binding_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("computed_binding_count"),
            computed_binding_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("effect_binding_count"),
            effect_binding_count,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("source_lane"),
            source_lane.as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("target_lane"),
            target_lane.as_str(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("basis_identity"),
            &basis_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("preview_basis_snapshot_identity"),
            &closeout
                .preview_basis_snapshot_identity()
                .evidence_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("target_basis_snapshot_identity"),
            &closeout
                .target_basis_snapshot_identity()
                .evidence_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("closeout_identity"),
            closeout.closeout_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("residue_identity"),
            &residue_identity,
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("rebinding_identity"),
            closeout.rebinding_identity(),
        )
        .seal();

        Self {
            label: outcome.session_label().clone(),
            closeout_kind: closeout.kind(),
            effect_policy: outcome.effect_policy(),
            promoted,
            discarded,
            write_count,
            preview_binding_count,
            live_binding_count,
            computed_binding_count,
            effect_binding_count,
            subscription_residue_count,
            derived_runtime_residue_count,
            effect_delivery_residue_count,
            pending_write_intent_residue_count,
            preview_write_staging_count,
            promoted_write_count,
            temporal_wake_residue_count,
            async_result_residue_count,
            mixed_cause_residue_count,
            crossed_authoritative_residue_count,
            authoritative_residue_count,
            source_lane,
            target_lane,
            basis_evidence,
            basis_identity,
            preview_basis_snapshot_identity: closeout.preview_basis_snapshot_identity().clone(),
            target_basis_snapshot_identity: closeout.target_basis_snapshot_identity().clone(),
            closeout_identity: closeout.closeout_identity().clone(),
            residue_identity,
            rebinding_identity: closeout.rebinding_identity().cloned(),
            inspection_identity,
        }
    }

    pub fn label(&self) -> &str {
        self.label.display()
    }
    pub fn session_label(&self) -> &ForgeQuerySessionLabel {
        &self.label
    }
    pub fn closeout_kind(&self) -> ForgeQueryPreviewCloseoutKind {
        self.closeout_kind
    }
    pub fn effect_policy(&self) -> ForgeQueryEffectPolicy {
        self.effect_policy
    }
    pub fn promoted(&self) -> bool {
        self.promoted
    }
    pub fn discarded(&self) -> bool {
        self.discarded
    }
    pub fn write_count(&self) -> usize {
        self.write_count
    }
    pub fn preview_binding_count(&self) -> usize {
        self.preview_binding_count
    }
    pub fn live_binding_count(&self) -> usize {
        self.live_binding_count
    }
    pub fn computed_binding_count(&self) -> usize {
        self.computed_binding_count
    }
    pub fn effect_binding_count(&self) -> usize {
        self.effect_binding_count
    }
    pub fn subscription_residue_count(&self) -> usize {
        self.subscription_residue_count
    }
    pub fn derived_runtime_residue_count(&self) -> usize {
        self.derived_runtime_residue_count
    }
    pub fn effect_delivery_residue_count(&self) -> usize {
        self.effect_delivery_residue_count
    }
    pub fn pending_write_intent_residue_count(&self) -> usize {
        self.pending_write_intent_residue_count
    }
    pub fn preview_write_staging_count(&self) -> usize {
        self.preview_write_staging_count
    }
    pub fn promoted_write_count(&self) -> usize {
        self.promoted_write_count
    }
    pub fn temporal_wake_residue_count(&self) -> usize {
        self.temporal_wake_residue_count
    }
    pub fn async_result_residue_count(&self) -> usize {
        self.async_result_residue_count
    }
    pub fn mixed_cause_residue_count(&self) -> usize {
        self.mixed_cause_residue_count
    }
    pub fn crossed_authoritative_residue_count(&self) -> usize {
        self.crossed_authoritative_residue_count
    }
    pub fn authoritative_residue_count(&self) -> usize {
        self.authoritative_residue_count
    }
    pub fn source_lane(&self) -> ForgeQueryAuthorityLane {
        self.source_lane
    }
    pub fn target_lane(&self) -> ForgeQueryAuthorityLane {
        self.target_lane
    }
    pub fn basis_evidence(&self) -> &[String] {
        &self.basis_evidence
    }
    pub fn basis_digest(&self) -> &str {
        self.basis_identity.as_str()
    }
    pub fn basis_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.basis_identity
    }
    pub fn preview_basis_snapshot_identity(&self) -> &ForgeQuerySnapshotIdentity {
        &self.preview_basis_snapshot_identity
    }
    pub fn closeout_digest(&self) -> &str {
        self.closeout_identity.as_str()
    }
    pub fn target_basis_snapshot_identity(&self) -> &ForgeQuerySnapshotIdentity {
        &self.target_basis_snapshot_identity
    }
    pub fn closeout_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.closeout_identity
    }
    pub fn residue_digest(&self) -> &str {
        self.residue_identity.as_str()
    }
    pub fn residue_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.residue_identity
    }
    pub fn rebinding_digest(&self) -> Option<&str> {
        self.rebinding_identity
            .as_ref()
            .map(ForgeQueryEvidenceIdentity::as_str)
    }
    pub fn rebinding_identity(&self) -> Option<&ForgeQueryEvidenceIdentity> {
        self.rebinding_identity.as_ref()
    }
    pub fn inspection_digest(&self) -> &str {
        self.inspection_identity.as_str()
    }
    pub fn inspection_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.inspection_identity
    }
}
