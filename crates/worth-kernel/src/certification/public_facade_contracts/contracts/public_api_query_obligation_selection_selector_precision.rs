use forge_query::facade::consumer_kit::ForgeQueryGraphObligationInMemoryTestWorkspace;
use forge_query::facade::runtime::{
    ForgeQueryGraphObligationOperatingWorldDescriptor,
    ForgeQueryGraphObligationOperatingWorldSelector, ForgeQueryGraphTouchDescriptor,
    ForgeQueryGraphTouchReadVerb, ForgeQueryMutationFamily,
};
use topology::facade::{
    topology_primitive_construction_birth_graph_obligation_registration,
    TOPOLOGY_PRIMITIVE_CONSTRUCTION_BIRTH_COMPOSE_COLLECTION,
};
use worth_kernel::query_obligation_selection::selection_substrate::{
    QueryObligationSelectionInput, QueryObligationSelectionSubstrate,
    QuerySelectorExpressivenessGapKind, QuerySelectorPrecisionPosture,
};

use super::public_api_query_obligation_selection_real_spatial_authority_support::{
    real_spatial_selection_case, RealSpatialSelectionCase,
};
use super::public_api_query_obligation_selection_support::primitive_construction_birth_cases;

#[test]
fn selector_precision_counters_scale_with_touched_descriptor_breadth() {
    let registration = topology_primitive_construction_birth_graph_obligation_registration(
        forge_query::facade::runtime::ForgeQueryGraphObligationSupportLane::GraphComposition,
        ForgeQueryGraphObligationOperatingWorldSelector::any_operating_world(),
    );
    let workspace =
        ForgeQueryGraphObligationInMemoryTestWorkspace::from_registrations([registration])
            .expect("primitive construction Query workspace");
    let operating_world =
        ForgeQueryGraphObligationOperatingWorldDescriptor::configured_domain_handle();
    let narrow = ForgeQueryGraphTouchDescriptor::declared_mutation_collection(
        TOPOLOGY_PRIMITIVE_CONSTRUCTION_BIRTH_COMPOSE_COLLECTION,
        ForgeQueryMutationFamily::Insert,
        None,
        ["set:topology.kind"],
        ["topology.kind"],
    )
    .expect("narrow descriptor");
    let broad = ForgeQueryGraphTouchDescriptor::declared_mutation_collection(
        TOPOLOGY_PRIMITIVE_CONSTRUCTION_BIRTH_COMPOSE_COLLECTION,
        ForgeQueryMutationFamily::Insert,
        None,
        [
            "set:topology.kind",
            "set:topology.structure",
            "set:naming.persistent_name",
        ],
        [
            "topology.kind",
            "topology.structure",
            "naming.persistent_name",
        ],
    )
    .expect("broad descriptor");

    let narrow_proof = workspace.prove_selection(&narrow, &operating_world);
    let broad_proof = workspace.prove_selection(&broad, &operating_world);
    let narrow_counters = narrow_proof.selection_counters();
    let broad_counters = broad_proof.selection_counters();

    assert_eq!(narrow_proof.selected_obligation_count(), 0);
    assert_eq!(broad_proof.selected_obligation_count(), 1);
    assert!(
        narrow_counters.attempted_bucket_lookup_count()
            < broad_counters.attempted_bucket_lookup_count()
    );
    assert_eq!(
        narrow_counters.attempted_bucket_lookup_count(),
        narrow_counters.touch_lookup_key_count()
            * narrow_counters.operating_world_lookup_key_count()
    );
    assert_eq!(
        broad_counters.attempted_bucket_lookup_count(),
        broad_counters.touch_lookup_key_count() * broad_counters.operating_world_lookup_key_count()
    );
    assert_eq!(narrow_counters.registration_full_scan_count(), 0);
    assert_eq!(broad_counters.registration_full_scan_count(), 0);
}

#[test]
fn selected_obligation_closeout_exposes_precision_report_beside_proof() {
    let case = primitive_construction_birth_cases()
        .into_iter()
        .next()
        .expect("primitive construction case");
    let touched_basis = case.declared_touched_basis("phase5-selector-precision");
    let input = QueryObligationSelectionInput::from_topology_touched_basis(touched_basis.proof())
        .expect("touched basis should lower to selection input");
    let selected = QueryObligationSelectionSubstrate::select_execution_backed_obligations(input)
        .expect("primitive construction selection");
    let closeout = selected.closeout();
    let report = closeout.selector_precision_report();

    assert_eq!(
        report.posture(),
        QuerySelectorPrecisionPosture::TouchedDescriptorBounded
    );
    assert!(report.is_touched_descriptor_bounded());
    assert!(report.has_touched_descriptor_bounded_counters());
    assert!(report.has_clean_selector_closeout());
    assert_eq!(
        report.attempted_bucket_lookup_count(),
        report.touch_lookup_key_count() * report.operating_world_lookup_key_count()
    );
    assert_eq!(report.registration_full_scan_count(), 0);
    assert_eq!(
        report.matched_obligation_count(),
        selected.selected_obligation_count()
    );
    assert_eq!(report.broad_selector_residue_count(), 0);
    assert_eq!(report.query_selector_gap_count(), 0);
    assert!(!report.counters_digest().is_empty());
    assert!(!report.report_digest().is_empty());
}

#[test]
fn broad_collection_or_lifecycle_only_selector_is_capped_residue() {
    let selected_case = real_spatial_case();
    let closeout = selected_case.selected().closeout();
    let report = closeout.selector_precision_report();
    let broad_rows = closeout.broad_selector_residue_rows();

    assert_eq!(
        report.posture(),
        QuerySelectorPrecisionPosture::QueryExpressivenessGap
    );
    assert!(report.has_touched_descriptor_bounded_counters());
    assert!(!report.is_touched_descriptor_bounded());
    assert!(!report.has_clean_selector_closeout());
    assert!(report.has_broad_selector_residue());
    assert!(report.has_query_selector_gaps());
    assert_eq!(report.broad_selector_residue_count(), 1);
    assert_eq!(broad_rows.len(), 1);
    let row = &broad_rows.rows()[0];
    assert_eq!(row.class(), "worth-spatial-broad-collection-selector");
    assert_eq!(row.owner(), "worth-spatial");
    assert_eq!(row.introduced_in(), "touched-graph-milestone-5-phase-5");
    assert_eq!(row.current_count(), 1);
    assert_eq!(row.must_not_exceed_count(), 1);
    assert_eq!(
        row.blocker(),
        "spatial graph obligation adoption still registers a broad collection selector because Query cannot yet express the spatial lookup product as a declared mutation selector"
    );
    assert_eq!(
        row.removal_trigger(),
        "replace the collection selector with a Query-owned spatial lookup-product selector or declared mutation selector expression"
    );
    assert_eq!(
        row.decision(),
        "capped broad selector residue; it may select only beside Query counters and typed selector expressiveness gaps"
    );
    assert!(!row.row_digest().is_empty());
}

#[test]
fn missing_selector_expressiveness_records_query_gap() {
    let selected_case = real_spatial_case();
    let closeout = selected_case.selected().closeout();
    let gaps = closeout.query_selector_gap_rows();

    assert_eq!(gaps.len(), 1);
    let gap = &gaps.rows()[0];
    assert_eq!(
        gap.kind(),
        QuerySelectorExpressivenessGapKind::DeclaredMutationCollectionNotExpressed
    );
    assert_eq!(
        gap.kind().as_str(),
        "declared-mutation-collection-not-expressed"
    );
    assert_eq!(gap.owner(), "forge-query");
    assert!(gap
        .needed_by()
        .contains("worth-spatial spatial evidence obligation selection"));
    assert!(gap.blocker().contains("selector expressiveness gap"));
    assert!(gap
        .follow_on_milestone()
        .contains("touched-graph-milestone-5"));
    assert!(!gap.source_gap_digest().is_empty());
}

#[test]
fn broad_read_family_descriptor_is_not_mistaken_for_selected_precision() {
    let registration = topology_primitive_construction_birth_graph_obligation_registration(
        forge_query::facade::runtime::ForgeQueryGraphObligationSupportLane::GraphComposition,
        ForgeQueryGraphObligationOperatingWorldSelector::any_operating_world(),
    );
    let workspace =
        ForgeQueryGraphObligationInMemoryTestWorkspace::from_registrations([registration])
            .expect("primitive construction Query workspace");
    let descriptor = ForgeQueryGraphTouchDescriptor::read_family(
        TOPOLOGY_PRIMITIVE_CONSTRUCTION_BIRTH_COMPOSE_COLLECTION,
        [ForgeQueryGraphTouchReadVerb::ObservesCollection],
    )
    .expect("read-family descriptor");
    let proof = workspace.prove_selection(
        &descriptor,
        &ForgeQueryGraphObligationOperatingWorldDescriptor::configured_domain_handle(),
    );
    let counters = proof.selection_counters();

    assert_eq!(proof.selected_obligation_count(), 0);
    assert_eq!(counters.matched_obligation_count(), 0);
    assert_eq!(counters.registration_full_scan_count(), 0);
    assert_eq!(
        counters.attempted_bucket_lookup_count(),
        counters.touch_lookup_key_count() * counters.operating_world_lookup_key_count()
    );
}

fn real_spatial_case() -> &'static RealSpatialSelectionCase {
    static REAL_SPATIAL_CASE: std::sync::OnceLock<RealSpatialSelectionCase> =
        std::sync::OnceLock::new();

    REAL_SPATIAL_CASE.get_or_init(|| real_spatial_selection_case("phase5-selector-precision"))
}
