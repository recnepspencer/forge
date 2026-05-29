use schema::facade::platform::authority::{DerivedInvalidationTarget, MutationOrigin};
use schema::facade::topology_authoring::DerivedTopologyReadBasis;

use crate::certification::support::parity::types::{
    DerivedEquivalenceContractReport, DerivedParityComparisonReport,
};
use crate::certification::DeterministicDigest;
use crate::derived_topology::materialized_graph::MaterializedTopologyView;
use crate::derived_topology::traversal_views::InterpretedTopologyView;
use crate::projection::diagnostic_surfaces::triggered_invalidation_targets;
use crate::validation::DerivedTopologyValidationReport;

pub fn digest_materialized_topology_view(
    materialized: &MaterializedTopologyView,
) -> DeterministicDigest {
    digest_structured_value(materialized)
}

pub fn digest_interpreted_topology_view(
    interpreted: &InterpretedTopologyView,
) -> DeterministicDigest {
    digest_structured_value(interpreted)
}

pub fn digest_derived_validation_report(
    validation: &DerivedTopologyValidationReport,
) -> DeterministicDigest {
    digest_structured_value(validation)
}

pub fn build_derived_equivalence_contract(
    read_basis: &DerivedTopologyReadBasis,
    materialized: &MaterializedTopologyView,
    interpreted: &InterpretedTopologyView,
    validation: &DerivedTopologyValidationReport,
) -> DerivedEquivalenceContractReport {
    build_derived_equivalence_contract_report(
        read_basis.snapshot().snapshot_id.0,
        read_basis.branch_id().0.clone(),
        read_basis.authoritative_mutation_origin(),
        read_basis.derivation_origin(),
        read_basis
            .authority
            .truth_basis_identity
            .mutation_batch_digest_hex
            .clone(),
        read_basis
            .authority
            .truth_basis_identity
            .touched_aspect_count,
        triggered_invalidation_targets(read_basis),
        read_basis.precision_fallbacks.len(),
        read_basis.precision_budget_fallbacks.len(),
        materialized,
        interpreted,
        validation,
    )
}

pub fn build_derived_equivalence_contract_report(
    authority_snapshot_id: u64,
    authority_branch_id: String,
    authoritative_mutation_origin: MutationOrigin,
    derivation_origin: MutationOrigin,
    truth_basis_digest_hex: String,
    touched_aspect_count: usize,
    triggered_invalidation_targets: Vec<DerivedInvalidationTarget>,
    precision_fallback_count: usize,
    precision_budget_fallback_count: usize,
    materialized: &MaterializedTopologyView,
    interpreted: &InterpretedTopologyView,
    validation: &DerivedTopologyValidationReport,
) -> DerivedEquivalenceContractReport {
    DerivedEquivalenceContractReport {
        authority_snapshot_id,
        authority_branch_id,
        authoritative_mutation_origin,
        derivation_origin,
        truth_basis_digest_hex,
        touched_aspect_count,
        triggered_invalidation_targets,
        precision_fallback_count,
        precision_budget_fallback_count,
        materialized_topology_digest: digest_materialized_topology_view(materialized),
        interpreted_topology_digest: digest_interpreted_topology_view(interpreted),
        derived_validation_digest: digest_derived_validation_report(validation),
    }
}

pub fn compare_derived_equivalence_contracts(
    lhs: &DerivedEquivalenceContractReport,
    rhs: &DerivedEquivalenceContractReport,
) -> DerivedParityComparisonReport {
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

    DerivedParityComparisonReport {
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

fn digest_structured_value<T: serde::Serialize>(value: &T) -> DeterministicDigest {
    let json = serde_json::to_string(value)
        .expect(" derived parity serialization should be deterministic");
    digest_rows(std::iter::once(json))
}

fn digest_rows(rows: impl IntoIterator<Item = String>) -> DeterministicDigest {
    let mut state: u64 = 0xcbf29ce484222325;
    let mut row_count = 0usize;
    for row in rows {
        row_count += 1;
        for byte in row.as_bytes() {
            state ^= u64::from(*byte);
            state = state.wrapping_mul(0x100000001b3);
        }
    }

    DeterministicDigest {
        algorithm: "fnv1a64".to_string(),
        digest_hex: format!("{state:016x}"),
        row_count,
    }
}




