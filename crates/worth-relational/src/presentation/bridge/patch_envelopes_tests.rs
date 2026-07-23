use worth_foundational::facade::{
    AspectBinding, AspectContractRevision, AspectIdentity, AspectKey,
    AuthoritativeAspectChangeKind, CanonicalFieldPath, FieldKey,
};
use worth_proof::TransitionOutcome;
use worth_runtime_bridge::facade::{
    BridgeAspectChangePrecision, BridgeAspectChangeWideningCause, BridgeRouteErrorKind,
    TruthDeltaSurfaceKind, TruthSnapshotIdentity,
};

use crate::history::data::{BranchId, CommitId};
use crate::identity::data::{EntityId, PartitionId};
use crate::publication::patch::data::{
    PatchDetail, PatchOrdering, PatchPublicationMode, PatchStreamPosition,
    PublishedAuthoritativeAspectChange, PublishedAuthoritativeFieldSet,
    PublishedAuthoritativePatch, PublishedAuthoritativePatchEnvelope,
    PublishedAuthoritativePatchOperation, PublishedAuthoritativeRecordPatch,
    RecordStructuralChange,
};
use crate::transactions::data::RecordRef;

use super::patch_envelopes::{
    publication_patch_to_bridge_envelope, publication_patch_to_bridge_envelope_with_widening,
    RelationalBridgePatchPublicationRequest,
};

#[test]
fn field_precision_and_structural_change_survive_one_bridge_envelope() {
    let key = AspectKey::new("profile").unwrap();
    let path = CanonicalFieldPath::single(FieldKey::new("name".to_string()).unwrap());
    let change = PublishedAuthoritativeAspectChange::exact(
        key.clone(),
        AspectIdentity(41),
        AspectContractRevision(7),
        AspectBinding::EntityField {
            field: FieldKey::new("profile".to_string()).unwrap(),
        },
        AuthoritativeAspectChangeKind::FieldSet,
        Some(path.clone()),
    );
    let patch = PublishedAuthoritativePatchEnvelope {
        ordering: PatchOrdering::CanonicalCommitOrder,
        publication_mode: PatchPublicationMode::CommitNative,
        position: PatchStreamPosition(3),
        authoritative_record_patches: vec![PublishedAuthoritativeRecordPatch {
            target: RecordRef::Entity(EntityId::new(PartitionId::main(), 9, 2)),
            structural_change: RecordStructuralChange::Updated,
            authoritative_patch: PublishedAuthoritativePatch::new(vec![
                PublishedAuthoritativePatchOperation::FieldLevelPatch {
                    aspect_key: key.clone(),
                    aspect_identity: AspectIdentity(41),
                    contract_revision: AspectContractRevision(7),
                    binding: AspectBinding::EntityField {
                        field: FieldKey::new("profile").unwrap(),
                    },
                    field_sets: vec![PublishedAuthoritativeFieldSet {
                        field: FieldKey::new("name".to_string()).unwrap(),
                        value: worth_foundational::facade::AspectValue::String("after".into()),
                    }],
                    field_clears: Vec::new(),
                },
            ]),
            semantic_changes: vec![change],
            contains_opaque_aspect: false,
            detail: PatchDetail::DenseBitset(Vec::new()),
        }],
    };

    let TransitionOutcome::Success(envelope) = publication_patch_to_bridge_envelope(
        CommitId(5),
        &BranchId("main".to_string()),
        TruthSnapshotIdentity::from_relational_snapshot(
            worth_runtime_bridge::facade::RelationalBridgeSnapshotIdentityParts::new(5, 1),
        ),
        &patch,
    ) else {
        panic!("exact field publication should be admitted");
    };
    assert_eq!(envelope.patch_body().canonical_items().len(), 1);
    assert_eq!(envelope.patch_body().canonical_record_changes().len(), 1);
    let item = &envelope.patch_body().canonical_items()[0];
    assert_eq!(item.surface_kind(), TruthDeltaSurfaceKind::EntityField);
    assert_eq!(item.field_locator().unwrap().field_path(), &path);
    let semantic = item.semantic_change().unwrap();
    assert_eq!(semantic.aspect_identity(), AspectIdentity(41));
    assert_eq!(semantic.contract_revision(), AspectContractRevision(7));
    assert_eq!(semantic.precision(), BridgeAspectChangePrecision::Exact);
    assert_eq!(semantic.kind(), AuthoritativeAspectChangeKind::FieldSet);
    assert_eq!(
        *envelope.patch_summary().authoritative_lowering(),
        worth_runtime_bridge::facade::BridgeAuthoritativePatchLoweringCounters {
            source_record_patches_examined: 1,
            source_record_patches_filtered_out: 0,
            record_patches_inspected: 1,
            authoritative_operations_inspected: 1,
            expected_operations_materialized: 1,
            semantic_changes_inspected: 1,
            semantic_changes_matched: 1,
            semantic_changes_emission_classified: 1,
            field_targets_emitted: 1,
            whole_aspect_targets_emitted: 0,
            endpoint_targets_emitted: 0,
            lifecycle_targets_emitted: 0,
            opaque_changes_emitted: 0,
            declared_widenings: 0,
        }
    );
}

#[test]
fn copied_semantic_metadata_cannot_override_the_canonical_patch() {
    let cases = [
        semantic_case(
            AspectIdentity(99),
            AspectContractRevision(7),
            "name",
            AuthoritativeAspectChangeKind::FieldSet,
            AspectBinding::EntityField {
                field: FieldKey::new("profile".to_string()).unwrap(),
            },
        ),
        semantic_case(
            AspectIdentity(41),
            AspectContractRevision(99),
            "name",
            AuthoritativeAspectChangeKind::FieldSet,
            AspectBinding::EntityField {
                field: FieldKey::new("profile".to_string()).unwrap(),
            },
        ),
        semantic_case(
            AspectIdentity(41),
            AspectContractRevision(7),
            "other",
            AuthoritativeAspectChangeKind::FieldSet,
            AspectBinding::EntityField {
                field: FieldKey::new("profile".to_string()).unwrap(),
            },
        ),
        semantic_case(
            AspectIdentity(41),
            AspectContractRevision(7),
            "name",
            AuthoritativeAspectChangeKind::FieldClear,
            AspectBinding::EntityField {
                field: FieldKey::new("profile".to_string()).unwrap(),
            },
        ),
        semantic_case(
            AspectIdentity(41),
            AspectContractRevision(7),
            "name",
            AuthoritativeAspectChangeKind::FieldSet,
            AspectBinding::RelationSourceEndpoint,
        ),
    ];

    for semantic_change in cases {
        let outcome = publication_patch_to_bridge_envelope(
            CommitId(8),
            &BranchId("main".to_string()),
            snapshot(8),
            &field_patch_with(semantic_change),
        );
        let TransitionOutcome::Denied(denial) = outcome else {
            panic!("drifted semantic metadata must deny before Bridge construction");
        };
        assert_eq!(
            denial.kind(),
            BridgeRouteErrorKind::InvalidAuthoritativePatchSemantics
        );
    }
}

fn semantic_case(
    identity: AspectIdentity,
    revision: AspectContractRevision,
    path: &str,
    kind: AuthoritativeAspectChangeKind,
    binding: AspectBinding,
) -> PublishedAuthoritativeAspectChange {
    PublishedAuthoritativeAspectChange::exact(
        AspectKey::new("profile").unwrap(),
        identity,
        revision,
        binding,
        kind,
        Some(CanonicalFieldPath::single(
            FieldKey::new(path.to_string()).unwrap(),
        )),
    )
}

fn field_patch_with(
    semantic_change: PublishedAuthoritativeAspectChange,
) -> PublishedAuthoritativePatchEnvelope {
    PublishedAuthoritativePatchEnvelope {
        ordering: PatchOrdering::CanonicalCommitOrder,
        publication_mode: PatchPublicationMode::CommitNative,
        position: PatchStreamPosition(5),
        authoritative_record_patches: vec![PublishedAuthoritativeRecordPatch {
            target: RecordRef::Entity(EntityId::new(PartitionId::main(), 11, 1)),
            structural_change: RecordStructuralChange::Updated,
            authoritative_patch: PublishedAuthoritativePatch::new(vec![
                PublishedAuthoritativePatchOperation::FieldLevelPatch {
                    aspect_key: AspectKey::new("profile").unwrap(),
                    aspect_identity: AspectIdentity(41),
                    contract_revision: AspectContractRevision(7),
                    binding: AspectBinding::EntityField {
                        field: FieldKey::new("profile").unwrap(),
                    },
                    field_sets: vec![PublishedAuthoritativeFieldSet {
                        field: FieldKey::new("name".to_string()).unwrap(),
                        value: worth_foundational::facade::AspectValue::String("after".into()),
                    }],
                    field_clears: Vec::new(),
                },
            ]),
            semantic_changes: vec![semantic_change],
            contains_opaque_aspect: false,
            detail: PatchDetail::DenseBitset(Vec::new()),
        }],
    }
}

fn snapshot(commit: u64) -> TruthSnapshotIdentity {
    TruthSnapshotIdentity::from_relational_snapshot(
        worth_runtime_bridge::facade::RelationalBridgeSnapshotIdentityParts::new(commit, 1),
    )
}

#[test]
fn structural_only_publication_does_not_fabricate_an_aspect() {
    let patch = PublishedAuthoritativePatchEnvelope {
        ordering: PatchOrdering::CanonicalCommitOrder,
        publication_mode: PatchPublicationMode::CommitNative,
        position: PatchStreamPosition(4),
        authoritative_record_patches: vec![PublishedAuthoritativeRecordPatch {
            target: RecordRef::Entity(EntityId::new(PartitionId::main(), 10, 1)),
            structural_change: RecordStructuralChange::Deleted,
            authoritative_patch: PublishedAuthoritativePatch::empty(),
            semantic_changes: Vec::new(),
            contains_opaque_aspect: false,
            detail: PatchDetail::DenseBitset(Vec::new()),
        }],
    };
    let TransitionOutcome::Success(envelope) = publication_patch_to_bridge_envelope(
        CommitId(6),
        &BranchId("main".to_string()),
        TruthSnapshotIdentity::from_relational_snapshot(
            worth_runtime_bridge::facade::RelationalBridgeSnapshotIdentityParts::new(6, 1),
        ),
        &patch,
    ) else {
        panic!("structural-only publication should be admitted");
    };
    assert!(envelope.patch_body().canonical_items().is_empty());
    assert_eq!(envelope.patch_body().canonical_record_changes().len(), 1);
}

#[test]
fn opaque_change_requires_explicit_precision_admission() {
    let key = AspectKey::new("opaque-payload").unwrap();
    let patch = PublishedAuthoritativePatchEnvelope {
        ordering: PatchOrdering::CanonicalCommitOrder,
        publication_mode: PatchPublicationMode::CommitNative,
        position: PatchStreamPosition(9),
        authoritative_record_patches: vec![PublishedAuthoritativeRecordPatch {
            target: RecordRef::Entity(EntityId::new(PartitionId::main(), 12, 1)),
            structural_change: RecordStructuralChange::Updated,
            authoritative_patch: PublishedAuthoritativePatch::new(vec![
                PublishedAuthoritativePatchOperation::WholeAspectSet {
                    aspect_key: key.clone(),
                    aspect_identity: AspectIdentity(51),
                    contract_revision: AspectContractRevision(4),
                    binding: AspectBinding::EntityField {
                        field: FieldKey::new("payload").unwrap(),
                    },
                    value:
                        crate::publication::patch::data::PublishedAuthoritativePatchValue::Scalar(
                            worth_foundational::facade::AspectValue::String("opaque".into()),
                        ),
                },
            ]),
            semantic_changes: vec![PublishedAuthoritativeAspectChange::exact(
                key,
                AspectIdentity(51),
                AspectContractRevision(4),
                AspectBinding::EntityField {
                    field: FieldKey::new("payload").unwrap(),
                },
                AuthoritativeAspectChangeKind::Opaque,
                None,
            )],
            contains_opaque_aspect: true,
            detail: PatchDetail::DenseBitset(Vec::new()),
        }],
    };

    let TransitionOutcome::Denied(denial) = publication_patch_to_bridge_envelope(
        CommitId(9),
        &BranchId("main".to_string()),
        snapshot(9),
        &patch,
    ) else {
        panic!("opaque payload must not silently become an exact whole-aspect target");
    };
    assert_eq!(
        denial.kind(),
        BridgeRouteErrorKind::UnsupportedAuthoritativePatchPrecision
    );
    assert_eq!(denial.counters().record_patches_inspected, 1);
    assert_eq!(denial.counters().authoritative_operations_inspected, 1);
    assert_eq!(denial.counters().semantic_changes_inspected, 1);
    assert_eq!(denial.counters().field_targets_emitted, 0);
    assert_eq!(denial.counters().whole_aspect_targets_emitted, 0);

    let branch = BranchId("main".to_string());
    let TransitionOutcome::Success(envelope) = publication_patch_to_bridge_envelope_with_widening(
        RelationalBridgePatchPublicationRequest {
            commit_id: CommitId(9),
            branch_id: &branch,
            snapshot_identity: snapshot(9),
            patch: &patch,
            admitted_widening: Some(BridgeAspectChangeWideningCause::OpaquePayloadToWholeAspect),
            producer_metadata:
                worth_runtime_bridge::facade::BridgeProducerMetadata::bridge_harness_fixture(),
            source_record_patches_examined: 1,
            source_record_patches_filtered_out: 0,
        },
    ) else {
        panic!("the exact owner-admitted opaque widening should publish");
    };
    let semantic = envelope.patch_body().canonical_items()[0]
        .semantic_change()
        .unwrap();
    assert_eq!(
        semantic.precision(),
        BridgeAspectChangePrecision::DeclaredWidening
    );
    assert_eq!(
        semantic.widening_cause(),
        Some(BridgeAspectChangeWideningCause::OpaquePayloadToWholeAspect)
    );
    assert_eq!(
        envelope
            .patch_summary()
            .authoritative_lowering()
            .declared_widenings,
        1
    );
}
