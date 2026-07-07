use crate::derived_topology::compiled_product_consumer_cutover::build_derived_equivalence_contract;
use crate::test_support::primitive_corpus::validated_topology::{
    build_test_runtime, committed_primitive_input,
};
use schema::facade::topology_authoring::MilestoneOnePrimitiveCase;

use super::{
    decide_topology_derived_reuse, execute_topology_derived_reuse,
    TopologyDerivedReuseDecisionPosture, TopologyDerivedReuseExecutionInput,
    TopologyDerivedReuseMismatchLocus,
};

#[test]
fn rebuild_required_is_first_class_not_fallback() {
    let baseline = real_report("phase-9-topology-baseline");
    let changed = real_report("phase-9-topology-changed");
    let baseline_input = TopologyDerivedReuseExecutionInput::lower(&baseline);
    let changed_input = TopologyDerivedReuseExecutionInput::lower(&changed);

    let decision = decide_topology_derived_reuse(&baseline_input, &changed_input);

    assert_eq!(
        decision.posture(),
        TopologyDerivedReuseDecisionPosture::FreshRebuildRequired
    );
    assert!(decision.rebuild_denial().is_some());
    assert!(decision
        .rebuild_denial()
        .expect("rebuild denial")
        .mismatch_loci()
        .contains(&TopologyDerivedReuseMismatchLocus::AuthorityTruthIdentity));
    assert_eq!(
        baseline_input.authority_truth_identity_digest(),
        baseline.authority_truth_identity_digest()
    );
    assert_eq!(
        baseline_input.materialized_topology_digest(),
        &baseline.materialized_topology_digest
    );
    assert_eq!(
        baseline_input.interpreted_topology_digest(),
        &baseline.interpreted_topology_digest
    );
    assert_eq!(
        baseline_input.derived_validation_digest(),
        &baseline.derived_validation_digest
    );
}

#[test]
fn reuse_decision_binds_identity_and_family_chain() {
    let baseline = real_report("phase-9-topology-stable");
    let identical = baseline.clone();
    let baseline_input = TopologyDerivedReuseExecutionInput::lower(&baseline);
    let identical_input = TopologyDerivedReuseExecutionInput::lower(&identical);

    let decision = decide_topology_derived_reuse(&baseline_input, &identical_input);

    assert_eq!(
        decision.posture(),
        TopologyDerivedReuseDecisionPosture::ReuseAdmitted
    );
    assert!(decision.reuse_decision_identity_digest().is_some());
    assert_eq!(decision.counters().compared_basis_dimension_count(), 8);
    assert!(decision.compiled_product_identity_digest().is_some());
    assert!(decision.equivalence_policy_identity_digest().is_some());
    assert!(decision.selected_equivalence_family_identity().is_some());
    assert!(decision.selected_equivalence_basis_identity_digest().is_some());
    assert!(decision
        .selected_compatibility_basis_identity_digest()
        .is_some());
    assert!(decision.selected_reuse_basis_identity_digest().is_some());
    assert!(decision.comparison_supported());
    assert_eq!(decision.unsupported_comparison_reason(), None);
}

#[test]
fn reuse_denial_localizes_mismatch_locus() {
    let baseline = real_report("phase-9-topology-denial");
    let hostile = baseline
        .clone()
        .with_test_selected_family_contract_removed();
    let baseline_input = TopologyDerivedReuseExecutionInput::lower(&baseline);
    let hostile_input = TopologyDerivedReuseExecutionInput::lower(&hostile);

    let decision = decide_topology_derived_reuse(&baseline_input, &hostile_input);

    assert_eq!(
        decision.posture(),
        TopologyDerivedReuseDecisionPosture::Denied
    );
    let denial = decision.rebuild_denial().expect("denied decision");
    assert!(denial
        .mismatch_loci()
        .contains(&TopologyDerivedReuseMismatchLocus::MissingSelectedFamilyContract));
    assert!(denial
        .mismatch_loci()
        .contains(&TopologyDerivedReuseMismatchLocus::ComparatorContract));
    assert!(denial.compiled_product_identity_digest().is_some());
    assert!(denial.equivalence_policy_identity_digest().is_some());
    assert!(denial.selected_equivalence_family_identity().is_none());
    assert!(denial.selected_equivalence_basis_identity_digest().is_none());
    assert!(denial
        .selected_compatibility_basis_identity_digest()
        .is_none());
    assert!(denial.selected_reuse_basis_identity_digest().is_none());
    assert!(!denial.denial_identity_digest().is_empty());
    assert_eq!(denial.counters().compared_basis_dimension_count(), 8);
}

#[test]
fn topology_parity_report_consumes_resolution_product() {
    let baseline = real_report("phase-9-topology-resolution");
    let identical = baseline.clone();
    let baseline_input = TopologyDerivedReuseExecutionInput::lower(&baseline);
    let identical_input = TopologyDerivedReuseExecutionInput::lower(&identical);

    let resolution = execute_topology_derived_reuse(&baseline_input, &identical_input);

    assert_eq!(
        resolution.decision().posture(),
        TopologyDerivedReuseDecisionPosture::ReuseAdmitted
    );
    assert!(resolution.equivalent_derived_meaning());
    assert!(resolution.authority_identity_match());
}

fn real_report(
    label: &str,
) -> crate::derived_topology::compiled_product_consumer_cutover::DerivedEquivalenceContractReport {
    let mut runtime = build_test_runtime().expect("phase 9 topology runtime");
    let committed = committed_primitive_input(
        &mut runtime,
        label,
        &MilestoneOnePrimitiveCase::SheetPatch { face_count: 3 },
    )
    .expect("committed primitive input");
    let mut query_runtime =
        crate::certification::support::read_basis_query_runtime::HistoricalReadBasisQueryRuntime::open(
            &runtime,
            committed.read_basis().clone(),
            "phase-nine-topology.snapshot",
        )
        .expect("historical read-basis query runtime");
    let snapshot = crate::certification::support::historical_query_snapshot::historical_query_snapshot_for_read_basis(
        &mut query_runtime,
    )
    .expect("historical query snapshot");
    build_derived_equivalence_contract(
        committed.read_basis(),
        snapshot.materialized(),
        snapshot.interpreted(),
        snapshot.validation(),
    )
}
