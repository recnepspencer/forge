use super::proof_fixture::{
    frame_handle, local_frame_basis, local_frame_receipt, precision_handle, precision_receipt,
};

#[test]
fn local_frame_basis_survives_scale_separation() {
    let precision = precision_receipt(
        &precision_handle("thin-feature-scale-separation"),
        "movement:rotation-cancelled",
    );
    let basis = local_frame_basis(
        &precision,
        "movement:rotation-cancelled",
        "transform:move-rotate-cancelled",
    );
    let receipt = local_frame_receipt(&frame_handle("thin-feature-scale-separation"), basis);

    assert_eq!(receipt.scale_separation_orders(), 21);
    assert_eq!(receipt.basis().origin(), [1.0e12, 0.0, 0.0]);
    assert_eq!(receipt.basis().normalization_scale(), 1.0e-9);
    assert_eq!(receipt.precision_fact_digest(), precision.fact_digest());
    assert_eq!(receipt.counters().local_frame_derivations(), 1);
    assert_eq!(receipt.counters().retained_precision_receipts_consumed(), 1);
    assert_eq!(receipt.counters().normalization_basis_count(), 1);
    assert!(receipt.counters().basis_digest_part_count() >= 17);
}
