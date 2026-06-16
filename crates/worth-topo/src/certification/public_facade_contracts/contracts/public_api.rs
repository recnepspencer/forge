use forge_query::facade::{
    ForgeQueryApplicationFacade, ForgeQueryComputedBuilder,
    ForgeQueryContributionComposedClassification, ForgeQueryLiveViewBuilder,
    ForgeQuerySupportContributionAuthoring,
};
use forge_relational::facade::runtime::RelationalRuntime;
use forge_relational::facade::runtime::RelationalRuntimeBuilder;
use forge_relational::facade::snapshots::SnapshotHandle;
use schema::facade::topology_authoring::DerivedTopologyReadBasis;
use schema::facade::{QueryAspectPath, QueryCollection, QuerySchemaBasis};
use topology::certification::{
    build_milestone_one_runtime, certify_milestone_one_read_basis_traced,
    certify_milestone_two_read_basis_traced, certify_topology_bridge_registration_closeout,
    certify_topology_historical_materialization_closeout,
    certify_topology_query_boundary_cleanup_closeout, configure_milestone_one_runtime_builder,
    milestone_one_runtime_builder, BoundaryFailure, MilestoneOneCertificationError,
    MilestoneOneRuntimeSetupError, TopologyBridgeRegistrationArea,
    TopologyBridgeRegistrationCloseoutReport, TopologyBridgeRegistrationRow,
    TopologyBridgeRegistrationStatus, TopologyCertificationError,
    TopologyHistoricalMaterializationArea, TopologyHistoricalMaterializationCloseoutReport,
    TopologyHistoricalMaterializationRow, TopologyHistoricalMaterializationStatus,
    TopologyQueryBoundaryCleanupArea, TopologyQueryBoundaryCleanupCloseoutReport,
    TopologyQueryBoundaryCleanupRow, TopologyQueryBoundaryCleanupStatus,
    TracedMilestoneOneCertificationReport, TracedMilestoneTwoDerivedReadReport,
};
use topology::facade::{
    prepare_primitive_construction_query_admitted_handoff_from_synopsis,
    prepare_primitive_construction_query_envelope, prepare_primitive_construction_query_handoff,
    prepare_primitive_construction_query_receipt, topology_grouped_operator_neighborhood,
    topology_operator_continuation_target, topology_operator_contribution_workflow,
    topology_operator_signal_workflow, topology_runtime, NmtTopologyConstruction,
    NmtTopologyConstructionDenialClass, NmtTopologyConstructionReceipt, NmtTopologyPosture,
    OpenLayerPattern, OpenLayerStackSpec, OpenRadialFanSpec, OpenSheetPatchSpec, OpenWireChainSpec,
    TopologyAttachBoundaryMembershipDeclaration, TopologyAttachShellOrWireMembershipDeclaration,
    TopologyConstructionQueryAdmittedHandoffError, TopologyConstructionQueryEnvelopeError,
    TopologyConstructionQueryFactKind, TopologyConstructionQueryFactProvenance,
    TopologyConstructionQueryFactRow, TopologyConstructionQueryHandoffError,
    TopologyConstructionQueryInspectionSurface, TopologyConstructionQueryMutationSurface,
    TopologyConstructionQueryReadSurface, TopologyConstructionQueryReceiptError,
    TopologyCreateInnerLoopOnExistingFaceDeclaration, TopologyCreateTopologyEntityDeclaration,
    TopologyDetachBoundaryMembershipDeclaration, TopologyDetachRadialAdjacencyDeclaration,
    TopologyDetachShellOrWireMembershipDeclaration, TopologyLoopSuccessorRewireMember,
    TopologyOperatorCanonicalDeclaration, TopologyOperatorContinuationExecution,
    TopologyOperatorContinuationExecutionChecked, TopologyOperatorContinuationExecutionOutcome,
    TopologyOperatorContinuationExecutionProof, TopologyOperatorContinuationTarget,
    TopologyOperatorContributionArtifact, TopologyOperatorContributionChecked,
    TopologyOperatorContributionCheckedOutcome, TopologyOperatorContributionInput,
    TopologyOperatorContributionIntent, TopologyOperatorContributionOutcome,
    TopologyOperatorContributionProof, TopologyOperatorDeclarationAdmissionError,
    TopologyOperatorDeclarationLegalityDenial, TopologyOperatorDeclarationLegalityEvidence,
    TopologyOperatorDeclarationOutcome, TopologyOperatorDeclarationReceipt,
    TopologyOperatorDeclarationReceiptChecked, TopologyOperatorDeclarationReceiptProof,
    TopologyOperatorDeclarationReceiptTerminalError, TopologyOperatorEnvelope,
    TopologyOperatorEnvelopeChecked, TopologyOperatorEnvelopeFromProgressedChecked,
    TopologyOperatorEnvelopeFromProgressedProof,
    TopologyOperatorEnvelopeFromProgressedTerminalError, TopologyOperatorEnvelopeProof,
    TopologyOperatorEnvelopeTerminalError, TopologyOperatorGroupedContributionComposition,
    TopologyOperatorGroupedContributionInput, TopologyOperatorGroupedContributionMemberContext,
    TopologyOperatorGroupedContributionStop, TopologyOperatorGroupedDeclaration,
    TopologyOperatorGroupedDeclarationStop, TopologyOperatorGroupedInput,
    TopologyOperatorGroupedOutcome, TopologyOperatorPreparedContinuation,
    TopologyOperatorPreparedContinuationChecked, TopologyOperatorPreparedContinuationOutcome,
    TopologyOperatorPreparedContinuationProof, TopologyOperatorProgressedDeclaration,
    TopologyOperatorProgressionError, TopologyOperatorRoutePlan, TopologyOperatorRoutePlanChecked,
    TopologyOperatorRoutePlanProof, TopologyOperatorRoutePlanTerminalError,
    TopologyOperatorSignalCompatibilityArtifact, TopologyOperatorSignalCompatibilityChecked,
    TopologyOperatorSignalCompatibilityInput, TopologyOperatorSignalCompatibilityOutcome,
    TopologyOperatorSignalCompatibilityProof, TopologyOperatorSignalCompatibilitySubject,
    TopologyOperatorWorkflowHandleExt, TopologyPrimitiveConstructionQueryAdmittedHandoff,
    TopologyPrimitiveConstructionQueryBirthSynopsis, TopologyPrimitiveConstructionQueryEnvelope,
    TopologyPrimitiveConstructionQueryHandoff, TopologyPrimitiveConstructionQueryReceipt,
    TopologyRadialSpliceMember, TopologyRehomeAllOwnedFacesToNewShellDeclaration,
    TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration, TopologyRetireTopologyEntityDeclaration,
    TopologyRewireLoopEndpointDeclaration, TopologyRewireLoopSuccessorProgramDeclaration,
    TopologyRuntimeAdapters, TopologyRuntimeFailure, TopologySeed, TopologySeedCleanFailClass,
    TopologySeedCleanFailReasonCode, TopologySeedCleanFailReceipt, TopologySeedCleanFailStage,
    TopologySeedCounters, TopologySeedEntityIdentities, TopologySeedKind,
    TopologySeedNeighborhoodReceipt, TopologySeedQueryReceipts, TopologySeedReceipt,
    TopologySeedRecipe, TopologySeedTopologyPosture, TopologySeedValidationReceipt,
    TopologyShellRehomeFaceMember, TopologySpliceRadialAdjacencyDeclaration,
    TopologySpliceRadialAdjacencyProgramDeclaration,
    TopologySplitConnectedHalfEdgeSetToNewWireDeclaration,
    TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration, TopologyWireRehomeHalfEdgeMember,
    TopologyWireSplitHalfEdgeMember, TopologyWorkload, TopologyWorkloadFamily,
    TopologyWorkloadSupport, TopologyWorkloadSupportPosture,
};
use topology::query_domain::{
    topology_current_head_authoritative_context, topology_query_domain,
    topology_query_domain_entry, topology_query_domain_entry_checked,
    topology_query_domain_proof_root, topology_snapshot_read_only_context,
    TopologyCurrentHeadAuthoritativeContext, TopologyCurrentHeadConfiguredDomainHandle,
    TopologyCurrentHeadConfiguredDomainHandleChecked, TopologyCurrentHeadReadHandleExt,
    TopologyCurrentHeadReadSession, TopologyHalfEdgeRadialNeighborhoodView,
    TopologyHalfEdgeSharedVertexNeighborhoodView, TopologyLocalRewireNeighborhoodView,
    TopologyLoopCycleView, TopologyNoNPlusOneContract, TopologyNoNPlusOneContractRow,
    TopologyNoNPlusOneContractStatus, TopologyQueryDomain, TopologyReadAggregateReport,
    TopologyReadCloseoutReport, TopologyReadCloseoutRow, TopologyReadCloseoutStatus,
    TopologyReadExecutionEngine, TopologyReadFallbackPosture, TopologyReadParityAggregateReport,
    TopologyReadPhaseThreeBlocker, TopologyReadPhaseThreeBlockerRow,
    TopologyReadPhaseThreeBlockerStatus, TopologyReadProofReport, TopologyReadRequestFamily,
    TopologyReadRequestReport, TopologySnapshotReadOnlyConfiguredDomainHandle,
    TopologySnapshotReadOnlyConfiguredDomainHandleChecked, TopologySnapshotReadOnlyContext,
    TopologySnapshotReadOnlyReadHandleExt, TopologySnapshotReadOnlyReadSession,
};
use topology::runtime_support::{
    TopologyQueryMutationFamilySupportStatus, TopologyQueryMutationLane,
    TopologyQueryMutationLaneExecutionShape, TopologyQueryMutationLaneSupportStatus,
    TopologyQueryReadFamilySupportStatus, TopologyRuntimeCloseout, TopologyRuntimeCloseoutFamily,
    TopologyRuntimeCloseoutStatus, TopologyRuntimeMutationFamilySupportRow,
    TopologyRuntimeMutationLaneSupportRow, TopologyRuntimePostureCapability,
    TopologyRuntimePostureRow, TopologyRuntimePostureStatus, TopologyRuntimeReadFamilySupportRow,
    TopologyRuntimeSupport,
};

fn _m1_read_cert_contract(
    runtime: &mut RelationalRuntime,
    basis: DerivedTopologyReadBasis,
) -> Result<TracedMilestoneOneCertificationReport, BoundaryFailure<MilestoneOneCertificationError>>
{
    certify_milestone_one_read_basis_traced(runtime, basis)
}

fn _m2_read_cert_contract(
    runtime: &mut RelationalRuntime,
    basis: DerivedTopologyReadBasis,
) -> Result<TracedMilestoneTwoDerivedReadReport, BoundaryFailure<MilestoneOneCertificationError>> {
    certify_milestone_two_read_basis_traced(runtime, basis)
}

fn _topology_construction_query_receipt_contract(
    synopsis: &TopologyPrimitiveConstructionQueryBirthSynopsis,
) -> Result<TopologyPrimitiveConstructionQueryReceipt, TopologyConstructionQueryReceiptError> {
    prepare_primitive_construction_query_receipt(synopsis)
}

fn _topology_construction_query_envelope_contract(
    synopsis: &TopologyPrimitiveConstructionQueryBirthSynopsis,
) -> Result<TopologyPrimitiveConstructionQueryEnvelope, TopologyConstructionQueryEnvelopeError> {
    prepare_primitive_construction_query_envelope(synopsis)
}

fn _topology_construction_query_handoff_contract(
    synopsis: &TopologyPrimitiveConstructionQueryBirthSynopsis,
) -> Result<TopologyPrimitiveConstructionQueryHandoff, TopologyConstructionQueryHandoffError> {
    prepare_primitive_construction_query_handoff(synopsis)
}

fn _topology_construction_query_admitted_handoff_contract(
    synopsis: &TopologyPrimitiveConstructionQueryBirthSynopsis,
) -> Result<
    TopologyPrimitiveConstructionQueryAdmittedHandoff,
    TopologyConstructionQueryAdmittedHandoffError,
> {
    prepare_primitive_construction_query_admitted_handoff_from_synopsis(
        synopsis,
        "completeness",
        "mapping",
        1,
        1,
    )
}

fn _topology_construction_query_surface_vocab_contract(
    row: &TopologyConstructionQueryFactRow,
) -> (
    TopologyConstructionQueryMutationSurface,
    TopologyConstructionQueryReadSurface,
    TopologyConstructionQueryInspectionSurface,
    TopologyConstructionQueryFactProvenance,
    TopologyConstructionQueryFactKind,
) {
    (
        TopologyConstructionQueryMutationSurface::ComposeGraph,
        TopologyConstructionQueryReadSurface::ProjectionConsumptionFromInspectionReceipt,
        TopologyConstructionQueryInspectionSurface::InspectReceipt,
        TopologyConstructionQueryFactProvenance::InspectionBackedProjectionConsumption,
        row.kind(),
    )
}

fn _milestone_one_runtime_builder_contract(
) -> Result<RelationalRuntimeBuilder, MilestoneOneRuntimeSetupError> {
    milestone_one_runtime_builder()
}

fn _build_milestone_one_runtime_contract(
) -> Result<RelationalRuntime, MilestoneOneRuntimeSetupError> {
    build_milestone_one_runtime()
}

fn _configure_milestone_one_runtime_builder_contract(
    builder: RelationalRuntimeBuilder,
) -> Result<RelationalRuntimeBuilder, MilestoneOneRuntimeSetupError> {
    configure_milestone_one_runtime_builder(builder)
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

fn _topology_workload_vocabulary_contract() {
    let receipt = TopologyWorkload::declared("topology seed")
        .from_query_declaration(".topology.seed")
        .unwrap();

    assert_eq!(receipt.identity().name(), "topology seed");
    assert_eq!(receipt.envelope().counters().support_rows(), 1);
    assert!(receipt
        .envelope()
        .support_posture()
        .reason()
        .contains("admitted"));

    let unsupported = TopologyWorkloadSupportPosture::unsupported(
        TopologyWorkloadFamily::PrimitiveCorpus,
        "primitive corpus workload is not supported by this runtime",
    );
    let blocked = TopologyWorkloadSupportPosture::blocked(
        TopologyWorkloadFamily::OperatorTopology,
        "operator topology workload is blocked until declaration evidence exists",
    );

    assert_eq!(unsupported.support(), TopologyWorkloadSupport::Unsupported);
    assert_eq!(blocked.support(), TopologyWorkloadSupport::Blocked);
    assert!(unsupported.reason().contains("not supported"));
    assert!(blocked.reason().contains("blocked"));

    let missing_reason =
        TopologyWorkloadSupportPosture::blocked(TopologyWorkloadFamily::OperatorTopology, "");
    assert!(missing_reason.reason().contains("human-readable reason"));
}

fn _topology_seed_public_facade_contract(
    recipe: TopologySeedRecipe,
) -> Result<TopologySeedReceipt, TopologySeedCleanFailReceipt> {
    let _: fn() -> TopologySeedRecipe = TopologySeed::cube;
    let _: fn() -> TopologySeedRecipe = TopologySeed::tetrahedron;
    let _: fn(usize) -> TopologySeedRecipe = TopologySeed::single_face_loop;
    let _: fn(usize) -> TopologySeedRecipe = TopologySeed::multi_face_shell;
    let _: fn() -> TopologySeedRecipe = TopologySeed::open_sheet;
    let _: fn() -> TopologySeedRecipe = TopologySeed::open_wire;
    let _: fn(usize) -> TopologySeedRecipe = TopologySeed::open_shell_nmt_edge_fan;
    let _: fn() -> TopologySeedRecipe = TopologySeed::high_valence_vertex;
    let _: fn() -> TopologySeedRecipe = TopologySeed::self_intersecting_loop;
    let _: fn() -> TopologySeedRecipe = TopologySeed::non_manifold_wire;
    let _: fn() -> TopologySeedRecipe = TopologySeed::thin_wall_local_basis;
    let _: fn() -> TopologySeedRecipe = TopologySeed::orientation_inconsistency;

    let _: TopologySeedKind = TopologySeedKind::Cube;
    let _: TopologySeedKind = TopologySeedKind::ThinWallLocalBasis;
    let _: TopologySeedKind = TopologySeedKind::OrientationInconsistency;
    let _: TopologySeedTopologyPosture = TopologySeedTopologyPosture::ClosedValid;
    let _: TopologySeedCleanFailStage = TopologySeedCleanFailStage::ParameterAdmission;
    let _: TopologySeedCleanFailClass = TopologySeedCleanFailClass::DirtyTopology;
    let _: TopologySeedCleanFailReasonCode =
        TopologySeedCleanFailReasonCode::TopologyValidationRejectedSeed;
    let _: Option<TopologySeedCounters> = None;
    let _: Option<TopologySeedEntityIdentities> = None;
    let _: Option<TopologySeedNeighborhoodReceipt> = None;
    let _: Option<TopologySeedQueryReceipts> = None;
    let _: Option<TopologySeedValidationReceipt> = None;

    recipe.build()
}

include!("nmt_topology_construction.rs");
include!("query_domain/entry.rs");
include!("public_api_topology_operator_surface.rs");
include!("public_api_topology_operator_scalar_surface.rs");
include!("public_api_topology_operator_grouped_rehome_surface.rs");
include!("public_api_topology_operator_radial_program_surface.rs");
include!("public_api_topology_operator_successor_surface.rs");
include!("public_api_topology_operator_split_surface.rs");


fn _topology_projection_cleanup_closeout_contracts() {
    let _: fn() -> Result<TopologyQueryBoundaryCleanupCloseoutReport, TopologyCertificationError> =
        certify_topology_query_boundary_cleanup_closeout;
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

    let _: fn() -> Result<TopologyBridgeRegistrationCloseoutReport, TopologyCertificationError> =
        certify_topology_bridge_registration_closeout;
    let _: fn(TopologyBridgeRegistrationArea) -> &'static str =
        TopologyBridgeRegistrationArea::as_str;
    let _: fn(TopologyBridgeRegistrationStatus) -> &'static str =
        TopologyBridgeRegistrationStatus::as_str;
    let _: fn(&TopologyBridgeRegistrationCloseoutReport) -> &[TopologyBridgeRegistrationRow] =
        TopologyBridgeRegistrationCloseoutReport::rows;
    let _: fn(&TopologyBridgeRegistrationCloseoutReport) -> bool =
        TopologyBridgeRegistrationCloseoutReport::phase_eight_ready;
    let _: fn(
        &TopologyBridgeRegistrationCloseoutReport,
        TopologyBridgeRegistrationArea,
    ) -> TopologyBridgeRegistrationStatus = TopologyBridgeRegistrationCloseoutReport::status;
    let _: fn(&TopologyBridgeRegistrationRow) -> TopologyBridgeRegistrationArea =
        TopologyBridgeRegistrationRow::area;
    let _: fn(&TopologyBridgeRegistrationRow) -> TopologyBridgeRegistrationStatus =
        TopologyBridgeRegistrationRow::status;
    let _: fn(&TopologyBridgeRegistrationRow) -> &str = TopologyBridgeRegistrationRow::reason;

    let _: fn() -> Result<
        TopologyHistoricalMaterializationCloseoutReport,
        TopologyCertificationError,
    > = certify_topology_historical_materialization_closeout;
    let _: fn(TopologyHistoricalMaterializationArea) -> &'static str =
        TopologyHistoricalMaterializationArea::as_str;
    let _: fn(TopologyHistoricalMaterializationStatus) -> &'static str =
        TopologyHistoricalMaterializationStatus::as_str;
    let _: fn(
        &TopologyHistoricalMaterializationCloseoutReport,
    ) -> &[TopologyHistoricalMaterializationRow] =
        TopologyHistoricalMaterializationCloseoutReport::rows;
    let _: fn(&TopologyHistoricalMaterializationCloseoutReport) -> bool =
        TopologyHistoricalMaterializationCloseoutReport::phase_seven_ready;
    let _: fn(
        &TopologyHistoricalMaterializationCloseoutReport,
        TopologyHistoricalMaterializationArea,
    ) -> TopologyHistoricalMaterializationStatus =
        TopologyHistoricalMaterializationCloseoutReport::status;
    let _: fn(&TopologyHistoricalMaterializationRow) -> TopologyHistoricalMaterializationArea =
        TopologyHistoricalMaterializationRow::area;
    let _: fn(&TopologyHistoricalMaterializationRow) -> TopologyHistoricalMaterializationStatus =
        TopologyHistoricalMaterializationRow::status;
    let _: fn(&TopologyHistoricalMaterializationRow) -> &str =
        TopologyHistoricalMaterializationRow::reason;
}

#[test]
fn topo_public_traced_boundaries_compile_with_envelope_contracts() {
    let _ = _m1_read_cert_contract;
    let _ = _m2_read_cert_contract;
    let _ = _vocab_live_query_declaration_contract;
    let _ = _vocab_computed_query_declaration_contract;
    _topology_workload_vocabulary_contract();
    _nmt_topology_construction_public_facade_contract()
        .expect("NMT topology construction public facade contract");
    let _ = _topology_query_domain_entry_contracts;
    let _ = _topology_operator_surface_contracts;
    let _ = _topology_operator_scalar_surface_contracts;
    let _ = _topology_operator_grouped_rehome_surface_contracts;
    let _ = _topology_operator_radial_program_surface_contracts;
    let _ = _topology_operator_successor_surface_contracts;
    let _ = _build_milestone_one_runtime_contract;
    let _ = _configure_milestone_one_runtime_builder_contract;
    let _ = _topology_projection_cleanup_closeout_contracts;
}
