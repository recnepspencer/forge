use crate::authoring::{
    AspectFieldSelector, AuthoredResultShapeField, GuidedAuthoringPath, RawAuthoredQuery,
    RawAuthoredResultShape, RootEntityKey,
};
use crate::authorized_projection::{
    derive_authorized_projection, PolicyAspectMask, PolicyInfluenceSet,
};
use crate::facade::{
    AuthorizedProjectionArtifact, CanonicalResultShapeArtifact, ForgeQueryReadReceipt,
    ForgeQueryWriteReceipt, ProjectMaterializedFacts, ProjectionConsumptionAuthoringSurface,
    ProjectionConsumptionDeclaration, ProjectionConsumptionDeclarationError,
    ProjectionSourceFamily, QueryContextExecutionArtifact,
};
use forge_relational::facade::grouped_truth::{
    materialize_relational_authoritative_row_set, project_relational_grouped_truth,
    GroupedProjectionContract, RelationalAuthoritativeRowSetArtifact,
    RelationalGroupedProjectionArtifact,
};
use forge_runtime_bridge::facade::{
    BridgeGroupedTruthViewArtifact, BridgeMaterializedRowSetArtifact,
};
use forge_runtime_bridge::facade::{
    SnapshotReadPacket, SnapshotReadPacketResult, SnapshotReadRecord, SnapshotReadRequest,
    TruthSnapshotIdentity,
};

fn projection_artifacts() -> (CanonicalResultShapeArtifact, AuthorizedProjectionArtifact) {
    let query = RawAuthoredQuery::detail_builder(RootEntityKey::new("task").unwrap())
        .project(AspectFieldSelector::new("identity", "id").unwrap())
        .project(AspectFieldSelector::new("status", "lane").unwrap())
        .build()
        .unwrap();
    let result_shape = RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .field(AuthoredResultShapeField::new("status", "lane", "lane").unwrap())
        .build()
        .unwrap();
    let canonical = GuidedAuthoringPath::canonicalize_detail(query, result_shape).unwrap();
    let authorized_projection = derive_authorized_projection(
        canonical.query(),
        canonical.result_shape(),
        "policy:test",
        "schema:test",
        &PolicyAspectMask::allow_all(),
        &PolicyInfluenceSet::none(),
        8,
        8,
    )
    .unwrap();

    (canonical.result_shape().clone(), authorized_projection)
}

fn relational_row_set() -> RelationalAuthoritativeRowSetArtifact {
    materialize_relational_authoritative_row_set(
        &SnapshotReadPacket::new(vec![
            SnapshotReadRequest::for_coarse("entity-1", "identity.id"),
            SnapshotReadRequest::for_coarse("entity-1", "status.lane"),
            SnapshotReadRequest::for_coarse("entity-2", "identity.id"),
            SnapshotReadRequest::for_coarse("entity-2", "status.lane"),
        ]),
        &SnapshotReadPacketResult::new(
            TruthSnapshotIdentity::new("snapshot-a"),
            vec![
                SnapshotReadRecord::new("entity-1:identity.id", b"task-1".to_vec()),
                SnapshotReadRecord::new("entity-1:status.lane", b"todo".to_vec()),
                SnapshotReadRecord::new("entity-2:identity.id", b"task-2".to_vec()),
                SnapshotReadRecord::new("entity-2:status.lane", b"doing".to_vec()),
            ],
        ),
    )
    .unwrap()
}

fn assert_declares(
    declaration: Result<ProjectionConsumptionDeclaration, ProjectionConsumptionDeclarationError>,
    expected_family: ProjectionSourceFamily,
    expected_shape: &str,
) {
    let declaration = declaration.expect("phase 1 declaration should build");
    assert_eq!(declaration.source().family(), expected_family);
    assert_eq!(declaration.binding().result_shape_digest(), expected_shape);
}

#[test]
fn query_owned_phase_one_entry_points_exist() {
    let _read_method: fn(
        &ForgeQueryReadReceipt,
        &CanonicalResultShapeArtifact,
        &AuthorizedProjectionArtifact,
        ProjectMaterializedFacts,
    ) -> Result<
        ProjectionConsumptionDeclaration,
        ProjectionConsumptionDeclarationError,
    > = ForgeQueryReadReceipt::declare_projection_fact_consumption;
    let _write_method: fn(
        &ForgeQueryWriteReceipt,
        &str,
        &AuthorizedProjectionArtifact,
        ProjectMaterializedFacts,
    ) -> Result<
        ProjectionConsumptionDeclaration,
        ProjectionConsumptionDeclarationError,
    > = ForgeQueryWriteReceipt::declare_projection_fact_consumption;
    let _query_context_method: fn(
        &QueryContextExecutionArtifact,
        &AuthorizedProjectionArtifact,
        ProjectMaterializedFacts,
    ) -> Result<
        ProjectionConsumptionDeclaration,
        ProjectionConsumptionDeclarationError,
    > = QueryContextExecutionArtifact::declare_projection_fact_consumption;
}

#[test]
fn source_locked_phase_one_authoring_surfaces_exist_for_all_declared_families() {
    let _relational_row_set_surface: fn(
        &RelationalAuthoritativeRowSetArtifact,
        &str,
        &AuthorizedProjectionArtifact,
    ) -> ProjectionConsumptionAuthoringSurface =
        ProjectionConsumptionAuthoringSurface::from_relational_row_set;
    let _relational_grouped_surface: fn(
        &RelationalGroupedProjectionArtifact,
        &str,
        &AuthorizedProjectionArtifact,
    ) -> ProjectionConsumptionAuthoringSurface =
        ProjectionConsumptionAuthoringSurface::from_relational_grouped_projection;
    let _bridge_row_set_surface: fn(
        &BridgeMaterializedRowSetArtifact,
        &str,
        &AuthorizedProjectionArtifact,
    ) -> ProjectionConsumptionAuthoringSurface =
        ProjectionConsumptionAuthoringSurface::from_bridge_truth_view_row_set;
    let _bridge_grouped_surface: fn(
        &BridgeGroupedTruthViewArtifact,
        &str,
        &AuthorizedProjectionArtifact,
    ) -> ProjectionConsumptionAuthoringSurface =
        ProjectionConsumptionAuthoringSurface::from_bridge_grouped_truth_view;
}

#[test]
fn relational_phase_one_authoring_surfaces_build_declarations() {
    let (result_shape, authorized_projection) = projection_artifacts();
    let row_set = relational_row_set();
    let grouped_projection = project_relational_grouped_truth(
        &row_set,
        GroupedProjectionContract::new("status", "identity.id", "status.lane"),
    )
    .unwrap();

    assert_declares(
        ProjectMaterializedFacts::declare()
            .source(
                ProjectionConsumptionAuthoringSurface::from_relational_row_set(
                    &row_set,
                    result_shape.digest().as_str(),
                    &authorized_projection,
                ),
            )
            .entity_identities()
            .view_local_identities()
            .build(),
        ProjectionSourceFamily::RelationalRowSet,
        result_shape.digest().as_str(),
    );
    assert_declares(
        ProjectMaterializedFacts::declare()
            .source(
                ProjectionConsumptionAuthoringSurface::from_relational_grouped_projection(
                    &grouped_projection,
                    result_shape.digest().as_str(),
                    &authorized_projection,
                ),
            )
            .memberships()
            .relation_endpoints()
            .view_local_identities()
            .build(),
        ProjectionSourceFamily::RelationalGroupedProjection,
        result_shape.digest().as_str(),
    );
}
