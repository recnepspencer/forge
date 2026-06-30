use serde_json::Value;

use super::{
    build_derived_equivalence_contract, compare_derived_equivalence_contracts,
    DerivedEquivalenceContractReport,
};
use crate::certification::support::historical_query_snapshot::historical_query_snapshot_for_read_basis;
use crate::certification::support::read_basis_query_runtime::HistoricalReadBasisQueryRuntime;
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
    assert!(compare_derived_equivalence_contracts(&left, &right).equivalent_derived_meaning);
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
fn missing_shared_identity_fields_cannot_fallback_to_local_equivalence() {
    let inputs = real_equivalence_inputs();
    let left = build_report(&inputs, 7, "branch-a", "authority-a");
    let right = report_from_retained_json(&left, |json| {
        json["authority_truth_identity"] = Value::Null;
        json["compiled_product_identity"] = Value::Null;
        json["equivalence_policy_identity"] = Value::Null;
    });

    let comparison = compare_derived_equivalence_contracts(&left, &right);
    assert!(!comparison.authority_identity_match);
    assert!(!comparison.equivalent_derived_meaning);
}

#[test]
fn forged_family_digest_cannot_mint_equivalence_identity() {
    let inputs = real_equivalence_inputs();
    let left = build_report(&inputs, 7, "branch-a", "authority-a");
    let right = report_from_retained_json(&left, |json| {
        json["topology_compiled_product_family_digest"] =
            Value::String("forged-family".to_string());
    });

    let comparison = compare_derived_equivalence_contracts(&left, &right);
    assert!(!comparison.authority_identity_match);
    assert!(!comparison.equivalent_derived_meaning);
}

#[test]
fn shared_payload_instance_cannot_substitute_for_family_identity() {
    let inputs = real_equivalence_inputs();
    let left = build_report(&inputs, 7, "branch-a", "authority-a");
    let baseline = build_report(&inputs, 8, "branch-a", "authority-a");
    let right = report_from_retained_json(&baseline, |json| {
        json["materialized_topology_digest"] =
            serde_json::to_value(&left.materialized_topology_digest).expect("materialized digest");
        json["interpreted_topology_digest"] =
            serde_json::to_value(&left.interpreted_topology_digest).expect("interpreted digest");
        json["derived_validation_digest"] =
            serde_json::to_value(&left.derived_validation_digest).expect("validation digest");
    });

    let comparison = compare_derived_equivalence_contracts(&left, &right);
    assert!(!comparison.authority_identity_match);
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

fn report_from_retained_json(
    report: &DerivedEquivalenceContractReport,
    mutate: impl FnOnce(&mut Value),
) -> DerivedEquivalenceContractReport {
    let mut json = serde_json::to_value(report).expect("equivalence report should serialize");
    mutate(&mut json);
    serde_json::from_value(json).expect("retained-equivalence report should deserialize")
}
