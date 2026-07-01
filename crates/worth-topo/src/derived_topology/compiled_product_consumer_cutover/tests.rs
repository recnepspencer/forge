use crate::certification::support::historical_query_snapshot::historical_query_snapshot_for_read_basis;
use crate::certification::support::read_basis_query_runtime::HistoricalReadBasisQueryRuntime;
use crate::derived_topology::compiled_product_consumer_cutover::residue_manifest::{
    TopologyConsumerResidueDisposition, TopologyConsumerResidueOwner,
};
use crate::derived_topology::compiled_product_consumer_cutover::{
    compare_derived_equivalence_contracts, current_topology_consumer_residue_manifest,
    require_exact_topology_consumer_closeout, DerivedInvalidationPlannedDisposition,
};
use crate::derived_topology::invalidation_plan::catalog::{
    DerivedTopologyLegalityReceiptPosture, DerivedTopologyProductFamilyIdentity,
    DerivedTopologyQueryReceiptPosture, DerivedTopologyUpdatePosture,
};
use crate::derived_topology::invalidation_plan::selection::selection_test_fixtures::{
    admitted_legality_support, admitted_query_support, catalog_closeout,
    catalog_closeout_with_loop_cycles_contract, loop_cycles_touched_closure,
};
use crate::derived_topology::invalidation_plan::selection::{
    DerivedInvalidationDensityPolicy, DerivedInvalidationSelectedPlan,
};
use crate::projection::diagnostic_surfaces::derived_read_diagnostics::build_derived_read_diagnostics;
use crate::test_support::primitive_corpus::validated_topology::{
    build_test_runtime, committed_primitive_input,
};
use schema::facade::platform::authority::DerivedInvalidationTarget;
use schema::facade::topology_authoring::MilestoneOnePrimitiveCase;

#[test]
fn topology_consumers_route_through_reuse_decision_products() {
    let inputs = real_equivalence_inputs();
    let diagnostics = build_derived_read_diagnostics(
        &inputs.read_basis,
        &inputs.materialized,
        &inputs.interpreted,
        &inputs.validation,
    );
    let selected_plan = real_selected_plan();

    assert!(diagnostics
        .equivalence_contract_report
        .selected_equivalence_family_identity()
        .is_some());
    assert!(diagnostics
        .equivalence_contract_report
        .selected_equivalence_basis_identity_digest()
        .is_some());
    assert!(diagnostics
        .equivalence_contract_report
        .selected_compatibility_basis_identity_digest()
        .is_some());
    assert!(diagnostics
        .equivalence_contract_report
        .selected_reuse_basis_identity_digest()
        .is_some());
    assert!(diagnostics
        .equivalence_contract_report
        .reuse_decision_identity_digest()
        .is_some());
    assert!(diagnostics
        .invalidation_report
        .rows
        .iter()
        .any(|row| { row.target == DerivedInvalidationTarget::TopologyBoundary && row.triggered }));
    assert!(diagnostics.invalidation_report.rows.iter().any(|row| {
        row.target == DerivedInvalidationTarget::TopologyStructure && row.triggered
    }));
    assert!(selected_plan.selected_rows().iter().any(|row| {
        row.family_identity() == DerivedTopologyProductFamilyIdentity::LoopCycles
            && row.query_posture() == DerivedTopologyQueryReceiptPosture::NativeReadReceiptRequired
            && row.legality_posture()
                == DerivedTopologyLegalityReceiptPosture::SelectedLegalityReceiptRequired
            && row.planned_disposition() == DerivedInvalidationPlannedDisposition::IncrementalUpdate
    }));
    assert!(selected_plan.selected_rows().iter().any(|row| {
        row.family_identity() == DerivedTopologyProductFamilyIdentity::TraversalViews
            && row.planned_disposition() == DerivedInvalidationPlannedDisposition::BoundedRebuild
    }));
}

#[test]
fn topology_cutover_preserves_zero_broad_scan_fallback() {
    let report = build_derived_read_diagnostics_report();
    let hostile = report.clone().with_test_selected_family_contract_removed();
    let comparison = compare_derived_equivalence_contracts(&report, &hostile);

    assert!(!comparison.comparison_supported);
    assert!(comparison
        .unsupported_comparison_reason
        .as_deref()
        .is_some_and(|reason| reason.contains("selected equivalence family contract")));
    assert!(!comparison.equivalent_derived_meaning);
}

#[test]
fn topology_residue_rows_are_exact_and_non_authoritative() {
    require_exact_topology_consumer_closeout();
    let residue = current_topology_consumer_residue_manifest();
    let deleted_helper_surface =
        "crates/worth-topo/src/projection/diagnostic_surfaces/equivalence_contract.rs";

    assert_eq!(residue.len(), 2);
    assert_eq!(
        residue[0].source_path(),
        "crates/worth-topo/src/projection/runtime_boundary/read_execution/basis_context.rs"
    );
    assert_eq!(
        residue[0].current_surface(),
        "HistoricalEvaluationRequest::retained_snapshot(... HistoricalPathReuseDescriptor::retained_reuse())"
    );
    assert_eq!(residue[0].owner(), TopologyConsumerResidueOwner::ForgeQuery);
    assert_eq!(
        residue[0].disposition(),
        TopologyConsumerResidueDisposition::ExplicitResidue
    );
    assert_eq!(
        residue[0].blocker(),
        "query-backed historical read-model path still declares retained reuse before phase 13 boundary cutover"
    );
    assert_eq!(
        residue[0].removal_trigger(),
        "replace once Query-backed public/read-model consumers lower typed retained reuse products"
    );
    assert_eq!(
        residue[1].source_path(),
        "crates/worth-topo/src/projection/runtime_boundary/read_execution/basis_context.rs"
    );
    assert_eq!(
        residue[1].current_surface(),
        "HistoricalCapabilityDescriptor::retained_snapshot(... HistoricalPathReuseDescriptor::retained_reuse())"
    );
    assert_eq!(residue[1].owner(), TopologyConsumerResidueOwner::ForgeQuery);
    assert_eq!(
        residue[1].disposition(),
        TopologyConsumerResidueDisposition::QueryGap
    );
    assert_eq!(
        residue[1].blocker(),
        "historical capability lane remains blocked on Forge Query compiled-product-aware retained capability support"
    );
    assert_eq!(
        residue[1].removal_trigger(),
        "remove once Forge Query exposes a compiled-product-aware historical retained capability boundary"
    );
    assert!(residue
        .iter()
        .all(|row| row.source_path() != deleted_helper_surface));
}

#[test]
fn ordinary_projection_diagnostics_consumer_uses_shared_cutover_equivalence_lane() {
    let report = build_derived_read_diagnostics_report();
    let hostile = report.clone().with_test_selected_family_contract_removed();
    let comparison = compare_derived_equivalence_contracts(&report, &hostile);

    assert!(report.selected_equivalence_family_identity().is_some());
    assert!(report
        .selected_equivalence_basis_identity_digest()
        .is_some());
    assert!(report
        .selected_compatibility_basis_identity_digest()
        .is_some());
    assert!(report.selected_reuse_basis_identity_digest().is_some());
    assert!(report.reuse_decision_identity_digest().is_some());
    assert!(!comparison.comparison_supported);
    assert!(comparison
        .unsupported_comparison_reason
        .as_deref()
        .is_some_and(|reason| reason.contains("selected equivalence family contract")));
}

#[test]
fn invalidation_selected_rows_use_shared_cutover_planned_disposition_lowering() {
    let baseline = real_selected_plan();
    let hostile = DerivedInvalidationSelectedPlan::lower(
        &catalog_closeout_with_loop_cycles_contract(
            DerivedTopologyQueryReceiptPosture::NativeReadReceiptRequired,
            DerivedTopologyLegalityReceiptPosture::SelectedLegalityReceiptRequired,
            DerivedTopologyUpdatePosture::BoundedRebuildRequired,
        ),
        &loop_cycles_touched_closure("phase-eleven.topology-consumer-lane-proof"),
        &admitted_query_support(),
        &admitted_legality_support(),
        DerivedInvalidationDensityPolicy::Sparse,
    )
    .expect("selected invalidation plan with hostile loop-cycles posture");
    let baseline_loop_cycles =
        selected_row(&baseline, DerivedTopologyProductFamilyIdentity::LoopCycles);
    let hostile_loop_cycles =
        selected_row(&hostile, DerivedTopologyProductFamilyIdentity::LoopCycles);
    let baseline_wire_views =
        selected_row(&baseline, DerivedTopologyProductFamilyIdentity::WireViews);
    let hostile_wire_views =
        selected_row(&hostile, DerivedTopologyProductFamilyIdentity::WireViews);
    let baseline_materialized = selected_row(
        &baseline,
        DerivedTopologyProductFamilyIdentity::MaterializedGraph,
    );
    let hostile_materialized = selected_row(
        &hostile,
        DerivedTopologyProductFamilyIdentity::MaterializedGraph,
    );

    assert_eq!(
        baseline_loop_cycles.planned_disposition(),
        DerivedInvalidationPlannedDisposition::IncrementalUpdate
    );
    assert_eq!(
        hostile_loop_cycles.planned_disposition(),
        DerivedInvalidationPlannedDisposition::BoundedRebuild
    );
    assert_ne!(
        baseline_loop_cycles.row_digest(),
        hostile_loop_cycles.row_digest()
    );
    assert_eq!(
        baseline_wire_views.planned_disposition(),
        DerivedInvalidationPlannedDisposition::IncrementalUpdate
    );
    assert_eq!(
        hostile_wire_views.planned_disposition(),
        DerivedInvalidationPlannedDisposition::IncrementalUpdate
    );
    assert_eq!(
        baseline_wire_views.row_digest(),
        hostile_wire_views.row_digest()
    );
    assert_eq!(
        baseline_loop_cycles.query_posture(),
        DerivedTopologyQueryReceiptPosture::NativeReadReceiptRequired
    );
    assert_eq!(
        baseline_loop_cycles.legality_posture(),
        DerivedTopologyLegalityReceiptPosture::SelectedLegalityReceiptRequired
    );
    assert_eq!(
        baseline_materialized.planned_disposition(),
        DerivedInvalidationPlannedDisposition::BoundedRebuild
    );
    assert_eq!(
        hostile_materialized.planned_disposition(),
        DerivedInvalidationPlannedDisposition::BoundedRebuild
    );
    assert_eq!(
        baseline_materialized.query_posture(),
        DerivedTopologyQueryReceiptPosture::NativeReadReceiptRequired
    );
    assert_eq!(
        baseline_materialized.legality_posture(),
        DerivedTopologyLegalityReceiptPosture::SelectedLegalityReceiptRequired
    );
    assert_eq!(
        baseline_materialized.row_digest(),
        hostile_materialized.row_digest()
    );
}

struct EquivalenceInputs {
    read_basis: schema::facade::topology_authoring::DerivedTopologyReadBasis,
    materialized: crate::derived_topology::materialized_graph::MaterializedTopologyView,
    interpreted: crate::derived_topology::traversal_views::InterpretedTopologyView,
    validation: crate::validation::DerivedTopologyValidationReport,
}

fn real_equivalence_inputs() -> EquivalenceInputs {
    let mut runtime = build_test_runtime().expect("phase 11 topology runtime");
    let committed = committed_primitive_input(
        &mut runtime,
        "phase-eleven.topology-consumer-cutover",
        &MilestoneOnePrimitiveCase::SheetPatch { face_count: 3 },
    )
    .expect("committed primitive input");
    let read_basis = committed.read_basis().clone();
    let mut query_runtime = HistoricalReadBasisQueryRuntime::open(
        &runtime,
        read_basis.clone(),
        "phase-eleven.topology-consumer-cutover.snapshot",
    )
    .expect("historical read-basis query runtime");
    let snapshot = historical_query_snapshot_for_read_basis(&mut query_runtime)
        .expect("historical query snapshot");
    EquivalenceInputs {
        read_basis,
        materialized: snapshot.materialized().clone(),
        interpreted: snapshot.interpreted().clone(),
        validation: snapshot.validation().clone(),
    }
}

fn build_derived_read_diagnostics_report(
) -> crate::derived_topology::compiled_product_consumer_cutover::DerivedEquivalenceContractReport {
    let inputs = real_equivalence_inputs();
    let EquivalenceInputs {
        read_basis,
        materialized,
        interpreted,
        validation,
    } = inputs;
    build_derived_read_diagnostics(&read_basis, &materialized, &interpreted, &validation)
        .equivalence_contract_report
}

fn real_selected_plan() -> DerivedInvalidationSelectedPlan {
    DerivedInvalidationSelectedPlan::lower(
        &catalog_closeout(),
        &loop_cycles_touched_closure("phase-eleven.topology-consumer-lane-proof"),
        &admitted_query_support(),
        &admitted_legality_support(),
        DerivedInvalidationDensityPolicy::Sparse,
    )
    .expect("selected invalidation plan")
}

fn selected_row(
    plan: &DerivedInvalidationSelectedPlan,
    family: DerivedTopologyProductFamilyIdentity,
) -> &crate::derived_topology::invalidation_plan::selection::DerivedInvalidationSelectedRow {
    plan.selected_rows()
        .iter()
        .find(|row| row.family_identity() == family)
        .unwrap_or_else(|| panic!("selected row missing for {}", family.as_str()))
}
