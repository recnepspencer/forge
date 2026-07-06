#[path = "../../certification/public_facade_contracts/contracts/public_api_planar_boolean_loop_reconstruction_workload_evidence_support.rs"]
mod workload_evidence_support;

use std::sync::OnceLock;

use topology::facade::{
    admit_milestone_seven_five_overlap_readiness_consumer, PlanarBooleanOverlapBlueprintRegistry,
    TopologyMilestoneSevenFiveOverlapReadinessConsumer,
};
use worth_spatial::facade::planar_boolean_loop_reconstruction::PlanarBooleanLoopReconstructionParticipationSupport;
use worth_spatial::facade::planar_boolean_overlap_region_extraction::{
    PlanarBooleanBoundaryContactClassificationBundle, PlanarBooleanCoplanarOverlapArrangementGraph,
    PlanarBooleanOverlapAdjacencyIndexInput, PlanarBooleanOverlapArrangementGraphInput,
    PlanarBooleanOverlapCellContainmentInput, PlanarBooleanOverlapCellContainmentMap,
    PlanarBooleanOverlapCellWindingField, PlanarBooleanOverlapCellWindingFieldInput,
    PlanarBooleanOverlapIslandCandidateInput, PlanarBooleanOverlapIslandComponentBundle,
    PlanarBooleanOverlapParticipationRecovery, PlanarBooleanOverlapParticipationRecoveryInput,
    PlanarBooleanOverlapReadinessLoopLedgerBinding, PlanarBooleanOverlapRegionAdjacencyIndex,
    PlanarBooleanOverlapRegionCanonicalWindingSourceKind,
    PlanarBooleanOverlapRegionExtractionRequest, PlanarBooleanOverlapRegionExtractionRequestInput,
    PlanarBooleanOverlapRegionLedgerAssemblyBundle, PlanarBooleanPreRegionNormalizationBundle,
    PlanarBooleanSharedAreaAdmissionBundle,
};
use worth_spatial::facade::retained_replay_workload::ReplayReceiptSet;

use crate::workload_composition::{
    admitted_metaboss_bundle_operand_pair_recipe, current_touched_graph_readiness_handoff,
    trace_scope, CompletedBooleanLoopReconstructionHandoff,
    CompletedPlanarBooleanOverlapRegionExtractionHandoff, PlanarBooleanOverlapRegionCloseoutInput,
    PlanarBooleanOverlapRegionMetabossSubcase,
};

pub(crate) struct RealOverlapOwnerSeamFixture {
    pub(crate) readiness: schema::facade::platform::authority::touched_graph_parity_closeout::TouchedGraphParityReadinessInput,
    pub(crate) readiness_consumer: TopologyMilestoneSevenFiveOverlapReadinessConsumer,
    pub(crate) request: PlanarBooleanOverlapRegionExtractionRequest,
    pub(crate) ledger_bundle: PlanarBooleanOverlapRegionLedgerAssemblyBundle,
    pub(crate) stage_counts: OverlapExtractionStageCounts,
    pub(crate) completed: CompletedPlanarBooleanOverlapRegionExtractionHandoff,
    pub(crate) replay_receipts: ReplayReceiptSet,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct OverlapExtractionStageCounts {
    source_overlap_lineage_rows: usize,
    recovered_overlap_lineage_rows: usize,
    adjacency_rows: usize,
    arrangement_cells: usize,
    boundary_contact_components: usize,
    area_overlap_components: usize,
    shared_area_rows: usize,
    mixed_boundary_area_rows: usize,
    pure_boundary_only_rows: usize,
    boundary_only_overlap_rows: usize,
    canonical_boundary_rows: usize,
}

pub(crate) struct OverlapExtractionAuthorityProducts {
    pub(crate) request: PlanarBooleanOverlapRegionExtractionRequest,
    pub(crate) shared_area_bundle: PlanarBooleanSharedAreaAdmissionBundle,
    pub(crate) canonical_winding_bundle: worth_spatial::facade::planar_boolean_overlap_region_extraction::PlanarBooleanPostAdmissionNormalizationBundle,
    pub(crate) ledger_bundle: PlanarBooleanOverlapRegionLedgerAssemblyBundle,
    pub(crate) stage_counts: OverlapExtractionStageCounts,
}

pub(crate) fn completed_overlap_owner_seam_fixture(
    label: &'static str,
) -> &'static RealOverlapOwnerSeamFixture {
    if label.contains("foreign ledger") {
        return foreign_overlap_owner_seam_fixture();
    }
    primary_overlap_owner_seam_fixture()
}

pub(crate) fn completed_overlap_owner_seam_fixture_for_subcase(
    subcase: PlanarBooleanOverlapRegionMetabossSubcase,
) -> &'static RealOverlapOwnerSeamFixture {
    match subcase {
        PlanarBooleanOverlapRegionMetabossSubcase::BoundaryOnlyCoincidentEdgesDoNotAdmitArea => {
            boundary_only_overlap_owner_seam_fixture()
        }
        PlanarBooleanOverlapRegionMetabossSubcase::MixedBoundaryAndAreaContactDoesNotCollapse => {
            mixed_boundary_area_overlap_owner_seam_fixture()
        }
        PlanarBooleanOverlapRegionMetabossSubcase::OverlapStormUsesIndexNotPairwiseRediscovery => {
            coplanar_overlap_owner_seam_fixture()
        }
        _ => primary_overlap_owner_seam_fixture(),
    }
}

pub(crate) fn run_stack_heavy_overlap_region_test(test: impl FnOnce() + Send + 'static) {
    let result = std::thread::Builder::new()
        .stack_size(256 * 1024 * 1024)
        .spawn(test)
        .expect("overlap-region closeout test should spawn on a larger stack")
        .join();
    if let Err(panic_payload) = result {
        std::panic::resume_unwind(panic_payload);
    }
}

fn primary_overlap_owner_seam_fixture() -> &'static RealOverlapOwnerSeamFixture {
    static FIXTURE: OnceLock<RealOverlapOwnerSeamFixture> = OnceLock::new();
    FIXTURE.get_or_init(|| {
        build_completed_overlap_owner_seam_fixture_with_pair(
            "phase7.5 overlap region summum bonum",
            admitted_metaboss_bundle_operand_pair_recipe("phase7.5 overlap region summum bonum"),
        )
    })
}

fn boundary_only_overlap_owner_seam_fixture() -> &'static RealOverlapOwnerSeamFixture {
    static FIXTURE: OnceLock<RealOverlapOwnerSeamFixture> = OnceLock::new();
    FIXTURE.get_or_init(|| {
        build_completed_overlap_owner_seam_fixture_with_pair(
            "phase7.5 overlap region boundary-only hostile",
            PlanarBooleanOverlapRegionMetabossSubcase::BoundaryOnlyCoincidentEdgesDoNotAdmitArea
                .admitted_operand_pair_recipe("phase7.5 overlap region boundary-only hostile")
                .expect("boundary-only hostile pair should exist"),
        )
    })
}

fn foreign_overlap_owner_seam_fixture() -> &'static RealOverlapOwnerSeamFixture {
    static FIXTURE: OnceLock<RealOverlapOwnerSeamFixture> = OnceLock::new();
    FIXTURE.get_or_init(|| {
        build_completed_overlap_owner_seam_fixture_with_pair(
            "phase7.5 overlap region summum bonum foreign ledger",
            admitted_metaboss_bundle_operand_pair_recipe(
                "phase7.5 overlap region summum bonum foreign ledger",
            ),
        )
    })
}

fn coplanar_overlap_owner_seam_fixture() -> &'static RealOverlapOwnerSeamFixture {
    static FIXTURE: OnceLock<RealOverlapOwnerSeamFixture> = OnceLock::new();
    FIXTURE.get_or_init(|| {
        build_completed_overlap_owner_seam_fixture_with_pair(
            "phase7.5 overlap region coplanar overlap hostile",
            PlanarBooleanOverlapRegionMetabossSubcase::OverlapStormUsesIndexNotPairwiseRediscovery
                .admitted_operand_pair_recipe("phase7.5 overlap region coplanar overlap hostile")
                .expect("overlap storm hostile pair should exist"),
        )
    })
}

fn mixed_boundary_area_overlap_owner_seam_fixture() -> &'static RealOverlapOwnerSeamFixture {
    static FIXTURE: OnceLock<RealOverlapOwnerSeamFixture> = OnceLock::new();
    FIXTURE.get_or_init(|| {
        build_completed_overlap_owner_seam_fixture_with_pair(
            "phase7.5 overlap region mixed boundary-area hostile",
            PlanarBooleanOverlapRegionMetabossSubcase::MixedBoundaryAndAreaContactDoesNotCollapse
                .admitted_operand_pair_recipe("phase7.5 overlap region mixed boundary-area hostile")
                .expect("mixed boundary-area hostile pair should exist"),
        )
    })
}

fn build_completed_overlap_owner_seam_fixture_with_pair(
    label: &'static str,
    pair_recipe: crate::workload_composition::WorkloadCatalogBooleanOperandPairRecipe,
) -> RealOverlapOwnerSeamFixture {
    trace_scope(
        "build_completed_overlap_owner_seam_fixture_with_pair",
        || {
            let pair = pair_recipe
                .build()
                .expect("hostile overlap operand pair should build");
            let loop_replay_chain =
                trace_scope("overlap_fixture_loop_replay_chain_for_pair", || {
                    workload_evidence_support::certified_loop_replay_closeout_chain_for_pair(
                        label, pair,
                    )
                });
            let readiness = trace_scope("overlap_fixture_readiness_handoff", || {
                current_touched_graph_readiness_handoff().expect("readiness handoff")
            });
            let readiness_consumer = trace_scope("overlap_fixture_readiness_consumer", || {
                admit_milestone_seven_five_overlap_readiness_consumer(&readiness).expect("consumer")
            });
            let overlap_products = trace_scope("overlap_fixture_request_and_ledger", || {
                overlap_request_and_ledger(&loop_replay_chain.original)
            });
            let overlap_registry = PlanarBooleanOverlapBlueprintRegistry::phase_2();
            let completed = trace_scope("overlap_fixture_closeout", || {
                loop_replay_chain
                .original
                .complete_planar_boolean_overlap_region_extraction(
                    PlanarBooleanOverlapRegionCloseoutInput::new(
                        &readiness,
                        &readiness_consumer,
                        &overlap_products.request,
                        &overlap_products.shared_area_bundle,
                        &overlap_products.canonical_winding_bundle,
                        &overlap_products.ledger_bundle,
                        &loop_replay_chain.replayed,
                        &loop_replay_chain.replay_receipts,
                        &overlap_registry.operator_classification_matrix(),
                        &overlap_registry.validator_registration_plan(),
                    ),
                )
                .unwrap_or_else(|error| {
                    panic!(
                        "overlap closeout should certify through the real owner seam; stage_counts={:?}; error={:?}",
                        overlap_products.stage_counts, error
                    )
                })
            });
            RealOverlapOwnerSeamFixture {
                readiness,
                readiness_consumer,
                request: overlap_products.request,
                ledger_bundle: overlap_products.ledger_bundle,
                stage_counts: overlap_products.stage_counts,
                completed,
                replay_receipts: loop_replay_chain.replay_receipts,
            }
        },
    )
}

pub(crate) fn overlap_request_and_ledger(
    loop_handoff: &CompletedBooleanLoopReconstructionHandoff,
) -> OverlapExtractionAuthorityProducts {
    let readiness = current_touched_graph_readiness_handoff().expect("readiness handoff");
    let readiness_consumer =
        admit_milestone_seven_five_overlap_readiness_consumer(&readiness).expect("consumer");
    let request = PlanarBooleanOverlapRegionExtractionRequest::admit(
        PlanarBooleanOverlapRegionExtractionRequestInput::from_readiness_consumer_and_loop_ledger(
            &readiness_consumer,
            loop_handoff.loop_ledger_receipt(),
        ),
    )
    .expect("overlap request");
    let loop_products = loop_handoff
        .products()
        .expect("real loop handoff should retain canonical phase products");
    let source_overlap_lineage_rows = loop_products
        .source_provenance()
        .overlap_chain_lineage_map()
        .rows()
        .len();
    trace_overlap_stage_count("source_overlap_lineage_rows", source_overlap_lineage_rows);
    let support =
        PlanarBooleanLoopReconstructionParticipationSupport::admit_from_ledger_and_products(
            loop_products.loop_ledger(),
            loop_products.role_outcomes(),
            loop_products.island_partition(),
            loop_products.persistent_name_propagation_map(),
            loop_products.source_provenance().fragment_membership_map(),
            loop_products
                .source_provenance()
                .overlap_chain_lineage_map(),
            loop_products.source_provenance().source_loop_carriers(),
        )
        .expect("participation support");
    let participation = PlanarBooleanOverlapParticipationRecovery::recover(
        PlanarBooleanOverlapParticipationRecoveryInput::from_request_and_loop_support(
            &request, &support,
        ),
    )
    .expect("real loop handoff should recover overlap participation from carried 7.4 provenance");
    let recovered_overlap_lineage_rows = participation.chain_lineage_map().rows().len();
    trace_overlap_stage_count(
        "recovered_overlap_lineage_rows",
        recovered_overlap_lineage_rows,
    );
    let adjacency = PlanarBooleanOverlapRegionAdjacencyIndex::admit(
        PlanarBooleanOverlapAdjacencyIndexInput::from_participation_products(
            participation.loop_participation_map(),
            participation.island_participation_map(),
            participation.chain_lineage_map(),
        ),
    )
    .expect("adjacency");
    let adjacency_rows = adjacency.rows().len();
    trace_overlap_stage_count("adjacency_rows", adjacency_rows);
    let arrangement = PlanarBooleanCoplanarOverlapArrangementGraph::admit(
        PlanarBooleanOverlapArrangementGraphInput::from_adjacency(
            &adjacency,
            adjacency.ordering_basis(),
        ),
    )
    .expect("arrangement");
    let arrangement_cells = arrangement.cell_set().cells().len();
    trace_overlap_stage_count("arrangement_cells", arrangement_cells);
    let (shared_area, boundary_contact_components, area_overlap_components) =
        shared_area_bundle_from_arrangement(&arrangement);
    trace_overlap_stage_count("boundary_contact_components", boundary_contact_components);
    trace_overlap_stage_count("area_overlap_components", area_overlap_components);
    let shared_area_rows = shared_area.shared_area_admission_outcomes().rows().len();
    trace_overlap_stage_count("shared_area_rows", shared_area_rows);
    let mixed_boundary_area_rows = shared_area.mixed_boundary_area_outcomes().rows().len();
    trace_overlap_stage_count("mixed_boundary_area_rows", mixed_boundary_area_rows);
    let pure_boundary_only_rows = shared_area.pure_boundary_only_outcomes().rows().len();
    trace_overlap_stage_count("pure_boundary_only_rows", pure_boundary_only_rows);
    let pre_region = PlanarBooleanPreRegionNormalizationBundle::from_shared_area_admission(
        &shared_area,
        participation.chain_lineage_map(),
    )
    .expect("pre-region normalization");
    let candidate_bundle = pre_region
        .promote_overlap_region_candidates(&shared_area)
        .expect("candidate promotion");
    let boundary_only_overlap_rows = candidate_bundle
        .boundary_only_overlap_outcomes()
        .rows()
        .len();
    trace_overlap_stage_count("boundary_only_overlap_rows", boundary_only_overlap_rows);
    let canonical_bundle = candidate_bundle
        .normalize_post_admission_canonical_winding()
        .expect("canonical winding");
    let canonical_boundary_rows = canonical_bundle
        .overlap_region_canonical_winding()
        .rows()
        .iter()
        .filter(|row| {
            row.source_kind()
                == PlanarBooleanOverlapRegionCanonicalWindingSourceKind::BoundaryOnlyOutcome
        })
        .count();
    trace_overlap_stage_count("canonical_boundary_rows", canonical_boundary_rows);
    let identity_lineage = canonical_bundle
        .mint_overlap_region_identity_lineage()
        .expect("identity lineage");

    OverlapExtractionAuthorityProducts {
        request,
        shared_area_bundle: shared_area,
        canonical_winding_bundle: canonical_bundle.clone(),
        ledger_bundle: identity_lineage
            .mint_overlap_region_ledger()
            .expect("ledger"),
        stage_counts: OverlapExtractionStageCounts {
            source_overlap_lineage_rows,
            recovered_overlap_lineage_rows,
            adjacency_rows,
            arrangement_cells,
            boundary_contact_components,
            area_overlap_components,
            shared_area_rows,
            mixed_boundary_area_rows,
            pure_boundary_only_rows,
            boundary_only_overlap_rows,
            canonical_boundary_rows,
        },
    }
}

fn trace_overlap_stage_count(stage: &str, count: usize) {
    if std::env::var_os("WORTH_TRACE_OVERLAP_STAGE_COUNTS").is_some() {
        eprintln!("overlap extraction stage count: {stage}={count}");
    }
}

fn shared_area_bundle_from_arrangement(
    arrangement: &PlanarBooleanCoplanarOverlapArrangementGraph,
) -> (PlanarBooleanSharedAreaAdmissionBundle, usize, usize) {
    let containment = PlanarBooleanOverlapCellContainmentMap::admit(
        PlanarBooleanOverlapCellContainmentInput::from_arrangement(arrangement),
    )
    .expect("containment");
    let winding = PlanarBooleanOverlapCellWindingField::admit(
        PlanarBooleanOverlapCellWindingFieldInput::from_arrangement(arrangement, &containment),
    )
    .expect("winding");
    let island_bundle = PlanarBooleanOverlapIslandComponentBundle::admit(
        PlanarBooleanOverlapIslandCandidateInput::from_cell_classification(
            arrangement,
            &containment,
            &winding,
        ),
    )
    .expect("island components");
    let boundary_contact_components = island_bundle.boundary_contact_components().rows().len();
    let area_overlap_components = island_bundle.area_overlap_components().rows().len();
    let boundary_bundle: PlanarBooleanBoundaryContactClassificationBundle = island_bundle
        .classify_boundary_contact_components()
        .expect("boundary contact");
    let shared_area = boundary_bundle
        .admit_shared_area_components(&containment, &winding)
        .expect("shared area");
    (
        shared_area,
        boundary_contact_components,
        area_overlap_components,
    )
}

pub(crate) fn foreign_readiness_binding(
    label: &'static str,
) -> PlanarBooleanOverlapReadinessLoopLedgerBinding {
    primary_overlap_owner_seam_fixture()
        .request
        .readiness_loop_ledger_binding()
        .clone()
        .with_test_selected_route_identity_digest(format!("foreign-readiness-binding:{label}"))
}
