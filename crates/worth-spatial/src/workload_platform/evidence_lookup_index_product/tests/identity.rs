use super::fixtures::{complete_ledger_for_plan, IndexProductSubject};
use crate::workload_platform::evidence_lookup_index_product::admit_evidence_lookup_index_product;

#[test]
fn index_product_identity_depends_on_selected_plan_and_basis() {
    let selected_plan = IndexProductSubject::sparse_event_ledger().select_plan();
    let ledger = complete_ledger_for_plan(&selected_plan);

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

    let changed_plan = IndexProductSubject::dense_projection_consumption().select_plan();
    let changed_ledger = complete_ledger_for_plan(&changed_plan);
    let changed = admit_evidence_lookup_index_product(&changed_plan, &changed_ledger)
        .expect("changed subject index");
    assert_ne!(left.index_product_digest(), changed.index_product_digest());
}
