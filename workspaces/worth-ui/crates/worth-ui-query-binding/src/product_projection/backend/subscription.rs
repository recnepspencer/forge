use worth_query::facade::{foundation, runtime};

pub(super) struct WorthUiScalarProjectionSubscription;

impl runtime::WorthQueryRuntimeSubscriptionActivationAdapter
    for WorthUiScalarProjectionSubscription
{
    fn support_evidence_identity(&self) -> runtime::WorthQueryEvidenceIdentity {
        runtime::runtime_subscription_support_evidence_identity(
            "worth-ui-product-subscription-activation",
        )
    }

    fn admit_activation(
        &mut self,
        view_name: &str,
        activation: &runtime::SubscriptionActivationInput,
    ) -> Result<runtime::SubscriptionActivationBoundaryReceipt, foundation::WorthQueryWorkspaceError>
    {
        let receipt = self.build_subscription_activation_receipt(view_name, activation);
        Ok(self.build_subscription_activation_boundary_receipt(view_name, activation, receipt))
    }
}
