use super::*;
use crate::workload_composition::{
    ConflictInputAdmissionErrorKind, LookupConsumedWorkloadReuseProduct, WorkloadCompositionError,
};

#[test]
fn lookup_consumed_workload_caller_path_preserves_selected_equivalence_authority() {
    ordinary_consumer_sweep_tests_support_lookup_routes::run_stack_heavy_lookup_test(|| {
        let label = "phase-8-kernel-caller-selected-equivalence-proof";
        let completed_split_handoff =
            ordinary_consumer_sweep_tests_support_completed_split::ordinary_completed_split_handoff(label);
        let handoff = completed_split_handoff.lookup_consumed_workload_handoff();

        let admitted = completed_split_handoff
            .completed_workload()
            .admit_lookup_consumed_workload(handoff)
            .expect("kernel caller path should admit the real lookup-consumed workload handoff");

        assert_eq!(
            admitted.handoff().selected_equivalence_family_identity(),
            handoff.selected_equivalence_family_identity()
        );
        assert_eq!(
            admitted.handoff().selected_reuse_basis_identity_digest(),
            handoff.selected_reuse_basis_identity_digest()
        );
        assert_eq!(
            admitted.handoff().lookup_product_output_digest(),
            handoff.lookup_product_output_digest()
        );

        let route_inputs = ordinary_consumer_sweep_tests_support_conflict_route_inputs::lookup_conflict_route_inputs(
            &completed_split_handoff,
        );
        let downstream_admitted = admitted
            .admit_spatial_conflict_input(
                route_inputs.authority(),
                route_inputs.execution_receipt(),
            )
            .expect("kernel caller path should feed selected equivalence authority into downstream conflict admission");
        assert!(!downstream_admitted.admission_digest().is_empty());
    });
}

#[test]
fn lookup_consumed_workload_consumer_accepts_reused_resolution_without_local_fallback() {
    ordinary_consumer_sweep_tests_support_lookup_routes::run_stack_heavy_lookup_test(|| {
        let label = "phase-9-kernel-reused-resolution-consumer";
        let completed_split_handoff =
            ordinary_consumer_sweep_tests_support_completed_split::ordinary_completed_split_handoff(label);
        let admitted = completed_split_handoff
            .completed_workload()
            .admit_lookup_consumed_workload(
                completed_split_handoff.lookup_consumed_workload_handoff(),
            )
            .expect("kernel caller path should admit the real lookup-consumed workload handoff");
        let packet = ordinary_consumer_sweep_tests_support_lookup_routes::lookup_packet(&completed_split_handoff);
        let resolution = admitted
            .route_lookup_reuse_resolution(packet.selected_plan(), packet, packet.index_product())
            .expect("kernel top seam lowers the real packet-backed reuse route");

        let consumed = admitted
            .admit_lookup_reuse_resolution(&resolution)
            .expect("kernel consumer should accept the reused resolution product");

        match consumed {
            LookupConsumedWorkloadReuseProduct::Reused(product) => {
                assert_eq!(
                    product.selected_plan_digest(),
                    admitted.handoff().selected_lookup_plan_digest()
                );
                assert_eq!(
                    product.selected_equivalence_family_identity().as_str(),
                    admitted.handoff().selected_equivalence_family_identity()
                );
                assert_eq!(
                    product.selected_reuse_basis_identity_digest(),
                    admitted.handoff().selected_reuse_basis_identity_digest()
                );
            }
            other => panic!("expected reused resolution product, got {other:?}"),
        }
    });
}

#[test]
fn lookup_consumed_workload_consumer_rejects_hostile_resolution_without_self_healing() {
    ordinary_consumer_sweep_tests_support_lookup_routes::run_stack_heavy_lookup_test(|| {
        let label = "phase-9-kernel-hostile-resolution-consumer";
        let completed_split_handoff =
            ordinary_consumer_sweep_tests_support_completed_split::ordinary_completed_split_handoff(label);
        let hostile_handoff = completed_split_handoff
            .lookup_consumed_workload_handoff()
            .clone()
            .with_test_selected_equivalence_family_identity(
                "spatial.selected-equivalence.retained-replay-semantic-parity",
            );
        let admitted = completed_split_handoff
            .completed_workload()
            .admit_lookup_consumed_workload(&hostile_handoff)
            .expect("kernel caller path should admit the hostile handoff before typed resolution consumption");
        let packet = ordinary_consumer_sweep_tests_support_lookup_routes::lookup_packet(&completed_split_handoff);
        let error = admitted
            .route_lookup_reuse_resolution(
                packet.selected_plan(),
                packet,
                packet.index_product(),
            )
            .expect_err("kernel top seam must reject packet-backed reuse whose selected-equivalence authority disagrees with the admitted handoff");

        assert!(matches!(
            error.lookup_consumed_workload_denial(),
            Some(crate::workload_composition::LookupConsumedWorkloadDenial::ReuseResolutionSelectedFamilyMismatch)
        ));
    });
}

#[test]
fn lookup_consumed_workload_caller_path_rejects_forged_selected_family_downstream() {
    ordinary_consumer_sweep_tests_support_lookup_routes::run_stack_heavy_lookup_test(|| {
        let label = "phase-8-kernel-caller-selected-family-hostile";
        let completed_split_handoff =
            ordinary_consumer_sweep_tests_support_completed_split::ordinary_completed_split_handoff(label);
        let hostile_handoff = completed_split_handoff
            .lookup_consumed_workload_handoff()
            .clone()
            .with_test_selected_equivalence_family_identity(
                "spatial.selected-equivalence.retained-replay-semantic-parity",
            );
        let admitted = completed_split_handoff
            .completed_workload()
            .admit_lookup_consumed_workload(&hostile_handoff)
            .expect("kernel caller path itself still admits the hostile handoff before downstream route consumption");
        let route_inputs = ordinary_consumer_sweep_tests_support_conflict_route_inputs::lookup_conflict_route_inputs(
            &completed_split_handoff,
        );

        let error = match admitted.admit_spatial_conflict_input(
            route_inputs.authority(),
            route_inputs.execution_receipt(),
        ) {
            Ok(_) => {
                panic!("downstream conflict admission must reject forged selected family authority")
            }
            Err(error) => error,
        };

        match error {
            WorkloadCompositionError::ConflictInput(error) => {
                assert_eq!(
                    error.kind(),
                    ConflictInputAdmissionErrorKind::WrongReceiptFamily
                );
            }
            other => panic!("expected conflict-input error, got {other:?}"),
        }
    });
}

#[test]
fn lookup_consumed_workload_caller_path_rejects_forged_selected_reuse_basis_downstream() {
    ordinary_consumer_sweep_tests_support_lookup_routes::run_stack_heavy_lookup_test(|| {
        let label = "phase-8-kernel-caller-selected-reuse-basis-hostile";
        let completed_split_handoff =
            ordinary_consumer_sweep_tests_support_completed_split::ordinary_completed_split_handoff(label);
        let hostile_handoff = completed_split_handoff
            .lookup_consumed_workload_handoff()
            .clone()
            .with_test_selected_reuse_basis_identity_digest("forged.selected-reuse-basis");
        let admitted = completed_split_handoff
            .completed_workload()
            .admit_lookup_consumed_workload(&hostile_handoff)
            .expect("kernel caller path itself still admits the hostile handoff before downstream route consumption");
        let route_inputs = ordinary_consumer_sweep_tests_support_conflict_route_inputs::lookup_conflict_route_inputs(
            &completed_split_handoff,
        );

        let error = match admitted.admit_spatial_conflict_input(
            route_inputs.authority(),
            route_inputs.execution_receipt(),
        ) {
            Ok(_) => panic!(
                "downstream conflict admission must reject forged selected reuse basis authority"
            ),
            Err(error) => error,
        };

        match error {
            WorkloadCompositionError::ConflictInput(error) => {
                assert_eq!(
                    error.kind(),
                    ConflictInputAdmissionErrorKind::WrongReceiptFamily
                );
            }
            other => panic!("expected conflict-input error, got {other:?}"),
        }
    });
}
