use crate::graph_read_access_plan_adoption::{
    WorthGraphReadAccessHardDeletionCappedResidueReport, WorthGraphReadAccessHardDeletionErrorKind,
    WorthGraphReadAccessHardDeletionProofReport, WorthGraphReadAccessHardDeletionProofRow,
    WorthGraphReadAccessHardDeletionStatus,
};

use super::{production_phase_seven_closeout, production_phase_seven_seed, TempWorkspace};

#[test]
fn migrated_local_execution_paths_are_deleted_or_capped() {
    let closeout = production_phase_seven_closeout();
    let deletion_proof = closeout.deletion_proof_report();

    assert!(deletion_proof.all_migrated_paths_deleted_or_capped());
    assert!(deletion_proof.deleted_count() > 0);
    assert_eq!(deletion_proof.unresolved_count(), 0);
    let source_seed = production_phase_seven_seed();
    assert!(deletion_proof.rows().iter().any(|row| {
        row.label() == "phase_four_vertical_slice_cutover_proof"
            && row.source_path()
                == source_seed
                    .phase_four_cutover_proof()
                    .deletion_target_identity()
            && row.row_digest() != source_seed.phase_four_cutover_proof().cutover_digest()
    }));
    assert_eq!(
        deletion_proof.rows().len(),
        deletion_proof.deleted_count()
            + deletion_proof.capped_residue_count()
            + deletion_proof.typed_query_gap_count()
    );
    assert!(deletion_proof.rows().iter().all(|row| {
        row.status() != WorthGraphReadAccessHardDeletionStatus::CappedResidue
            || (row.blocker().is_some() && !row.removal_trigger().is_empty())
    }));
    assert_eq!(closeout.capped_residue_report().uncapped_residue_count(), 0);
}

#[test]
fn existing_migrated_target_path_fails_closeout() {
    let workspace = TempWorkspace::new("unresolved_target");
    workspace.write_source(
        "crates/worth-kernel/src/query_adoption/graph_read_access/mod.rs",
        "pub fn still_here() {}",
    );

    let err = super::super::closeout::closeout_for_workspace_root(
        &production_phase_seven_seed(),
        workspace.root(),
    )
    .expect_err("an old migrated execution target still on disk must fail closeout");

    assert_eq!(
        WorthGraphReadAccessHardDeletionErrorKind::UnresolvedMigratedExecutionPath,
        err.kind()
    );
}

#[test]
fn zero_cap_rejects_capped_residue_rows() {
    let capped_row = WorthGraphReadAccessHardDeletionProofRow::capped_residue_for_test(
        "crates/worth-kernel/src/query_adoption/graph_read_access",
        "worth-kernel.graph_read_access_plan_adoption",
        "temporary residue still exists",
        "delete once Query access-plan receipt proves the migration",
    );
    let deletion_proof =
        WorthGraphReadAccessHardDeletionProofReport::from_rows_for_test(vec![capped_row]);

    let err =
        WorthGraphReadAccessHardDeletionCappedResidueReport::from_deletion_proof(&deletion_proof)
            .expect_err("hard-deletion residue cap defaults to zero");

    assert_eq!(
        WorthGraphReadAccessHardDeletionErrorKind::CappedResidueCapExceeded,
        err.kind()
    );
}
