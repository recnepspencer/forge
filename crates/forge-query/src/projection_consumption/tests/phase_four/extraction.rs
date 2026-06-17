use forge_foundational::facade::{AspectKey, AspectValue, ScalarAspectType};
use forge_relational::facade::grouped_truth::{
    encode_snapshot_aspect_read_value, materialize_relational_authoritative_row_set,
};
use forge_runtime_bridge::facade::{
    RelationalBridgeRecordIdentityParts, SnapshotReadContract, SnapshotReadPacket,
    SnapshotReadPacketResult, SnapshotReadRecord, SnapshotReadRequest,
};

use super::super::super::{
    ConsumedProjectionFactSet, MaterializedProjectionContract, ProjectMaterializedFacts,
    ProjectionConsumptionSource, ProjectionFactExtractionError, ProjectionFactKind,
    ProjectionSourceFamily,
};
use super::support::{
    admitted, binding, phase_four_commit_identity, phase_four_snapshot_identity,
    phase_four_truth_snapshot_identity, relational_row_set, test_entity_identity, write_receipt,
};
use crate::runtime::{ForgeQueryMutationTargetClass, ForgeQueryWriteReceipt};

#[test]
fn relational_row_set_extracts_identity_and_field_facts() {
    let row_set = relational_row_set();
    let contract = admitted(
        ProjectionConsumptionSource::from_relational_row_set(&row_set),
        binding(&["identity.id", "profile.display_name"]),
        ProjectMaterializedFacts::declare()
            .entity_identities()
            .view_local_identities()
            .display_field("profile.display_name")
            .derived_scalar_field("profile.display_name"),
    )
    .bind_contract();

    let consumed = contract.extract_from_relational_row_set(&row_set).unwrap();

    assert_eq!(consumed.entity_identities().len(), 2);
    assert_eq!(
        consumed.entity_identities()[0].source_row_identity(),
        "entity:1:1:1"
    );
    assert_eq!(
        consumed.entity_identities()[0].entity_identity(),
        &test_entity_identity("task-1")
    );
    assert_eq!(consumed.view_local_identities().len(), 2);
    assert_eq!(
        consumed.view_local_identities()[0].view_local_identity(),
        "entity:1:1:1"
    );
    assert_eq!(consumed.display_fields().len(), 2);
    assert_eq!(consumed.derived_scalar_fields().len(), 2);
    assert_eq!(consumed.counters().declared_fact_family_count(), 4);
    assert_eq!(consumed.counters().extracted_fact_count(), 8);
    assert_eq!(consumed.counters().source_row_width_consumed(), 6);
    assert_eq!(
        consumed.display_fields()[0].field_key(),
        "profile.display_name"
    );
}

#[test]
fn write_receipt_extracts_aftermath_and_source_reference_facts() {
    let receipt = write_receipt();
    let contract = admitted(
        ProjectionConsumptionSource::from_write_receipt(&receipt),
        binding(&[]),
        ProjectMaterializedFacts::declare()
            .target_identity()
            .source_references()
            .effect_continuity_facts()
            .relation_endpoints(),
    )
    .bind_contract();

    let consumed = contract.extract_from_write_receipt(&receipt).unwrap();

    assert_eq!(
        consumed.target_identities()[0].target_identity(),
        &test_entity_identity("task-1")
    );
    assert_eq!(consumed.source_references().len(), 2);
    assert_eq!(
        consumed.effect_continuity_facts()[0]
            .prior_authoritative_identity()
            .label(),
        "task-0"
    );
    assert_eq!(
        consumed.relation_endpoints()[0].target_class(),
        Some(ForgeQueryMutationTargetClass::Entity)
    );
    assert_eq!(consumed.relation_endpoints()[0].collection(), Some("tasks"));
    assert_eq!(consumed.counters().source_evidence_lookup_width(), 4);
    assert_eq!(consumed.counters().extracted_fact_count(), 5);
}

#[test]
fn effect_continuity_fact_digest_changes_with_typed_authority_identity() {
    let left = write_receipt();
    let right = ForgeQueryWriteReceipt::test_only(
        phase_four_commit_identity("commit:test"),
        phase_four_snapshot_identity("snapshot:test"),
        ForgeQueryMutationTargetClass::Entity,
        Some("tasks"),
        Some(test_entity_identity("task-1")),
        Some("bridge-record:test"),
        Some("$same_batch_target"),
        Some(
            crate::runtime::ForgeQueryContinuityMutationEvidence::test_only(
                crate::runtime::ForgeQueryContinuityMutationFamily::RebindExistingTarget,
                crate::runtime::ForgeQueryContinuityOutcomeClass::ContinuesAsSingleSuccessor,
                "task-0",
                vec!["task-2".to_string()],
                Some(test_entity_identity("task-1")),
                Some("tasks"),
            ),
        ),
    );
    let contract = admitted(
        ProjectionConsumptionSource::from_write_receipt(&left),
        binding(&[]),
        ProjectMaterializedFacts::declare()
            .effect_continuity_facts()
            .source_references(),
    )
    .bind_contract();

    let left_consumed = contract.extract_from_write_receipt(&left).unwrap();
    let right_consumed = contract.extract_from_write_receipt(&right).unwrap();

    assert_ne!(
        left_consumed.fact_set_digest(),
        right_consumed.fact_set_digest()
    );
    assert_ne!(
        left_consumed.effect_continuity_facts()[0].successor_authoritative_identities()[0]
            .evidence_identity(),
        right_consumed.effect_continuity_facts()[0].successor_authoritative_identities()[0]
            .evidence_identity()
    );
}

#[test]
fn extraction_rejects_missing_field_evidence_and_family_mismatch() {
    let row_set = relational_row_set();
    let display_contract = admitted(
        ProjectionConsumptionSource::from_relational_row_set(&row_set),
        binding(&["profile.missing"]),
        ProjectMaterializedFacts::declare().display_field("profile.missing"),
    )
    .bind_contract();
    let field_error = display_contract
        .extract_from_relational_row_set(&row_set)
        .unwrap_err();
    assert!(matches!(
        field_error,
        ProjectionFactExtractionError::MissingDeclaredFieldEvidence { .. }
    ));

    let entity_one_display_read = relational_snapshot_read(
        RelationalBridgeRecordIdentityParts::entity(1, 1, 1),
        "profile.display_name",
    );
    let entity_two_display_read = relational_snapshot_read(
        RelationalBridgeRecordIdentityParts::entity(1, 2, 1),
        "profile.display_name",
    );
    let missing_identity_packet = SnapshotReadPacket::new(vec![
        entity_one_display_read.clone(),
        entity_two_display_read.clone(),
    ]);
    let missing_identity_row_set = materialize_relational_authoritative_row_set(
        &missing_identity_packet,
        &SnapshotReadPacketResult::new(
            phase_four_truth_snapshot_identity("snapshot-a"),
            vec![
                SnapshotReadRecord::for_request(
                    &entity_one_display_read,
                    aspect_value(AspectValue::String("Task One".into())),
                ),
                SnapshotReadRecord::for_request(
                    &entity_two_display_read,
                    aspect_value(AspectValue::String("Task Two".into())),
                ),
            ],
        ),
    )
    .unwrap();
    let identity_contract = admitted(
        ProjectionConsumptionSource::from_relational_row_set(&missing_identity_row_set),
        binding(&["identity.id"]),
        ProjectMaterializedFacts::declare().entity_identities(),
    )
    .bind_contract();
    let identity_error = identity_contract
        .extract_from_relational_row_set(&missing_identity_row_set)
        .unwrap_err();
    assert!(matches!(
        identity_error,
        ProjectionFactExtractionError::MissingDeclaredFieldEvidence {
            fact_kind: ProjectionFactKind::EntityIdentity,
            ..
        }
    ));

    let _ = ProjectionSourceFamily::BridgeTruthViewRowSet;
}

fn aspect_value(value: AspectValue) -> AspectValue {
    encode_snapshot_aspect_read_value(&value)
}

fn relational_snapshot_read(
    entity: RelationalBridgeRecordIdentityParts,
    aspect: &str,
) -> SnapshotReadRequest {
    SnapshotReadRequest::for_relational_record(
        entity,
        SnapshotReadContract::scalar(
            AspectKey::new(aspect).expect("valid snapshot aspect key"),
            ScalarAspectType::String,
        ),
    )
}

#[test]
fn write_receipt_extraction_rejects_source_identity_mismatch() {
    let contract = admitted(
        ProjectionConsumptionSource::from_write_receipt(&write_receipt()),
        binding(&[]),
        ProjectMaterializedFacts::declare().target_identity(),
    )
    .bind_contract();
    let mismatched_receipt = ForgeQueryWriteReceipt::test_only(
        phase_four_commit_identity("commit:other"),
        phase_four_snapshot_identity("snapshot:test"),
        ForgeQueryMutationTargetClass::Entity,
        Some("tasks"),
        Some(test_entity_identity("task-1")),
        None,
        None,
        None,
    );

    let error = contract
        .extract_from_write_receipt(&mismatched_receipt)
        .unwrap_err();
    assert!(matches!(
        error,
        ProjectionFactExtractionError::SourceIdentityMismatch { .. }
    ));
}

#[test]
fn write_receipt_extraction_rejects_missing_admitted_evidence() {
    let contract = admitted(
        ProjectionConsumptionSource::from_write_receipt(&write_receipt()),
        binding(&[]),
        ProjectMaterializedFacts::declare()
            .target_identity()
            .effect_continuity_facts()
            .relation_endpoints(),
    )
    .bind_contract();
    let stripped_receipt = ForgeQueryWriteReceipt::test_only(
        phase_four_commit_identity("commit:test"),
        phase_four_snapshot_identity("snapshot:test"),
        ForgeQueryMutationTargetClass::Entity,
        None,
        None,
        None,
        None,
        None,
    );

    let error = contract
        .extract_from_write_receipt(&stripped_receipt)
        .unwrap_err();
    assert!(matches!(
        error,
        ProjectionFactExtractionError::MissingWriteReceiptEvidence { .. }
    ));
}

#[test]
fn bridge_row_set_extraction_surface_is_publicly_reachable() {
    let _extract: fn(
        &MaterializedProjectionContract,
        &forge_runtime_bridge::facade::BridgeMaterializedRowSetArtifact,
    ) -> Result<ConsumedProjectionFactSet, ProjectionFactExtractionError> =
        MaterializedProjectionContract::extract_from_bridge_truth_view_row_set;
    let _ = ProjectionSourceFamily::BridgeTruthViewRowSet;
}
