use super::super::{LaterPlanarBooleanMilestoneBoundary, PlanarBoolean7_0CloseoutError};

use super::support::{closeout_bundle, closeout_harness, run_with_large_stack};

#[test]
fn later_boolean_milestones_must_consume_registered_7_0_entry_boundary() {
    run_with_large_stack(|| {
        let missing_error = LaterPlanarBooleanMilestoneBoundary::try_from_registered_closeout(None)
            .expect_err("later milestone must reject missing registration");
        assert_eq!(
            missing_error,
            PlanarBoolean7_0CloseoutError::MissingRegisteredEntryBoundary
        );
        assert!(missing_error
            .human_reason()
            .contains("registered 7.0 entry boundary"));

        let harness = closeout_harness();
        let registered = closeout_bundle(&harness)
            .register()
            .expect("full closeout bundle should register");
        let downstream =
            LaterPlanarBooleanMilestoneBoundary::try_from_registered_closeout(Some(&registered))
                .expect("registered closeout should hand off entry boundary");

        assert_eq!(
            downstream.consumed_entry_boundary_digest(),
            harness.evidence_proof.entry_boundary_digest()
        );
        assert!(downstream
            .handoff_note()
            .contains("registered 7.0 entry boundary"));
    });
}
