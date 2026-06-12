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
    ) -> Result<SubscriptionActivationBoundaryReceipt, ForgeQueryWorkspaceError> {
        let receipt = self.build_subscription_activation_receipt(view_name, activation);
        Ok(self.build_subscription_activation_boundary_receipt(view_name, activation, receipt))
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
    ) -> Result<SubscriptionActivationBoundaryReceipt, ForgeQueryWorkspaceError> {
        Err(ForgeQueryWorkspaceError::new(
            "activation denied by test adapter",
        ))
    }
}

pub(in crate::runtime::tests) struct DriftingSubscriptionActivation;

impl ForgeQueryRuntimeSubscriptionActivationAdapter for DriftingSubscriptionActivation {
    fn support_evidence(&self) -> String {
        "drifting-subscription-activation".to_string()
    }

    fn admit_activation(
        &mut self,
        _view_name: &str,
        activation: &crate::subscription::SubscriptionActivationInput,
    ) -> Result<SubscriptionActivationBoundaryReceipt, ForgeQueryWorkspaceError> {
        let receipt = self.build_subscription_activation_receipt("drifted.view", activation);
        Ok(
            self.build_subscription_activation_boundary_receipt(
                "drifted.view",
                activation,
                receipt,
            ),
        )
    }
}

pub(in crate::runtime::tests) struct RemaskingSubscriptionActivation {
    pub(in crate::runtime::tests) projection: ForgeQueryRuntimeRemaskProjection,
}

impl ForgeQueryRuntimeSubscriptionActivationAdapter for RemaskingSubscriptionActivation {
    fn support_evidence(&self) -> String {
        "test-subscription-activation".to_string()
    }

    fn remask_projection(
        &self,
        _view_name: &str,
        _activation: &crate::subscription::SubscriptionActivationInput,
    ) -> Option<ForgeQueryRuntimeRemaskProjection> {
        Some(self.projection.clone())
    }

    fn admit_activation(
        &mut self,
        view_name: &str,
        activation: &crate::subscription::SubscriptionActivationInput,
    ) -> Result<SubscriptionActivationBoundaryReceipt, ForgeQueryWorkspaceError> {
        let receipt = self.build_subscription_activation_receipt(view_name, activation);
        Ok(self.build_subscription_activation_boundary_receipt(view_name, activation, receipt))
    }
}

pub(in crate::runtime::tests) struct TestPreviewBasis;

impl ForgeQueryRuntimePreviewBasisAdapter for TestPreviewBasis {
    fn admit_preview_basis(
        &self,
        label: &ForgeQuerySessionLabel,
        effect_policy: ForgeQueryEffectPolicy,
        authority: &ForgeQueryRuntimeEvidenceAuthority,
    ) -> Result<ForgeQueryPreviewBasisAdmission, ForgeQueryWorkspaceError> {
        Ok(ForgeQueryPreviewBasisAdmission::new(
            authority,
            label.clone(),
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
