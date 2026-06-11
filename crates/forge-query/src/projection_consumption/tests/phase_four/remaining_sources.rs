use forge_foundational::facade::AspectKey;
use forge_relational::facade::grouped_truth::{
    project_relational_grouped_truth, GroupedProjectionContract,
};
use serde_json::json;

use super::super::super::{
    ConsumedProjectionFactSet, MaterializedProjectionContract, ProjectMaterializedFacts,
    ProjectionConsumptionSource, ProjectionFactExtractionError,
};
use super::support::{
    admitted, binding, binding_for_result_shape, read_result, read_result_shape,
    relational_grouped_projection, relational_row_set,
};
use crate::runtime::{ForgeQueryReadExecutionEngine, ForgeQueryReadReceipt, ForgeQueryReadResult};

#[test]
fn read_result_extracts_identity_and_payload_fields_without_reopening_authority() {
    let result = read_result();
    let result_shape = read_result_shape();
    let contract = admitted(
        ProjectionConsumptionSource::from_read_receipt(result.receipt(), &result_shape),
        binding_for_result_shape(
            result_shape.digest().as_str(),
            &["profile.display_name", "metrics.priority"],
        ),
        ProjectMaterializedFacts::declare()
            .entity_identities()
            .view_local_identities()
            .display_field("profile.display_name")
            .derived_scalar_field("metrics.priority"),
    )
    .bind_contract();

    let consumed = contract.extract_from_read_result(&result).unwrap();

    assert_eq!(consumed.entity_identities().len(), 2);
    assert_eq!(consumed.entity_identities()[0].entity_identity(), "task-1");
    assert_eq!(
        consumed.view_local_identities()[0].view_local_identity(),
        "task-1"
    );
    assert_eq!(
        consumed.display_fields()[0].field_key(),
        "profile.display_name"
    );
    assert_eq!(consumed.derived_scalar_fields()[1].value(), &json!(2));
    assert_eq!(consumed.counters().source_row_width_consumed(), 6);
    assert_eq!(consumed.counters().authority_reopen_count(), 0);
}

#[test]
fn read_result_extraction_rejects_query_basis_or_result_receipt_drift() {
    let result = read_result();
    let result_shape = read_result_shape();
    let contract = admitted(
        ProjectionConsumptionSource::from_read_receipt(result.receipt(), &result_shape),
        binding_for_result_shape(result_shape.digest().as_str(), &["profile.display_name"]),
        ProjectMaterializedFacts::declare().display_field("profile.display_name"),
    )
    .bind_contract();
    let mismatched_result = ForgeQueryReadResult::test_only(
        result.rows().to_vec(),
        ForgeQueryReadReceipt::test_only(
            "read-graph:test",
            "query:other",
            "basis:test",
            "result:test",
            ForgeQueryReadExecutionEngine::QueryRuntimeCurrent,
        ),
    );

    let error = contract
        .extract_from_read_result(&mismatched_result)
        .unwrap_err();
    assert!(matches!(
        error,
        ProjectionFactExtractionError::SourceArtifactMetadataMismatch {
            metadata_label: "query_digest",
            ..
        }
    ));
}

#[test]
fn grouped_sources_extract_memberships_and_grouped_relation_endpoints() {
    let relational_projection = relational_grouped_projection();
    let relational_contract = admitted(
        ProjectionConsumptionSource::from_relational_grouped_projection(&relational_projection),
        binding(&[]),
        ProjectMaterializedFacts::declare()
            .view_local_identities()
            .memberships()
            .relation_endpoints(),
    )
    .bind_contract();
    let relational_consumed = relational_contract
        .extract_from_relational_grouped_projection(&relational_projection)
        .unwrap();

    assert_eq!(relational_consumed.view_local_identities().len(), 2);
    assert_eq!(relational_consumed.memberships().len(), 2);
    assert_eq!(
        relational_consumed.memberships()[0].member_identity(),
        &json!("task-1")
    );
    assert_eq!(
        relational_consumed.relation_endpoints()[0].grouping_aspect(),
        Some("status")
    );
    assert_eq!(
        relational_consumed.memberships()[0].native_grouping_aspect_key(),
        &aspect_key("status")
    );
    assert_eq!(
        relational_consumed.relation_endpoints()[0].native_grouping_aspect_key(),
        Some(&aspect_key("status"))
    );
    assert_eq!(
        relational_consumed.counters().source_row_width_consumed(),
        6
    );
}

#[test]
fn grouped_relation_endpoint_width_counts_unique_source_surfaces() {
    let relational_projection = relational_grouped_projection();
    let contract = admitted(
        ProjectionConsumptionSource::from_relational_grouped_projection(&relational_projection),
        binding(&[]),
        ProjectMaterializedFacts::declare().relation_endpoints(),
    )
    .bind_contract();

    let consumed = contract
        .extract_from_relational_grouped_projection(&relational_projection)
        .unwrap();

    assert_eq!(consumed.relation_endpoints().len(), 2);
    assert_eq!(consumed.counters().source_row_width_consumed(), 6);
}

#[test]
fn grouped_extraction_rejects_source_identity_mismatch() {
    let grouped_projection = relational_grouped_projection();
    let contract = admitted(
        ProjectionConsumptionSource::from_relational_grouped_projection(&grouped_projection),
        binding(&[]),
        ProjectMaterializedFacts::declare().memberships(),
    )
    .bind_contract();
    let mismatched = project_relational_grouped_truth(
        &relational_row_set(),
        grouped_projection_contract("status", "identity.id", "profile.display_name"),
    )
    .unwrap();

    let error = contract
        .extract_from_relational_grouped_projection(&mismatched)
        .unwrap_err();
    assert!(matches!(
        error,
        ProjectionFactExtractionError::SourceIdentityMismatch { .. }
    ));
}

fn grouped_projection_contract(
    grouping_aspect: &str,
    identity_binding_aspect: &str,
    grouping_binding_aspect: &str,
) -> GroupedProjectionContract {
    GroupedProjectionContract::new(
        aspect_key(grouping_aspect),
        aspect_key(identity_binding_aspect),
        aspect_key(grouping_binding_aspect),
    )
}

fn aspect_key(label: &str) -> AspectKey {
    AspectKey::new(label).expect("test aspect key must be foundational")
}

#[test]
fn bridge_phase_four_entry_points_are_publicly_reachable() {
    let _row_set_extract: fn(
        &MaterializedProjectionContract,
        &forge_runtime_bridge::facade::BridgeMaterializedRowSetArtifact,
    )
        -> Result<ConsumedProjectionFactSet, ProjectionFactExtractionError> =
        MaterializedProjectionContract::extract_from_bridge_truth_view_row_set;
    let _grouped_extract: fn(
        &MaterializedProjectionContract,
        &forge_runtime_bridge::facade::BridgeGroupedTruthViewArtifact,
    )
        -> Result<ConsumedProjectionFactSet, ProjectionFactExtractionError> =
        MaterializedProjectionContract::extract_from_bridge_grouped_truth_view;
}
