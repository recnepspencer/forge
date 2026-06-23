use super::super::PlanarBoolean7_0CloseoutError;

use super::support::{
    closeout_bundle, closeout_harness, closeout_harness_named, run_with_large_stack,
};

#[test]
fn m7_0_closeout_bundle_rejects_real_rows_from_different_boundaries() {
    run_with_large_stack(|| {
        let canonical = closeout_harness();
        let foreign = closeout_harness_named("phase7-closeout-foreign");

        let mut mixed_basis = closeout_bundle(&canonical);
        mixed_basis
            .rows
            .retain(|row| row.kind != super::super::PlanarBoolean7_0ProofRowKind::EntryBasis);
        let mixed_basis = mixed_basis
            .with_entry_basis_proof(&foreign.basis)
            .register()
            .expect_err("foreign basis proof must deny closeout");
        assert_eq!(
            mixed_basis,
            PlanarBoolean7_0CloseoutError::MismatchedProofBoundary("readiness basis boundary")
        );

        let mut mixed_outcome = closeout_bundle(&canonical);
        mixed_outcome.rows.retain(|row| {
            row.kind != super::super::PlanarBoolean7_0ProofRowKind::OutcomeProvenance
        });
        let mixed_outcome = mixed_outcome
            .with_outcome_and_provenance_proof(&foreign.outcome)
            .register()
            .expect_err("foreign outcome proof must deny closeout");
        assert_eq!(
            mixed_outcome,
            PlanarBoolean7_0CloseoutError::MismatchedProofBoundary("readiness basis boundary")
        );

        let mut mixed_anti_theatre = closeout_bundle(&canonical);
        mixed_anti_theatre
            .rows
            .retain(|row| row.kind != super::super::PlanarBoolean7_0ProofRowKind::AntiTheatre);
        let mixed_anti_theatre = mixed_anti_theatre
            .with_anti_theatre_proof(&foreign.anti_theatre_proof)
            .register()
            .expect_err("foreign anti-theatre proof must deny closeout");
        assert_eq!(
            mixed_anti_theatre,
            PlanarBoolean7_0CloseoutError::MismatchedProofBoundary("blocker provenance boundary")
        );
    });
}
