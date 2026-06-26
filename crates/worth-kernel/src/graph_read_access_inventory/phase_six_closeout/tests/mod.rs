mod inventory_conversion;

use forge_query::facade::{
    ForgeQueryGraphReadAccessAdmissionPosture, ForgeQueryGraphReadAccessDenialKind,
};

use super::super::candidates::{
    WorthGraphReadDeclarationCandidate, WorthGraphReadReadFamilyTarget,
    WorthGraphReadRequirementVocabulary,
};
use super::super::capability_gaps::{
    WorthGraphReadExpectedDenial, WorthGraphReadMissingQueryCapability,
    WorthGraphReadQueryAccessCapabilityGap,
};
use super::super::current_worth_graph_read_access_surface_inventory_for_tests;
use super::super::deletion_ledger::WorthGraphReadDeletionLedgerItem;
use super::super::inventory_lane::{
    WorthGraphReadAccessClassification, WorthGraphReadAccessInventoryCloseout,
    WorthGraphReadAccessInventoryRow, WorthGraphReadAccessInventorySeed,
};
use super::{
    reject_keep_local_graph_read_disposition, WorthGraphReadAccessPhaseSixCollector,
    WorthGraphReadAccessPhaseSixErrorKind,
};

#[test]
fn graph_read_declaration_candidate_requires_touched_authority_and_read_family() {
    let inventory = phase_six_inventory();
    let row = row_for_classification(
        &inventory,
        WorthGraphReadAccessClassification::QueryDeclarationCandidate,
    );

    assert_phase_six_error(
        WorthGraphReadDeclarationCandidate::for_inventory_row(row)
            .touched_authority_input("authority-a")
            .requirement_vocabulary(WorthGraphReadRequirementVocabulary::relation_frontier())
            .milestone_seven_lowering_target("Milestone 7 query declaration seed")
            .build(),
        WorthGraphReadAccessPhaseSixErrorKind::MissingReadFamilyTarget,
    );
    assert_phase_six_error(
        WorthGraphReadDeclarationCandidate::for_inventory_row(row)
            .read_family_target(WorthGraphReadReadFamilyTarget::TopologyLoopCycleNeighborhood)
            .requirement_vocabulary(WorthGraphReadRequirementVocabulary::relation_frontier())
            .milestone_seven_lowering_target("Milestone 7 query declaration seed")
            .build(),
        WorthGraphReadAccessPhaseSixErrorKind::MissingTouchedAuthorityInput,
    );
}

#[test]
fn query_access_gap_requires_owner_cap_blocker_and_query_trigger() {
    let inventory = phase_six_inventory();
    let row = row_for_classification(
        &inventory,
        WorthGraphReadAccessClassification::QueryAccessCapabilityGap,
    );

    assert_phase_six_error(
        WorthGraphReadQueryAccessCapabilityGap::for_inventory_row(row)
            .expected_denial(expected_persistent_index_denial())
            .must_not_exceed_count(1)
            .blocker("Query lacks persistent continuation index support")
            .removal_trigger("Milestone 8 access plan support lands")
            .build(),
        WorthGraphReadAccessPhaseSixErrorKind::MissingQueryCapability,
    );
    assert_phase_six_error(
        WorthGraphReadQueryAccessCapabilityGap::for_inventory_row(row)
            .missing_capability(WorthGraphReadMissingQueryCapability::PersistentContinuationIndex)
            .expected_denial(expected_persistent_index_denial())
            .blocker("Query lacks persistent continuation index support")
            .removal_trigger("Milestone 8 access plan support lands")
            .build(),
        WorthGraphReadAccessPhaseSixErrorKind::MissingCapabilityGapCap,
    );
    assert_phase_six_error(
        WorthGraphReadQueryAccessCapabilityGap::for_inventory_row(row)
            .missing_capability(WorthGraphReadMissingQueryCapability::PersistentContinuationIndex)
            .expected_denial(expected_persistent_index_denial())
            .must_not_exceed_count(1)
            .removal_trigger("Milestone 8 access plan support lands")
            .build(),
        WorthGraphReadAccessPhaseSixErrorKind::MissingCapabilityGapBlocker,
    );
}

#[test]
fn broad_boolean_and_dense_frontier_reads_cannot_be_keep_local_rows() {
    let error = reject_keep_local_graph_read_disposition(
        "broad boolean scans and dense frontiers must become candidates or gaps",
    )
    .expect_err("keep-local graph-read disposition must not exist");

    assert_eq!(
        error.kind(),
        WorthGraphReadAccessPhaseSixErrorKind::KeepLocalGraphReadDispositionDenied
    );
}

#[test]
fn phase_six_closeout_rejects_missing_or_wrong_dispositions() {
    let inventory = phase_six_inventory();
    let missing = WorthGraphReadAccessPhaseSixCollector::from_inventory(&inventory)
        .admit_declaration_candidate(declaration_candidate(&inventory))
        .unwrap()
        .closeout()
        .expect_err("required capability and deletion rows must not be omitted");
    assert_eq!(
        missing.kind(),
        WorthGraphReadAccessPhaseSixErrorKind::MissingInventoryRowDisposition
    );

    let certification_only = row_for_classification(
        &inventory,
        WorthGraphReadAccessClassification::CertificationOnlySupport,
    );
    let wrong = WorthGraphReadAccessPhaseSixCollector::from_inventory(&inventory)
        .admit_deletion_item(
            WorthGraphReadDeletionLedgerItem::for_inventory_row(certification_only)
                .deletion_trigger("should not delete certification-only proof support")
                .build()
                .unwrap(),
        )
        .expect_err("certification-only support cannot become a deletion item");
    assert_eq!(
        wrong.kind(),
        WorthGraphReadAccessPhaseSixErrorKind::InventoryRowDispositionMismatch
    );

    let capability_gap = row_for_classification(
        &inventory,
        WorthGraphReadAccessClassification::QueryAccessCapabilityGap,
    );
    let wrong_capability_gap = WorthGraphReadAccessPhaseSixCollector::from_inventory(&inventory)
        .admit_deletion_item(
            WorthGraphReadDeletionLedgerItem::for_inventory_row(capability_gap)
                .deletion_trigger("capability gap cannot be deleted")
                .build()
                .unwrap(),
        )
        .expect_err("Phase 6 must respect the inventory row's Milestone 7 disposition");
    assert_eq!(
        wrong_capability_gap.kind(),
        WorthGraphReadAccessPhaseSixErrorKind::InventoryRowDispositionMismatch
    );
}

fn phase_six_inventory() -> WorthGraphReadAccessInventoryCloseout {
    current_worth_graph_read_access_surface_inventory_for_tests(
        WorthGraphReadAccessInventorySeed::for_tests(),
    )
    .expect("test inventory should close before Phase 6 ledger conversion")
}

fn declaration_candidate(
    inventory: &WorthGraphReadAccessInventoryCloseout,
) -> WorthGraphReadDeclarationCandidate {
    let row = row_for_classification(
        inventory,
        WorthGraphReadAccessClassification::QueryDeclarationCandidate,
    );
    WorthGraphReadDeclarationCandidate::for_inventory_row(row)
        .read_family_target(WorthGraphReadReadFamilyTarget::TopologyLoopCycleNeighborhood)
        .touched_authority_input(
            row.scope_binding()
                .authority_digest()
                .expect("declaration row should retain touched authority digest"),
        )
        .requirement_vocabulary(WorthGraphReadRequirementVocabulary::relation_frontier())
        .milestone_seven_lowering_target("Milestone 7 query declaration seed")
        .build()
        .unwrap()
}

fn row_for_classification(
    inventory: &WorthGraphReadAccessInventoryCloseout,
    classification: WorthGraphReadAccessClassification,
) -> &WorthGraphReadAccessInventoryRow {
    inventory
        .rows()
        .iter()
        .find(|row| row.classification() == classification)
        .expect("test inventory should contain requested classification")
}

fn expected_persistent_index_denial() -> WorthGraphReadExpectedDenial {
    WorthGraphReadExpectedDenial::new(
        ForgeQueryGraphReadAccessDenialKind::RequiredPersistentIndex,
        ForgeQueryGraphReadAccessAdmissionPosture::PersistentIndexRequired,
    )
}

fn assert_phase_six_error<T: std::fmt::Debug>(
    result: Result<T, super::WorthGraphReadAccessPhaseSixError>,
    expected: WorthGraphReadAccessPhaseSixErrorKind,
) {
    let error = result.expect_err("Phase 6 builder should reject incomplete proof");
    assert_eq!(error.kind(), expected);
}
