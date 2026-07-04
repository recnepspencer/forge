use crate::workload_platform::evidence_ledger::WorkloadEvidenceStage;
use crate::workload_platform::evidence_lookup_family_catalog::current_evidence_lookup_family_catalog;
use crate::workload_platform::evidence_lookup_stage_cutover::current_path::admit_current_family_stage_cutover_path;

use super::planning_seed::EvidenceLookupMilestoneTwelveReplayPlanningSeed;
use super::EvidenceLookupConsumedWorkloadHandoff;

#[test]
fn milestone_twelve_seed_has_lookup_scope_without_rescan() {
    let catalog = current_evidence_lookup_family_catalog().expect("family catalog");
    let family = catalog
        .families_for_stage(
            WorkloadEvidenceStage::BooleanEventLedger,
            &crate::workload_platform::evidence_lookup_family_catalog::EvidenceLookupStageReceiptFamilyIdentity::boolean_event_ledger(),
        )
        .family_identities()
        .first()
        .and_then(|identity| catalog.family_by_identity(identity))
        .expect("covered family declaration")
        .clone();
    let path = admit_current_family_stage_cutover_path(
        &catalog,
        &family,
        WorkloadEvidenceStage::BooleanEventLedger,
    )
    .expect("current cutover path");
    let proof = path
        .prove_for_family(family.identity().as_str())
        .expect("proof");

    let handoff =
        EvidenceLookupConsumedWorkloadHandoff::lower_from_stage_proof(&proof).expect("handoff");
    let replay_seed = EvidenceLookupMilestoneTwelveReplayPlanningSeed::admit_from_handoff(&handoff)
        .expect("milestone twelve replay planning seed");

    assert_eq!(
        replay_seed
            .milestone_twelve_seed()
            .selected_lookup_plan_digest(),
        proof.selected_lookup_plan_digest()
    );
    assert_eq!(
        replay_seed
            .milestone_twelve_seed()
            .lookup_execution_receipt_digest(),
        proof.lookup_execution_receipt_digest()
    );
    assert_eq!(
        replay_seed
            .milestone_twelve_seed()
            .lookup_product_output_digest(),
        proof.lookup_product_output_digest()
    );
    assert_eq!(
        replay_seed.stage_receipt_identity(),
        handoff.stage_receipt_identity()
    );
    assert_eq!(
        replay_seed.workload_stage_index_identity(),
        handoff.workload_stage_index_identity()
    );
    assert_eq!(
        replay_seed
            .milestone_twelve_seed()
            .covered_family_identities(),
        handoff.milestone_twelve_seed().covered_family_identities()
    );
    assert_eq!(replay_seed.indexed_lookup_count(), 1);
    assert_eq!(
        replay_seed.topology_receipt_ref_count(),
        handoff.counters().topology_receipt_ref_count()
    );
    assert_eq!(
        handoff
            .milestone_twelve_seed()
            .selected_lookup_plan_digest(),
        proof.selected_lookup_plan_digest()
    );
    assert_eq!(
        handoff.selected_equivalence_family_identity(),
        proof.selected_equivalence_family_identity()
    );
    assert_eq!(
        handoff.selected_reuse_basis_identity_digest(),
        proof.selected_reuse_basis_identity_digest()
    );
    assert_eq!(handoff.counters().raw_row_scan_count(), 0);
    assert_eq!(handoff.counters().broad_receipt_scan_count(), 0);
    assert_eq!(handoff.counters().caller_owned_scan_count(), 0);
}
