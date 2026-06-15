use super::super::{
    PlanarBoolean7_0CloseoutError, PlanarBoolean7_0ProofRow, PlanarBoolean7_0ProofRowKind,
    PlanarBoolean7_0ProofSource,
};
use worth_spatial::facade::workload_vocabulary::WorkloadEvidenceStage;

use super::support::{closeout_bundle, closeout_harness, run_with_large_stack};

#[test]
fn m7_0_closeout_bundle_rejects_duplicate_or_synthetic_boolean_rows() {
    run_with_large_stack(|| {
        let harness = closeout_harness();

        let mut duplicate = closeout_bundle(&harness);
        duplicate.rows.push(duplicate.rows[0].clone());
        let duplicate_error = duplicate
            .register()
            .expect_err("duplicate row must deny closeout");
        assert_eq!(
            duplicate_error,
            PlanarBoolean7_0CloseoutError::DuplicateProofRow(
                PlanarBoolean7_0ProofRowKind::DeclarationFamily
            )
        );
        assert!(duplicate_error.human_reason().contains("duplicate"));

        let mut synthetic = closeout_bundle(&harness);
        synthetic
            .rows
            .retain(|row| row.kind != PlanarBoolean7_0ProofRowKind::AntiTheatre);
        synthetic.rows.push(PlanarBoolean7_0ProofRow {
            kind: PlanarBoolean7_0ProofRowKind::AntiTheatre,
            identity: "synthetic-anti-theatre".to_string(),
            source: PlanarBoolean7_0ProofSource::Synthetic,
        });
        let synthetic_error = synthetic
            .register()
            .expect_err("synthetic row must deny closeout");
        assert_eq!(
            synthetic_error,
            PlanarBoolean7_0CloseoutError::SyntheticProofRow(
                PlanarBoolean7_0ProofRowKind::AntiTheatre
            )
        );
        assert!(synthetic_error.human_reason().contains("synthetic"));

        let mut future = closeout_bundle(&harness);
        future.rows.push(PlanarBoolean7_0ProofRow {
            kind: PlanarBoolean7_0ProofRowKind::FutureExecutionStage(
                WorkloadEvidenceStage::BooleanSplit,
            ),
            identity: "future-boolean-split".to_string(),
            source: PlanarBoolean7_0ProofSource::Synthetic,
        });
        let future_error = future
            .register()
            .expect_err("future execution proof must deny 7.0 closeout");
        assert_eq!(
            future_error,
            PlanarBoolean7_0CloseoutError::ForbiddenFutureExecutionRow(
                WorkloadEvidenceStage::BooleanSplit
            )
        );
        assert!(future_error
            .human_reason()
            .contains("future execution stage"));
    });
}
