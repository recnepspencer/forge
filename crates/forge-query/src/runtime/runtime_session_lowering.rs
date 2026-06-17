use super::live_subscription::{
    live_subscription_source_identity, live_subscription_view_shape_source_identity,
};
use super::*;
use crate::subscription::SubscriptionActivationInput;

pub(super) struct LoweredRuntimeLiveSubscriptionRequest {
    pub(super) query_identity: crate::ForgeQueryEvidenceIdentity,
    pub(super) live_view_identity: crate::ForgeQueryEvidenceIdentity,
    pub(super) canonical_result_shape_digest: crate::identity::CanonicalResultShapeDigest,
    pub(super) subscription_family: crate::subscription::QuerySubscriptionFamily,
    pub(super) subscription_declaration_identity: crate::ForgeQueryEvidenceIdentity,
    pub(super) admission_identity: crate::ForgeQueryEvidenceIdentity,
    pub(super) bridge_declaration_identity: crate::ForgeQueryEvidenceIdentity,
    pub(super) basis_binding_identity: crate::ForgeQueryEvidenceIdentity,
    pub(super) signal_strategy_identity: crate::ForgeQueryEvidenceIdentity,
    pub(super) activation: SubscriptionActivationInput,
}

pub(super) fn lower_runtime_live_subscription_request(
    backend: &dyn ForgeQueryRuntimeBackend,
    view_name: &str,
    request: &DeclarativeLiveQueryRequest,
    schema_view: QuerySchemaView,
) -> Result<LoweredRuntimeLiveSubscriptionRequest, ForgeQueryRuntimeError> {
    let session = declare_runtime_live_query_session_with_grouped_baseline(
        request.clone(),
        schema_view,
        backend.current_snapshot_identity(),
        grouped_baseline_members_or_error(backend, view_name, request)?,
    )
    .map_err(|error| live_subscription_error(view_name, "live-lowering", error))?;
    let view_family = session.live_view().lowering().family();
    let dimensions = subscription_dimensions_for_request(request, view_family)?;
    let live_admission =
        crate::subscription::LiveQueryAdmissionArtifact::from_live_promotion_with_view(
            session.live_view().core_live_plan().descriptor(),
            crate::subscription::QuerySubscriptionBasisPosture::CurrentHead,
            view_family,
            dimensions,
        );
    let selection = select_runtime_subscription_family(view_name, live_admission)?;
    let subscription_family = selection.family().clone();
    let declaration =
        declare_query_subscription(selection, runtime_slice_budget()).map_err(|error| {
            ForgeQueryRuntimeError::LiveSubscriptionInstallation {
                view_name: view_name.to_string(),
                stage: "declaration",
                message: format!("{error:?}"),
            }
        })?;
    let lowering =
        lower_query_subscription_to_bridge(declaration, runtime_bridge_lowering_budget()).map_err(
            |error| ForgeQueryRuntimeError::LiveSubscriptionInstallation {
                view_name: view_name.to_string(),
                stage: "bridge-lowering",
                message: format!("{error:?}"),
            },
        )?;
    let admission = admit_query_subscription(lowering, runtime_subscription_admission_budget())
        .map_err(
            |error| ForgeQueryRuntimeError::LiveSubscriptionInstallation {
                view_name: view_name.to_string(),
                stage: "subscription-admission",
                message: format!("{error:?}"),
            },
        )?;

    Ok(LoweredRuntimeLiveSubscriptionRequest {
        query_identity: live_subscription_source_identity(
            "query",
            admission.query_declaration_identity(),
        ),
        live_view_identity: live_subscription_source_identity(
            "live_view",
            &live_subscription_view_shape_source_identity(view_family),
        ),
        canonical_result_shape_digest: session.canonical().result_shape().digest().clone(),
        subscription_family,
        subscription_declaration_identity: live_subscription_source_identity(
            "subscription_declaration",
            admission.query_declaration_identity(),
        ),
        admission_identity: live_subscription_source_identity(
            "admission",
            admission.evidence_identity(),
        ),
        bridge_declaration_identity: live_subscription_source_identity(
            "bridge_declaration",
            admission.bridge_declaration_identity(),
        ),
        basis_binding_identity: live_subscription_source_identity(
            "basis_binding",
            admission.basis_binding_identity(),
        ),
        signal_strategy_identity: live_subscription_source_identity(
            "signal_strategy",
            admission.signal_strategy_identity(),
        ),
        activation: prepare_subscription_activation(admission),
    })
}

pub(super) fn grouped_baseline_members_or_error(
    backend: &dyn ForgeQueryRuntimeBackend,
    view_name: &str,
    request: &DeclarativeLiveQueryRequest,
) -> Result<Option<Vec<(String, String)>>, ForgeQueryRuntimeError> {
    backend.grouped_baseline_members(request).map_err(|error| {
        ForgeQueryRuntimeError::LiveSubscriptionInstallation {
            view_name: view_name.to_string(),
            stage: "grouped-baseline",
            message: error.to_string(),
        }
    })
}

pub(super) fn select_runtime_subscription_family(
    view_name: &str,
    live_admission: crate::subscription::LiveQueryAdmissionArtifact,
) -> Result<crate::subscription::QuerySubscriptionFamilySelection, ForgeQueryRuntimeError> {
    select_query_subscription_family(live_admission, runtime_family_budget()).map_err(|error| {
        ForgeQueryRuntimeError::LiveSubscriptionInstallation {
            view_name: view_name.to_string(),
            stage: "family-selection",
            message: format!("{error:?}"),
        }
    })
}

pub(super) fn install_live_subscription_activation(
    backend: &mut dyn ForgeQueryRuntimeBackend,
    view_name: &str,
    activation: &SubscriptionActivationInput,
) -> Result<SubscriptionActivationReceipt, ForgeQueryRuntimeError> {
    let activation_receipt = backend
        .install_live_subscription(view_name, activation)
        .map_err(
            |error| ForgeQueryRuntimeError::LiveSubscriptionInstallation {
                view_name: view_name.to_string(),
                stage: "activation-admission",
                message: error.to_string(),
            },
        )?;
    if let Some(message) = activation_receipt.drift_from_activation(view_name, activation) {
        return Err(ForgeQueryRuntimeError::LiveSubscriptionInstallation {
            view_name: view_name.to_string(),
            stage: "activation-receipt",
            message,
        });
    }
    Ok(activation_receipt)
}
