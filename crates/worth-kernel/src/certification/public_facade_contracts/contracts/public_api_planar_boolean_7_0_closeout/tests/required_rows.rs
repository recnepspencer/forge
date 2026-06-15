use super::super::{PlanarBoolean7_0CloseoutError, PlanarBoolean7_0ProofRowKind};

use super::support::{closeout_bundle, closeout_harness, run_with_large_stack};

#[test]
fn m7_0_closeout_bundle_requires_all_boolean_entry_proof_rows() {
    run_with_large_stack(|| {
        let harness = closeout_harness();

        for missing in [
            PlanarBoolean7_0ProofRowKind::DeclarationFamily,
            PlanarBoolean7_0ProofRowKind::EntryBasis,
            PlanarBoolean7_0ProofRowKind::OutcomeProvenance,
            PlanarBoolean7_0ProofRowKind::CatalogRecipe,
            PlanarBoolean7_0ProofRowKind::EvidenceStage,
            PlanarBoolean7_0ProofRowKind::AntiTheatre,
        ] {
            let mut bundle = closeout_bundle(&harness);
            bundle.rows.retain(|row| row.kind != missing);

            let error = bundle
                .register()
                .expect_err("missing proof row must deny closeout");
            assert_eq!(
                error,
                PlanarBoolean7_0CloseoutError::MissingProofRow(missing)
            );
            assert!(error.human_reason().contains("missing"));
        }

        let registered = closeout_bundle(&harness)
            .register()
            .expect("full proof bundle should register");
        assert_eq!(
            registered.entry_boundary().digest,
            harness.evidence_proof.entry_boundary_digest()
        );
    });
}
