use super::*;
use crate::declarative_live::{DeclarativeLiveQueryRequest, DeclarativeLiveViewShape};
use crate::memory_workspace::{
    ForgeQueryMutationDelta, ForgeQueryMutationKind, ForgeQueryMutationReceipt,
};
use crate::runtime::tests::support::test_bridge_with_writeback_authority;
use crate::runtime::{
    build_bridge_authority_bundle, ForgeQueryAdmittedAspectValue, ForgeQueryAspectTouch,
    ForgeQueryBackendAdmissibleMutation, ForgeQueryWriteCommand,
};
use crate::ForgeQueryEvidenceScope;
use forge_foundational::facade::{AspectKey, CanonicalFieldPath, FieldKey};
use forge_runtime_bridge::facade::RelationalBridgeSnapshotIdentityParts;

#[test]
fn live_view_declaration_receipt_captures_request_shape() {
    let request = DeclarativeLiveQueryRequest::new("Task", DeclarativeLiveViewShape::table());
    let receipt = LiveViewDeclarationAdmissionReceipt::from_request("tasks.table", &request);

    assert_eq!(receipt.view_name(), "tasks.table");
    assert_eq!(receipt.target_collection_for_reporting(), "Task");
    assert_eq!(receipt.view_shape(), &DeclarativeLiveViewShape::table());
    assert_eq!(receipt.view_shape_for_reporting(), "table");
    assert!(!receipt.receipt_for_reporting().is_empty());
}

#[test]
fn signal_invalidation_routing_receipt_rejects_authority_less_receipt() {
    let receipt = ForgeQueryMutationReceipt::from_authoritative_parts(
        crate::memory_workspace::admit_external_commit_label("commit-1"),
        crate::memory_workspace::admit_external_snapshot_label("snapshot-1"),
        Vec::new(),
    );

    let error = SignalInvalidationRoutingReceipt::from_mutation_receipt(&receipt)
        .expect_err("authority-less receipt must not route signal invalidation");

    assert!(error
        .to_string()
        .contains("requires bridge-authored mutation authority"));
}

#[test]
fn signal_invalidation_routing_receipt_summarizes_delta_width() {
    let command_entity_identity = crate::memory_workspace::admit_authored_entity_label("task-1");
    let command = ForgeQueryWriteCommand::UpdateAspects {
        entity_identity: command_entity_identity.clone(),
        aspects: vec![
            ForgeQueryAdmittedAspectValue::new_set(
                title_value_touch(),
                crate::runtime::ForgeQueryAdmittedAspectValue::native_string_value("Done"),
            )
            .expect("title aspect should build"),
            ForgeQueryAdmittedAspectValue::new_set(
                status_value_touch(),
                crate::runtime::ForgeQueryAdmittedAspectValue::native_string_value("closed"),
            )
            .expect("status aspect should build"),
        ],
        metadata: Default::default(),
        naming_intent: None,
        continuity_intent: None,
    };
    let mutation = ForgeQueryBackendAdmissibleMutation::from_admitted_command(command);
    let bridge = test_bridge_with_writeback_authority();
    let snapshot_identity =
        crate::memory_workspace::ForgeQuerySnapshotIdentity::from_relational_snapshot(
            RelationalBridgeSnapshotIdentityParts::new(1, 1),
        );
    let bridge_authority = build_bridge_authority_bundle(
        &bridge,
        &snapshot_identity,
        &mutation,
        "Task",
        &command_entity_identity,
        ForgeQueryMutationKind::Updated,
    )
    .expect("test bridge authority should build");
    let receipt = ForgeQueryMutationReceipt::from_bridge_authoritative_parts(
        crate::memory_workspace::ForgeQueryCommitIdentity::from_relational_commit_id(1),
        snapshot_identity,
        vec![
            ForgeQueryMutationDelta::from_touched_aspects(
                "Task",
                crate::memory_workspace::admit_authored_entity_label("task-1"),
                ForgeQueryMutationKind::Created,
                vec![title_value_touch()],
            ),
            ForgeQueryMutationDelta::from_touched_aspects(
                "Task",
                crate::memory_workspace::admit_authored_entity_label("task-2"),
                ForgeQueryMutationKind::Updated,
                vec![status_value_touch()],
            ),
        ],
        bridge_authority,
    );

    let routed = SignalInvalidationRoutingReceipt::from_mutation_receipt(&receipt)
        .expect("bridge-authored receipt should route");

    assert!(!routed.causality_digest().is_empty());
    assert_eq!(routed.delta_count(), 2);
    assert_eq!(routed.routed_collection_count(), 1);
    assert_eq!(
        routed.receipt_identity().scope(),
        ForgeQueryEvidenceScope::SignalInvalidationRoutingReceipt
    );
    assert!(!routed.receipt_identity().as_str().is_empty());
}

#[test]
fn bridge_writeback_effect_intent_is_bound_to_admitted_aspect_patch() {
    let baseline = bridge_authority_for_title_value("Done");
    let changed = bridge_authority_for_title_value("Blocked");

    let baseline_basis = baseline.provenance().effect_intent_patch_canonical_basis();
    let changed_basis = changed.provenance().effect_intent_patch_canonical_basis();

    assert_ne!(baseline_basis, changed_basis);
    assert!(baseline_basis.contains("domain=authoritative-patch"));
    assert!(baseline_basis.contains("title.field.value.set"));
    assert!(
        baseline_basis.contains("exact-text:Done"),
        "{baseline_basis}"
    );
    assert!(!baseline_basis.contains("forge.query.writeback"));
}

#[test]
fn bridge_writeback_effect_intent_accepts_whole_entity_delete_empty_patch() {
    let entity_identity = crate::memory_workspace::admit_authored_entity_label("task-1");
    let command = ForgeQueryWriteCommand::Delete {
        entity_identity: entity_identity.clone(),
    };
    let mutation = ForgeQueryBackendAdmissibleMutation::from_admitted_command(command);
    let bridge = test_bridge_with_writeback_authority();
    let snapshot_identity =
        crate::memory_workspace::ForgeQuerySnapshotIdentity::from_relational_snapshot(
            RelationalBridgeSnapshotIdentityParts::new(1, 1),
        );

    let bridge_authority = build_bridge_authority_bundle(
        &bridge,
        &snapshot_identity,
        &mutation,
        "Task",
        &entity_identity,
        ForgeQueryMutationKind::Deleted,
    )
    .expect("whole-entity delete should lower through an empty native patch");
    let basis = bridge_authority
        .provenance()
        .effect_intent_patch_canonical_basis();

    assert!(basis.contains("domain=authoritative-patch"));
    assert!(!basis.contains(".set"));
    assert!(!basis.contains(".clear"));
}

fn bridge_authority_for_title_value(
    value: &str,
) -> forge_runtime_bridge::facade::BridgeMutationAuthorityBundle {
    let entity_identity = crate::memory_workspace::admit_authored_entity_label("task-1");
    let command = ForgeQueryWriteCommand::UpdateAspects {
        entity_identity: entity_identity.clone(),
        aspects: vec![ForgeQueryAdmittedAspectValue::new_set(
            title_value_touch(),
            crate::runtime::ForgeQueryAdmittedAspectValue::native_string_value(value),
        )
        .expect("title aspect should build")],
        metadata: Default::default(),
        naming_intent: None,
        continuity_intent: None,
    };
    let mutation = ForgeQueryBackendAdmissibleMutation::from_admitted_command(command);
    let bridge = test_bridge_with_writeback_authority();
    let snapshot_identity =
        crate::memory_workspace::ForgeQuerySnapshotIdentity::from_relational_snapshot(
            RelationalBridgeSnapshotIdentityParts::new(1, 1),
        );
    build_bridge_authority_bundle(
        &bridge,
        &snapshot_identity,
        &mutation,
        "Task",
        &entity_identity,
        ForgeQueryMutationKind::Updated,
    )
    .expect("test bridge authority should build")
}

fn title_value_touch() -> ForgeQueryAspectTouch {
    aspect_field_touch("title", "value")
}

fn status_value_touch() -> ForgeQueryAspectTouch {
    aspect_field_touch("status", "value")
}

fn aspect_field_touch(
    aspect_label: &'static str,
    field_label: &'static str,
) -> ForgeQueryAspectTouch {
    let aspect_key =
        AspectKey::new(aspect_label).expect("receipt test static aspect key should admit");
    let field_key = FieldKey::new(field_label).expect("receipt test static field key should admit");
    let field_path =
        CanonicalFieldPath::new([field_key]).expect("receipt test static field path should admit");
    ForgeQueryAspectTouch::aspect_field_path(aspect_key, field_path)
}
