use super::*;

pub(in crate::runtime::tests) struct TestSubscriptionActivation;

impl ForgeQueryRuntimeSubscriptionActivationAdapter for TestSubscriptionActivation {
    fn support_evidence(&self) -> String {
        "test-subscription-activation".to_string()
    }

    fn admit_activation(
        &mut self,
        view_name: &str,
        activation: &crate::subscription::SubscriptionActivationInput,
    ) -> Result<String, ForgeQueryWorkspaceError> {
        Ok(format!(
            "test-subscription-activation:{view_name}:{}",
            activation.activation_digest()
        ))
    }
}

pub(in crate::runtime::tests) struct DenyingSubscriptionActivation;

impl ForgeQueryRuntimeSubscriptionActivationAdapter for DenyingSubscriptionActivation {
    fn support_evidence(&self) -> String {
        "denying-subscription-activation".to_string()
    }

    fn admit_activation(
        &mut self,
        _view_name: &str,
        _activation: &crate::subscription::SubscriptionActivationInput,
    ) -> Result<String, ForgeQueryWorkspaceError> {
        Err(ForgeQueryWorkspaceError::new(
            "activation denied by test adapter",
        ))
    }
}

pub(in crate::runtime::tests) struct TestPreviewBasis;

impl ForgeQueryRuntimePreviewBasisAdapter for TestPreviewBasis {
    fn admit_preview_basis(
        &self,
        label: &str,
        effect_policy: ForgeQueryEffectPolicy,
        authority: &ForgeQueryRuntimeEvidenceAuthority,
    ) -> Result<ForgeQueryPreviewBasisAdmission, ForgeQueryWorkspaceError> {
        Ok(ForgeQueryPreviewBasisAdmission::new(
            authority,
            label,
            effect_policy,
            ["test-preview-basis"],
        ))
    }
}

pub(in crate::runtime::tests) struct TestInspectorEvidence;

impl ForgeQueryRuntimeInspectorEvidenceAdapter for TestInspectorEvidence {
    fn inspect_write_receipt(
        &self,
        receipt: &ForgeQueryWriteReceipt,
        authority: &ForgeQueryRuntimeEvidenceAuthority,
    ) -> Result<ForgeQueryRuntimeInspectionEvidence, ForgeQueryWorkspaceError> {
        Ok(ForgeQueryRuntimeInspectionEvidence::new(
            authority,
            "test-write-receipt",
            receipt.authority_lane(),
            ["test-inspector-evidence"],
        ))
    }
}
