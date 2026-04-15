use worth_schema::facade::DerivedTopologyReadBasis;

use crate::certification::WorthDeterministicDigest;
use crate::diagnostics::triggered_invalidation_targets;
use crate::interpretation::InterpretedTopologyView;
use crate::materialization::MaterializedTopologyView;
use crate::parity::types::{
    WorthDerivedEquivalenceContractReport, WorthDerivedParityComparisonReport,
};
use crate::validators::DerivedTopologyValidationReport;

pub fn digest_materialized_topology_view(
    materialized: &MaterializedTopologyView,
) -> WorthDeterministicDigest {
    digest_structured_value(materialized)
}

pub fn digest_interpreted_topology_view(
    interpreted: &InterpretedTopologyView,
) -> WorthDeterministicDigest {
    digest_structured_value(interpreted)
}

pub fn digest_derived_validation_report(
    validation: &DerivedTopologyValidationReport,
) -> WorthDeterministicDigest {
    digest_structured_value(validation)
}

pub fn build_derived_equivalence_contract(
    read_basis: &DerivedTopologyReadBasis,
    materialized: &MaterializedTopologyView,
    interpreted: &InterpretedTopologyView,
    validation: &DerivedTopologyValidationReport,
) -> WorthDerivedEquivalenceContractReport {
    WorthDerivedEquivalenceContractReport {
        authority_snapshot_id: read_basis.snapshot().snapshot_id.0,
        authority_branch_id: read_basis.branch_id().0.clone(),
        authoritative_mutation_origin: read_basis.authoritative_mutation_origin(),
        derivation_origin: read_basis.derivation_origin(),
        truth_basis_digest_hex: read_basis
            .authority
            .truth_basis_identity
            .mutation_batch_digest_hex
            .clone(),
        touched_aspect_count: read_basis
            .authority
            .truth_basis_identity
            .touched_aspect_count,
        triggered_invalidation_targets: triggered_invalidation_targets(read_basis),
        precision_fallback_count: read_basis.precision_fallbacks.len(),
        precision_budget_fallback_count: read_basis.precision_budget_fallbacks.len(),
        materialized_topology_digest: digest_materialized_topology_view(materialized),
        interpreted_topology_digest: digest_interpreted_topology_view(interpreted),
        derived_validation_digest: digest_derived_validation_report(validation),
    }
}

pub fn compare_derived_equivalence_contracts(
    lhs: &WorthDerivedEquivalenceContractReport,
    rhs: &WorthDerivedEquivalenceContractReport,
) -> WorthDerivedParityComparisonReport {
    let authority_identity_match = lhs.authority_snapshot_id == rhs.authority_snapshot_id
        && lhs.truth_basis_digest_hex == rhs.truth_basis_digest_hex
        && lhs.authoritative_mutation_origin == rhs.authoritative_mutation_origin
        && lhs.touched_aspect_count == rhs.touched_aspect_count;
    let branch_identity_match = lhs.authority_branch_id == rhs.authority_branch_id;
    let invalidation_target_match =
        lhs.triggered_invalidation_targets == rhs.triggered_invalidation_targets;
    let materialized_topology_digest_match =
        lhs.materialized_topology_digest == rhs.materialized_topology_digest;
    let interpreted_topology_digest_match =
        lhs.interpreted_topology_digest == rhs.interpreted_topology_digest;
    let derived_validation_digest_match =
        lhs.derived_validation_digest == rhs.derived_validation_digest;

    WorthDerivedParityComparisonReport {
        authority_identity_match,
        branch_identity_match,
        invalidation_target_match,
        materialized_topology_digest_match,
        interpreted_topology_digest_match,
        derived_validation_digest_match,
        equivalent_derived_meaning: materialized_topology_digest_match
            && interpreted_topology_digest_match
            && derived_validation_digest_match,
    }
}

fn digest_structured_value<T: serde::Serialize>(value: &T) -> WorthDeterministicDigest {
    let json = serde_json::to_string(value)
        .expect("worth derived parity serialization should be deterministic");
    digest_rows(std::iter::once(json))
}

fn digest_rows(rows: impl IntoIterator<Item = String>) -> WorthDeterministicDigest {
    let mut state: u64 = 0xcbf29ce484222325;
    let mut row_count = 0usize;
    for row in rows {
        row_count += 1;
        for byte in row.as_bytes() {
            state ^= u64::from(*byte);
            state = state.wrapping_mul(0x100000001b3);
        }
    }

    WorthDeterministicDigest {
        algorithm: "fnv1a64".to_string(),
        digest_hex: format!("{state:016x}"),
        row_count,
    }
}
