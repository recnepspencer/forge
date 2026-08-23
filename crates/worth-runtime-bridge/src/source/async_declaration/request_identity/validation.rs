use super::super::{BridgeAsyncSourceDeclarationFamilyKind, LoweredBridgeAsyncSourceDeclaration};
use super::binding::ValidatedBridgeAsyncRequestBasisBinding;
use super::rejection::{
    BridgeAsyncRequestIdentityRejection, BridgeAsyncRequestIdentityRejectionKind,
};
use super::subscription_instance::{
    BridgeAsyncRequestSubscriptionInstance, BridgeAsyncRequestSubscriptionInstanceKind,
};
use super::truth_basis::BridgeAsyncRequestTruthViewBasisKind;

pub(super) fn validate_request_response(
    lowered: &LoweredBridgeAsyncSourceDeclaration,
    basis_binding: &ValidatedBridgeAsyncRequestBasisBinding,
) -> Result<(), BridgeAsyncRequestIdentityRejection> {
    if lowered.family_kind() != BridgeAsyncSourceDeclarationFamilyKind::RequestResponse
        || basis_binding.family_kind() != BridgeAsyncSourceDeclarationFamilyKind::RequestResponse
    {
        return Err(family_kind_mismatch(
            "request-response family artifacts for request identity admission",
        ));
    }
    validate_shared_binding(lowered, basis_binding)
}

pub(super) fn validate_subscription_backed(
    lowered: &LoweredBridgeAsyncSourceDeclaration,
    basis_binding: &ValidatedBridgeAsyncRequestBasisBinding,
    subscription_instance: &BridgeAsyncRequestSubscriptionInstance,
) -> Result<(), BridgeAsyncRequestIdentityRejection> {
    if lowered.family_kind() != BridgeAsyncSourceDeclarationFamilyKind::SubscriptionBacked
        || basis_binding.family_kind() != BridgeAsyncSourceDeclarationFamilyKind::SubscriptionBacked
    {
        return Err(family_kind_mismatch(
            "subscription-backed family artifacts for request identity admission",
        ));
    }
    validate_shared_binding(lowered, basis_binding)?;
    match (
        basis_binding.truth_view_basis_kind(),
        subscription_instance.kind(),
        basis_binding
            .truth_view_basis()
            .preview_active_subscription_identity(),
        subscription_instance.preview_active_subscription_identity(),
    ) {
        (
            BridgeAsyncRequestTruthViewBasisKind::Preview,
            BridgeAsyncRequestSubscriptionInstanceKind::Preview,
            Some(left),
            Some(right),
        ) if left == right
            && basis_binding.truth_view_basis().preview_parent_truth_view_basis_digest()
                == subscription_instance.parent_truth_view_basis_digest() => Ok(()),
        (BridgeAsyncRequestTruthViewBasisKind::Preview, _, _, _)
        | (_, BridgeAsyncRequestSubscriptionInstanceKind::Preview, _, _) => Err(
            BridgeAsyncRequestIdentityRejection::new(
                BridgeAsyncRequestIdentityRejectionKind::PreviewBasisSubscriptionInstanceMismatch,
                "bridge async preview truth-view basis must bind to the exact matching preview subscription instance",
            ),
        ),
        _ => Ok(()),
    }
}

fn validate_shared_binding(
    lowered: &LoweredBridgeAsyncSourceDeclaration,
    basis_binding: &ValidatedBridgeAsyncRequestBasisBinding,
) -> Result<(), BridgeAsyncRequestIdentityRejection> {
    if lowered.declaration_identity() != basis_binding.declaration_identity()
        || lowered.lowering_identity() != basis_binding.lowering_identity()
    {
        return Err(BridgeAsyncRequestIdentityRejection::new(
            BridgeAsyncRequestIdentityRejectionKind::LoweringIdentityMismatch,
            "bridge async request identity binding requires one exact lowered declaration and matching request-basis binding",
        ));
    }
    Ok(())
}

pub(super) fn family_kind_mismatch(expected: &str) -> BridgeAsyncRequestIdentityRejection {
    BridgeAsyncRequestIdentityRejection::new(
        BridgeAsyncRequestIdentityRejectionKind::FamilyKindMismatch,
        format!("bridge async request identity binding requires {expected}"),
    )
}
