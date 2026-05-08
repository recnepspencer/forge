use forge_relational::facade::runtime::RelationalRuntime;
use forge_relational::facade::snapshots::SnapshotHandle;
use schema::facade::{
    BoundaryFailure, DerivedTopologyReadBasis, QueryAspectPath, QueryCollection,
    QueryComputedDeclarationBuilder, QueryLiveDeclarationBuilder, QuerySchemaBasis,
    VerifiedTopologyCommit,
};
use topology::facade::{
    certify_milestone_one_read_basis_traced, certify_milestone_two_read_basis_traced,
    certify_milestone_two_verified_topology_commit_traced, certify_verified_topology_commit_traced,
    declare_persistent_name_live_view, declare_topology_diagnostics_surface,
    declare_topology_entity_live_view, declare_topology_equivalence_contract_surface,
    declare_topology_interpreted_surface, declare_topology_materialized_surface,
    declare_topology_relation_live_view, declare_topology_validation_surface,
    derived_read_diagnostics_from_query_rows, equivalence_contract_from_diagnostics_rows,
    interpreted_topology_from_materialized_rows, naming_attachment_report_from_query_rows,
    persistent_name_live_view_declaration, topology_runtime, validation_report_from_query_rows,
    MilestoneOneCertificationError, TopologyDomainQuery, TopologyDomainQueryAggregateReport,
    TopologyDomainQueryCloseoutReport, TopologyDomainQueryCloseoutRow,
    TopologyDomainQueryCloseoutStatus, TopologyDomainQueryExecutionEngine,
    TopologyDomainQueryParityAggregateReport, TopologyDomainQueryPhaseThreeBlocker,
    TopologyDomainQueryPhaseThreeBlockerRow, TopologyDomainQueryPhaseThreeBlockerStatus,
    TopologyDomainQueryProofReport, TopologyDomainQueryRequestFamily,
    TopologyDomainQueryRequestReport, TopologyEditApplicationMode, TopologyEditBatch,
    TopologyHalfEdgeRadialNeighborhoodView, TopologyHalfEdgeSharedVertexNeighborhoodView,
    TopologyLocalRewireNeighborhoodView, TopologyLoopCycleView, TopologyNoNPlusOneContract,
    TopologyNoNPlusOneContractRow, TopologyNoNPlusOneContractStatus, TopologyQueryAppliedIntent,
    TopologyQueryApplyError, TopologyQueryAssembly, TopologyQueryEditExecution,
    TopologyQueryEditExecutionError, TopologyQueryEditFamilySupportStatus, TopologyQueryEditLane,
    TopologyQueryEditLaneExecutionShape, TopologyQueryEditLaneSupportStatus,
    TopologyQueryMutationEvidence, TopologyQueryReadFamilySupportStatus, TopologyQuerySnapshot,
    TopologyRuntimeAdapters, TopologyRuntimeCloseout, TopologyRuntimeCloseoutFamily,
    TopologyRuntimeCloseoutStatus, TopologyRuntimeEditFamilySupportRow,
    TopologyRuntimeEditLaneSupportRow, TopologyRuntimeFailure, TopologyRuntimePostureCapability,
    TopologyRuntimePostureRow, TopologyRuntimePostureStatus, TopologyRuntimeReadFamilySupportRow,
    TopologyRuntimeSupport, TracedMilestoneOneCertificationReport,
    TracedMilestoneTwoDerivedReadReport,
};

fn _m1_read_cert_contract(
    runtime: &mut RelationalRuntime,
    basis: DerivedTopologyReadBasis,
) -> Result<TracedMilestoneOneCertificationReport, BoundaryFailure<MilestoneOneCertificationError>>
{
    certify_milestone_one_read_basis_traced(runtime, basis)
}

fn _m1_commit_cert_contract(
    runtime: &mut RelationalRuntime,
    verified: &VerifiedTopologyCommit,
) -> Result<TracedMilestoneOneCertificationReport, BoundaryFailure<MilestoneOneCertificationError>>
{
    certify_verified_topology_commit_traced(runtime, verified)
}

fn _m2_read_cert_contract(
    runtime: &mut RelationalRuntime,
    basis: DerivedTopologyReadBasis,
) -> Result<TracedMilestoneTwoDerivedReadReport, BoundaryFailure<MilestoneOneCertificationError>> {
    certify_milestone_two_read_basis_traced(runtime, basis)
}

fn _m2_commit_cert_contract(
    runtime: &mut RelationalRuntime,
    verified: &VerifiedTopologyCommit,
) -> Result<TracedMilestoneTwoDerivedReadReport, BoundaryFailure<MilestoneOneCertificationError>> {
    certify_milestone_two_verified_topology_commit_traced(runtime, verified)
}

fn _edit_apply_contract(
    assembly: &TopologyQueryAssembly,
    workspace: &mut forge_query::facade::ForgeQueryWorkspace,
    batch: TopologyEditBatch,
    mode: TopologyEditApplicationMode,
) -> Result<TopologyQueryEditExecution, TopologyQueryEditExecutionError> {
    assembly.apply_edit(workspace, batch, mode)
}

fn _vocab_live_query_declaration_contract() {
    let _ = QueryLiveDeclarationBuilder::new(
        ".topo.query.entities",
        QueryCollection::TopologyEntity,
        QuerySchemaBasis::TopologyEntityLiveView,
    )
    .select([
        QueryAspectPath::TOPOLOGY_STRUCTURE,
        QueryAspectPath::NAMING_PERSISTENT_NAME,
    ])
    .build()
    .unwrap();
}

fn _vocab_computed_query_declaration_contract() {
    let _ = QueryComputedDeclarationBuilder::new(".topo.query.validation")
        .reads([QueryAspectPath::TOPOLOGY_STRUCTURE])
        .produces([QueryAspectPath::DIAGNOSTICS_DECISIONS])
        .build()
        .unwrap();
}

fn _query_native_topology_surface_contracts() {
    let _: fn() -> TopologyDomainQuery = TopologyDomainQuery::load;
    let _: fn(&TopologyDomainQuery) -> Vec<TopologyDomainQueryRequestFamily> =
        TopologyDomainQuery::supported_request_families;
    let _: fn(&TopologyDomainQuery) -> TopologyDomainQueryAggregateReport =
        TopologyDomainQuery::aggregate_report;
    let _: fn(&TopologyDomainQuery) -> TopologyDomainQueryProofReport =
        TopologyDomainQuery::proof_report;
    let _: fn(&TopologyDomainQuery) -> TopologyDomainQueryCloseoutReport =
        TopologyDomainQuery::closeout_report;
    let _: fn(TopologyDomainQueryExecutionEngine) -> &'static str =
        TopologyDomainQueryExecutionEngine::as_str;
    let _: fn(TopologyDomainQueryRequestFamily) -> &'static str =
        TopologyDomainQueryRequestFamily::as_str;
    let _: fn(&TopologyDomainQueryRequestReport) -> TopologyDomainQueryRequestFamily =
        TopologyDomainQueryRequestReport::request_family;
    let _: fn(&TopologyDomainQueryRequestReport) -> TopologyDomainQueryExecutionEngine =
        TopologyDomainQueryRequestReport::execution_engine;
    let _: fn(&TopologyHalfEdgeSharedVertexNeighborhoodView) -> &TopologyDomainQueryRequestReport =
        TopologyHalfEdgeSharedVertexNeighborhoodView::request_report;
    let _: fn(&TopologyHalfEdgeRadialNeighborhoodView) -> &TopologyDomainQueryRequestReport =
        TopologyHalfEdgeRadialNeighborhoodView::request_report;
    let _: fn(&TopologyLoopCycleView) -> &TopologyDomainQueryRequestReport =
        TopologyLoopCycleView::request_report;
    let _: fn(&TopologyLocalRewireNeighborhoodView) -> &TopologyDomainQueryRequestReport =
        TopologyLocalRewireNeighborhoodView::request_report;
    let _: fn(&TopologyDomainQueryProofReport) -> &TopologyDomainQueryAggregateReport =
        TopologyDomainQueryProofReport::request_aggregate;
    let _: fn(&TopologyDomainQueryProofReport) -> &TopologyDomainQueryParityAggregateReport =
        TopologyDomainQueryProofReport::parity_aggregate;
    let _: fn(TopologyDomainQueryCloseoutStatus) -> &'static str =
        TopologyDomainQueryCloseoutStatus::as_str;
    let _: fn(&TopologyDomainQueryCloseoutReport) -> &[TopologyDomainQueryCloseoutRow] =
        TopologyDomainQueryCloseoutReport::family_rows;
    let _: fn(
        &TopologyDomainQueryCloseoutReport,
        TopologyDomainQueryRequestFamily,
    ) -> TopologyDomainQueryCloseoutStatus = TopologyDomainQueryCloseoutReport::status;
    let _: fn(&TopologyDomainQueryCloseoutRow) -> &str = TopologyDomainQueryCloseoutRow::reason;
    let _: fn(&TopologyDomainQueryCloseoutRow) -> &str = TopologyDomainQueryCloseoutRow::row_digest;
    let _: fn(TopologyDomainQueryPhaseThreeBlocker) -> &'static str =
        TopologyDomainQueryPhaseThreeBlocker::as_str;
    let _: fn(TopologyDomainQueryPhaseThreeBlockerStatus) -> &'static str =
        TopologyDomainQueryPhaseThreeBlockerStatus::as_str;
    let _: fn(&TopologyDomainQueryCloseoutReport) -> &[TopologyDomainQueryPhaseThreeBlockerRow] =
        TopologyDomainQueryCloseoutReport::phase_three_blocker_rows;
    let _: fn(
        &TopologyDomainQueryCloseoutReport,
        TopologyDomainQueryPhaseThreeBlocker,
    ) -> TopologyDomainQueryPhaseThreeBlockerStatus =
        TopologyDomainQueryCloseoutReport::phase_three_blocker_status;
    let _: fn(TopologyNoNPlusOneContract) -> &'static str = TopologyNoNPlusOneContract::as_str;
    let _: fn(TopologyNoNPlusOneContractStatus) -> &'static str =
        TopologyNoNPlusOneContractStatus::as_str;
    let _: fn(&TopologyDomainQueryCloseoutReport) -> &[TopologyNoNPlusOneContractRow] =
        TopologyDomainQueryCloseoutReport::no_n_plus_one_contract_rows;
    let _: fn(
        &TopologyDomainQueryCloseoutReport,
        TopologyNoNPlusOneContract,
    ) -> TopologyNoNPlusOneContractStatus =
        TopologyDomainQueryCloseoutReport::no_n_plus_one_contract_status;
    let _: fn(&TopologyNoNPlusOneContractRow) -> TopologyNoNPlusOneContract =
        TopologyNoNPlusOneContractRow::contract;
    let _: fn(&TopologyNoNPlusOneContractRow) -> TopologyNoNPlusOneContractStatus =
        TopologyNoNPlusOneContractRow::status;
    let _: fn(&TopologyNoNPlusOneContractRow) -> &str = TopologyNoNPlusOneContractRow::reason;
    let _: fn(&TopologyNoNPlusOneContractRow) -> &str = TopologyNoNPlusOneContractRow::row_digest;
    let _: fn(
        TopologyRuntimeAdapters,
        String,
    ) -> Result<forge_query::facade::ForgeQueryWorkspace, TopologyRuntimeFailure> =
        topology_runtime;
    let _: fn(forge_relational::facade::runtime::RelationalRuntime) -> TopologyRuntimeAdapters =
        TopologyRuntimeAdapters::current_head;
    let _: fn(
        forge_relational::facade::runtime::RelationalReadView,
        SnapshotHandle,
    ) -> TopologyRuntimeAdapters = TopologyRuntimeAdapters::snapshot_read_only;
    let _: fn(&TopologyRuntimeAdapters) -> &TopologyRuntimeSupport =
        TopologyRuntimeAdapters::support;
    let _: fn(
        &TopologyRuntimeSupport,
        topology::facade::TopologyEditFamily,
    ) -> TopologyQueryEditFamilySupportStatus =
        TopologyRuntimeSupport::query_edit_family_support_status;
    let _: fn(&TopologyRuntimeSupport) -> &[TopologyRuntimeEditFamilySupportRow] =
        TopologyRuntimeSupport::query_edit_family_support_rows;
    let _: fn(&TopologyRuntimeSupport) -> &[TopologyRuntimeEditLaneSupportRow] =
        TopologyRuntimeSupport::query_edit_lane_support_rows;
    let _: fn(
        &TopologyRuntimeSupport,
        TopologyQueryEditLane,
    ) -> TopologyQueryEditLaneSupportStatus =
        TopologyRuntimeSupport::query_edit_lane_support_status;
    let _: fn(&TopologyRuntimeSupport) -> &[TopologyRuntimePostureRow] =
        TopologyRuntimeSupport::runtime_posture_rows;
    let _: fn(
        &TopologyRuntimeSupport,
        TopologyRuntimePostureCapability,
    ) -> TopologyRuntimePostureStatus = TopologyRuntimeSupport::runtime_posture_status;
    let _: fn(&TopologyRuntimeSupport) -> &[TopologyRuntimeReadFamilySupportRow] =
        TopologyRuntimeSupport::query_read_family_support_rows;
    let _: fn(
        &TopologyRuntimeSupport,
        TopologyDomainQueryRequestFamily,
    ) -> TopologyQueryReadFamilySupportStatus =
        TopologyRuntimeSupport::query_read_family_support_status;
    let _: fn(&TopologyRuntimeSupport) -> &TopologyRuntimeCloseout =
        TopologyRuntimeSupport::closeout;
    let _: fn(
        &TopologyRuntimeCloseout,
        TopologyRuntimeCloseoutFamily,
    ) -> TopologyRuntimeCloseoutStatus = TopologyRuntimeCloseout::status;
    let _: fn(TopologyQueryEditLane) -> &'static str = TopologyQueryEditLane::as_str;
    let _: Option<TopologyQueryEditLaneExecutionShape> = None;
    let _: fn(
        &mut forge_query::facade::ForgeQueryWorkspace,
    ) -> Result<TopologyQueryAssembly, forge_query::facade::ForgeQueryRuntimeError> =
        TopologyQueryAssembly::declare;
    let _: fn(
        &TopologyQueryAssembly,
        &mut forge_query::facade::ForgeQueryWorkspace,
    ) -> Result<TopologyQuerySnapshot, topology::facade::TopologyQuerySurfaceError> =
        TopologyQueryAssembly::snapshot;
    let _: fn(
        &TopologyQueryAssembly,
        &mut forge_query::facade::ForgeQueryWorkspace,
        schema::facade::RawTopologyIntent,
        &schema::facade::DerivedTopologyReadBasis,
    ) -> Result<TopologyQueryAppliedIntent, TopologyQueryApplyError> =
        TopologyQueryAssembly::apply_raw_intent;
    let _: fn(
        &mut forge_query::facade::ForgeQueryWorkspace,
        String,
    ) -> Result<
        forge_query::facade::ForgeQueryLiveView<serde_json::Value>,
        forge_query::facade::ForgeQueryRuntimeError,
    > = declare_topology_entity_live_view::<serde_json::Value>;
    let _: fn(
        &mut forge_query::facade::ForgeQueryWorkspace,
        String,
    ) -> Result<
        forge_query::facade::ForgeQueryLiveView<serde_json::Value>,
        forge_query::facade::ForgeQueryRuntimeError,
    > = declare_topology_relation_live_view::<serde_json::Value>;
    let _: fn(
        &mut forge_query::facade::ForgeQueryWorkspace,
        String,
    ) -> Result<
        forge_query::facade::ForgeQueryLiveView<serde_json::Value>,
        forge_query::facade::ForgeQueryRuntimeError,
    > = declare_persistent_name_live_view::<serde_json::Value>;
    let _: fn(
        String,
    ) -> Result<
        forge_query::facade::ForgeQueryWorkspaceLiveViewDeclaration,
        schema::facade::QueryDeclarationError,
    > = persistent_name_live_view_declaration;
    let _: fn(
        &mut forge_query::facade::ForgeQueryWorkspace,
        String,
        &forge_query::facade::ForgeQueryLiveView<serde_json::Value>,
        &forge_query::facade::ForgeQueryLiveView<serde_json::Value>,
    ) -> Result<
        forge_query::facade::ForgeQueryDerivedViewHandle<serde_json::Value>,
        forge_query::facade::ForgeQueryRuntimeError,
    > = declare_topology_materialized_surface::<
        serde_json::Value,
        serde_json::Value,
        serde_json::Value,
    >;
    let _: fn(
        &mut forge_query::facade::ForgeQueryWorkspace,
        String,
        &forge_query::facade::ForgeQueryDerivedViewHandle<serde_json::Value>,
    ) -> Result<
        forge_query::facade::ForgeQueryDerivedViewHandle<serde_json::Value>,
        forge_query::facade::ForgeQueryRuntimeError,
    > = declare_topology_interpreted_surface::<serde_json::Value, serde_json::Value>;
    let _: fn(
        &mut forge_query::facade::ForgeQueryWorkspace,
        String,
        &forge_query::facade::ForgeQueryDerivedViewHandle<serde_json::Value>,
        &forge_query::facade::ForgeQueryDerivedViewHandle<serde_json::Value>,
    ) -> Result<
        forge_query::facade::ForgeQueryDerivedViewHandle<serde_json::Value>,
        forge_query::facade::ForgeQueryRuntimeError,
    > = declare_topology_validation_surface::<
        serde_json::Value,
        serde_json::Value,
        serde_json::Value,
    >;
    let _: fn(
        &mut forge_query::facade::ForgeQueryWorkspace,
        String,
        &forge_query::facade::ForgeQueryDerivedViewHandle<serde_json::Value>,
        &forge_query::facade::ForgeQueryDerivedViewHandle<serde_json::Value>,
        &forge_query::facade::ForgeQueryDerivedViewHandle<serde_json::Value>,
    ) -> Result<
        forge_query::facade::ForgeQueryDerivedViewHandle<serde_json::Value>,
        forge_query::facade::ForgeQueryRuntimeError,
    > = declare_topology_diagnostics_surface::<
        serde_json::Value,
        serde_json::Value,
        serde_json::Value,
        serde_json::Value,
    >;
    let _: fn(
        &mut forge_query::facade::ForgeQueryWorkspace,
        String,
        &forge_query::facade::ForgeQueryDerivedViewHandle<serde_json::Value>,
    ) -> Result<
        forge_query::facade::ForgeQueryDerivedViewHandle<serde_json::Value>,
        forge_query::facade::ForgeQueryRuntimeError,
    > = declare_topology_equivalence_contract_surface::<serde_json::Value, serde_json::Value>;
    let _: fn(
        &[serde_json::Value],
    ) -> Result<
        topology::facade::InterpretedTopologyView,
        topology::facade::TopologyQuerySurfaceError,
    > = interpreted_topology_from_materialized_rows;
    let _: fn(
        &[serde_json::Value],
        &[serde_json::Value],
    ) -> Result<
        topology::facade::DerivedTopologyValidationReport,
        topology::facade::TopologyQuerySurfaceError,
    > = validation_report_from_query_rows;
    let _: fn(
        &forge_query::facade::ForgeQueryRetainedMutationContext,
        &[serde_json::Value],
        &[serde_json::Value],
        &[serde_json::Value],
    ) -> Result<
        topology::facade::DerivedReadDiagnostics,
        topology::facade::TopologyQuerySurfaceError,
    > = derived_read_diagnostics_from_query_rows;
    let _: fn(
        &[serde_json::Value],
    ) -> Result<
        topology::facade::DerivedEquivalenceContractReport,
        topology::facade::TopologyQuerySurfaceError,
    > = equivalence_contract_from_diagnostics_rows;
    let _: fn(
        &[forge_query::facade::ForgeQueryEntity],
        &[forge_query::facade::ForgeQueryEntity],
    ) -> Result<
        topology::facade::NamingAttachmentReport,
        topology::facade::TopologyQuerySurfaceError,
    > = naming_attachment_report_from_query_rows;
    let _: fn(
        &schema::facade::DerivedTopologyReadBasis,
    ) -> topology::facade::TopologyQueryMutationEvidence =
        TopologyQueryMutationEvidence::from_read_basis;
}

#[test]
fn topo_public_traced_boundaries_compile_with_envelope_contracts() {
    let _ = _m1_read_cert_contract;
    let _ = _m1_commit_cert_contract;
    let _ = _m2_read_cert_contract;
    let _ = _m2_commit_cert_contract;
    let _ = _edit_apply_contract;
    let _ = _vocab_live_query_declaration_contract;
    let _ = _vocab_computed_query_declaration_contract;
    let _ = _query_native_topology_surface_contracts;
}
