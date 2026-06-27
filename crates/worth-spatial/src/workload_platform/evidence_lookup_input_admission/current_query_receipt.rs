#![allow(dead_code)]

use forge_foundational::facade::{AspectKey, AspectValue, FieldKey, ScalarAspectType};
use forge_foundational::InternedString;
use forge_query::facade::{
    declare_projection_consumption, evaluate_projection_consumption_eligibility,
    AuthorizedProjectionFieldPath, ProjectMaterializedFacts, ProjectionConsumptionBindingContext,
    ProjectionConsumptionEligibility, ProjectionConsumptionReceipt, ProjectionConsumptionSource,
};
use forge_relational::facade::grouped_truth::{
    materialize_relational_authoritative_row_set, RelationalAuthoritativeRowSetArtifact,
};
use forge_runtime_bridge::facade::{
    RelationalBridgeRecordIdentityParts, RelationalBridgeSnapshotIdentityParts,
    SnapshotReadContract, SnapshotReadPacket, SnapshotReadPacketResult, SnapshotReadRecord,
    SnapshotReadRequest, TruthSnapshotIdentity,
};

pub(crate) fn current_projection_consumption_receipt() -> ProjectionConsumptionReceipt {
    let row_set = projection_row_set();
    let declaration = declare_projection_consumption(
        ProjectionConsumptionSource::from_relational_row_set(&row_set),
        ProjectionConsumptionBindingContext::from_projection_metadata(
            "result-shape:worth-spatial-phase-five",
            "query:worth-spatial-phase-five",
            "result-shape:worth-spatial-phase-five",
            "authorized-projection:worth-spatial-phase-five",
            "narrowed-result-shape:worth-spatial-phase-five",
            "policy:worth-spatial-phase-five",
            "tenant-schema:worth-spatial-phase-five",
            vec![field_path("identity", "id")],
        ),
        ProjectMaterializedFacts::declare().entity_identities(),
    )
    .expect("projection receipt declaration should author");
    let contract = match evaluate_projection_consumption_eligibility(&declaration) {
        ProjectionConsumptionEligibility::Admitted(admitted) => admitted.bind_contract(),
        ProjectionConsumptionEligibility::AdmittedWithWarnings(admitted, _) => {
            admitted.bind_contract()
        }
        other => panic!("expected admitted projection receipt fixture, got {other:?}"),
    };
    contract
        .extract_from_relational_row_set(&row_set)
        .expect("projection receipt extraction should succeed")
        .issue_receipt()
}

#[cfg(test)]
pub(crate) fn real_projection_consumption_receipt() -> ProjectionConsumptionReceipt {
    current_projection_consumption_receipt()
}

fn projection_row_set() -> RelationalAuthoritativeRowSetArtifact {
    let entity = RelationalBridgeRecordIdentityParts::entity(1, 1, 1);
    let identity_read = string_read(entity.clone(), "identity.id");
    materialize_relational_authoritative_row_set(
        &SnapshotReadPacket::new(vec![identity_read.clone()]),
        &SnapshotReadPacketResult::new(
            TruthSnapshotIdentity::from_relational_snapshot(
                RelationalBridgeSnapshotIdentityParts::new(1, 1),
            ),
            vec![SnapshotReadRecord::for_request(
                &identity_read,
                string_value("fixture-entity-1"),
            )],
        ),
    )
    .expect("projection receipt row set should materialize")
}

fn field_path(aspect: &str, field: &str) -> AuthorizedProjectionFieldPath {
    AuthorizedProjectionFieldPath::from_native_keys(
        AspectKey::new(aspect.to_string()).expect("fixture aspect key should admit"),
        FieldKey::new(field.to_string()).expect("fixture field key should admit"),
    )
}

fn string_read(entity: RelationalBridgeRecordIdentityParts, aspect: &str) -> SnapshotReadRequest {
    SnapshotReadRequest::for_relational_record(
        entity,
        SnapshotReadContract::scalar(
            AspectKey::new(aspect.to_string()).expect("fixture read aspect should admit"),
            ScalarAspectType::String,
        ),
    )
}

fn string_value(value: &str) -> AspectValue {
    forge_relational::facade::grouped_truth::encode_snapshot_aspect_read_value(
        &AspectValue::String(InternedString::Raw(value.to_string())),
    )
}
