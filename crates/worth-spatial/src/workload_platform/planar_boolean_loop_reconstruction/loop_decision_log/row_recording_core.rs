use crate::workload_platform::planar_boolean_loop_reconstruction::{
    PlanarBooleanLoopIslandKind, PlanarBooleanSourceLoopSplitAttributionKind,
    PlanarBooleanWalkOutcomeKind,
};

use super::counters::PlanarBooleanLoopDecisionLogCounters;
use super::denial::PlanarBooleanLoopDecisionLogDenial;
use super::identity::decision_identity;
use super::input::PlanarBooleanLoopDecisionLogInput;
use super::row::PlanarBooleanLoopDecisionRow;
use super::row_recording::push_row;
use super::vocabulary::{
    PlanarBooleanLoopDecisionAffectedArtifact as Artifact,
    PlanarBooleanLoopDecisionKind as KindRow, PlanarBooleanLoopDecisionPhase as Phase,
    PlanarBooleanLoopDecisionReason as Reason,
};

pub(super) fn record_core_rows(
    input: PlanarBooleanLoopDecisionLogInput<'_>,
    rows: &mut Vec<PlanarBooleanLoopDecisionRow>,
    seen_decision_identities: &mut std::collections::BTreeSet<String>,
    counters: &mut PlanarBooleanLoopDecisionLogCounters,
) -> Result<(), PlanarBooleanLoopDecisionLogDenial> {
    for row in input.continuation_index().rows() {
        counters.consumed_continuation_row();
        push_row(
            rows,
            seen_decision_identities,
            counters,
            PlanarBooleanLoopDecisionRow::new(
                decision_identity(
                    Phase::ContinuationIndexing.as_str(),
                    Artifact::Continuation.as_str(),
                    row.continuation_identity(),
                    row.fragment_identity(),
                ),
                Phase::ContinuationIndexing,
                KindRow::Indexed,
                Artifact::Continuation,
                row.continuation_identity().to_string(),
                vec![row.source_loop_identity().to_string()],
                vec![row.fragment_identity().to_string()],
                vec![row.split_vertex_identity().to_string()],
                vec![
                    row.neighborhood_identity().to_string(),
                    row.source_loop_carrier_identity().to_string(),
                ],
                None,
                Reason::IndexedFragmentContinuation,
                "indexed fragment continuation into the canonical loop continuation view"
                    .to_string(),
            ),
        )?;
    }

    for row in input.walk_outcomes().rows() {
        counters.consumed_walk_outcome();
        push_row(
            rows,
            seen_decision_identities,
            counters,
            PlanarBooleanLoopDecisionRow::new(
                decision_identity(
                    Phase::WalkOutcomeClassification.as_str(),
                    Artifact::WalkOutcome.as_str(),
                    row.walk_outcome_identity(),
                    row.closed_walk_candidate_identity(),
                ),
                Phase::WalkOutcomeClassification,
                walk_kind(row.kind()),
                Artifact::WalkOutcome,
                row.walk_outcome_identity().to_string(),
                vec![row.source_loop_identity().to_string()],
                row.fragment_identities().to_vec(),
                row.split_vertex_identities().to_vec(),
                row.continuation_identities().to_vec(),
                Some(format!("{:?}", row.cause())),
                Reason::ClassifiedWalkOutcome,
                row.human_reason().to_string(),
            ),
        )?;
    }

    for row in input.loop_candidates().rows() {
        counters.consumed_loop_candidate();
        push_row(
            rows,
            seen_decision_identities,
            counters,
            PlanarBooleanLoopDecisionRow::new(
                decision_identity(
                    Phase::LoopCandidatePromotion.as_str(),
                    Artifact::LoopCandidate.as_str(),
                    row.loop_candidate_identity(),
                    row.walk_outcome_identity(),
                ),
                Phase::LoopCandidatePromotion,
                KindRow::Admitted,
                Artifact::LoopCandidate,
                row.loop_candidate_identity().to_string(),
                vec![row.source_loop_identity().to_string()],
                row.fragment_identities().to_vec(),
                row.split_vertex_identities().to_vec(),
                vec![row.walk_outcome_identity().to_string()],
                None,
                Reason::PromotedClosedWalk,
                "promoted a closed walk outcome into a typed loop candidate".to_string(),
            ),
        )?;
    }
    for row in input.denied_loop_candidates().rows() {
        counters.consumed_denied_loop_candidate();
        push_row(
            rows,
            seen_decision_identities,
            counters,
            PlanarBooleanLoopDecisionRow::new(
                decision_identity(
                    Phase::LoopCandidatePromotion.as_str(),
                    Artifact::DeniedLoopCandidate.as_str(),
                    row.denied_loop_candidate_identity(),
                    row.walk_outcome_identity(),
                ),
                Phase::LoopCandidatePromotion,
                KindRow::Denied,
                Artifact::DeniedLoopCandidate,
                row.denied_loop_candidate_identity().to_string(),
                vec![row.source_loop_identity().to_string()],
                row.fragment_identities().to_vec(),
                row.split_vertex_identities().to_vec(),
                vec![row.walk_outcome_identity().to_string()],
                Some(format!("{:?}", row.kind())),
                Reason::RejectedLoopCandidate,
                row.human_reason().to_string(),
            ),
        )?;
    }

    for row in input.reconstructed_loops().rows() {
        counters.consumed_reconstructed_loop();
        push_row(
            rows,
            seen_decision_identities,
            counters,
            PlanarBooleanLoopDecisionRow::new(
                decision_identity(
                    Phase::LoopProductAssembly.as_str(),
                    Artifact::ReconstructedLoop.as_str(),
                    row.reconstructed_loop_identity(),
                    row.loop_candidate_identity(),
                ),
                Phase::LoopProductAssembly,
                KindRow::Admitted,
                Artifact::ReconstructedLoop,
                row.reconstructed_loop_identity().to_string(),
                vec![row.source_loop_identity().to_string()],
                row.fragment_identities().to_vec(),
                row.split_vertex_identities().to_vec(),
                vec![row.loop_candidate_identity().to_string()],
                None,
                Reason::BuiltReconstructedLoop,
                "built a reconstructed loop from an admitted loop candidate".to_string(),
            ),
        )?;
    }
    for row in input.born_loops().rows() {
        counters.consumed_born_loop();
        push_row(
            rows,
            seen_decision_identities,
            counters,
            PlanarBooleanLoopDecisionRow::new(
                decision_identity(
                    Phase::LoopProductAssembly.as_str(),
                    Artifact::BornLoop.as_str(),
                    row.born_loop_identity(),
                    row.loop_candidate_identity(),
                ),
                Phase::LoopProductAssembly,
                KindRow::Derived,
                Artifact::BornLoop,
                row.born_loop_identity().to_string(),
                row.source_loop_identities().to_vec(),
                row.fragment_identities().to_vec(),
                row.split_vertex_identities().to_vec(),
                row.contributing_chain_identities().to_vec(),
                None,
                Reason::BuiltBornLoop,
                "built a born loop from admitted source evidence".to_string(),
            ),
        )?;
    }

    for row in input.island_partition().rows() {
        counters.consumed_island_row();
        push_row(
            rows,
            seen_decision_identities,
            counters,
            PlanarBooleanLoopDecisionRow::new(
                decision_identity(
                    Phase::IslandPartition.as_str(),
                    Artifact::IslandRow.as_str(),
                    row.island_identity(),
                    row.source_loop_identity(),
                ),
                Phase::IslandPartition,
                island_kind(row.kind()),
                Artifact::IslandRow,
                row.island_identity().to_string(),
                vec![row.source_loop_identity().to_string()],
                Vec::new(),
                Vec::new(),
                row.member_loop_identities().to_vec(),
                None,
                Reason::PartitionedLoopIsland,
                "partitioned reconstructed loop products into canonical loop islands".to_string(),
            ),
        )?;
    }

    for row in input.split_attribution().rows() {
        counters.consumed_split_attribution_row();
        push_row(
            rows,
            seen_decision_identities,
            counters,
            PlanarBooleanLoopDecisionRow::new(
                decision_identity(
                    Phase::SourceLoopAttribution.as_str(),
                    Artifact::SplitAttributionRow.as_str(),
                    row.attribution_identity(),
                    row.source_loop_identity(),
                ),
                Phase::SourceLoopAttribution,
                split_attribution_kind(row.kind()),
                Artifact::SplitAttributionRow,
                row.attribution_identity().to_string(),
                vec![row.source_loop_identity().to_string()],
                Vec::new(),
                Vec::new(),
                row.island_identities().to_vec(),
                Some(format!("{:?}", row.kind())),
                Reason::AttributedSourceLoopSplit,
                "bound reconstructed loop islands back to source loop lineage".to_string(),
            ),
        )?;
    }
    Ok(())
}

fn walk_kind(kind: PlanarBooleanWalkOutcomeKind) -> KindRow {
    match kind {
        PlanarBooleanWalkOutcomeKind::Closed => KindRow::Admitted,
        PlanarBooleanWalkOutcomeKind::Open
        | PlanarBooleanWalkOutcomeKind::Residual
        | PlanarBooleanWalkOutcomeKind::SelfColliding
        | PlanarBooleanWalkOutcomeKind::Denied => KindRow::Denied,
        PlanarBooleanWalkOutcomeKind::Unsupported => KindRow::PolicyRequired,
    }
}

fn island_kind(kind: PlanarBooleanLoopIslandKind) -> KindRow {
    match kind {
        PlanarBooleanLoopIslandKind::PreservedSourceLoop => KindRow::Preserved,
        PlanarBooleanLoopIslandKind::BornFromOverlapNeighborhood => KindRow::Derived,
    }
}

fn split_attribution_kind(kind: PlanarBooleanSourceLoopSplitAttributionKind) -> KindRow {
    match kind {
        PlanarBooleanSourceLoopSplitAttributionKind::Preserved => KindRow::Preserved,
        PlanarBooleanSourceLoopSplitAttributionKind::SplitIntoMultipleIslands
        | PlanarBooleanSourceLoopSplitAttributionKind::ContributedToBornLoop => KindRow::Derived,
    }
}
