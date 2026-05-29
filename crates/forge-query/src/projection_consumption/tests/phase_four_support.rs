use crate::authoring::{
    AspectFieldSelector, AuthoredResultShapeField, GuidedAuthoringPath, RawAuthoredQuery,
    RawAuthoredResultShape, RootEntityKey,
};
use crate::authorized_projection::{
    AuthorizedProjectionArtifact, AuthorizedProjectionCounters, MaskedProjectionArtifact,
    PolicyFieldInfluenceSet,
};
use crate::memory_workspace::ForgeQueryEntity;
use crate::query_context::{QueryContextExecutionArtifact, QueryContextExecutionFamily};
use crate::runtime::{
    ForgeQueryContinuityMutationEvidence, ForgeQueryContinuityMutationFamily,
    ForgeQueryContinuityOutcomeClass, ForgeQueryMutationTargetClass, ForgeQueryReadExecutionEngine,
    ForgeQueryReadReceipt, ForgeQueryReadResult, ForgeQueryWriteReceipt,
};
use forge_foundational::facade::{AspectKey, AspectValue};
use forge_relational::facade::grouped_truth::{
    encode_snapshot_aspect_read_value, materialize_relational_authoritative_row_set,
    project_relational_grouped_truth, GroupedProjectionContract,
    RelationalAuthoritativeRowSetArtifact, RelationalGroupedProjectionArtifact,
};
use forge_runtime_bridge::facade::{
    SnapshotReadPacket, SnapshotReadPacketResult, SnapshotReadRecord, SnapshotReadRequest,
    TruthSnapshotIdentity,
};
use serde_json::json;

use super::super::{
    declare_projection_consumption, evaluate_projection_consumption_eligibility,
    AdmittedProjectionConsumption, ProjectMaterializedFacts, ProjectionConsumptionBindingContext,
    ProjectionConsumptionEligibility, ProjectionConsumptionSource,
};

pub(super) fn binding(visible_fields: &[&str]) -> ProjectionConsumptionBindingContext {
    ProjectionConsumptionBindingContext::test_only(
        "result-shape:test",
        "authorized-projection:test",
        visible_fields
            .iter()
            .map(|field| field.to_string())
            .collect(),
    )
}

pub(super) fn authorized_projection(
    query_digest: &str,
    result_shape_digest: &str,
    visible_fields: &[&str],
) -> AuthorizedProjectionArtifact {
    AuthorizedProjectionArtifact::new(
        query_digest,
        result_shape_digest,
        "policy:test",
        "tenant-schema:test",
        visible_fields
            .iter()
            .map(|field| field.to_string())
            .collect(),
        MaskedProjectionArtifact::new(Vec::new(), Vec::new()),
        "narrowed-result-shape:test".to_string(),
        PolicyFieldInfluenceSet::new(&["influence:test".to_string()], 1),
        AuthorizedProjectionCounters::default(),
    )
}

pub(super) fn binding_for_result_shape(
    result_shape_digest: &str,
    visible_fields: &[&str],
) -> ProjectionConsumptionBindingContext {
    ProjectionConsumptionBindingContext::test_only_with_projection_metadata(
        result_shape_digest,
        "query:test",
        result_shape_digest,
        "authorized-projection:test",
        "narrowed-result-shape:test",
        "policy:test",
        "tenant-schema:test",
        visible_fields
            .iter()
            .map(|field| field.to_string())
            .collect(),
    )
}

pub(super) fn admitted(
    source: ProjectionConsumptionSource,
    binding: ProjectionConsumptionBindingContext,
    requested: ProjectMaterializedFacts,
) -> AdmittedProjectionConsumption {
    let declaration = declare_projection_consumption(source, binding, requested).unwrap();
    match evaluate_projection_consumption_eligibility(&declaration) {
        ProjectionConsumptionEligibility::Admitted(admitted) => admitted,
        ProjectionConsumptionEligibility::AdmittedWithWarnings(admitted, _) => admitted,
        other => panic!("expected admitted eligibility, got {other:?}"),
    }
}

pub(super) fn relational_row_set() -> RelationalAuthoritativeRowSetArtifact {
    let packet = SnapshotReadPacket::new(vec![
        SnapshotReadRequest::for_coarse("entity-1", "identity.id"),
        SnapshotReadRequest::for_coarse("entity-1", "status.lane"),
        SnapshotReadRequest::for_coarse("entity-1", "profile.display_name"),
        SnapshotReadRequest::for_coarse("entity-2", "identity.id"),
        SnapshotReadRequest::for_coarse("entity-2", "status.lane"),
        SnapshotReadRequest::for_coarse("entity-2", "profile.display_name"),
    ]);
    let result = SnapshotReadPacketResult::new(
        TruthSnapshotIdentity::new("snapshot-a"),
        vec![
            SnapshotReadRecord::new(
                "entity-1:identity.id",
                aspect_bytes(AspectValue::String("task-1".into())),
            ),
            SnapshotReadRecord::new(
                "entity-1:status.lane",
                aspect_bytes(AspectValue::String("todo".into())),
            ),
            SnapshotReadRecord::new(
                "entity-1:profile.display_name",
                aspect_bytes(AspectValue::String("Task One".into())),
            ),
            SnapshotReadRecord::new(
                "entity-2:identity.id",
                aspect_bytes(AspectValue::String("task-2".into())),
            ),
            SnapshotReadRecord::new(
                "entity-2:status.lane",
                aspect_bytes(AspectValue::String("doing".into())),
            ),
            SnapshotReadRecord::new(
                "entity-2:profile.display_name",
                aspect_bytes(AspectValue::String("Task Two".into())),
            ),
        ],
    );
    materialize_relational_authoritative_row_set(&packet, &result).unwrap()
}

pub(super) fn relational_grouped_projection() -> RelationalGroupedProjectionArtifact {
    project_relational_grouped_truth(
        &relational_row_set(),
        grouped_projection_contract("status", "identity.id", "status.lane"),
    )
    .expect("grouped projection")
}

fn aspect_bytes(value: AspectValue) -> Vec<u8> {
    encode_snapshot_aspect_read_value(&value).expect("test aspect value bytes")
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

pub(super) fn write_receipt() -> ForgeQueryWriteReceipt {
    ForgeQueryWriteReceipt::test_only(
        "commit:test",
        "snapshot:test",
        ForgeQueryMutationTargetClass::Entity,
        Some("tasks"),
        Some("task-1"),
        Some("bridge-record:test"),
        Some("$same_batch_target"),
        Some(ForgeQueryContinuityMutationEvidence::test_only(
            ForgeQueryContinuityMutationFamily::RebindExistingTarget,
            ForgeQueryContinuityOutcomeClass::ContinuesAsSingleSuccessor,
            "task-0",
            vec!["task-1".to_string()],
            Some("task-1"),
            Some("tasks"),
        )),
    )
}

pub(super) fn write_receipt_without_source_references() -> ForgeQueryWriteReceipt {
    ForgeQueryWriteReceipt::test_only(
        "commit:test:no-source-ref",
        "snapshot:test",
        ForgeQueryMutationTargetClass::Entity,
        Some("tasks"),
        Some("task-1"),
        None,
        None,
        Some(ForgeQueryContinuityMutationEvidence::test_only(
            ForgeQueryContinuityMutationFamily::RebindExistingTarget,
            ForgeQueryContinuityOutcomeClass::ContinuesAsSingleSuccessor,
            "task-0",
            vec!["task-1".to_string()],
            Some("task-1"),
            Some("tasks"),
        )),
    )
}

pub(super) fn read_result() -> ForgeQueryReadResult {
    ForgeQueryReadResult::test_only(
        vec![
            ForgeQueryEntity {
                identity: "task-1".to_string(),
                payload: json!({
                    "profile": { "display_name": "Task One" },
                    "metrics": { "priority": 1 }
                }),
            },
            ForgeQueryEntity {
                identity: "task-2".to_string(),
                payload: json!({
                    "profile": { "display_name": "Task Two" },
                    "metrics": { "priority": 2 }
                }),
            },
        ],
        ForgeQueryReadReceipt::test_only(
            "read-graph:test",
            "query:test",
            "basis:test",
            "result:test",
            ForgeQueryReadExecutionEngine::QueryRuntimeCurrent,
        ),
    )
}

pub(super) fn read_result_shape() -> crate::canonicalization::CanonicalResultShapeArtifact {
    let query = RawAuthoredQuery::detail_builder(RootEntityKey::new("task").unwrap())
        .project(AspectFieldSelector::new("profile", "display_name").unwrap())
        .project(AspectFieldSelector::new("metrics", "priority").unwrap())
        .build()
        .unwrap();
    let result_shape = RawAuthoredResultShape::detail_builder()
        .field(
            AuthoredResultShapeField::new("profile", "display_name", "profile.display_name")
                .unwrap(),
        )
        .field(AuthoredResultShapeField::new("metrics", "priority", "metrics.priority").unwrap())
        .build()
        .unwrap();
    GuidedAuthoringPath::canonicalize_detail(query, result_shape)
        .unwrap()
        .result_shape()
        .clone()
}

pub(super) fn query_context_execution_current() -> QueryContextExecutionArtifact {
    QueryContextExecutionArtifact::test_only(
        QueryContextExecutionFamily::RuntimeCurrent,
        "query:test",
        "basis:test",
        "result:test",
        "result-shape:test",
        vec!["payload-row-0".to_string(), "payload-row-1".to_string()],
        None,
        None,
    )
}

pub(super) fn query_context_execution_historical() -> QueryContextExecutionArtifact {
    QueryContextExecutionArtifact::test_only(
        QueryContextExecutionFamily::HistoricalMaterialized,
        "query:test",
        "basis:test",
        "result:test",
        "result-shape:test",
        vec!["historical-row-0".to_string()],
        Some("materialization-path:test"),
        None,
    )
}

pub(super) fn query_context_execution_preview() -> QueryContextExecutionArtifact {
    QueryContextExecutionArtifact::test_only(
        QueryContextExecutionFamily::PreviewDerivedHistorical,
        "query:test",
        "basis:test",
        "result:test",
        "result-shape:test",
        vec!["preview-row-0".to_string()],
        None,
        Some("preview-provenance:test"),
    )
}
