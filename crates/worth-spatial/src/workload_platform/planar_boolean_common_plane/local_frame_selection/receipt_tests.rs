use super::super::denial::PlanarBooleanCommonPlaneLocalFrameSelectionDenialKind;
use super::super::validation::validate_local_frame_selection;
use super::receipt_test_support::{readiness_receipt, shared_plane_identity_receipt};
use super::*;

#[test]
fn local_frame_selection_denies_missing_topology_basis_identity() {
    run_with_large_stack(|| {
        let readiness = readiness_receipt();
        let denial = validate_local_frame_selection(
            &receipt_with_mutation(&readiness, |receipt| {
                receipt.topology_basis_identity.clear();
            }),
            &readiness,
        )
        .expect_err("missing topology basis must deny");

        assert_eq!(
            denial.kind(),
            PlanarBooleanCommonPlaneLocalFrameSelectionDenialKind::MissingTopologyBasisIdentity
        );
    });
}

#[test]
fn local_frame_selection_denies_frame_identity_drift() {
    run_with_large_stack(|| {
        let readiness = readiness_receipt();
        let denial = validate_local_frame_selection(
            &receipt_with_mutation(&readiness, |receipt| {
                receipt.frame_identity.push_str("-drift");
            }),
            &readiness,
        )
        .expect_err("frame-identity drift must deny");

        assert_eq!(
            denial.kind(),
            PlanarBooleanCommonPlaneLocalFrameSelectionDenialKind::FrameIdentityMismatch
        );
    });
}

#[test]
fn local_frame_selection_denies_topology_basis_drift() {
    run_with_large_stack(|| {
        let readiness = readiness_receipt();
        let denial = validate_local_frame_selection(
            &receipt_with_mutation(&readiness, |receipt| {
                receipt.topology_basis_identity.push_str("-drift");
            }),
            &readiness,
        )
        .expect_err("topology-basis drift must deny");

        assert_eq!(
            denial.kind(),
            PlanarBooleanCommonPlaneLocalFrameSelectionDenialKind::TopologyBasisIdentityMismatch
        );
    });
}

fn receipt_with_mutation(
    readiness: &crate::planar_contracts::contract_bundle::PlanarM7ReadinessReceipt,
    mutate: impl FnOnce(&mut PlanarBooleanCommonPlaneLocalFrameSelectionReceipt),
) -> PlanarBooleanCommonPlaneLocalFrameSelectionReceipt {
    let shared_plane = shared_plane_identity_receipt("phase7.1 local-frame denial shared plane");
    let mut receipt =
        PlanarBooleanCommonPlaneLocalFrameSelectionReceipt::from_shared_plane_identity_and_m7_readiness(
            &shared_plane,
            readiness,
        )
        .expect("baseline local-frame selection should certify");
    mutate(&mut receipt);
    receipt
}

fn run_with_large_stack(body: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .name("planar-boolean-common-plane-local-frame-selection-denial".to_string())
        .stack_size(16 * 1024 * 1024)
        .spawn(body)
        .expect("local-frame denial contract thread should spawn")
        .join()
        .expect("local-frame denial contract thread should finish");
}
