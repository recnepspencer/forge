use crate::workload_composition::{
    current_conflict_batch_admission_inventory, worth_workload_ordinary_consumer_residue_rows,
    ConflictBatchAdmissionDisposition, ConflictBatchAdmissionReplacementPhase,
    ConflictBatchAdmissionSurfaceIdentity,
};

#[test]
fn phase_eleven_consumer_sweep_rows_are_exact() {
    let inventory =
        current_conflict_batch_admission_inventory().expect("current inventory should build");
    let rows = inventory
        .rows()
        .iter()
        .filter(|row| {
            row.replacement_phase()
                == ConflictBatchAdmissionReplacementPhase::PhaseElevenConsumerSweep
        })
        .collect::<Vec<_>>();

    assert_eq!(rows.len(), 5);
    assert_phase_eleven_row(
        &inventory,
        ConflictBatchAdmissionSurfaceIdentity::WorthWorkloadAdmitLookupConsumedWorkload,
        ConflictBatchAdmissionDisposition::Migrate,
    );
    assert_phase_eleven_row(
        &inventory,
        ConflictBatchAdmissionSurfaceIdentity::CompletedBooleanSplitHandoffAdmitDownstreamSplitConsumption,
        ConflictBatchAdmissionDisposition::Migrate,
    );
    assert_phase_eleven_row(
        &inventory,
        ConflictBatchAdmissionSurfaceIdentity::BooleanSplitReplayUndoBoundaryAdmission,
        ConflictBatchAdmissionDisposition::Migrate,
    );
    assert_phase_eleven_row(
        &inventory,
        ConflictBatchAdmissionSurfaceIdentity::PlanarBooleanLoopRuntimeRegistrationProof,
        ConflictBatchAdmissionDisposition::Cap,
    );
    assert_phase_eleven_row(
        &inventory,
        ConflictBatchAdmissionSurfaceIdentity::BooleanChainIntegrationHandoff,
        ConflictBatchAdmissionDisposition::Cap,
    );
}

#[test]
fn phase_eleven_residue_rows_match_worth_workload_consumer_sweep_manifest() {
    let inventory =
        current_conflict_batch_admission_inventory().expect("current inventory should build");
    let residue_rows = worth_workload_ordinary_consumer_residue_rows();

    assert_eq!(residue_rows.len(), 2);
    for residue_row in residue_rows {
        let inventory_row = inventory
            .rows()
            .iter()
            .find(|row| row.surface_name() == residue_row.surface().surface_name())
            .expect("phase 11 residue surface should stay inventoried");
        assert_eq!(
            inventory_row.disposition(),
            ConflictBatchAdmissionDisposition::Cap
        );
        assert!(
            inventory_row.blocker().contains(residue_row.blocker()),
            "inventory blocker must preserve the residue blocker for {}",
            residue_row.surface().surface_name(),
        );
    }
}

fn assert_phase_eleven_row(
    inventory: &crate::workload_composition::ConflictBatchAdmissionInventory,
    surface: ConflictBatchAdmissionSurfaceIdentity,
    disposition: ConflictBatchAdmissionDisposition,
) {
    let row = inventory
        .row_for_surface(surface)
        .expect("phase 11 surface should exist");
    assert_eq!(
        row.replacement_phase(),
        ConflictBatchAdmissionReplacementPhase::PhaseElevenConsumerSweep
    );
    assert_eq!(row.disposition(), disposition);
}
