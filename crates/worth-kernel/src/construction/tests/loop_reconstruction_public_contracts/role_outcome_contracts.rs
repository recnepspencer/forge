use worth_spatial::facade::planar_boolean_loop_reconstruction::{
    PlanarBooleanLoopContainmentEvidencePostureKind, PlanarBooleanLoopRoleOutcomeKind,
};

use super::support::{completed_loop_products, run_with_large_stack, ReplayBranch};

#[test]
fn loop_reconstruction_role_contract_preserves_real_role_and_containment_outcomes() {
    run_with_large_stack(|| {
        let label = "phase7.4 public loop role contract";
        let original = completed_loop_products(label, ReplayBranch::Original);
        let replayed = completed_loop_products(label, ReplayBranch::Replayed);

        assert_eq!(
            original.role_outcomes().rows(),
            replayed.role_outcomes().rows()
        );
        assert_eq!(
            original.containment_postures().rows(),
            replayed.containment_postures().rows()
        );

        assert!(original
            .reconstructed_boundary()
            .reconstructed_loops()
            .rows()
            .iter()
            .all(|row| original
                .role_outcomes()
                .rows()
                .iter()
                .any(|outcome| { outcome.loop_identity() == row.reconstructed_loop_identity() })));
        assert!(original
            .reconstructed_boundary()
            .born_loops()
            .rows()
            .iter()
            .all(|row| original
                .role_outcomes()
                .rows()
                .iter()
                .any(|outcome| { outcome.loop_identity() == row.born_loop_identity() })));
        for born_loop in original.reconstructed_boundary().born_loops().rows() {
            let born_role_outcome = original
                .role_outcomes()
                .rows()
                .iter()
                .find(|row| row.loop_identity() == born_loop.born_loop_identity())
                .expect("every born loop should receive a typed role outcome");
            if born_loop.source_loop_identities().len() > 1 {
                assert_eq!(
                    born_role_outcome.kind(),
                    PlanarBooleanLoopRoleOutcomeKind::BornLoopRoleAmbiguous
                );
                assert!(born_role_outcome.preserved_source_role().is_none());
            }

            let born_containment_posture = original
                .containment_postures()
                .rows()
                .iter()
                .find(|row| row.loop_identity() == born_loop.born_loop_identity())
                .expect("every born loop should receive containment posture");
            if born_loop.source_loop_identities().len() > 1 {
                assert_eq!(
                    born_containment_posture.kind(),
                    PlanarBooleanLoopContainmentEvidencePostureKind::MultiSourceBornLoopContainmentEvidence
                );
            }
        }
    });
}
