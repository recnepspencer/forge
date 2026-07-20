use super::*;
use crate::runtime::runtime_subscription_support_evidence_identity;
use crate::WorthQueryEvidenceIdentity;

pub(in crate::runtime::tests) struct TestSubscriptionActivation;

impl WorthQueryRuntimeSubscriptionActivationAdapter for TestSubscriptionActivation {
    fn support_evidence_identity(&self) -> WorthQueryEvidenceIdentity {
        runtime_subscription_support_evidence_identity("test-subscription-activation")
    }

    fn admit_activation(
        &mut self,
        view_name: &str,
        activation: &crate::subscription::SubscriptionActivationInput,
    ) -> Result<SubscriptionActivationBoundaryReceipt, WorthQueryWorkspaceError> {
        let receipt = self.build_subscription_activation_receipt(view_name, activation);
        Ok(self.build_subscription_activation_boundary_receipt(view_name, activation, receipt))
    }
}

pub(in crate::runtime::tests) struct DenyingSubscriptionActivation;

impl WorthQueryRuntimeSubscriptionActivationAdapter for DenyingSubscriptionActivation {
    fn support_evidence_identity(&self) -> WorthQueryEvidenceIdentity {
        runtime_subscription_support_evidence_identity("denying-subscription-activation")
    }

    fn admit_activation(
        &mut self,
        _view_name: &str,
        _activation: &crate::subscription::SubscriptionActivationInput,
    ) -> Result<SubscriptionActivationBoundaryReceipt, WorthQueryWorkspaceError> {
        Err(WorthQueryWorkspaceError::new(
            "activation denied by test adapter",
        ))
    }
}

pub(in crate::runtime::tests) struct DriftingSubscriptionActivation;

impl WorthQueryRuntimeSubscriptionActivationAdapter for DriftingSubscriptionActivation {
    fn support_evidence_identity(&self) -> WorthQueryEvidenceIdentity {
        runtime_subscription_support_evidence_identity("drifting-subscription-activation")
    }

    fn admit_activation(
        &mut self,
        _view_name: &str,
        activation: &crate::subscription::SubscriptionActivationInput,
    ) -> Result<SubscriptionActivationBoundaryReceipt, WorthQueryWorkspaceError> {
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
    pub(in crate::runtime::tests) projection: WorthQueryRuntimeRemaskProjection,
}

impl WorthQueryRuntimeSubscriptionActivationAdapter for RemaskingSubscriptionActivation {
    fn support_evidence_identity(&self) -> WorthQueryEvidenceIdentity {
        runtime_subscription_support_evidence_identity("test-subscription-activation")
    }

    fn remask_projection(
        &self,
        _view_name: &str,
        _activation: &crate::subscription::SubscriptionActivationInput,
    ) -> Option<WorthQueryRuntimeRemaskProjection> {
        Some(self.projection.clone())
    }

    fn admit_activation(
        &mut self,
        view_name: &str,
        activation: &crate::subscription::SubscriptionActivationInput,
    ) -> Result<SubscriptionActivationBoundaryReceipt, WorthQueryWorkspaceError> {
        let receipt = self.build_subscription_activation_receipt(view_name, activation);
        Ok(self.build_subscription_activation_boundary_receipt(view_name, activation, receipt))
    }
}

pub(in crate::runtime::tests) struct TestPreviewBasis;

impl WorthQueryRuntimePreviewBasisAdapter for TestPreviewBasis {
    fn admit_preview_basis(
        &self,
        label: &WorthQuerySessionLabel,
        effect_policy: WorthQueryEffectPolicy,
        authority: &WorthQueryRuntimeEvidenceAuthority,
    ) -> Result<WorthQueryPreviewBasisAdmission, WorthQueryWorkspaceError> {
        Ok(WorthQueryPreviewBasisAdmission::new(
            authority,
            label.clone(),
            effect_policy,
            [WorthQueryBasisAdmissionEvidenceRow::tagged(
                "preview-basis-admission",
                "test-preview-basis",
            )],
        ))
    }
}

pub(in crate::runtime::tests) struct TestInspectorEvidence;

impl WorthQueryRuntimeInspectorEvidenceAdapter for TestInspectorEvidence {
    fn inspect_write_receipt(
        &self,
        receipt: &WorthQueryWriteReceipt,
        authority: &WorthQueryRuntimeEvidenceAuthority,
    ) -> Result<WorthQueryRuntimeInspectionEvidence, WorthQueryWorkspaceError> {
        Ok(WorthQueryRuntimeInspectionEvidence::new(
            authority,
            "test-write-receipt",
            receipt.authority_lane(),
            ["test-inspector-evidence"],
        ))
    }
}
