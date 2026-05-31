use forge_query::facade::{
    ForgeQueryApplicationFacade, ForgeQueryComputedBuilder, ForgeQueryLiveViewBuilder,
};
use forge_relational::facade::runtime::RelationalRuntime;
use forge_relational::facade::snapshots::SnapshotHandle;
use schema::facade::platform::authority::{MutationOrigin, RawTopologyIntent, TopologyMutation};
use schema::facade::topology_authoring::DerivedTopologyReadBasis;
use schema::facade::{QueryAspectPath, QueryCollection, QuerySchemaBasis};
use topology::facade::topology_runtime;
use topology::facade::BoundaryFailure;
use topology::facade::{
    build_topology_construction_fact_report, certify_milestone_one_read_basis_traced,
    certify_milestone_two_read_basis_traced, certify_milestone_two_verified_topology_commit_traced,
    certify_topology_query_boundary_cleanup_closeout, certify_verified_topology_commit_traced,
    declare_persistent_name_live_view, declare_topology_diagnostics_surface,
    declare_topology_entity_live_view, declare_topology_equivalence_contract_surface,
    declare_topology_interpreted_surface, declare_topology_materialized_surface,
    declare_topology_relation_live_view, declare_topology_validation_surface,
    lower_primitive_construction_birth_plan, naming_attachment_report_from_query_input,
    persistent_name_live_view_declaration, prepare_primitive_construction_certification,
    prepare_primitive_construction_execution, topology_construction_authority,
    MilestoneOneCertificationError, TopologyAttachBoundaryMembershipDeclaration,
    TopologyAttachShellOrWireMembershipDeclaration, TopologyCommittedArtifact,
    TopologyConstructionAuthority, TopologyConstructionCertificationPlan,
    TopologyConstructionCertificationReadSurface, TopologyConstructionExecutionError,
    TopologyConstructionExecutionPlan, TopologyConstructionFactKind,
    TopologyConstructionFactProvenance, TopologyConstructionFactReport,
    TopologyConstructionInspectionSurface, TopologyConstructionLoweringError,
    TopologyConstructionLoweringPlan, TopologyConstructionMutationSurface,
    TopologyCreateInnerLoopOnExistingFaceDeclaration, TopologyCreateTopologyEntityDeclaration,
    TopologyCurrentHeadReadHandleExt, TopologyCurrentHeadReadSession,
    TopologyDetachBoundaryMembershipDeclaration, TopologyDetachRadialAdjacencyDeclaration,
    TopologyDetachShellOrWireMembershipDeclaration, TopologyDomainQueryAggregateReport,
    TopologyDomainQueryCloseoutReport, TopologyDomainQueryCloseoutRow,
    TopologyDomainQueryCloseoutStatus, TopologyDomainQueryExecutionEngine,
    TopologyDomainQueryFallbackPosture, TopologyDomainQueryParityAggregateReport,
    TopologyDomainQueryPhaseThreeBlocker, TopologyDomainQueryPhaseThreeBlockerRow,
    TopologyDomainQueryPhaseThreeBlockerStatus, TopologyDomainQueryProofReport,
    TopologyDomainQueryRequestFamily, TopologyDomainQueryRequestReport,
    TopologyHalfEdgeRadialNeighborhoodView, TopologyHalfEdgeSharedVertexNeighborhoodView,
    TopologyLocalRewireNeighborhoodView, TopologyLoopCycleView, TopologyLoopSuccessorRewireMember,
    TopologyNamingAttachmentInput, TopologyNoNPlusOneContract, TopologyNoNPlusOneContractRow,
    TopologyNoNPlusOneContractStatus, TopologyQueryBoundaryCleanupArea,
    TopologyQueryBoundaryCleanupCloseoutReport, TopologyQueryBoundaryCleanupRow,
    TopologyQueryBoundaryCleanupStatus, TopologyQueryMutationEvidence,
    TopologyQueryMutationFamilySupportStatus, TopologyQueryMutationLane,
    TopologyQueryMutationLaneExecutionShape, TopologyQueryMutationLaneSupportStatus,
    TopologyQueryReadFamilySupportStatus, TopologyRadialSpliceMember,
    TopologyRehomeAllOwnedFacesToNewShellDeclaration,
    TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration, TopologyRetireTopologyEntityDeclaration,
    TopologyRewireLoopEndpointDeclaration, TopologyRewireLoopSuccessorProgramDeclaration,
    TopologyRuntimeAdapters, TopologyRuntimeCloseout, TopologyRuntimeCloseoutFamily,
    TopologyRuntimeCloseoutStatus, TopologyRuntimeFailure, TopologyRuntimeMutationFamilySupportRow,
    TopologyRuntimeMutationLaneSupportRow, TopologyRuntimePostureCapability,
    TopologyRuntimePostureRow, TopologyRuntimePostureStatus, TopologyRuntimeReadFamilySupportRow,
    TopologyRuntimeSupport, TopologyShellRehomeFaceMember, TopologySnapshotReadOnlyReadHandleExt,
    TopologySnapshotReadOnlyReadSession, TopologySpliceRadialAdjacencyDeclaration,
    TopologySpliceRadialAdjacencyProgramDeclaration,
    TopologySplitConnectedHalfEdgeSetToNewWireDeclaration,
    TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration, TopologyWireRehomeHalfEdgeMember,
    TopologyWireSplitHalfEdgeMember, TracedMilestoneOneCertificationReport,
    TracedMilestoneTwoDerivedReadReport,
};
use topology::query_domain::{
    topology_current_head_authoritative_context, topology_query_domain,
    topology_query_domain_entry, topology_query_domain_entry_checked,
    topology_query_domain_proof_root, topology_snapshot_read_only_context,
    TopologyCurrentHeadAuthoritativeContext, TopologyCurrentHeadConfiguredDomainHandle,
    TopologyCurrentHeadConfiguredDomainHandleChecked, TopologyQueryDomain,
    TopologySnapshotReadOnlyConfiguredDomainHandle,
    TopologySnapshotReadOnlyConfiguredDomainHandleChecked, TopologySnapshotReadOnlyContext,
};
use worth_spatial::facade::SpatialConstructionBirthPlan;

fn _m1_read_cert_contract(
    runtime: &mut RelationalRuntime,
    basis: DerivedTopologyReadBasis,
) -> Result<TracedMilestoneOneCertificationReport, BoundaryFailure<MilestoneOneCertificationError>>
{
    certify_milestone_one_read_basis_traced(runtime, basis)
}

fn _m1_commit_cert_contract(
    runtime: &mut RelationalRuntime,
    verified: &TopologyCommittedArtifact,
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
    verified: &TopologyCommittedArtifact,
) -> Result<TracedMilestoneTwoDerivedReadReport, BoundaryFailure<MilestoneOneCertificationError>> {
    certify_milestone_two_verified_topology_commit_traced(runtime, verified)
}

fn _committed_artifact_contracts(verified: &TopologyCommittedArtifact) {
    let _: fn(&TopologyCommittedArtifact) -> &[TopologyMutation] =
        TopologyCommittedArtifact::mutations;
    let _: fn(&TopologyCommittedArtifact) -> MutationOrigin =
        TopologyCommittedArtifact::mutation_origin;
    let _: fn(&TopologyCommittedArtifact) -> RawTopologyIntent =
        TopologyCommittedArtifact::raw_intent;
    let _: &[TopologyMutation] = verified.mutations();
}

fn _vocab_live_query_declaration_contract() {
    let _ = ForgeQueryLiveViewBuilder::surface(".topo.query.entities")
        .select([
            QueryAspectPath::TOPOLOGY_STRUCTURE.as_str(),
            QueryAspectPath::NAMING_PERSISTENT_NAME.as_str(),
        ])
        .from(QueryCollection::TopologyEntity.as_str())
        .schema_basis(QuerySchemaBasis::TopologyEntityLiveView.as_str())
        .build()
        .unwrap();
}

fn _vocab_computed_query_declaration_contract() {
    let _ = ForgeQueryComputedBuilder::surface(".topo.query.validation")
        .reads([QueryAspectPath::TOPOLOGY_STRUCTURE.as_str()])
        .produces([QueryAspectPath::DIAGNOSTICS_DECISIONS.as_str()])
        .build()
        .unwrap();
}

include!("query_domain/entry.rs");
include!("public_api_topology_operator_surface.rs");
include!("public_api_topology_operator_scalar_surface.rs");
include!("public_api_topology_operator_grouped_rehome_surface.rs");
include!("public_api_topology_operator_radial_program_surface.rs");
include!("public_api_topology_operator_successor_surface.rs");
include!("public_api_topology_operator_split_surface.rs");

fn _topology_projection_surface_contracts() {
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
        forge_query::facade::ForgeQueryRuntimeError,
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
        TopologyNamingAttachmentInput<'_>,
    ) -> Result<
        topology::facade::NamingAttachmentReport,
        topology::facade::TopologyQuerySurfaceError,
    > = naming_attachment_report_from_query_input;
    let _: fn(&DerivedTopologyReadBasis) -> topology::facade::TopologyQueryMutationEvidence =
        TopologyQueryMutationEvidence::from_read_basis;
    let _: fn() -> TopologyConstructionAuthority = topology_construction_authority;
    let _: fn(
        &SpatialConstructionBirthPlan,
    ) -> Result<TopologyConstructionLoweringPlan, TopologyConstructionLoweringError> =
        lower_primitive_construction_birth_plan;
    let _: fn(
        &TopologyConstructionLoweringPlan,
    )
        -> Result<TopologyConstructionExecutionPlan, TopologyConstructionExecutionError> =
        prepare_primitive_construction_execution;
    let _: fn(&TopologyConstructionExecutionPlan) -> TopologyConstructionCertificationPlan =
        prepare_primitive_construction_certification;
    let _: fn(
        &TopologyConstructionLoweringPlan,
        &TopologyConstructionCertificationPlan,
    ) -> TopologyConstructionFactReport = build_topology_construction_fact_report;
    let _: fn(TopologyConstructionMutationSurface) -> &'static str =
        TopologyConstructionMutationSurface::as_str;
    let _: fn(TopologyConstructionCertificationReadSurface) -> &'static str =
        TopologyConstructionCertificationReadSurface::as_str;
    let _: fn(TopologyConstructionInspectionSurface) -> &'static str =
        TopologyConstructionInspectionSurface::as_str;
    let _: fn(TopologyConstructionFactKind) -> &'static str = TopologyConstructionFactKind::as_str;
    let _: fn(TopologyConstructionFactProvenance) -> &'static str =
        TopologyConstructionFactProvenance::as_str;
    let _: fn() -> Result<
        TopologyQueryBoundaryCleanupCloseoutReport,
        topology::facade::TopologyCertificationError,
    > = certify_topology_query_boundary_cleanup_closeout;
    let _: fn(TopologyQueryBoundaryCleanupArea) -> &'static str =
        TopologyQueryBoundaryCleanupArea::as_str;
    let _: fn(TopologyQueryBoundaryCleanupStatus) -> &'static str =
        TopologyQueryBoundaryCleanupStatus::as_str;
    let _: fn(&TopologyQueryBoundaryCleanupCloseoutReport) -> &[TopologyQueryBoundaryCleanupRow] =
        TopologyQueryBoundaryCleanupCloseoutReport::rows;
    let _: fn(&TopologyQueryBoundaryCleanupCloseoutReport) -> bool =
        TopologyQueryBoundaryCleanupCloseoutReport::cleanup_complete;
    let _: fn(
        &TopologyQueryBoundaryCleanupCloseoutReport,
        TopologyQueryBoundaryCleanupArea,
    ) -> TopologyQueryBoundaryCleanupStatus = TopologyQueryBoundaryCleanupCloseoutReport::status;
    let _: fn(&TopologyQueryBoundaryCleanupRow) -> TopologyQueryBoundaryCleanupArea =
        TopologyQueryBoundaryCleanupRow::area;
    let _: fn(&TopologyQueryBoundaryCleanupRow) -> TopologyQueryBoundaryCleanupStatus =
        TopologyQueryBoundaryCleanupRow::status;
    let _: fn(&TopologyQueryBoundaryCleanupRow) -> &str = TopologyQueryBoundaryCleanupRow::reason;
}

#[test]
fn topo_public_traced_boundaries_compile_with_envelope_contracts() {
    let _ = _m1_read_cert_contract;
    let _ = _m1_commit_cert_contract;
    let _ = _m2_read_cert_contract;
    let _ = _m2_commit_cert_contract;
    let _ = _vocab_live_query_declaration_contract;
    let _ = _vocab_computed_query_declaration_contract;
    let _ = _topology_query_domain_entry_contracts;
    let _ = _topology_operator_surface_contracts;
    let _ = _topology_operator_scalar_surface_contracts;
    let _ = _topology_operator_grouped_rehome_surface_contracts;
    let _ = _topology_operator_radial_program_surface_contracts;
    let _ = _topology_operator_successor_surface_contracts;
    let _ = _topology_projection_surface_contracts;
}
