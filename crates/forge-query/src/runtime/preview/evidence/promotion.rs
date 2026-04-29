use super::super::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryPreviewPromotionDenialKind {
    StaleBasis,
    WriteFailed,
    AtomicBatchUnsupported,
}

impl ForgeQueryPreviewPromotionDenialKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::StaleBasis => "stale-basis",
            Self::WriteFailed => "write-failed",
            Self::AtomicBatchUnsupported => "atomic-batch-unsupported",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryPreviewPromotionDenialEvidence {
    label: String,
    kind: ForgeQueryPreviewPromotionDenialKind,
    effect_policy: ForgeQueryEffectPolicy,
    basis_evidence: Vec<String>,
    basis_snapshot_token: String,
    promotion_snapshot_token: String,
    staged_preview_write_count: usize,
    promoted_write_count: usize,
    failed_write_sequence: Option<usize>,
    preview_binding_count: usize,
    reason: String,
    denial_digest: String,
}

impl ForgeQueryPreviewPromotionDenialEvidence {
    #[allow(clippy::too_many_arguments)]
    pub(in crate::runtime::preview) fn new(
        label: &str,
        kind: ForgeQueryPreviewPromotionDenialKind,
        effect_policy: ForgeQueryEffectPolicy,
        basis_admission: &ForgeQueryPreviewBasisAdmission,
        basis_snapshot_token: &str,
        promotion_snapshot_token: &str,
        staged_preview_write_count: usize,
        promoted_write_count: usize,
        failed_write_sequence: Option<usize>,
        preview_binding_count: usize,
        reason: String,
    ) -> Self {
        let basis_evidence = basis_admission.evidence().to_vec();
        let denial_digest = hash_parts(&[
            "forge_query_preview_promotion_denial_evidence_v1".to_string(),
            format!("label:{label}"),
            format!("kind:{}", kind.as_str()),
            format!("policy:{}", effect_policy.as_str()),
            format!("basis_label:{}", basis_admission.label()),
            format!("basis_lane:{}", basis_admission.authority_lane()),
            format!("basis_snapshot:{basis_snapshot_token}"),
            format!("promotion_snapshot:{promotion_snapshot_token}"),
            format!("basis_evidence:{}", basis_evidence.join("|")),
            format!("staged_preview_writes:{staged_preview_write_count}"),
            format!("promoted_writes:{promoted_write_count}"),
            format!(
                "failed_write_sequence:{}",
                failed_write_sequence
                    .map(|sequence| sequence.to_string())
                    .unwrap_or_else(|| "none".to_string())
            ),
            format!("preview_bindings:{preview_binding_count}"),
            format!("reason:{reason}"),
        ]);
        Self {
            label: label.to_string(),
            kind,
            effect_policy,
            basis_evidence,
            basis_snapshot_token: basis_snapshot_token.to_string(),
            promotion_snapshot_token: promotion_snapshot_token.to_string(),
            staged_preview_write_count,
            promoted_write_count,
            failed_write_sequence,
            preview_binding_count,
            reason,
            denial_digest,
        }
    }

    pub(in crate::runtime::preview) fn stale_basis(
        label: &str,
        effect_policy: ForgeQueryEffectPolicy,
        basis_admission: &ForgeQueryPreviewBasisAdmission,
        basis_snapshot_token: &str,
        promotion_snapshot_token: &str,
        staged_preview_write_count: usize,
        preview_binding_count: usize,
    ) -> Self {
        Self::new(
            label,
            ForgeQueryPreviewPromotionDenialKind::StaleBasis,
            effect_policy,
            basis_admission,
            basis_snapshot_token,
            promotion_snapshot_token,
            staged_preview_write_count,
            0,
            None,
            preview_binding_count,
            "preview promotion rejected because authoritative basis changed before promotion"
                .to_string(),
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub(in crate::runtime::preview) fn write_failed(
        label: &str,
        effect_policy: ForgeQueryEffectPolicy,
        basis_admission: &ForgeQueryPreviewBasisAdmission,
        basis_snapshot_token: &str,
        promotion_snapshot_token: &str,
        staged_preview_write_count: usize,
        promoted_write_count: usize,
        failed_write_sequence: usize,
        preview_binding_count: usize,
        reason: String,
    ) -> Self {
        Self::new(
            label,
            ForgeQueryPreviewPromotionDenialKind::WriteFailed,
            effect_policy,
            basis_admission,
            basis_snapshot_token,
            promotion_snapshot_token,
            staged_preview_write_count,
            promoted_write_count,
            Some(failed_write_sequence),
            preview_binding_count,
            reason,
        )
    }

    pub(in crate::runtime::preview) fn atomic_batch_unsupported(
        label: &str,
        effect_policy: ForgeQueryEffectPolicy,
        basis_admission: &ForgeQueryPreviewBasisAdmission,
        basis_snapshot_token: &str,
        promotion_snapshot_token: &str,
        staged_preview_write_count: usize,
        preview_binding_count: usize,
    ) -> Self {
        Self::new(
            label,
            ForgeQueryPreviewPromotionDenialKind::AtomicBatchUnsupported,
            effect_policy,
            basis_admission,
            basis_snapshot_token,
            promotion_snapshot_token,
            staged_preview_write_count,
            0,
            None,
            preview_binding_count,
            "preview promotion rejected because multiple staged writes require atomic promotion support"
                .to_string(),
        )
    }

    pub fn label(&self) -> &str {
        &self.label
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

    pub fn basis_snapshot_token(&self) -> &str {
        &self.basis_snapshot_token
    }

    pub fn promotion_snapshot_token(&self) -> &str {
        &self.promotion_snapshot_token
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

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn denial_digest(&self) -> &str {
        &self.denial_digest
    }
}
