use worth_spatial::facade::planar_boolean_loop_reconstruction::PlanarBooleanDegenerateLoopOutcomeKind;

use super::support::{completed_loop_products, run_with_large_stack, ReplayBranch};

#[test]
fn loop_reconstruction_phase_twelve_contract_preserves_typed_degenerate_loop_outcomes() {
    run_with_large_stack(|| {
        let label = "phase7.4 public degenerate loop contract";
        let original = completed_loop_products(label, ReplayBranch::Original);
        let replayed = completed_loop_products(label, ReplayBranch::Replayed);
        let expected_loop_count = original
            .reconstructed_boundary()
            .reconstructed_loops()
            .rows()
            .len()
            + original.reconstructed_boundary().born_loops().rows().len();

        assert_eq!(
            original.degenerate_outcomes().rows(),
            replayed.degenerate_outcomes().rows()
        );
        assert_eq!(
            original
                .degenerate_outcomes()
                .degenerate_loop_outcome_set_identity(),
            replayed
                .degenerate_outcomes()
                .degenerate_loop_outcome_set_identity()
        );
        assert_eq!(
            original.degenerate_outcomes().rows().len(),
            expected_loop_count
        );
        assert!(original.degenerate_outcomes().rows().iter().all(|row| {
            !row.degenerate_loop_outcome_identity().is_empty()
                && !row.local_frame_identity().is_empty()
                && !row.precision_basis_identity().is_empty()
        }));
        let admitted_count = original
            .degenerate_outcomes()
            .rows()
            .iter()
            .filter(|row| {
                row.kind() == PlanarBooleanDegenerateLoopOutcomeKind::AdmittedForIdentityMinting
            })
            .count();
        let tiny_count = original
            .degenerate_outcomes()
            .rows()
            .iter()
            .filter(|row| {
                row.kind() == PlanarBooleanDegenerateLoopOutcomeKind::DeniedTinyCardinality
            })
            .count();
        let self_touching_count = original
            .degenerate_outcomes()
            .rows()
            .iter()
            .filter(|row| row.kind() == PlanarBooleanDegenerateLoopOutcomeKind::DeniedSelfTouching)
            .count();
        let zero_area_count = original
            .degenerate_outcomes()
            .rows()
            .iter()
            .filter(|row| row.kind() == PlanarBooleanDegenerateLoopOutcomeKind::DeniedZeroArea)
            .count();
        let geometry_policy_required_count = original
            .degenerate_outcomes()
            .rows()
            .iter()
            .filter(|row| {
                row.kind() == PlanarBooleanDegenerateLoopOutcomeKind::PolicyRequiredGeometryEvidence
            })
            .count();
        let policy_required_count = original
            .degenerate_outcomes()
            .rows()
            .iter()
            .filter(|row| {
                matches!(
                    row.kind(),
                    PlanarBooleanDegenerateLoopOutcomeKind::PolicyRequiredRoleEvidence
                        | PlanarBooleanDegenerateLoopOutcomeKind::PolicyRequiredContainmentEvidence
                )
            })
            .count();
        assert_eq!(
            admitted_count
                + tiny_count
                + self_touching_count
                + zero_area_count
                + geometry_policy_required_count
                + policy_required_count,
            expected_loop_count
        );
        assert_eq!(admitted_count, 0);
        assert_eq!(tiny_count, 19);
        assert_eq!(self_touching_count, 0);
        assert_eq!(zero_area_count, 0);
        assert_eq!(geometry_policy_required_count, 4);
        assert_eq!(policy_required_count, 0);
        assert!(original.degenerate_outcomes().rows().iter().all(|row| {
            matches!(
                row.kind(),
                PlanarBooleanDegenerateLoopOutcomeKind::DeniedTinyCardinality
                    | PlanarBooleanDegenerateLoopOutcomeKind::PolicyRequiredGeometryEvidence
            )
        }));
        assert_eq!(
            original
                .reconstructed_boundary()
                .reconstructed_loops()
                .rows()
                .iter()
                .map(|row| row.reconstructed_loop_identity())
                .collect::<Vec<_>>(),
            original
                .degenerate_outcomes()
                .rows()
                .iter()
                .filter(|row| row.loop_kind()
                    == worth_spatial::facade::planar_boolean_loop_reconstruction::PlanarBooleanLoopClassifiedProductKind::ReconstructedLoop)
                .map(|row| row.loop_identity())
                .collect::<Vec<_>>()
        );

        let original_counters = original.degenerate_boundary().counters();
        let replayed_counters = replayed.degenerate_boundary().counters();
        assert_eq!(original_counters, replayed_counters);
        assert_eq!(original_counters.loops_consumed(), expected_loop_count);
        assert_eq!(
            original_counters.reconstructed_loops_consumed(),
            original
                .reconstructed_boundary()
                .reconstructed_loops()
                .rows()
                .len()
        );
        assert_eq!(
            original_counters.born_loops_consumed(),
            original.reconstructed_boundary().born_loops().rows().len()
        );
        assert_eq!(
            original_counters.admitted_for_identity_minting(),
            admitted_count
        );
        assert_eq!(
            original_counters.tiny_cardinality_outcomes_emitted(),
            tiny_count
        );
        assert_eq!(
            original_counters.self_touching_outcomes_emitted(),
            self_touching_count
        );
        assert_eq!(
            original_counters.zero_area_outcomes_emitted(),
            zero_area_count
        );
        assert_eq!(
            original_counters.geometry_policy_required_outcomes_emitted(),
            geometry_policy_required_count
        );
        assert_eq!(
            original_counters.policy_required_outcomes_emitted(),
            policy_required_count
        );
    });
}
