use super::{
    ForgeQueryAuthorityLane, ForgeQueryBranchBasisAdmission, ForgeQueryBranchIntentReceipt,
    ForgeQueryBranchOptions, ForgeQueryEffectPolicy, ForgeQueryIntentDeclaration,
    ForgeQueryIntentDenialEvidence, ForgeQueryIntentSourceLane, ForgeQueryRuntime,
    ForgeQueryRuntimeError, ForgeQueryRuntimeFacadeFamily,
};

pub struct ForgeQueryBranchSession<'a> {
    label: String,
    runtime: &'a mut ForgeQueryRuntime,
    effect_policy: ForgeQueryEffectPolicy,
    basis_admission: ForgeQueryBranchBasisAdmission,
    basis_snapshot_token: String,
    intent_receipts: Vec<ForgeQueryBranchIntentReceipt>,
}

impl<'a> ForgeQueryBranchSession<'a> {
    pub(super) fn new(
        label: impl Into<String>,
        runtime: &'a mut ForgeQueryRuntime,
        options: ForgeQueryBranchOptions,
        basis_admission: ForgeQueryBranchBasisAdmission,
    ) -> Self {
        let basis_snapshot_token = runtime.snapshot_token();
        Self {
            label: label.into(),
            runtime,
            effect_policy: options.effect_policy(),
            basis_admission,
            basis_snapshot_token,
            intent_receipts: Vec::new(),
        }
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn effect_policy(&self) -> ForgeQueryEffectPolicy {
        self.effect_policy
    }

    pub fn basis_admission(&self) -> &ForgeQueryBranchBasisAdmission {
        &self.basis_admission
    }

    pub fn branch_intent_receipts(&self) -> &[ForgeQueryBranchIntentReceipt] {
        &self.intent_receipts
    }

    pub fn execute_intent(
        &mut self,
        declaration: ForgeQueryIntentDeclaration,
    ) -> Result<ForgeQueryBranchIntentReceipt, ForgeQueryRuntimeError> {
        self.runtime.admit_facade_family_lane(
            ForgeQueryRuntimeFacadeFamily::Intent,
            ForgeQueryAuthorityLane::BranchLocalTruth,
        )?;
        let declaration = declaration
            .with_source_lane(ForgeQueryIntentSourceLane::BranchLocal)
            .with_target_lane(ForgeQueryAuthorityLane::BranchLocalTruth);
        let admission = crate::runtime::intent::admit_branch_intent_declaration(
            &declaration,
            self.effect_policy,
        )
        .map_err(|denial| ForgeQueryRuntimeError::IntentCommitDenied {
            intent_name: declaration.name().to_string(),
            stage: denial.stage(),
            message: denial.message().to_string(),
            evidence: ForgeQueryIntentDenialEvidence::new(&declaration, &denial, None),
        })?;
        let receipt = ForgeQueryBranchIntentReceipt::new(
            &declaration,
            self.effect_policy,
            &self.basis_admission,
            &self.basis_snapshot_token,
            admission,
        );
        self.intent_receipts.push(receipt.clone());
        Ok(receipt)
    }
}
