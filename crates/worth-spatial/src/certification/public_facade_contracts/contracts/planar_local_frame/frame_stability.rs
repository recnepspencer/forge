use super::proof_fixture::{
    frame_handle, local_frame_basis, local_frame_receipt, precision_handle, precision_receipt,
};

#[test]
fn planar_local_frame_certificate_is_stable_under_equivalent_translation_and_rotation() {
    let precision_world = precision_handle("stable");
    let precision_a = precision_receipt(&precision_world, "movement:rotation-cancelled");
    let precision_b = precision_receipt(&precision_world, "movement:rotation-cancelled");
    let frame = frame_handle("stable");

    let receipt_a = local_frame_receipt(
        &frame,
        local_frame_basis(
            &precision_a,
            "movement:rotation-cancelled",
            "transform:canonical-planar-equivalence",
        ),
    );
    let receipt_b = local_frame_receipt(
        &frame,
        local_frame_basis(
            &precision_b,
            "movement:rotation-cancelled",
            "transform:canonical-planar-equivalence",
        ),
    );

    assert_eq!(receipt_a.fact_digest(), receipt_b.fact_digest());
    assert_eq!(receipt_a.basis().u_axis(), receipt_b.basis().u_axis());
    assert_eq!(receipt_a.basis().v_axis(), receipt_b.basis().v_axis());
    assert_eq!(receipt_a.basis().w_axis(), [0.0, 0.0, 1.0]);
}

#[test]
fn planar_local_frame_certificate_digest_changes_with_movement_rotation_posture() {
    let precision_a =
        precision_receipt(&precision_handle("motion-a"), "movement:rotation-cancelled");
    let precision_b = precision_receipt(
        &precision_handle("motion-b"),
        "movement:translated-large-world",
    );
    let frame = frame_handle("motion");

    let receipt_a = local_frame_receipt(
        &frame,
        local_frame_basis(
            &precision_a,
            "movement:rotation-cancelled",
            "transform:canonical-planar-equivalence",
        ),
    );
    let receipt_b = local_frame_receipt(
        &frame,
        local_frame_basis(
            &precision_b,
            "movement:translated-large-world",
            "transform:canonical-planar-equivalence",
        ),
    );

    assert_ne!(receipt_a.fact_digest(), receipt_b.fact_digest());
}

#[test]
fn planar_local_frame_certificate_digest_changes_with_transform_chain() {
    let precision = precision_receipt(
        &precision_handle("transform-chain"),
        "movement:rotation-cancelled",
    );
    let frame = frame_handle("transform-chain");

    let canonical_receipt = local_frame_receipt(
        &frame,
        local_frame_basis(
            &precision,
            "movement:rotation-cancelled",
            "transform:canonical-planar-equivalence",
        ),
    );
    let alternate_receipt = local_frame_receipt(
        &frame,
        local_frame_basis(
            &precision,
            "movement:rotation-cancelled",
            "transform:non-equivalent-planar-history",
        ),
    );

    assert_ne!(
        canonical_receipt.fact_digest(),
        alternate_receipt.fact_digest(),
        "transform-chain digest must participate in retained local-frame identity"
    );
}
