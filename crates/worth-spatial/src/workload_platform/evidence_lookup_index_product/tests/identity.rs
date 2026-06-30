use super::fixtures::{selected_lookup_slice_for_plan, IndexProductSubject};
use crate::workload_platform::evidence_lookup_index_product::admit_evidence_lookup_index_product;
use crate::workload_platform::evidence_lookup_index_product::identity::lower_index_family_identity;

#[test]
fn index_product_identity_depends_on_selected_plan_and_basis() {
    let selected_plan = IndexProductSubject::sparse_event_ledger().select_plan();
    let ledger = selected_lookup_slice_for_plan(&selected_plan);

    let left = admit_evidence_lookup_index_product(&selected_plan, &ledger).expect("left index");
    let right = admit_evidence_lookup_index_product(&selected_plan, &ledger).expect("right index");

    assert_eq!(left.index_product_digest(), right.index_product_digest());
    assert_eq!(
        left.evidence_ledger_basis_digest(),
        right.evidence_ledger_basis_digest()
    );
    assert_eq!(
        left.selected_plan_digest(),
        selected_plan.selected_plan_digest()
    );
    assert_eq!(
        left.topology_support_digest(),
        right.topology_support_digest()
    );
    assert_eq!(left.query_support_digest(), right.query_support_digest());
    assert_eq!(
        left.disposal_posture().kind(),
        right.disposal_posture().kind()
    );
    assert_eq!(
        left.compiled_product_identity_digest(),
        right.compiled_product_identity_digest()
    );
    assert_eq!(
        left.equivalence_policy_identity_digest(),
        right.equivalence_policy_identity_digest()
    );
    assert_eq!(left.reuse_decision_identity_digest(), None);
    assert_eq!(right.reuse_decision_identity_digest(), None);
    let lowered_identity = lower_index_family_identity(&selected_plan, &ledger);
    assert_eq!(
        left.compiled_product_identity_digest(),
        lowered_identity
            .compiled_product_identity()
            .identity_digest()
    );
    assert_eq!(
        left.equivalence_policy_identity_digest(),
        lowered_identity
            .equivalence_policy_identity()
            .identity_digest()
    );

    let changed_plan = IndexProductSubject::dense_projection_consumption().select_plan();
    let changed_ledger = selected_lookup_slice_for_plan(&changed_plan);
    let changed = admit_evidence_lookup_index_product(&changed_plan, &changed_ledger)
        .expect("changed subject index");
    assert_ne!(left.index_product_digest(), changed.index_product_digest());
    assert_ne!(
        left.compiled_product_identity_digest(),
        changed.compiled_product_identity_digest()
    );
}

#[test]
fn index_product_identity_changes_with_real_stage_authority_change() {
    let baseline_subject = IndexProductSubject::dense_projection_consumption();
    let changed_subject = IndexProductSubject::dense_projection_consumption_with_world(
        "phase-5-foreign-projection-consumption-receipt",
    );
    let baseline_plan = baseline_subject.select_plan();
    let changed_plan = changed_subject.select_plan();
    let baseline_ledger = selected_lookup_slice_for_plan(&baseline_plan);
    let changed_ledger = selected_lookup_slice_for_plan(&changed_plan);

    let baseline = admit_evidence_lookup_index_product(&baseline_plan, &baseline_ledger)
        .expect("baseline admitted index");
    let changed = admit_evidence_lookup_index_product(&changed_plan, &changed_ledger)
        .expect("changed admitted index");

    assert_eq!(baseline_plan.stage(), changed_plan.stage());
    assert_ne!(
        baseline.stage_receipt_digest(),
        changed.stage_receipt_digest()
    );
    assert_ne!(
        baseline.compiled_product_identity_digest(),
        changed.compiled_product_identity_digest()
    );
    assert_ne!(
        baseline.index_product_digest(),
        changed.index_product_digest()
    );
}
