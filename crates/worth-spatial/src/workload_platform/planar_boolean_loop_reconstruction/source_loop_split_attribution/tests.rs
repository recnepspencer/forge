use crate::workload_platform::planar_boolean_loop_reconstruction::test_support::{
    prepared_loop_continuation_subject, LoopFixtureEntryOrder,
};
use crate::workload_platform::planar_boolean_loop_reconstruction::{
    PlanarBooleanClosedWalkCandidateAssembly, PlanarBooleanClosedWalkCandidateSetInput,
    PlanarBooleanFragmentContinuationIndex, PlanarBooleanFragmentContinuationIndexInput,
    PlanarBooleanLoopCandidateBoundary, PlanarBooleanLoopCandidateBoundaryInput,
    PlanarBooleanLoopIslandKind, PlanarBooleanLoopIslandPartition,
    PlanarBooleanLoopIslandPartitionInput, PlanarBooleanLoopIslandPartitionRow,
    PlanarBooleanReconstructedLoopBoundary, PlanarBooleanReconstructedLoopBoundaryInput,
    PlanarBooleanSourceLoopSplitAttribution, PlanarBooleanSourceLoopSplitAttributionInput,
    PlanarBooleanSourceLoopSplitAttributionKind, PlanarBooleanWalkOutcomeSet,
    PlanarBooleanWalkOutcomeSetInput,
};

#[test]
fn split_attribution_records_preserved_single_island_source_loops() {
    let prepared = prepared_loop_continuation_subject(LoopFixtureEntryOrder::Canonical);
    let boundary = loop_candidate_boundary(&prepared);
    let reconstructed = PlanarBooleanReconstructedLoopBoundary::admit(
        PlanarBooleanReconstructedLoopBoundaryInput::from_loop_candidates_and_provenance(
            boundary.loop_candidates(),
            &prepared.source_provenance,
        ),
    )
    .expect("prepared loop continuation subject should reconstruct");
    let partition = PlanarBooleanLoopIslandPartition::partition(
        PlanarBooleanLoopIslandPartitionInput::from_reconstructed_loop_boundary(
            reconstructed.reconstructed_loops(),
            reconstructed.born_loops(),
        ),
    );
    let attribution = PlanarBooleanSourceLoopSplitAttribution::attribute(
        PlanarBooleanSourceLoopSplitAttributionInput::from_island_partition(&partition),
    );

    assert_eq!(attribution.counters().attribution_rows_emitted(), 1);
    let row = attribution
        .rows()
        .first()
        .expect("split attribution row should exist");
    assert_eq!(
        row.kind(),
        PlanarBooleanSourceLoopSplitAttributionKind::Preserved
    );
    assert_eq!(row.island_identities().len(), 1);
    let partition_row = partition
        .rows()
        .first()
        .expect("partition row should exist");
    assert_eq!(
        row.island_identities(),
        &[partition_row.island_identity().to_string()]
    );
}

#[test]
fn split_attribution_classifies_split_source_loops_with_multiple_islands() {
    let partition = PlanarBooleanLoopIslandPartition::new(
        "partition:split".to_string(),
        "request:split".to_string(),
        vec![
            PlanarBooleanLoopIslandPartitionRow::new(
                "island:split:1".to_string(),
                "source-loop:split".to_string(),
                vec!["reconstructed-loop:1".to_string()],
                PlanarBooleanLoopIslandKind::PreservedSourceLoop,
            ),
            PlanarBooleanLoopIslandPartitionRow::new(
                "island:split:2".to_string(),
                "source-loop:split".to_string(),
                vec!["reconstructed-loop:2".to_string()],
                PlanarBooleanLoopIslandKind::PreservedSourceLoop,
            ),
        ],
        Default::default(),
    );

    let attribution = PlanarBooleanSourceLoopSplitAttribution::attribute(
        PlanarBooleanSourceLoopSplitAttributionInput::from_island_partition(&partition),
    );

    assert_eq!(attribution.counters().attribution_rows_emitted(), 1);
    let row = attribution
        .rows()
        .first()
        .expect("split source attribution row should exist");
    assert_eq!(row.source_loop_identity(), "source-loop:split");
    assert_eq!(
        row.island_identities(),
        &["island:split:1".to_string(), "island:split:2".to_string()]
    );
    assert_eq!(
        row.kind(),
        PlanarBooleanSourceLoopSplitAttributionKind::SplitIntoMultipleIslands
    );
}

#[test]
fn split_attribution_classifies_born_loop_contribution_without_collapsing_island_ids() {
    let partition = PlanarBooleanLoopIslandPartition::new(
        "partition:born".to_string(),
        "request:born".to_string(),
        vec![
            PlanarBooleanLoopIslandPartitionRow::new(
                "island:born".to_string(),
                "source-loop:born".to_string(),
                vec!["born-loop:solo".to_string()],
                PlanarBooleanLoopIslandKind::BornFromOverlapNeighborhood,
            ),
            PlanarBooleanLoopIslandPartitionRow::new(
                "island:preserved".to_string(),
                "source-loop:preserved".to_string(),
                vec!["reconstructed-loop:preserved".to_string()],
                PlanarBooleanLoopIslandKind::PreservedSourceLoop,
            ),
        ],
        Default::default(),
    );

    let attribution = PlanarBooleanSourceLoopSplitAttribution::attribute(
        PlanarBooleanSourceLoopSplitAttributionInput::from_island_partition(&partition),
    );

    assert_eq!(attribution.counters().attribution_rows_emitted(), 2);
    let born_row = attribution
        .rows()
        .iter()
        .find(|row| row.source_loop_identity() == "source-loop:born")
        .expect("born source-loop attribution row should exist");
    assert_eq!(born_row.island_identities(), &["island:born".to_string()]);
    assert_eq!(
        born_row.kind(),
        PlanarBooleanSourceLoopSplitAttributionKind::ContributedToBornLoop
    );
    let preserved_row = attribution
        .rows()
        .iter()
        .find(|row| row.source_loop_identity() == "source-loop:preserved")
        .expect("preserved source-loop attribution row should exist");
    assert_eq!(
        preserved_row.island_identities(),
        &["island:preserved".to_string()]
    );
    assert_eq!(
        preserved_row.kind(),
        PlanarBooleanSourceLoopSplitAttributionKind::Preserved
    );
}

fn loop_candidate_boundary(
    prepared: &crate::workload_platform::planar_boolean_loop_reconstruction::test_support::PreparedLoopContinuationIndexSubject,
) -> PlanarBooleanLoopCandidateBoundary {
    let index = PlanarBooleanFragmentContinuationIndex::admit(
        PlanarBooleanFragmentContinuationIndexInput::from_request_and_provenance(
            &prepared.request,
            &prepared.source_provenance,
            &prepared.subject.vertices,
            &prepared.subject.fragments,
            &prepared.subject.overlap_chains,
        ),
    )
    .expect("prepared continuation subject should admit");
    let assembly = PlanarBooleanClosedWalkCandidateAssembly::assemble(
        PlanarBooleanClosedWalkCandidateSetInput::from_continuation_index(&index),
    );
    let outcomes = PlanarBooleanWalkOutcomeSet::classify(
        PlanarBooleanWalkOutcomeSetInput::from_closed_walk_candidates(
            assembly.closed_walk_candidates(),
            assembly.fragment_consumption_proof(),
        ),
    );
    PlanarBooleanLoopCandidateBoundary::promote(
        PlanarBooleanLoopCandidateBoundaryInput::from_walk_outcomes(&outcomes),
    )
}
