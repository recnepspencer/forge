use crate::derived_topology::compiled_product_consumer_cutover::topology_derived_cluster::admitted_contract::DerivedEquivalenceContractReport;
use crate::selected_equivalence_family::{
    TopologySelectedEquivalenceComparable, TopologySelectedEquivalenceComparisonReport,
};

pub fn compare_selected_equivalence_contracts(
    lhs: &DerivedEquivalenceContractReport,
    rhs: &DerivedEquivalenceContractReport,
) -> TopologySelectedEquivalenceComparisonReport {
    match (
        lhs.selected_comparator_contract.as_ref(),
        rhs.selected_comparator_contract.as_ref(),
    ) {
        (Some(left), Some(right)) if left == right => {
            left.compare(&selected_comparable(lhs), &selected_comparable(rhs))
        }
        (Some(_), Some(_)) => TopologySelectedEquivalenceComparisonReport::unsupported(
            "topology reports declared different comparator contracts",
        ),
        _ => TopologySelectedEquivalenceComparisonReport::unsupported(
            "selected equivalence family contract is required before topology comparison",
        ),
    }
}

fn selected_comparable(
    report: &DerivedEquivalenceContractReport,
) -> TopologySelectedEquivalenceComparable<'_> {
    TopologySelectedEquivalenceComparable::new(
        report.selected_equivalence_family_identity,
        report.selected_equivalence_basis_identity_digest.as_deref(),
        report.selected_reuse_basis_identity_digest.as_deref(),
        report.selected_ordering_noise_posture,
        report.selected_rendered_output_comparison_posture,
        Some(&report.materialized_topology_digest),
        Some(&report.interpreted_topology_digest),
        Some(&report.derived_validation_digest),
    )
}

#[cfg(test)]
mod tests {
    use super::compare_selected_equivalence_contracts;
    use crate::derived_topology::compiled_product_consumer_cutover::build_derived_equivalence_contract;
    use crate::test_support::primitive_corpus::validated_topology::{
        build_test_runtime, committed_primitive_input,
    };
    use schema::facade::topology_authoring::MilestoneOnePrimitiveCase;

    #[test]
    fn selected_family_contract_comparison_rejects_missing_contract() {
        let baseline = real_report("phase-9-selected-family-contract");
        let hostile = baseline.clone().with_test_selected_family_contract_removed();

        let comparison = compare_selected_equivalence_contracts(&baseline, &hostile);

        assert!(!comparison.comparison_supported);
        assert!(comparison
            .unsupported_comparison_reason
            .as_deref()
            .is_some_and(|reason| reason.contains("selected equivalence family contract")));
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
        let mut query_runtime = crate::certification::support::read_basis_query_runtime::HistoricalReadBasisQueryRuntime::open(
            &runtime,
            committed.read_basis().clone(),
            "phase-nine-topology.selected-family-contract",
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
}
