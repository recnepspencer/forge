use super::current::{current_matrix_authority, current_worth_touched_graph_cross_family_closeout_matrix};
use super::matrix::WorthTouchedGraphCrossFamilyCloseoutMatrix;
use super::row::WorthTouchedGraphCrossFamilyCloseoutMatrixRow;
use super::validation::{
    validate_closeout_matrix, WorthTouchedGraphCrossFamilyCloseoutMatrixErrorKind,
};
use crate::workload_composition::{
    current_live_coverage_ledger, current_representative_selected_route_parity_path,
    current_touched_graph_readiness_handoff,
    current_worth_touched_graph_conflict_public_closeout,
};

fn run_stack_heavy_closeout_matrix_test(test: impl FnOnce() + Send + 'static) {
    let result = std::thread::Builder::new()
        .stack_size(256 * 1024 * 1024)
        .spawn(test)
        .expect("closeout-matrix test should spawn on a larger stack")
        .join();
    if let Err(panic_payload) = result {
        std::panic::resume_unwind(panic_payload);
    }
}

#[test]
fn cross_family_closeout_matrix_rejects_uncertified_family_kinds() {
    run_stack_heavy_closeout_matrix_test(|| {
        let matrix = current_worth_touched_graph_cross_family_closeout_matrix()
            .expect("current cross-family closeout matrix");
        let live_ledger = current_live_coverage_ledger().expect("live coverage ledger");
        let readiness = current_touched_graph_readiness_handoff().expect("current readiness");
        let representative_path =
            current_representative_selected_route_parity_path().expect("representative path");
        let closeout = current_worth_touched_graph_conflict_public_closeout()
            .expect("current public closeout");
        let authority = current_matrix_authority(
            &live_ledger,
            &readiness,
            &representative_path,
            closeout.architecture_alignment_report().deleted_authority_rows(),
        )
        .expect("current matrix authority");

        let hostile_rows = matrix
            .rows()
            .iter()
            .enumerate()
            .map(|(index, row)| {
                if index == 0 {
                    WorthTouchedGraphCrossFamilyCloseoutMatrixRow::new(
                        row.family_kind(),
                        row.covered_surface_count(),
                        row.representative_path_covered(),
                        false,
                        row.public_proof_parity_passed(),
                        row.diagnostic_parity_passed(),
                        row.readiness_handoff_passed(),
                        row.deleted_count(),
                        row.capped_residue_count(),
                        row.query_gap_count(),
                        row.blocked_outside_roadmap_count(),
                    )
                } else {
                    row.clone()
                }
            })
            .collect::<Vec<_>>();
        let hostile = WorthTouchedGraphCrossFamilyCloseoutMatrix::new(
            hostile_rows,
            matrix.closeout_architecture_claim_digest(),
        );

        let error = validate_closeout_matrix(&hostile, &authority)
            .expect_err("uncertified family kinds must fail closeout matrix validation");
        assert_eq!(
            error.kind(),
            WorthTouchedGraphCrossFamilyCloseoutMatrixErrorKind::MissingFamilyParity
        );
    });
}

#[test]
fn cross_family_closeout_matrix_rejects_missing_public_projection_certification() {
    run_stack_heavy_closeout_matrix_test(|| {
        let matrix = current_worth_touched_graph_cross_family_closeout_matrix()
            .expect("current cross-family closeout matrix");
        let live_ledger = current_live_coverage_ledger().expect("live coverage ledger");
        let readiness = current_touched_graph_readiness_handoff().expect("current readiness");
        let representative_path =
            current_representative_selected_route_parity_path().expect("representative path");
        let closeout = current_worth_touched_graph_conflict_public_closeout()
            .expect("current public closeout");
        let mut authority = current_matrix_authority(
            &live_ledger,
            &readiness,
            &representative_path,
            closeout.architecture_alignment_report().deleted_authority_rows(),
        )
        .expect("current matrix authority");
        authority.public_proof_parity_passed = false;

        let error = validate_closeout_matrix(&matrix, &authority)
            .expect_err("missing public-proof certification must fail matrix validation");
        assert_eq!(
            error.kind(),
            WorthTouchedGraphCrossFamilyCloseoutMatrixErrorKind::MissingPublicProjectionParity
        );
    });
}
