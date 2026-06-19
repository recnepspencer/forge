use super::*;
use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::memory_workspace::ForgeQuerySnapshotIdentity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryBranchIntentReceipt {
    intent_name: String,
    strategy_identity: String,
    strategy_version: String,
    canonical_input_digest: String,
    source_lane: ForgeQueryIntentSourceLane,
    target_lane: ForgeQueryAuthorityLane,
    effect_policy: ForgeQueryEffectPolicy,
    basis_evidence: Vec<String>,
    basis_snapshot_identity: ForgeQuerySnapshotIdentity,
    admission_identity: ForgeQueryEvidenceIdentity,
    obligation_dispatch: Option<crate::runtime::ForgeQueryAuthoritativeMutationObligationDispatch>,
    receipt_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryBranchIntentReceipt {
    pub(in crate::runtime) fn new(
        declaration: &ForgeQueryIntentDeclaration,
        effect_policy: ForgeQueryEffectPolicy,
        basis_admission: &ForgeQueryBranchBasisAdmission,
        basis_snapshot_identity: &ForgeQuerySnapshotIdentity,
        admission: ForgeQueryEffectAdmission,
        obligation_dispatch: Option<
            crate::runtime::ForgeQueryAuthoritativeMutationObligationDispatch,
        >,
    ) -> Self {
        let basis_evidence = basis_admission.evidence().to_vec();
        let canonical_input_digest = declaration.input_digest();
        let admission_identity = forge_query_evidence_identity(
            ForgeQueryEvidenceScope::BranchIntentAdmission,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("intent_name"),
            declaration.name(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("strategy_identity"),
            declaration.strategy_name(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("strategy_version"),
            declaration.strategy_version(),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("canonical_input_digest"),
            &canonical_input_digest,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("source_lane"),
            declaration.source_lane().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("target_lane"),
            declaration.target_lane().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("effect_policy"),
            effect_policy.as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("admitted_action"),
            admission.action().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("admitted_lane"),
            admission.target_lane().as_str(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("basis_admission_identity"),
            basis_admission.admission_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("basis_snapshot_identity"),
            &basis_snapshot_identity.evidence_identity(),
        )
        .optional_value(
            ForgeQueryEvidenceTag::new("graph_obligation_dispatch"),
            obligation_dispatch.as_ref().map(
                crate::runtime::ForgeQueryAuthoritativeMutationObligationDispatch::dispatch_digest,
            ),
        )
        .seal();
        let receipt_identity = forge_query_evidence_identity(
            ForgeQueryEvidenceScope::BranchIntentReceipt,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("admission_identity"),
            &admission_identity,
        )
        .optional_value(
            ForgeQueryEvidenceTag::new("graph_obligation_dispatch"),
            obligation_dispatch.as_ref().map(
                crate::runtime::ForgeQueryAuthoritativeMutationObligationDispatch::dispatch_digest,
            ),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("posture"),
            "branch-local-staged-no-authoritative-execution",
        )
        .seal();
        Self {
            intent_name: declaration.name().to_string(),
            strategy_identity: declaration.strategy_name().to_string(),
            strategy_version: declaration.strategy_version().to_string(),
            canonical_input_digest,
            source_lane: declaration.source_lane(),
            target_lane: declaration.target_lane(),
            effect_policy,
            basis_evidence,
            basis_snapshot_identity: basis_snapshot_identity.clone(),
            admission_identity,
            obligation_dispatch,
            receipt_identity,
        }
    }

    pub fn intent_name(&self) -> &str {
        &self.intent_name
    }

    pub fn strategy_identity(&self) -> &str {
        &self.strategy_identity
    }

    pub fn strategy_version(&self) -> &str {
        &self.strategy_version
    }

    pub fn canonical_input_digest(&self) -> &str {
        &self.canonical_input_digest
    }

    pub fn source_lane(&self) -> ForgeQueryIntentSourceLane {
        self.source_lane
    }

    pub fn target_lane(&self) -> ForgeQueryAuthorityLane {
        self.target_lane
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

    pub fn admission_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.admission_identity
    }

    pub fn admission_digest(&self) -> &str {
        self.admission_identity.as_str()
    }

    pub fn obligation_dispatch(
        &self,
    ) -> Option<&crate::runtime::ForgeQueryAuthoritativeMutationObligationDispatch> {
        self.obligation_dispatch.as_ref()
    }

    pub fn receipt_digest(&self) -> &str {
        self.receipt_identity.as_str()
    }

    pub fn receipt_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.receipt_identity
    }
}
