use worth_spatial::facade::evidence_lookup_index_product::EvidenceLookupReuseMismatchLocus;

use super::super::super::LookupConsumedWorkloadDenial;

#[path = "../ordinary_consumer_sweep/tests_support.rs"]
mod ordinary_consumer_sweep_tests_support;

#[test]
fn denied_reuse_resolution_preserves_localized_kernel_boundary_meaning() {
    ordinary_consumer_sweep_tests_support::run_stack_heavy_lookup_test(|| {
        let label = "phase-9-kernel-denied-resolution-localization";
        let completed_split_handoff =
            ordinary_consumer_sweep_tests_support::ordinary_completed_split_handoff(label);
        let admitted = completed_split_handoff
            .completed_workload()
            .admit_lookup_consumed_workload(
                completed_split_handoff.lookup_consumed_workload_handoff(),
            )
            .expect("kernel caller path should admit the real lookup-consumed workload handoff");
        let packet = ordinary_consumer_sweep_tests_support::lookup_packet(&completed_split_handoff);
        let hostile_prior_product = packet
            .index_product()
            .clone()
            .with_test_selected_equivalence_family_identity(
                "spatial.selected-equivalence.retained-replay-semantic-parity",
            );
        let resolution = admitted
            .route_lookup_reuse_resolution(packet.selected_plan(), packet, &hostile_prior_product)
            .expect("kernel top seam lowers the denied packet-backed route");
        let carried_denial = resolution
            .denial()
            .expect("denied resolution should carry the spatial denial product");

        let error = admitted
            .admit_lookup_reuse_resolution(&resolution)
            .expect_err("kernel consumer must preserve denied reuse as a typed boundary denial");

        let Some(LookupConsumedWorkloadDenial::ReuseResolutionDenied(denial)) =
            error.lookup_consumed_workload_denial()
        else {
            panic!("expected typed denied reuse boundary");
        };

        assert_eq!(
            denial.mismatch_loci(),
            &[EvidenceLookupReuseMismatchLocus::SelectedEquivalenceFamilyIdentity]
        );
        assert_eq!(
            denial.selected_equivalence_family_identity(),
            carried_denial.selected_equivalence_family_identity()
        );
        assert_eq!(
            denial.selected_equivalence_basis_identity_digest(),
            carried_denial.selected_equivalence_basis_identity_digest()
        );
        assert_eq!(
            denial.selected_compatibility_basis_identity_digest(),
            carried_denial.selected_compatibility_basis_identity_digest()
        );
        assert_eq!(
            denial.selected_reuse_basis_identity_digest(),
            carried_denial.selected_reuse_basis_identity_digest()
        );
        assert_eq!(
            denial.selected_equivalence_family_identity().as_str(),
            admitted.handoff().selected_equivalence_family_identity()
        );
        assert_eq!(
            denial.selected_reuse_basis_identity_digest(),
            admitted.handoff().selected_reuse_basis_identity_digest()
        );
        assert!(!denial.denial_identity_digest().is_empty());
        assert!(denial.counters().selected_basis_row_count() > 0);
    });
}

#[test]
fn rebuild_required_resolution_stays_a_real_consumer_success_posture() {
    ordinary_consumer_sweep_tests_support::run_stack_heavy_lookup_test(|| {
        let label = "phase-9-kernel-rebuild-resolution-consumer";
        let completed_split_handoff =
            ordinary_consumer_sweep_tests_support::ordinary_completed_split_handoff(label);
        let admitted = completed_split_handoff
            .completed_workload()
            .admit_lookup_consumed_workload(
                completed_split_handoff.lookup_consumed_workload_handoff(),
            )
            .expect("kernel caller path should admit the real lookup-consumed workload handoff");
        let packet = ordinary_consumer_sweep_tests_support::lookup_packet(&completed_split_handoff);
        let hostile_prior_product = packet
            .index_product()
            .clone()
            .with_test_selected_reuse_basis_identity_digest(
                "phase-9-kernel-rebuild-required-selected-reuse-basis",
            );
        let resolution = admitted
            .route_lookup_reuse_resolution(packet.selected_plan(), packet, &hostile_prior_product)
            .expect("kernel top seam lowers the rebuild-required packet-backed route");

        let consumed = admitted
            .admit_lookup_reuse_resolution(&resolution)
            .expect("kernel consumer should keep rebuild-required as a success posture");

        let crate::workload_composition::LookupConsumedWorkloadReuseProduct::Rebuilt(product) =
            consumed
        else {
            panic!("expected rebuilt resolution product");
        };

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
        assert_eq!(product.reuse_decision_identity_digest(), None);
    });
}
