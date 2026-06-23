use crate::declarative_live::DeclarativeLiveQueryRequest;
use crate::evidence_identity::ForgeQueryEvidenceTag;
use crate::lower_runtime_routing::ForgeQueryLowerRuntimeSubjectIdentity;
use crate::runtime::{
    ForgeQueryBackendAdmissibleMutation, ForgeQueryWriteCommand, SignalInvalidationRoutingReceipt,
    SubscriptionActivationReceipt,
};
use crate::subscription::SubscriptionActivationInput;

pub(super) fn live_view_subject_identity(
    view_name: &str,
    request: &DeclarativeLiveQueryRequest,
) -> ForgeQueryLowerRuntimeSubjectIdentity {
    ForgeQueryLowerRuntimeSubjectIdentity::compose("live-view-route-subject")
        .field_value(ForgeQueryEvidenceTag::new("view"), view_name)
        .field_value(ForgeQueryEvidenceTag::new("target"), request.target())
        .field_shape(
            ForgeQueryEvidenceTag::new("shape"),
            request.view_shape().as_str(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("projection_count"),
            request.query_projection().len(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("result_count"),
            request.result_fields().len(),
        )
        .seal()
}

pub(super) fn write_command_subject_identity(
    command: &ForgeQueryWriteCommand,
) -> ForgeQueryLowerRuntimeSubjectIdentity {
    let mut encoder = ForgeQueryLowerRuntimeSubjectIdentity::compose("write-command-route-subject")
        .field_shape(
            ForgeQueryEvidenceTag::new("family"),
            command.mutation_family().as_str(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("aspect_operations"),
            command.declared_aspect_operations().len(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("touched_aspects"),
            command.declared_aspect_touches().len(),
        );
    if let Some(collection) = command.declared_collection_identity() {
        encoder = encoder.field_evidence_identity(
            ForgeQueryEvidenceTag::new("collection"),
            collection.evidence_identity(),
        );
    }
    if let Some(identity) = command.declared_entity_identity_ref() {
        let entity_identity = identity.evidence_identity();
        encoder =
            encoder.field_evidence_identity(ForgeQueryEvidenceTag::new("entity"), &entity_identity);
    }
    encoder.seal()
}

pub(super) fn backend_admissible_mutation_subject_identity(
    mutation: &ForgeQueryBackendAdmissibleMutation,
) -> ForgeQueryLowerRuntimeSubjectIdentity {
    let mut encoder = ForgeQueryLowerRuntimeSubjectIdentity::compose("write-command-route-subject")
        .field_shape(
            ForgeQueryEvidenceTag::new("family"),
            mutation.mutation_family().as_str(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("aspect_operations"),
            mutation.declared_aspect_operations().len(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("touched_aspects"),
            mutation.declared_aspect_touches().len(),
        );
    if let Some(collection) = mutation.declared_collection_identity() {
        encoder = encoder.field_evidence_identity(
            ForgeQueryEvidenceTag::new("collection"),
            collection.evidence_identity(),
        );
    }
    if let Some(identity) = mutation.declared_entity_identity_ref() {
        let entity_identity = identity.evidence_identity();
        encoder =
            encoder.field_evidence_identity(ForgeQueryEvidenceTag::new("entity"), &entity_identity);
    }
    encoder.seal()
}

pub(super) fn activation_subject_identity(
    view_name: &str,
    activation: &SubscriptionActivationInput,
    activation_receipt: &SubscriptionActivationReceipt,
) -> ForgeQueryLowerRuntimeSubjectIdentity {
    ForgeQueryLowerRuntimeSubjectIdentity::compose("subscription-activation-route-subject")
        .field_value(ForgeQueryEvidenceTag::new("view"), view_name)
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("activation_receipt"),
            activation_receipt.receipt_identity(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("async_request_identity_width"),
            activation.future_selection().async_request_identity().len(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("future_selection_class"),
            activation.future_selection().class().as_str(),
        )
        .seal()
}

pub(super) fn signal_invalidation_subject_identity(
    receipt: &SignalInvalidationRoutingReceipt,
) -> ForgeQueryLowerRuntimeSubjectIdentity {
    let commit_identity = receipt.commit_identity().evidence_identity();
    let snapshot_identity = receipt.snapshot_identity().evidence_identity();
    ForgeQueryLowerRuntimeSubjectIdentity::compose("signal-invalidation-route-subject")
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("routing_receipt"),
            receipt.receipt_identity(),
        )
        .field_evidence_identity(ForgeQueryEvidenceTag::new("commit"), &commit_identity)
        .field_evidence_identity(ForgeQueryEvidenceTag::new("snapshot"), &snapshot_identity)
        .field_usize(
            ForgeQueryEvidenceTag::new("delta_count"),
            receipt.delta_count(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("routed_collection_count"),
            receipt.routed_collection_count(),
        )
        .seal()
}
