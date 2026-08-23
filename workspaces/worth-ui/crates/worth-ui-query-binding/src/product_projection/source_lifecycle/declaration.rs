use worth_foundational::facade::ScalarAspectType;
use worth_query::facade::{foundation, read, runtime};
use worth_runtime_bridge::facade::{
    AdmittedBridgeAsyncRequestIdentity, BridgeAsyncRequestAdmissionRequest,
    BridgeAsyncRequestTruthViewBasis, BridgeAsyncSourceDeclarationDraft,
    BridgeAsyncSourceDeclarationIdentity, BridgeAsyncSourceLegacyDeclarationIdentity,
    RelationalBridgeSnapshotIdentityParts, RuntimeBridge, TruthBranchIdentity, TruthCommitIdentity,
    TruthSnapshotIdentity,
};
use worth_signal::facade::{
    NodeId, ResourceNodeDeclaration, ResourceObservationPolicyDeclaration, ResourcePayloadContract,
    ResourcePayloadContractId,
};

use crate::{
    UiProjectionFieldRequirement, UiScalarProjectionBinding, UiScalarProjectionBindingAdmission,
    UiScalarProjectionRegistration, WorthUiQueryHost,
};

use super::super::WorthUiScalarProjectionInstallationError;
use super::{ScalarLiveView, WorthUiScalarProjectionAdvanceError};

#[allow(
    clippy::result_large_err,
    reason = "cold binding installation preserves exact Query failure topology"
)]
pub(super) fn scalar_binding(
    workspace: &runtime::WorthQueryWorkspace,
) -> Result<
    (UiScalarProjectionBinding, UiScalarProjectionRegistration),
    WorthUiScalarProjectionInstallationError,
> {
    let installed = WorthUiQueryHost::from_workspace(workspace)
        .installed_domain()
        .map_err(|error| {
            WorthUiScalarProjectionInstallationError::SourceLifecycle(format!("{error:?}"))
        })?;
    let view = installed
        .projection_view("platform.pulse.status")
        .map_err(|error| {
            WorthUiScalarProjectionInstallationError::SourceLifecycle(format!("{error:?}"))
        })?;
    let registration = UiScalarProjectionRegistration::text(
        view,
        UiProjectionFieldRequirement::query_text_status(),
    );
    match registration.clone().admit(workspace) {
        UiScalarProjectionBindingAdmission::Ready(binding) => Ok((binding, registration)),
        other => Err(WorthUiScalarProjectionInstallationError::SourceLifecycle(
            format!("{other:?}"),
        )),
    }
}

#[allow(
    clippy::result_large_err,
    reason = "cold view declaration preserves Query's exact runtime denial"
)]
pub(super) fn declare_scalar_view(
    workspace: &mut runtime::WorthQueryWorkspace,
    request: &AdmittedBridgeAsyncRequestIdentity,
) -> Result<ScalarLiveView, runtime::WorthQueryRuntimeError> {
    workspace.declare_bridge_async_live_view(
        "platform.pulse.status",
        foundation::DeclarativeLiveQueryRequest::new(
            "WorthUiProjectionText",
            foundation::DeclarativeLiveViewShape::table(),
        )
        .project(projection("identity", "id"))
        .project(projection("query_text", "status"))
        .project(projection("query_revision", "value")),
        read::QuerySchemaView::new(
            "worth-ui-platform-pulse",
            [
                schema_field("identity", "id", ScalarAspectType::String),
                schema_field("query_text", "status", ScalarAspectType::String),
                schema_field("query_revision", "value", ScalarAspectType::UInt64),
            ],
            [],
        ),
        request,
    )
}

fn projection(aspect: &str, field: &str) -> foundation::DeclarativeProjectionField {
    foundation::DeclarativeProjectionField::new(
        foundation::AspectFieldKey::from_authoring_parts(aspect, field)
            .expect("static projection field must admit"),
    )
    .delivered_as(format!("{aspect}.{field}"))
}

fn schema_field(aspect: &str, field: &str, family: ScalarAspectType) -> read::SchemaFieldView {
    read::SchemaFieldView::new(
        read::AspectName::new(aspect).expect("static aspect must admit"),
        read::FieldName::new(field).expect("static field must admit"),
        family,
    )
}

pub(super) fn async_request(
    bridge: &RuntimeBridge,
    revision: u64,
) -> Result<AdmittedBridgeAsyncRequestIdentity, String> {
    let draft = BridgeAsyncSourceDeclarationDraft::request_response(
        BridgeAsyncSourceDeclarationIdentity::from_stable_name("worth-ui:platform-pulse"),
        BridgeAsyncSourceLegacyDeclarationIdentity::from_stable_name(
            "worth-ui:platform-pulse:legacy",
        ),
        ResourceNodeDeclaration::new(
            worth_signal::facade::ResourceNodeId::from_node(NodeId::new(313, 13)),
            ResourcePayloadContract::new(ResourcePayloadContractId::new(313))
                .with_max_payload_bytes(65_544),
        )
        .with_observation_policy(ResourceObservationPolicyDeclaration::LifecycleOnly),
    );
    let validated = bridge
        .validate_async_source_declaration(draft)
        .map_err(|error| format!("{error:?}"))?;
    let lowered = bridge
        .lower_async_source_declaration(&validated)
        .map_err(|error| format!("{error:?}"))?;
    let binding = bridge.bind_async_request_basis(&lowered, truth_basis(revision));
    let request = BridgeAsyncRequestAdmissionRequest::request_response(&lowered, &binding)
        .map_err(|error| format!("{error:?}"))?;
    bridge
        .admit_async_request_identity(request)
        .map_err(|error| format!("{error:?}"))
}

pub(super) fn truth_basis(revision: u64) -> BridgeAsyncRequestTruthViewBasis {
    BridgeAsyncRequestTruthViewBasis::authoritative(
        TruthBranchIdentity::from_relational_branch_id("worth-ui-platform-pulse"),
        TruthCommitIdentity::from_relational_commit_id(revision),
        TruthSnapshotIdentity::from_relational_snapshot(
            RelationalBridgeSnapshotIdentityParts::new(313, revision),
        ),
    )
}

pub(super) fn admitted_completion(
    bridge: &RuntimeBridge,
    request: &AdmittedBridgeAsyncRequestIdentity,
    payload_bytes: u64,
) -> Result<
    worth_runtime_bridge::facade::AdmittedBridgeAsyncCompletion,
    WorthUiScalarProjectionAdvanceError,
> {
    let envelope = worth_signal::facade::RawCompletionEnvelope::new(
        request.request_handle().request_id(),
        request.request_handle().generation(),
        request.request_handle().branch_epoch(),
        request.attempt(),
        request
            .lowered()
            .resource_descriptor()
            .expect("request-response declaration retains its descriptor")
            .payload_contract_digest()
            .clone(),
        payload_bytes,
    );
    let validated = bridge
        .validate_async_completion_envelope(request, envelope)
        .map_err(|error| WorthUiScalarProjectionAdvanceError::Bridge(format!("{error:?}")))?;
    bridge
        .admit_async_completion(request, &validated)
        .map_err(|error| WorthUiScalarProjectionAdvanceError::Bridge(format!("{error:?}")))?
        .admitted_completion()
        .cloned()
        .ok_or_else(|| {
            WorthUiScalarProjectionAdvanceError::Bridge(
                "validated scalar completion did not admit".to_string(),
            )
        })
}
