use super::test_fixtures::*;
use super::{
    WorthGraphReadAccessCappedResidueRow, WorthGraphReadAccessClassification,
    WorthGraphReadAccessCostPosture, WorthGraphReadAccessDeletionAction,
    WorthGraphReadAccessInventoryErrorKind, WorthGraphReadAccessInventoryRow,
    WorthGraphReadAccessInventorySeed, WorthGraphReadAccessMilestoneSevenDisposition,
    WorthGraphReadAccessOwner,
};

#[test]
fn parallel_graph_read_inventory_lane_accepts_milestone_five_seed_facts_only() {
    let seed = WorthGraphReadAccessInventorySeed::for_tests();

    assert_eq!(seed.selected_obligation_count(), 2);
    assert_eq!(seed.selected_registration_count(), 2);
    assert_eq!(seed.execution_row_count(), 2);
    assert_no_empty_digest(seed.authority_digests());
    assert_no_empty_digest(seed.touch_descriptor_digests());
    assert_no_empty_digest(seed.selected_registration_digests());
    assert_no_empty_digest(seed.residue_manifest_digests());
    assert_no_empty_digest(seed.execution_proof_digests());
    assert_no_empty_digest(seed.adoption_manifest_digests());
    assert_no_empty_digest(seed.selector_precision_report_digests());

    assert_seed_error(
        seed_parts_with_authority_digests(Vec::new()),
        WorthGraphReadAccessInventoryErrorKind::MissingAuthorityDigest,
    );
    assert_seed_error(
        seed_parts_with_selected_obligation_count(0),
        WorthGraphReadAccessInventoryErrorKind::MissingSelectedObligations,
    );
    assert_seed_error(
        seed_parts_with_selected_registration_digests(vec!["registration-a".to_string()]),
        WorthGraphReadAccessInventoryErrorKind::SelectedRegistrationDigestCountMismatch,
    );
    assert_seed_error(
        seed_parts_with_touch_descriptor_digests(vec!["touch-a".to_string()]),
        WorthGraphReadAccessInventoryErrorKind::TouchDescriptorDigestCountMismatch,
    );
}

#[test]
fn inventory_row_requires_owner_cost_posture_and_deletion_action() {
    let row = declaration_candidate_row()
        .build()
        .expect("complete declaration candidate row should build");

    assert_eq!(
        row.source_path(),
        "crates/worth-topo/src/projection/read_views/domain"
    );
    assert_eq!(row.owner(), WorthGraphReadAccessOwner::WorthTopo);
    assert_eq!(row.current_caller(), "TopologyReadGraphAccessProof");
    assert!(!row.claims_execution_authority());

    assert_row_error(
        WorthGraphReadAccessInventoryRow::builder()
            .owner(WorthGraphReadAccessOwner::WorthTopo)
            .current_caller("TopologyReadGraphAccessProof")
            .classification(WorthGraphReadAccessClassification::QueryDeclarationCandidate)
            .cost_posture(WorthGraphReadAccessCostPosture::PerResultNeighborLookup)
            .deletion_action(WorthGraphReadAccessDeletionAction::MigrateToQueryDeclaration)
            .milestone_seven_disposition(
                WorthGraphReadAccessMilestoneSevenDisposition::DeclarationCandidate,
            ),
        WorthGraphReadAccessInventoryErrorKind::MissingSourcePath,
    );
    assert_row_error(
        declaration_candidate_row_without_owner(),
        WorthGraphReadAccessInventoryErrorKind::MissingOwner,
    );
    assert_row_error(
        declaration_candidate_row_without_cost_posture(),
        WorthGraphReadAccessInventoryErrorKind::MissingCostPosture,
    );
    assert_row_error(
        declaration_candidate_row_without_deletion_action(),
        WorthGraphReadAccessInventoryErrorKind::MissingDeletionAction,
    );
    assert_row_error(
        declaration_candidate_row_without_disposition(),
        WorthGraphReadAccessInventoryErrorKind::MissingMilestoneSevenDisposition,
    );
    assert_row_error(
        WorthGraphReadAccessInventoryRow::builder()
            .source_path("crates/worth-topo/src/projection/read_views/domain")
            .owner(WorthGraphReadAccessOwner::WorthTopo)
            .current_caller("TopologyReadGraphAccessProof")
            .classification(WorthGraphReadAccessClassification::QueryDeclarationCandidate)
            .cost_posture(WorthGraphReadAccessCostPosture::PerResultNeighborLookup)
            .deletion_action(WorthGraphReadAccessDeletionAction::MigrateToQueryDeclaration)
            .milestone_seven_disposition(
                WorthGraphReadAccessMilestoneSevenDisposition::DeclarationCandidate,
            ),
        WorthGraphReadAccessInventoryErrorKind::MissingScopeBinding,
    );
}

#[test]
fn capped_graph_read_residue_requires_cap_blocker_and_removal_trigger() {
    let residue = capped_residue_row()
        .build()
        .expect("complete capped residue row should build");
    assert_eq!(residue.current_count(), 1);
    assert_eq!(residue.must_not_exceed_count(), 1);

    assert_residue_error(
        WorthGraphReadAccessCappedResidueRow::builder()
            .source_path("crates/worth-kernel/src/query_adoption/graph_read_access")
            .owner(WorthGraphReadAccessOwner::WorthKernel)
            .must_not_exceed_count(1)
            .blocker("Milestone 7 declaration seed must replace old graph-read adoption")
            .removal_trigger("Milestone 7 declaration seed owns the old path"),
        WorthGraphReadAccessInventoryErrorKind::MissingResidueCurrentCount,
    );
    assert_residue_error(
        WorthGraphReadAccessCappedResidueRow::builder()
            .source_path("crates/worth-kernel/src/query_adoption/graph_read_access")
            .owner(WorthGraphReadAccessOwner::WorthKernel)
            .current_count(1)
            .must_not_exceed_count(1)
            .removal_trigger("Milestone 7 declaration seed owns the old path"),
        WorthGraphReadAccessInventoryErrorKind::MissingResidueBlocker,
    );
    assert_residue_error(
        capped_residue_row()
            .current_count(2)
            .must_not_exceed_count(1),
        WorthGraphReadAccessInventoryErrorKind::ResidueCountExceedsCap,
    );

    assert_row_error(
        WorthGraphReadAccessInventoryRow::builder()
            .source_path("crates/worth-kernel/src/query_adoption/graph_read_access")
            .owner(WorthGraphReadAccessOwner::WorthKernel)
            .current_caller("deleted graph-read adoption scaffolding")
            .classification(WorthGraphReadAccessClassification::CappedResidue)
            .cost_posture(WorthGraphReadAccessCostPosture::FabricatedReceiptOrSupportRow)
            .deletion_action(WorthGraphReadAccessDeletionAction::CapUntilQueryCapabilityExists)
            .milestone_seven_disposition(
                WorthGraphReadAccessMilestoneSevenDisposition::CapabilityGap,
            )
            .scope_binding(deleted_source_scope(
                "crates/worth-kernel/src/query_adoption/graph_read_access",
            )),
        WorthGraphReadAccessInventoryErrorKind::CappedResidueMissingResidueRow,
    );
    assert_row_error(
        declaration_candidate_row()
            .deletion_action(WorthGraphReadAccessDeletionAction::OutOfScopeNoGraphRead),
        WorthGraphReadAccessInventoryErrorKind::DeclarationCandidateContractMismatch,
    );
    assert_row_error(
        capability_gap_row().milestone_seven_disposition(
            WorthGraphReadAccessMilestoneSevenDisposition::DeletionOnly,
        ),
        WorthGraphReadAccessInventoryErrorKind::CapabilityGapContractMismatch,
    );
}

#[test]
fn phase_two_inventory_closeout_counts_rows_by_disposition() {
    let rows = vec![
        declaration_candidate_row(),
        deletion_target_row(),
        capped_residue_inventory_row().current_caller("capped graph-read adoption residue"),
        certification_only_row(),
        capability_gap_row(),
        out_of_scope_row(),
    ];

    let closeout = closeout_from_rows(WorthGraphReadAccessInventorySeed::for_tests(), rows)
        .expect("valid inventory rows should close Phase 2 substrate");
    let counters = closeout.counters();

    assert_eq!(counters.total_row_count(), 6);
    assert_eq!(counters.declaration_candidate_count(), 1);
    assert_eq!(counters.deletion_target_count(), 1);
    assert_eq!(counters.capped_residue_count(), 1);
    assert_eq!(counters.certification_only_count(), 1);
    assert_eq!(counters.capability_gap_count(), 1);
    assert_eq!(counters.out_of_scope_count(), 1);

    let scope_report = closeout.scope_report();
    assert_eq!(scope_report.scoped_row_count(), 6);
    assert_eq!(scope_report.unscoped_row_count(), 0);
    assert_eq!(scope_report.selected_obligation_scoped_count(), 1);
    assert_eq!(scope_report.topology_read_proof_scoped_count(), 0);
    assert_eq!(scope_report.spatial_continuation_scoped_count(), 1);
    assert_eq!(scope_report.deleted_graph_read_source_scoped_count(), 2);
    assert_eq!(scope_report.certification_only_scoped_count(), 1);
    assert_eq!(scope_report.out_of_scope_count(), 1);

    let error = closeout_from_rows(WorthGraphReadAccessInventorySeed::for_tests(), Vec::new())
        .expect_err("empty inventory rows cannot close Phase 2");
    assert_eq!(
        error.kind(),
        WorthGraphReadAccessInventoryErrorKind::EmptyInventoryRows
    );
}
