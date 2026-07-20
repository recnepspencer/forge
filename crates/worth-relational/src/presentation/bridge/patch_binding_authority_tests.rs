use worth_foundational::facade::{
    AspectBinding, AspectContractRevision, AspectIdentity, AspectKey, AspectValue,
    AuthoritativeAspectChangeKind, FieldKey,
};
use worth_proof::TransitionOutcome;
use worth_runtime_bridge::facade::{BridgeRouteErrorKind, TruthSnapshotIdentity};

use crate::history::data::{BranchId, CommitId};
use crate::identity::data::{EntityId, PartitionId};
use crate::publication::patch::data::{
    PatchDetail, PatchOrdering, PatchPublicationMode, PatchStreamPosition,
    PublishedAuthoritativeAspectChange, PublishedAuthoritativePatch,
    PublishedAuthoritativePatchEnvelope, PublishedAuthoritativePatchOperation,
    PublishedAuthoritativePatchValue, PublishedAuthoritativeRecordPatch, RecordStructuralChange,
};
use crate::transactions::data::RecordRef;

use super::patch_envelopes::publication_patch_to_bridge_envelope;

#[test]
fn derived_endpoint_label_cannot_reclassify_an_authoritative_entity_field_operation() {
    let key = AspectKey::new("profile").unwrap();
    let authoritative_binding = AspectBinding::EntityField {
        field: FieldKey::new("profile").unwrap(),
    };
    let patch = PublishedAuthoritativePatchEnvelope {
        ordering: PatchOrdering::CanonicalCommitOrder,
        publication_mode: PatchPublicationMode::CommitNative,
        position: PatchStreamPosition(12),
        authoritative_record_patches: vec![PublishedAuthoritativeRecordPatch {
            target: RecordRef::Entity(EntityId::new(PartitionId::main(), 13, 1)),
            structural_change: RecordStructuralChange::Updated,
            authoritative_patch: PublishedAuthoritativePatch::new(vec![
                PublishedAuthoritativePatchOperation::WholeAspectSet {
                    aspect_key: key.clone(),
                    aspect_identity: AspectIdentity(61),
                    contract_revision: AspectContractRevision(2),
                    binding: authoritative_binding,
                    value: PublishedAuthoritativePatchValue::Scalar(AspectValue::String(
                        "after".into(),
                    )),
                },
            ]),
            semantic_changes: vec![PublishedAuthoritativeAspectChange::exact(
                key,
                AspectIdentity(61),
                AspectContractRevision(2),
                AspectBinding::RelationSourceEndpoint,
                AuthoritativeAspectChangeKind::RelationSourceEndpoint,
                None,
            )],
            contains_opaque_aspect: false,
            detail: PatchDetail::DenseBitset(Vec::new()),
        }],
    };

    let outcome = publication_patch_to_bridge_envelope(
        CommitId(12),
        &BranchId("main".to_string()),
        TruthSnapshotIdentity::from_relational_snapshot(
            worth_runtime_bridge::facade::RelationalBridgeSnapshotIdentityParts::new(12, 1),
        ),
        &patch,
    );
    assert!(matches!(
        outcome,
        TransitionOutcome::Denied(denial)
            if denial.kind() == BridgeRouteErrorKind::InvalidAuthoritativePatchSemantics
    ));
}
