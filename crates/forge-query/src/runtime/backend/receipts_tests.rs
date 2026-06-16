use super::*;
use crate::declarative_live::{DeclarativeLiveQueryRequest, DeclarativeLiveViewShape};
use crate::memory_workspace::{
    ForgeQueryMutationDelta, ForgeQueryMutationKind, ForgeQueryMutationReceipt,
};
use crate::runtime::tests::support::test_bridge_with_writeback_authority;
use crate::runtime::{
    build_bridge_authority_bundle, ForgeQueryAspectValue, ForgeQueryWriteCommand,
};
use crate::ForgeQueryEvidenceScope;
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
    let receipt = ForgeQueryMutationReceipt {
        commit_identity:
            crate::memory_workspace::admit_external_commit_label(
                "commit-1",
            ),
        snapshot_identity:
            crate::memory_workspace::admit_external_snapshot_label(
                "snapshot-1",
            ),
        deltas: Vec::new(),
        bridge_authority: None,
    };

    let error = SignalInvalidationRoutingReceipt::from_mutation_receipt(&receipt)
        .expect_err("authority-less receipt must not route signal invalidation");

    assert!(error
        .to_string()
        .contains("requires bridge-authored mutation authority"));
}

#[test]
fn signal_invalidation_routing_receipt_summarizes_delta_width() {
    let command_entity_identity =
        crate::memory_workspace::admit_authored_entity_label("task-1");
    let command = ForgeQueryWriteCommand::UpdateAspects {
        entity_identity: command_entity_identity.clone(),
        aspects: vec![
            ForgeQueryAspectValue::new_set("title.value", "Done")
                .expect("title aspect should build"),
            ForgeQueryAspectValue::new_set("status.value", "closed")
                .expect("status aspect should build"),
        ],
        metadata: Default::default(),
        naming_intent: None,
        continuity_intent: None,
    };
    let bridge = test_bridge_with_writeback_authority();
    let snapshot_identity =
        crate::memory_workspace::ForgeQuerySnapshotIdentity::from_relational_snapshot(
            RelationalBridgeSnapshotIdentityParts::new(1, 1),
        );
    let bridge_authority = build_bridge_authority_bundle(
        &bridge,
        &snapshot_identity,
        &command,
        "Task",
        &command_entity_identity,
        ForgeQueryMutationKind::Updated,
    )
    .expect("test bridge authority should build");
    let receipt = ForgeQueryMutationReceipt {
        commit_identity:
            crate::memory_workspace::ForgeQueryCommitIdentity::from_relational_commit_id(1),
        snapshot_identity,
        deltas: vec![
            ForgeQueryMutationDelta {
                collection: "Task".to_string(),
                entity_identity:
                    crate::memory_workspace::admit_authored_entity_label("task-1"),
                kind: ForgeQueryMutationKind::Created,
                aspect_paths: vec!["title.value".to_string()],
            },
            ForgeQueryMutationDelta {
                collection: "Task".to_string(),
                entity_identity:
                    crate::memory_workspace::admit_authored_entity_label("task-2"),
                kind: ForgeQueryMutationKind::Updated,
                aspect_paths: vec!["status.value".to_string()],
            },
        ],
        bridge_authority: Some(bridge_authority),
    };

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
