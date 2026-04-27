use super::*;

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
    basis_snapshot_token: String,
    admission_digest: String,
    receipt_digest: String,
}

impl ForgeQueryBranchIntentReceipt {
    pub(in crate::runtime) fn new(
        declaration: &ForgeQueryIntentDeclaration,
        effect_policy: ForgeQueryEffectPolicy,
        basis_admission: &ForgeQueryBranchBasisAdmission,
        basis_snapshot_token: &str,
        admission: ForgeQueryEffectAdmission,
    ) -> Self {
        let basis_evidence = basis_admission.evidence().to_vec();
        let canonical_input_digest = declaration.input_digest();
        let admission_digest = hash_parts(&[
            "forge_query_branch_intent_admission_v1".to_string(),
            format!("intent:{}", declaration.name()),
            format!("strategy:{}", declaration.strategy_name()),
            format!("version:{}", declaration.strategy_version()),
            format!("input:{canonical_input_digest}"),
            format!("source:{}", declaration.source_lane().as_str()),
            format!("target:{}", declaration.target_lane()),
            format!("policy:{}", effect_policy.as_str()),
            format!("admitted_action:{}", admission.action()),
            format!("admitted_lane:{}", admission.target_lane()),
            format!("basis_label:{}", basis_admission.label()),
            format!("basis_lane:{}", basis_admission.authority_lane()),
            format!("basis_snapshot:{basis_snapshot_token}"),
            format!("basis_evidence:{}", basis_evidence.join("|")),
        ]);
        let receipt_digest = hash_parts(&[
            "forge_query_branch_intent_receipt_v1".to_string(),
            admission_digest.clone(),
            "posture:branch-local-staged-no-authoritative-execution".to_string(),
        ]);
        Self {
            intent_name: declaration.name().to_string(),
            strategy_identity: declaration.strategy_name().to_string(),
            strategy_version: declaration.strategy_version().to_string(),
            canonical_input_digest,
            source_lane: declaration.source_lane(),
            target_lane: declaration.target_lane(),
            effect_policy,
            basis_evidence,
            basis_snapshot_token: basis_snapshot_token.to_string(),
            admission_digest,
            receipt_digest,
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

    pub fn basis_snapshot_token(&self) -> &str {
        &self.basis_snapshot_token
    }

    pub fn admission_digest(&self) -> &str {
        &self.admission_digest
    }

    pub fn receipt_digest(&self) -> &str {
        &self.receipt_digest
    }
}
