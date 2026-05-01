use forge_relational::facade::runtime::RelationalRuntime;
use forge_relational::facade::snapshots::SnapshotHandle;
use worth_schema::facade::{
    DerivedTopologyReadBasis, VerifiedTopologyCommit, WorthBoundaryFailure, WorthQueryAspectPath,
    WorthQueryCollection, WorthQueryComputedDeclarationBuilder, WorthQueryLiveDeclarationBuilder,
    WorthQuerySchemaBasis,
};
use worth_topo::facade::{
    certify_milestone_one_read_basis_traced, certify_milestone_two_read_basis_traced,
    certify_milestone_two_verified_topology_commit_traced, certify_verified_topology_commit_traced,
    declare_worth_persistent_name_live_view, declare_worth_topology_diagnostics_surface,
    declare_worth_topology_entity_live_view, declare_worth_topology_equivalence_contract_surface,
    declare_worth_topology_interpreted_surface, declare_worth_topology_materialized_surface,
    declare_worth_topology_relation_live_view, declare_worth_topology_validation_surface,
    derived_read_diagnostics_from_query_rows, equivalence_contract_from_diagnostics_rows,
    interpreted_topology_from_materialized_rows, naming_attachment_report_from_query_rows,
    validation_report_from_query_rows, worth_persistent_name_live_view_declaration,
    worth_topology_runtime, WorthMilestoneOneCertificationError, WorthTopologyEditApplicationMode,
    WorthTopologyEditBatch, WorthTopologyQueryAppliedIntent, WorthTopologyQueryApplyError,
    WorthTopologyQueryAssembly, WorthTopologyQueryEditExecution,
    WorthTopologyQueryEditExecutionError, WorthTopologyQueryEditRunner,
    WorthTopologyQueryMutationEvidence, WorthTopologyQuerySnapshot, WorthTopologyRuntimeAdapters,
    WorthTopologyRuntimeFailure, WorthTopologyRuntimeSupport,
    WorthTracedMilestoneOneCertificationReport, WorthTracedMilestoneTwoDerivedReadReport,
};

fn _m1_read_cert_contract(
    runtime: &mut RelationalRuntime,
    basis: DerivedTopologyReadBasis,
) -> Result<
    WorthTracedMilestoneOneCertificationReport,
    WorthBoundaryFailure<WorthMilestoneOneCertificationError>,
> {
    certify_milestone_one_read_basis_traced(runtime, basis)
}

fn _m1_commit_cert_contract(
    runtime: &mut RelationalRuntime,
    verified: &VerifiedTopologyCommit,
) -> Result<
    WorthTracedMilestoneOneCertificationReport,
    WorthBoundaryFailure<WorthMilestoneOneCertificationError>,
> {
    certify_verified_topology_commit_traced(runtime, verified)
}

fn _m2_read_cert_contract(
    runtime: &mut RelationalRuntime,
    basis: DerivedTopologyReadBasis,
) -> Result<
    WorthTracedMilestoneTwoDerivedReadReport,
    WorthBoundaryFailure<WorthMilestoneOneCertificationError>,
> {
    certify_milestone_two_read_basis_traced(runtime, basis)
}

fn _m2_commit_cert_contract(
    runtime: &mut RelationalRuntime,
    verified: &VerifiedTopologyCommit,
) -> Result<
    WorthTracedMilestoneTwoDerivedReadReport,
    WorthBoundaryFailure<WorthMilestoneOneCertificationError>,
> {
    certify_milestone_two_verified_topology_commit_traced(runtime, verified)
}

fn _edit_apply_contract(
    runner: &mut WorthTopologyQueryEditRunner<'_, '_>,
    batch: WorthTopologyEditBatch,
    mode: WorthTopologyEditApplicationMode,
) -> Result<WorthTopologyQueryEditExecution, WorthTopologyQueryEditExecutionError> {
    runner.apply(batch, mode)
}

fn _worth_vocab_live_query_declaration_contract() {
    let _ = WorthQueryLiveDeclarationBuilder::new(
        "worth.topo.query.entities",
        WorthQueryCollection::TopologyEntity,
        WorthQuerySchemaBasis::TopologyEntityLiveView,
    )
    .select([
        WorthQueryAspectPath::TOPOLOGY_STRUCTURE,
        WorthQueryAspectPath::NAMING_PERSISTENT_NAME,
    ])
    .build()
    .unwrap();
}

fn _worth_vocab_computed_query_declaration_contract() {
    let _ = WorthQueryComputedDeclarationBuilder::new("worth.topo.query.validation")
        .reads([WorthQueryAspectPath::TOPOLOGY_STRUCTURE])
        .produces([WorthQueryAspectPath::DIAGNOSTICS_DECISIONS])
        .build()
        .unwrap();
}

fn _worth_query_native_topology_surface_contracts() {
    let _: fn(
        WorthTopologyRuntimeAdapters,
        String,
    )
        -> Result<forge_query::facade::ForgeQueryWorkspace, WorthTopologyRuntimeFailure> =
        worth_topology_runtime;
    let _: fn(
        forge_relational::facade::runtime::RelationalRuntime,
    ) -> WorthTopologyRuntimeAdapters = WorthTopologyRuntimeAdapters::current_head;
    let _: fn(
        forge_relational::facade::runtime::RelationalReadView,
        SnapshotHandle,
    ) -> WorthTopologyRuntimeAdapters = WorthTopologyRuntimeAdapters::snapshot_read_only;
    let _: fn(&WorthTopologyRuntimeAdapters) -> &WorthTopologyRuntimeSupport =
        WorthTopologyRuntimeAdapters::support;
    let _: fn(&WorthTopologyRuntimeSupport) -> bool =
        WorthTopologyRuntimeSupport::query_edit_execution_supported;
    let _: fn(
        &mut forge_query::facade::ForgeQueryWorkspace,
    )
        -> Result<WorthTopologyQueryAssembly, forge_query::facade::ForgeQueryRuntimeError> =
        WorthTopologyQueryAssembly::declare;
    let _: fn(
        &WorthTopologyQueryAssembly,
        &mut forge_query::facade::ForgeQueryWorkspace,
    ) -> Result<
        WorthTopologyQuerySnapshot,
        worth_topo::facade::WorthTopologyQuerySurfaceError,
    > = WorthTopologyQueryAssembly::snapshot;
    let _: fn(
        &WorthTopologyQueryAssembly,
        &mut forge_query::facade::ForgeQueryWorkspace,
        worth_schema::facade::RawWorthTopologyIntent,
        &worth_schema::facade::DerivedTopologyReadBasis,
    ) -> Result<WorthTopologyQueryAppliedIntent, WorthTopologyQueryApplyError> =
        WorthTopologyQueryAssembly::apply_raw_intent;
    let _: fn(
        &mut forge_query::facade::ForgeQueryWorkspace,
        String,
    ) -> Result<
        forge_query::facade::ForgeQueryLiveView<serde_json::Value>,
        forge_query::facade::ForgeQueryRuntimeError,
    > = declare_worth_topology_entity_live_view::<serde_json::Value>;
    let _: fn(
        &mut forge_query::facade::ForgeQueryWorkspace,
        String,
    ) -> Result<
        forge_query::facade::ForgeQueryLiveView<serde_json::Value>,
        forge_query::facade::ForgeQueryRuntimeError,
    > = declare_worth_topology_relation_live_view::<serde_json::Value>;
    let _: fn(
        &mut forge_query::facade::ForgeQueryWorkspace,
        String,
    ) -> Result<
        forge_query::facade::ForgeQueryLiveView<serde_json::Value>,
        forge_query::facade::ForgeQueryRuntimeError,
    > = declare_worth_persistent_name_live_view::<serde_json::Value>;
    let _: fn(
        String,
    ) -> Result<
        forge_query::facade::ForgeQueryWorkspaceLiveViewDeclaration,
        worth_schema::facade::WorthQueryDeclarationError,
    > = worth_persistent_name_live_view_declaration;
    let _: fn(
        &mut forge_query::facade::ForgeQueryWorkspace,
        String,
        &forge_query::facade::ForgeQueryLiveView<serde_json::Value>,
        &forge_query::facade::ForgeQueryLiveView<serde_json::Value>,
    ) -> Result<
        forge_query::facade::ForgeQueryDerivedViewHandle<serde_json::Value>,
        forge_query::facade::ForgeQueryRuntimeError,
    > = declare_worth_topology_materialized_surface::<
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
    > = declare_worth_topology_interpreted_surface::<serde_json::Value, serde_json::Value>;
    let _: fn(
        &mut forge_query::facade::ForgeQueryWorkspace,
        String,
        &forge_query::facade::ForgeQueryDerivedViewHandle<serde_json::Value>,
        &forge_query::facade::ForgeQueryDerivedViewHandle<serde_json::Value>,
    ) -> Result<
        forge_query::facade::ForgeQueryDerivedViewHandle<serde_json::Value>,
        forge_query::facade::ForgeQueryRuntimeError,
    > = declare_worth_topology_validation_surface::<
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
    > = declare_worth_topology_diagnostics_surface::<
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
    > = declare_worth_topology_equivalence_contract_surface::<serde_json::Value, serde_json::Value>;
    let _: fn(
        &[serde_json::Value],
    ) -> Result<
        worth_topo::facade::InterpretedTopologyView,
        worth_topo::facade::WorthTopologyQuerySurfaceError,
    > = interpreted_topology_from_materialized_rows;
    let _: fn(
        &[serde_json::Value],
        &[serde_json::Value],
    ) -> Result<
        worth_topo::facade::DerivedTopologyValidationReport,
        worth_topo::facade::WorthTopologyQuerySurfaceError,
    > = validation_report_from_query_rows;
    let _: fn(
        &forge_query::facade::ForgeQueryRetainedMutationContext,
        &[serde_json::Value],
        &[serde_json::Value],
        &[serde_json::Value],
    ) -> Result<
        worth_topo::facade::WorthDerivedReadDiagnostics,
        worth_topo::facade::WorthTopologyQuerySurfaceError,
    > = derived_read_diagnostics_from_query_rows;
    let _: fn(
        &[serde_json::Value],
    ) -> Result<
        worth_topo::facade::WorthDerivedEquivalenceContractReport,
        worth_topo::facade::WorthTopologyQuerySurfaceError,
    > = equivalence_contract_from_diagnostics_rows;
    let _: fn(
        &[forge_query::facade::ForgeQueryEntity],
        &[forge_query::facade::ForgeQueryEntity],
    ) -> Result<
        worth_topo::facade::WorthNamingAttachmentReport,
        worth_topo::facade::WorthTopologyQuerySurfaceError,
    > = naming_attachment_report_from_query_rows;
    let _: fn(
        &worth_schema::facade::DerivedTopologyReadBasis,
    ) -> worth_topo::facade::WorthTopologyQueryMutationEvidence =
        WorthTopologyQueryMutationEvidence::from_read_basis;
}

#[test]
fn worth_topo_public_traced_boundaries_compile_with_envelope_contracts() {
    let _ = _m1_read_cert_contract;
    let _ = _m1_commit_cert_contract;
    let _ = _m2_read_cert_contract;
    let _ = _m2_commit_cert_contract;
    let _ = _edit_apply_contract;
    let _ = _worth_vocab_live_query_declaration_contract;
    let _ = _worth_vocab_computed_query_declaration_contract;
    let _ = _worth_query_native_topology_surface_contracts;
}
