use super::contract_subject::structural_identity_receipt;

#[test]
fn retained_planar_history_cancellation_identity_rows() {
    let checkpoint = structural_identity_receipt("structural-cancellation", "topology:checkpoint");
    let regrouped = structural_identity_receipt("structural-cancellation", "topology:regrouped");

    assert_eq!(
        checkpoint.structural_identity_digest(),
        regrouped.structural_identity_digest()
    );
    assert_eq!(
        checkpoint.canonical_transform_basis_digest(),
        regrouped.canonical_transform_basis_digest()
    );
    assert_eq!(checkpoint.counters().structural_basis_rows_inspected(), 1);
    assert_eq!(checkpoint.counters().transform_basis_rows_inspected(), 4);
    assert_eq!(checkpoint.counters().contrast_identity_rows_inspected(), 4);
}
