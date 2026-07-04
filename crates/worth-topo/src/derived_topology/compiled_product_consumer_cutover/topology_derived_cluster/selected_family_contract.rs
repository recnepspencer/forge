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
