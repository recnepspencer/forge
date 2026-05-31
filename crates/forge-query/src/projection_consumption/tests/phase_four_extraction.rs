use forge_foundational::facade::AspectValue;
use forge_relational::facade::grouped_truth::{
    encode_snapshot_aspect_read_value, materialize_relational_authoritative_row_set,
};
use forge_runtime_bridge::facade::{
    SnapshotReadPacket, SnapshotReadPacketResult, SnapshotReadRecord, SnapshotReadRequest,
    TruthSnapshotIdentity,
};

use super::super::{
    ConsumedProjectionFactSet, MaterializedProjectionContract, ProjectMaterializedFacts,
    ProjectionConsumptionSource, ProjectionFactExtractionError, ProjectionFactKind,
    ProjectionSourceFamily,
};
use super::phase_four_support::{admitted, binding, relational_row_set, write_receipt};
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
        "entity-1"
    );
    assert_eq!(consumed.entity_identities()[0].entity_identity(), "task-1");
    assert_eq!(consumed.view_local_identities().len(), 2);
    assert_eq!(
        consumed.view_local_identities()[0].view_local_identity(),
        "entity-1"
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

    assert_eq!(consumed.target_identities()[0].target_identity(), "task-1");
    assert_eq!(consumed.source_references().len(), 2);
    assert_eq!(
        consumed.effect_continuity_facts()[0].prior_authoritative_identity(),
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

    let missing_identity_row_set = materialize_relational_authoritative_row_set(
        &SnapshotReadPacket::new(vec![
            SnapshotReadRequest::for_coarse(
                "entity-1",
                forge_foundational::facade::AspectKey::new("profile.display_name")
                    .expect("valid snapshot aspect key"),
            ),
            SnapshotReadRequest::for_coarse(
                "entity-2",
                forge_foundational::facade::AspectKey::new("profile.display_name")
                    .expect("valid snapshot aspect key"),
            ),
        ]),
        &SnapshotReadPacketResult::new(
            TruthSnapshotIdentity::new("snapshot-a"),
            vec![
                SnapshotReadRecord::new(
                    "entity-1:profile.display_name",
                    aspect_bytes(AspectValue::String("Task One".into())),
                ),
                SnapshotReadRecord::new(
                    "entity-2:profile.display_name",
                    aspect_bytes(AspectValue::String("Task Two".into())),
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

fn aspect_bytes(value: AspectValue) -> Vec<u8> {
    encode_snapshot_aspect_read_value(&value)
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
        "commit:other",
        "snapshot:test",
        ForgeQueryMutationTargetClass::Entity,
        Some("tasks"),
        Some("task-1"),
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
        "commit:test",
        "snapshot:test",
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
