use crate::certification::support::historical_query_snapshot::historical_query_snapshot_for_read_basis;
use crate::certification::support::read_basis_query_runtime::HistoricalReadBasisQueryRuntime;
use crate::derived_topology::compiled_product_consumer_cutover::{
    build_derived_equivalence_contract, compare_derived_equivalence_contracts,
    DerivedEquivalenceContractReport,
};
use crate::derived_topology::materialized_graph::MaterializedTopologyView;
use crate::derived_topology::traversal_views::InterpretedTopologyView;
use crate::test_support::primitive_corpus::validated_topology::{
    build_test_runtime, committed_primitive_input,
};
use crate::validation::DerivedTopologyValidationReport;
use schema::facade::platform::authority::MutationOrigin;
use schema::facade::topology_authoring::{DerivedTopologyReadBasis, MilestoneOnePrimitiveCase};

#[test]
fn shared_identity_vocabulary_is_rerun_stable() {
    let inputs = real_equivalence_inputs();
    let left = build_report(&inputs, 7, "branch-a", "authority-a");
    let right = build_report(&inputs, 7, "branch-a", "authority-a");

    assert_eq!(
        left.authority_truth_identity_digest(),
        right.authority_truth_identity_digest()
    );
    assert_eq!(
        left.compiled_product_identity_digest(),
        right.compiled_product_identity_digest()
    );
    assert_eq!(
        left.equivalence_policy_identity_digest(),
        right.equivalence_policy_identity_digest()
    );
    assert_eq!(
        left.selected_equivalence_family_identity(),
        right.selected_equivalence_family_identity()
    );
    assert_eq!(
        left.selected_equivalence_basis_identity_digest(),
        right.selected_equivalence_basis_identity_digest()
    );
    let comparison = compare_derived_equivalence_contracts(&left, &right);
    assert!(comparison.comparison_supported);
    assert!(comparison.equivalent_derived_meaning);
}

#[test]
fn changed_authority_instance_cannot_impersonate_compiled_product_identity() {
    let inputs = real_equivalence_inputs();
    let snapshot_changed = build_report(&inputs, 8, "branch-a", "authority-a");
    let branch_changed = build_report(&inputs, 7, "branch-b", "authority-a");
    let baseline = build_report(&inputs, 7, "branch-a", "authority-a");

    assert_ne!(
        baseline.authority_truth_identity_digest(),
        snapshot_changed.authority_truth_identity_digest()
    );
    assert_ne!(
        baseline.compiled_product_identity_digest(),
        snapshot_changed.compiled_product_identity_digest()
    );
    assert!(
        !compare_derived_equivalence_contracts(&baseline, &snapshot_changed)
            .equivalent_derived_meaning
    );

    assert_ne!(
        baseline.authority_truth_identity_digest(),
        branch_changed.authority_truth_identity_digest()
    );
    assert_ne!(
        baseline.compiled_product_identity_digest(),
        branch_changed.compiled_product_identity_digest()
    );
    assert!(
        !compare_derived_equivalence_contracts(&baseline, &branch_changed)
            .equivalent_derived_meaning
    );
}

#[test]
fn changed_authority_instance_stays_typed_rebuild_required_at_consumer_seam() {
    let inputs = real_equivalence_inputs();
    let baseline = build_report(&inputs, 7, "branch-a", "authority-a");
    let snapshot_changed = build_report(&inputs, 8, "branch-a", "authority-a");

    let comparison = compare_derived_equivalence_contracts(&baseline, &snapshot_changed);

    assert!(comparison.comparison_supported);
    assert_eq!(
        comparison.reuse_decision_posture,
        Some(
            crate::compiled_product_reuse_decision::TopologyDerivedReuseDecisionPosture::FreshRebuildRequired
        )
    );
    assert!(comparison.rebuild_denial_identity_digest.is_some());
    assert!(
        comparison
            .mismatch_loci
            .contains(&crate::compiled_product_reuse_decision::TopologyDerivedReuseMismatchLocus::AuthorityTruthIdentity)
    );
    assert!(!comparison.authority_identity_match);
    assert!(comparison.compared_basis_dimension_count > 0);
    assert!(!comparison.equivalent_derived_meaning);
}

#[test]
fn missing_shared_identity_fields_cannot_fallback_to_local_equivalence() {
    let inputs = real_equivalence_inputs();
    let left = build_report(&inputs, 7, "branch-a", "authority-a");
    let right = left.clone().with_test_shared_identity_fields_removed();

    let comparison = compare_derived_equivalence_contracts(&left, &right);
    assert!(comparison.comparison_supported);
    assert!(!comparison.authority_identity_match);
    assert!(!comparison.equivalent_derived_meaning);
}

#[test]
fn forged_family_digest_cannot_mint_equivalence_identity() {
    let inputs = real_equivalence_inputs();
    let left = build_report(&inputs, 7, "branch-a", "authority-a");
    let right = left
        .clone()
        .with_test_topology_compiled_product_family_digest("forged-family");

    let comparison = compare_derived_equivalence_contracts(&left, &right);
    assert!(comparison.comparison_supported);
    assert!(!comparison.authority_identity_match);
    assert!(!comparison.equivalent_derived_meaning);
}

#[test]
fn shared_payload_instance_cannot_substitute_for_family_identity() {
    let inputs = real_equivalence_inputs();
    let left = build_report(&inputs, 7, "branch-a", "authority-a");
    let baseline = build_report(&inputs, 8, "branch-a", "authority-a");
    let right = baseline.clone().with_test_surface_digests_from(&left);

    let comparison = compare_derived_equivalence_contracts(&left, &right);
    assert!(comparison.comparison_supported);
    assert!(!comparison.authority_identity_match);
    assert!(!comparison.equivalent_derived_meaning);
}

#[test]
fn missing_selected_family_contract_cannot_fallback_to_rendered_output_equality() {
    let inputs = real_equivalence_inputs();
    let left = build_report(&inputs, 7, "branch-a", "authority-a");
    let right = left.clone().with_test_selected_family_contract_removed();

    let comparison = compare_derived_equivalence_contracts(&left, &right);
    assert!(!comparison.comparison_supported);
    assert!(comparison
        .unsupported_comparison_reason
        .as_deref()
        .is_some_and(|reason| reason.contains("selected equivalence family contract")));
    assert!(!comparison.equivalent_derived_meaning);
}

#[test]
fn mismatched_comparator_contract_cannot_fallback_to_local_payload_parity() {
    let inputs = real_equivalence_inputs();
    let left = build_report(&inputs, 7, "branch-a", "authority-a");
    let right = left
        .clone()
        .with_test_selected_comparator_dimensions(vec![
            crate::selected_equivalence_family::TopologySelectedEquivalenceDimension::MaterializedTopologyDigest,
        ]);

    let comparison = compare_derived_equivalence_contracts(&left, &right);
    assert!(!comparison.comparison_supported);
    assert!(comparison
        .unsupported_comparison_reason
        .as_deref()
        .is_some_and(|reason| reason.contains("different comparator contracts")));
    assert!(!comparison.equivalent_derived_meaning);
}

struct EquivalenceInputs {
    read_basis: DerivedTopologyReadBasis,
    materialized: MaterializedTopologyView,
    interpreted: InterpretedTopologyView,
    validation: DerivedTopologyValidationReport,
}

fn real_equivalence_inputs() -> EquivalenceInputs {
    let mut runtime = build_test_runtime().expect("phase 6 equivalence runtime");
    let committed = committed_primitive_input(
        &mut runtime,
        "phase-six.equivalence-contract",
        &MilestoneOnePrimitiveCase::SheetPatch { face_count: 3 },
    )
    .expect("committed primitive input");
    let read_basis = committed.read_basis().clone();
    let mut query_runtime = HistoricalReadBasisQueryRuntime::open(
        &runtime,
        read_basis.clone(),
        "phase-six.equivalence-contract.snapshot",
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

fn build_report(
    inputs: &EquivalenceInputs,
    snapshot_id: u64,
    branch_id: &str,
    truth_digest: &str,
) -> DerivedEquivalenceContractReport {
    let read_basis = rebound_read_basis(&inputs.read_basis, snapshot_id, branch_id, truth_digest);
    build_derived_equivalence_contract(
        &read_basis,
        &inputs.materialized,
        &inputs.interpreted,
        &inputs.validation,
    )
}

fn rebound_read_basis(
    read_basis: &DerivedTopologyReadBasis,
    snapshot_id: u64,
    branch_id: &str,
    truth_digest: &str,
) -> DerivedTopologyReadBasis {
    use forge_relational::facade::history::BranchId;
    use forge_relational::facade::snapshots::SnapshotHandle;

    let mut rebound = read_basis.clone();
    rebound.authority.snapshot = SnapshotHandle::new(snapshot_id, 0);
    rebound.authority.branch_id = BranchId(branch_id.to_string());
    rebound.authority.authoritative_mutation_origin = MutationOrigin::Replay;
    rebound.authority.truth_basis_identity.mutation_digest_hex = truth_digest.to_string();
    rebound.derivation_origin = MutationOrigin::Replay;
    rebound
}

impl DerivedEquivalenceContractReport {
    fn with_test_shared_identity_fields_removed(mut self) -> Self {
        self.authority_truth_identity = None;
        self.compiled_product_identity = None;
        self.equivalence_policy_identity = None;
        self
    }

    fn with_test_topology_compiled_product_family_digest(mut self, digest: &str) -> Self {
        self.topology_compiled_product_family_digest = Some(digest.to_string());
        self
    }

    fn with_test_surface_digests_from(mut self, other: &Self) -> Self {
        self.materialized_topology_digest = other.materialized_topology_digest.clone();
        self.interpreted_topology_digest = other.interpreted_topology_digest.clone();
        self.derived_validation_digest = other.derived_validation_digest.clone();
        self
    }

    fn with_test_selected_comparator_dimensions(
        mut self,
        dimensions: Vec<crate::selected_equivalence_family::TopologySelectedEquivalenceDimension>,
    ) -> Self {
        self.selected_comparator_contract = Some(
            self.selected_comparator_contract
                .take()
                .expect("selected comparator contract")
                .with_test_equivalence_dimensions(dimensions),
        );
        self
    }
}
