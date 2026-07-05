use topology::facade::admit_milestone_seven_five_overlap_readiness_consumer;
use worth_kernel::workload_composition::current_touched_graph_readiness_handoff;

use crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanOverlapChainBoundaryRole;
use crate::workload_platform::planar_boolean_loop_reconstruction::test_support::{
    admitted_phase_fourteen_identity_products, prepared_phase_fourteen_subject,
    LoopFixtureEntryOrder,
};
use crate::workload_platform::planar_boolean_loop_reconstruction::{
    PlanarBooleanLoopDecisionLog, PlanarBooleanLoopIslandPartition,
    PlanarBooleanLoopIslandPartitionRow, PlanarBooleanLoopOverlapChainLineageMap,
    PlanarBooleanLoopOverlapChainLineageRow, PlanarBooleanLoopReconstructionLedger,
    PlanarBooleanLoopReconstructionParticipationSupport, PlanarBooleanLoopRoleOutcomeSet,
};

use super::{
    PlanarBooleanOverlapParticipationRecovery, PlanarBooleanOverlapParticipationRecoveryDenialKind,
    PlanarBooleanOverlapParticipationRecoveryInput,
};
use crate::workload_platform::planar_boolean_overlap_region_extraction::{
    PlanarBooleanOverlapRegionExtractionRequest, PlanarBooleanOverlapRegionExtractionRequestInput,
};

#[test]
fn overlap_participation_is_replay_stable_for_real_loop_ledger_products() {
    let readiness = current_touched_graph_readiness_handoff()
        .expect("current readiness handoff should assemble");
    let canonical = overlap_request_and_support(LoopFixtureEntryOrder::Canonical, &readiness);
    let replayed = overlap_request_and_support(LoopFixtureEntryOrder::Replayed, &readiness);
    assert!(
        !canonical.1.ledger_rows().is_empty(),
        "canonical support should expose real loop-ledger rows"
    );
    assert!(
        !canonical.1.island_partition().rows().is_empty(),
        "canonical support should expose real loop-island rows"
    );
    assert!(
        !canonical.1.persistent_name_map().rows().is_empty(),
        "canonical support should expose real persistent-name rows"
    );

    let canonical_recovery = PlanarBooleanOverlapParticipationRecovery::recover(
        PlanarBooleanOverlapParticipationRecoveryInput::from_request_and_loop_support(
            &canonical.0,
            &canonical.1,
        ),
    )
    .expect("canonical participation should recover");
    let replayed_recovery = PlanarBooleanOverlapParticipationRecovery::recover(
        PlanarBooleanOverlapParticipationRecoveryInput::from_request_and_loop_support(
            &replayed.0,
            &replayed.1,
        ),
    )
    .expect("replayed participation should recover");

    assert_eq!(
        canonical_recovery.loop_participation_map(),
        replayed_recovery.loop_participation_map()
    );
    assert_eq!(
        canonical_recovery.island_participation_map(),
        replayed_recovery.island_participation_map()
    );
    assert_eq!(
        canonical_recovery.chain_lineage_map(),
        replayed_recovery.chain_lineage_map()
    );
    assert!(
        canonical_recovery
            .chain_lineage_map()
            .rows()
            .iter()
            .any(|row| !row.fragment_identities().is_empty()),
        "chain lineage should preserve real fragment provenance"
    );
    assert!(
        canonical_recovery
            .chain_lineage_map()
            .rows()
            .iter()
            .any(|row| !row.source_loop_identities().is_empty()),
        "chain lineage should preserve real source-loop provenance"
    );
    assert!(
        canonical_recovery
            .chain_lineage_map()
            .rows()
            .iter()
            .any(|row| !row.source_edge_identities().is_empty()),
        "chain lineage should preserve real source-edge provenance"
    );
    assert!(
        canonical_recovery
            .chain_lineage_map()
            .rows()
            .iter()
            .any(|row| !row.boundary_roles().is_empty()),
        "chain lineage should preserve real boundary-role provenance"
    );
    assert!(
        canonical_recovery
            .loop_participation_map()
            .rows()
            .iter()
            .any(|row| !row.propagated_persistent_name_identities().is_empty()),
        "loop participation should preserve real persistent-name inputs"
    );
    assert!(
        canonical_recovery
            .island_participation_map()
            .rows()
            .iter()
            .any(|row| !row.propagated_persistent_name_identities().is_empty()),
        "island participation should preserve real persistent-name inputs"
    );
    assert!(
        canonical_recovery
            .chain_lineage_map()
            .rows()
            .iter()
            .any(|row| !row.propagated_persistent_name_identities().is_empty()),
        "chain lineage should preserve real persistent-name inputs"
    );
}

#[test]
fn overlap_participation_rejects_dangling_loop_role_membership_before_adjacency() {
    let readiness = current_touched_graph_readiness_handoff()
        .expect("current readiness handoff should assemble");
    let (request, support) =
        overlap_request_and_support(LoopFixtureEntryOrder::Canonical, &readiness);
    assert!(
        !support.ledger_rows().is_empty(),
        "hostile role fixture should expose real loop-ledger rows"
    );

    let hostile_role_outcomes = PlanarBooleanLoopRoleOutcomeSet::new(
        support
            .role_outcomes()
            .role_outcome_set_identity()
            .to_string(),
        support.role_outcomes().request_identity().to_string(),
        support.role_outcomes().rows()[1..].to_vec(),
    );
    let hostile_support = support.with_role_outcomes_for_tests(hostile_role_outcomes);

    let denial = PlanarBooleanOverlapParticipationRecovery::recover(
        PlanarBooleanOverlapParticipationRecoveryInput::from_request_and_loop_support(
            &request,
            &hostile_support,
        ),
    )
    .expect_err("participation should reject dangling loop-role membership");

    assert_eq!(
        denial.kind(),
        PlanarBooleanOverlapParticipationRecoveryDenialKind::DanglingLoopParticipationDenied
    );
}

#[test]
fn overlap_participation_rejects_contradictory_island_membership_before_adjacency() {
    let readiness = current_touched_graph_readiness_handoff()
        .expect("current readiness handoff should assemble");
    let (request, support) =
        overlap_request_and_support(LoopFixtureEntryOrder::Canonical, &readiness);
    assert!(
        !support.island_partition().rows().is_empty(),
        "hostile island fixture should expose real loop-island rows"
    );
    let first_loop = support.ledger_rows()[0].tracked_loop_identity().to_string();
    let first_island = &support.island_partition().rows()[0];
    let hostile_rows = vec![
        PlanarBooleanLoopIslandPartitionRow::new(
            first_island.island_identity().to_string(),
            first_island.source_loop_identity().to_string(),
            vec![first_loop.clone()],
            first_island.kind(),
        ),
        PlanarBooleanLoopIslandPartitionRow::new(
            format!("{}-hostile", first_island.island_identity()),
            format!("{}-hostile", first_island.source_loop_identity()),
            vec![first_loop],
            first_island.kind(),
        ),
    ];
    let hostile_partition = PlanarBooleanLoopIslandPartition::new(
        support.island_partition().partition_identity().to_string(),
        support.island_partition().request_identity().to_string(),
        hostile_rows,
        support.island_partition().counters(),
    );
    let hostile_support = support.with_island_partition_for_tests(hostile_partition);

    let denial = PlanarBooleanOverlapParticipationRecovery::recover(
        PlanarBooleanOverlapParticipationRecoveryInput::from_request_and_loop_support(
            &request,
            &hostile_support,
        ),
    )
    .expect_err("participation should reject contradictory island membership");

    assert_eq!(
        denial.kind(),
        PlanarBooleanOverlapParticipationRecoveryDenialKind::ContradictoryIslandMembershipDenied
    );
}

#[test]
fn overlap_participation_rejects_foreign_overlap_chain_lineage_before_adjacency() {
    let readiness = current_touched_graph_readiness_handoff()
        .expect("current readiness handoff should assemble");
    let (request, support) =
        overlap_request_and_support(LoopFixtureEntryOrder::Canonical, &readiness);
    assert!(
        !support.overlap_chain_lineage_map().rows().is_empty(),
        "hostile chain fixture should expose real overlap-chain lineage rows"
    );
    let canonical_lineage = support.overlap_chain_lineage_map().rows()[0].clone();
    let mut hostile_rows = support.overlap_chain_lineage_map().rows().to_vec();
    hostile_rows[0] = PlanarBooleanLoopOverlapChainLineageRow::new(
        canonical_lineage.lineage_identity().to_string(),
        canonical_lineage.chain_identity().to_string(),
        canonical_lineage.member_identities().to_vec(),
        canonical_lineage.fragment_identities().to_vec(),
        canonical_lineage.source_loop_identities().to_vec(),
        vec!["foreign-source-edge".to_string()],
        vec![PlanarBooleanOverlapChainBoundaryRole::OverlapInteriorFragment],
    );
    let hostile_lineage_map = PlanarBooleanLoopOverlapChainLineageMap::new(
        support
            .overlap_chain_lineage_map()
            .lineage_map_identity()
            .to_string(),
        support
            .overlap_chain_lineage_map()
            .request_identity()
            .to_string(),
        support
            .overlap_chain_lineage_map()
            .overlap_chain_set_identity()
            .to_string(),
        hostile_rows,
    );
    let hostile_support = support.with_overlap_chain_lineage_map_for_tests(hostile_lineage_map);

    let denial = PlanarBooleanOverlapParticipationRecovery::recover(
        PlanarBooleanOverlapParticipationRecoveryInput::from_request_and_loop_support(
            &request,
            &hostile_support,
        ),
    )
    .expect_err("participation should reject foreign overlap-chain lineage");

    assert_eq!(
        denial.kind(),
        PlanarBooleanOverlapParticipationRecoveryDenialKind::ForeignOverlapChainLineageDenied
    );
}

#[test]
fn overlap_participation_rejects_mismatched_real_loop_support_before_adjacency() {
    let readiness = current_touched_graph_readiness_handoff()
        .expect("current readiness handoff should assemble");
    let (request, support) =
        overlap_request_and_support(LoopFixtureEntryOrder::Canonical, &readiness);
    let hostile_support = support.with_loop_ledger_receipt_for_tests(
        support
            .loop_ledger_receipt()
            .with_receipt_identity_for_tests("hostile-loop-ledger-receipt"),
    );

    let denial = PlanarBooleanOverlapParticipationRecovery::recover(
        PlanarBooleanOverlapParticipationRecoveryInput::from_request_and_loop_support(
            &request,
            &hostile_support,
        ),
    )
    .expect_err(
        "participation should reject loop support that no longer matches the admitted request",
    );

    assert_eq!(
        denial.kind(),
        PlanarBooleanOverlapParticipationRecoveryDenialKind::LoopLedgerParticipationSupportMismatch
    );
}

fn overlap_request_and_support(
    order: LoopFixtureEntryOrder,
    readiness: &schema::facade::platform::authority::touched_graph_parity_closeout::TouchedGraphParityReadinessInput,
) -> (
    PlanarBooleanOverlapRegionExtractionRequest,
    PlanarBooleanLoopReconstructionParticipationSupport,
) {
    let fixture = prepared_phase_fourteen_subject(order);
    let decision_log = PlanarBooleanLoopDecisionLog::record(fixture.decision_log_input())
        .expect("phase fourteen products should admit loop decision-log recording");
    let (identity_map, persistent_name_map, subshape_signature_map) =
        admitted_phase_fourteen_identity_products(&fixture);
    let (ledger, receipt) = PlanarBooleanLoopReconstructionLedger::assemble(
        fixture.ledger_input_with_identity_products(
            &decision_log,
            &identity_map,
            &persistent_name_map,
            &subshape_signature_map,
        ),
    )
    .expect("phase fourteen products should assemble the loop ledger");
    let readiness_consumer = admit_milestone_seven_five_overlap_readiness_consumer(readiness)
        .expect("7.5 readiness consumer should admit");
    let request = PlanarBooleanOverlapRegionExtractionRequest::admit(
        PlanarBooleanOverlapRegionExtractionRequestInput::from_readiness_consumer_and_loop_ledger(
            &readiness_consumer,
            &receipt,
        ),
    )
    .expect("overlap request should admit from readiness and real 7.4 receipt");
    let support =
        PlanarBooleanLoopReconstructionParticipationSupport::admit_from_ledger_and_products(
            &ledger,
            fixture.role_boundary.role_outcomes(),
            &fixture.island_partition,
            &persistent_name_map,
            fixture.source_provenance.fragment_membership_map(),
            fixture.source_provenance.overlap_chain_lineage_map(),
            fixture.source_provenance.source_loop_carriers(),
        )
        .expect("phase fourteen products should admit participation support");
    (request, support)
}
