use super::super::*;
use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryPreviewPromotionDenialKind {
    StaleBasis,
    WriteFailed,
    AtomicBatchUnsupported,
    RebindingRequired,
}

impl ForgeQueryPreviewPromotionDenialKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::StaleBasis => "stale-basis",
            Self::WriteFailed => "write-failed",
            Self::AtomicBatchUnsupported => "atomic-batch-unsupported",
            Self::RebindingRequired => "rebinding-required",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryPreviewPromotionDenialEvidence {
    session_label: ForgeQuerySessionLabel,
    kind: ForgeQueryPreviewPromotionDenialKind,
    effect_policy: ForgeQueryEffectPolicy,
    basis_evidence: Vec<String>,
    basis_snapshot_identity: ForgeQuerySnapshotIdentity,
    promotion_snapshot_identity: ForgeQuerySnapshotIdentity,
    staged_preview_write_count: usize,
    promoted_write_count: usize,
    failed_write_sequence: Option<usize>,
    preview_binding_count: usize,
    crossed_authoritative_residue_count: usize,
    recovery_posture: String,
    rebinding_identity: Option<ForgeQueryEvidenceIdentity>,
    reason: String,
    denial_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryPreviewPromotionDenialEvidence {
    #[allow(clippy::too_many_arguments)]
    pub(in crate::runtime::preview) fn new(
        kind: ForgeQueryPreviewPromotionDenialKind,
        effect_policy: ForgeQueryEffectPolicy,
        basis_admission: &ForgeQueryPreviewBasisAdmission,
        basis_snapshot_identity: &ForgeQuerySnapshotIdentity,
        promotion_snapshot_identity: &ForgeQuerySnapshotIdentity,
        staged_preview_write_count: usize,
        promoted_write_count: usize,
        failed_write_sequence: Option<usize>,
        preview_binding_count: usize,
        crossed_authoritative_residue_count: usize,
        recovery_posture: String,
        rebinding_identity: Option<ForgeQueryEvidenceIdentity>,
        reason: String,
    ) -> Self {
        let basis_evidence_rows = basis_admission.evidence_rows();
        let mut denial_builder =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::PreviewPromotionDenialEvidence)
                .field_value(
                    ForgeQueryEvidenceTag::new("session_label_identity"),
                    basis_admission.label_identity().as_str(),
                )
                .field_shape(ForgeQueryEvidenceTag::new("kind"), kind.as_str())
                .field_shape(
                    ForgeQueryEvidenceTag::new("effect_policy"),
                    effect_policy.as_str(),
                )
                .field_value(
                    ForgeQueryEvidenceTag::new("basis_admission_digest"),
                    basis_admission.admission_digest().as_str(),
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("basis_snapshot_identity"),
                    &basis_snapshot_identity.evidence_identity(),
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("promotion_snapshot_identity"),
                    &promotion_snapshot_identity.evidence_identity(),
                )
                .field_value_sequence(
                    ForgeQueryEvidenceTag::new("basis_evidence_row"),
                    basis_evidence_rows
                        .iter()
                        .map(|row| row.row_digest().as_str()),
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("staged_preview_write_count"),
                    staged_preview_write_count,
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("promoted_write_count"),
                    promoted_write_count,
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("preview_binding_count"),
                    preview_binding_count,
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("crossed_authoritative_residue_count"),
                    crossed_authoritative_residue_count,
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("recovery_posture"),
                    recovery_posture.as_str(),
                )
                .field_value(ForgeQueryEvidenceTag::new("reason"), reason.as_str());
        if let Some(failed_write_sequence) = failed_write_sequence {
            denial_builder = denial_builder.field_usize(
                ForgeQueryEvidenceTag::new("failed_write_sequence"),
                failed_write_sequence,
            );
        }
        if let Some(rebinding_identity) = rebinding_identity.as_ref() {
            denial_builder = denial_builder.field_value(
                ForgeQueryEvidenceTag::new("rebinding_digest"),
                rebinding_identity.as_str(),
            );
        }
        let denial_identity = denial_builder.seal();
        let basis_evidence = basis_admission.evidence();
        Self {
            session_label: basis_admission.session_label().clone(),
            kind,
            effect_policy,
            basis_evidence,
            basis_snapshot_identity: basis_snapshot_identity.clone(),
            promotion_snapshot_identity: promotion_snapshot_identity.clone(),
            staged_preview_write_count,
            promoted_write_count,
            failed_write_sequence,
            preview_binding_count,
            crossed_authoritative_residue_count,
            recovery_posture,
            rebinding_identity,
            reason,
            denial_identity,
        }
    }

    pub(in crate::runtime::preview) fn stale_basis(
        effect_policy: ForgeQueryEffectPolicy,
        basis_admission: &ForgeQueryPreviewBasisAdmission,
        basis_snapshot_identity: &ForgeQuerySnapshotIdentity,
        promotion_snapshot_identity: &ForgeQuerySnapshotIdentity,
        staged_preview_write_count: usize,
        preview_binding_count: usize,
    ) -> Self {
        Self::new(
            ForgeQueryPreviewPromotionDenialKind::StaleBasis,
            effect_policy,
            basis_admission,
            basis_snapshot_identity,
            promotion_snapshot_identity,
            staged_preview_write_count,
            0,
            None,
            preview_binding_count,
            0,
            "refresh_preview_basis".to_string(),
            None,
            "preview promotion rejected because authoritative basis changed before promotion"
                .to_string(),
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub(in crate::runtime::preview) fn write_failed(
        effect_policy: ForgeQueryEffectPolicy,
        basis_admission: &ForgeQueryPreviewBasisAdmission,
        basis_snapshot_identity: &ForgeQuerySnapshotIdentity,
        promotion_snapshot_identity: &ForgeQuerySnapshotIdentity,
        staged_preview_write_count: usize,
        promoted_write_count: usize,
        failed_write_sequence: usize,
        preview_binding_count: usize,
        reason: String,
    ) -> Self {
        Self::new(
            ForgeQueryPreviewPromotionDenialKind::WriteFailed,
            effect_policy,
            basis_admission,
            basis_snapshot_identity,
            promotion_snapshot_identity,
            staged_preview_write_count,
            promoted_write_count,
            Some(failed_write_sequence),
            preview_binding_count,
            0,
            "retry_authoritative_write".to_string(),
            None,
            reason,
        )
    }

    pub(in crate::runtime::preview) fn atomic_batch_unsupported(
        effect_policy: ForgeQueryEffectPolicy,
        basis_admission: &ForgeQueryPreviewBasisAdmission,
        basis_snapshot_identity: &ForgeQuerySnapshotIdentity,
        promotion_snapshot_identity: &ForgeQuerySnapshotIdentity,
        staged_preview_write_count: usize,
        preview_binding_count: usize,
    ) -> Self {
        Self::new(
            ForgeQueryPreviewPromotionDenialKind::AtomicBatchUnsupported,
            effect_policy,
            basis_admission,
            basis_snapshot_identity,
            promotion_snapshot_identity,
            staged_preview_write_count,
            0,
            None,
            preview_binding_count,
            0,
            "promote_with_atomic_batch_support".to_string(),
            None,
            "preview promotion rejected because multiple staged writes require atomic promotion support"
                .to_string(),
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub(in crate::runtime::preview) fn rebinding_required(
        effect_policy: ForgeQueryEffectPolicy,
        basis_admission: &ForgeQueryPreviewBasisAdmission,
        basis_snapshot_identity: &ForgeQuerySnapshotIdentity,
        promotion_snapshot_identity: &ForgeQuerySnapshotIdentity,
        staged_preview_write_count: usize,
        preview_binding_count: usize,
        crossed_authoritative_residue_count: usize,
        rebinding_identity: ForgeQueryEvidenceIdentity,
    ) -> Self {
        Self::new(
            ForgeQueryPreviewPromotionDenialKind::RebindingRequired,
            effect_policy,
            basis_admission,
            basis_snapshot_identity,
            promotion_snapshot_identity,
            staged_preview_write_count,
            0,
            None,
            preview_binding_count,
            crossed_authoritative_residue_count,
            "discard_preview_and_readmit_authoritative".to_string(),
            Some(rebinding_identity),
            "preview promotion rejected because preview-owned temporal or async residue requires authoritative re-admission before promotion"
                .to_string(),
        )
    }

    pub fn label(&self) -> &str {
        self.session_label.display()
    }

    pub fn session_label(&self) -> &ForgeQuerySessionLabel {
        &self.session_label
    }

    pub fn label_identity(&self) -> &crate::ForgeQueryEvidenceIdentity {
        self.session_label.identity_digest()
    }

    pub fn kind(&self) -> ForgeQueryPreviewPromotionDenialKind {
        self.kind
    }

    pub fn effect_policy(&self) -> ForgeQueryEffectPolicy {
        self.effect_policy
    }

    pub fn basis_evidence(&self) -> &[String] {
        &self.basis_evidence
    }

    pub fn basis_snapshot_identity(&self) -> &ForgeQuerySnapshotIdentity {
        &self.basis_snapshot_identity
    }

    pub fn promotion_snapshot_identity(&self) -> &ForgeQuerySnapshotIdentity {
        &self.promotion_snapshot_identity
    }

    pub fn staged_preview_write_count(&self) -> usize {
        self.staged_preview_write_count
    }

    pub fn promoted_write_count(&self) -> usize {
        self.promoted_write_count
    }

    pub fn failed_write_sequence(&self) -> Option<usize> {
        self.failed_write_sequence
    }

    pub fn preview_binding_count(&self) -> usize {
        self.preview_binding_count
    }

    pub fn crossed_authoritative_residue_count(&self) -> usize {
        self.crossed_authoritative_residue_count
    }

    pub fn recovery_posture(&self) -> &str {
        &self.recovery_posture
    }

    pub fn rebinding_digest(&self) -> Option<&str> {
        self.rebinding_identity
            .as_ref()
            .map(ForgeQueryEvidenceIdentity::as_str)
    }

    pub fn rebinding_identity(&self) -> Option<&ForgeQueryEvidenceIdentity> {
        self.rebinding_identity.as_ref()
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn denial_digest(&self) -> &str {
        self.denial_identity.as_str()
    }

    pub fn denial_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.denial_identity
    }
}
