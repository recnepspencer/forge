use std::collections::{BTreeMap, BTreeSet};

use crate::workload_platform::planar_boolean_loop_reconstruction::walk_candidates::consumption_proof_matches_candidate;
use crate::workload_platform::planar_boolean_loop_reconstruction::{
    PlanarBooleanClosedWalkCandidate, PlanarBooleanClosedWalkCandidateContinuation,
    PlanarBooleanFragmentContinuationEndpointRole,
};

use super::counters::PlanarBooleanWalkOutcomeCounters;
use super::identity::{walk_outcome_identity, walk_outcome_set_identity};
use super::input::PlanarBooleanWalkOutcomeSetInput;
use super::product::PlanarBooleanWalkOutcomeSet;
use super::row::{
    PlanarBooleanWalkOutcomeCause, PlanarBooleanWalkOutcomeKind, PlanarBooleanWalkOutcomeRow,
};

pub(crate) fn classify_walk_outcomes(
    input: PlanarBooleanWalkOutcomeSetInput<'_>,
) -> PlanarBooleanWalkOutcomeSet {
    let request_identity = input
        .closed_walk_candidates()
        .request_identity()
        .to_string();
    let continuation_index_identity = input
        .closed_walk_candidates()
        .continuation_index_identity()
        .to_string();

    let mut counters = PlanarBooleanWalkOutcomeCounters::default();
    let mut rows = input
        .closed_walk_candidates()
        .rows()
        .iter()
        .map(|candidate| {
            build_walk_outcome_row(
                &request_identity,
                &continuation_index_identity,
                candidate,
                input.fragment_consumption_proof(),
                &mut counters,
            )
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| {
        left.closed_walk_candidate_identity()
            .cmp(right.closed_walk_candidate_identity())
            .then_with(|| {
                left.walk_outcome_identity()
                    .cmp(right.walk_outcome_identity())
            })
    });
    let walk_outcome_set_identity =
        walk_outcome_set_identity(&request_identity, &continuation_index_identity, &rows);
    PlanarBooleanWalkOutcomeSet::new(
        walk_outcome_set_identity,
        request_identity,
        continuation_index_identity,
        rows,
        counters,
    )
}

fn build_walk_outcome_row(
    request_identity: &str,
    continuation_index_identity: &str,
    candidate: &PlanarBooleanClosedWalkCandidate,
    fragment_consumption_proof: &crate::workload_platform::planar_boolean_loop_reconstruction::PlanarBooleanFragmentConsumptionProof,
    counters: &mut PlanarBooleanWalkOutcomeCounters,
) -> PlanarBooleanWalkOutcomeRow {
    let source_face_identities = candidate.source_face_identities().to_vec();
    let source_loop_carrier_identities = candidate.source_loop_carrier_identities().to_vec();
    let source_senses = candidate.source_senses().to_vec();
    let fragment_identities = candidate.fragment_identities().to_vec();
    let split_vertex_identities = candidate.split_vertex_identities().to_vec();
    let neighborhood_identities = unique_strings(
        candidate
            .continuations()
            .iter()
            .map(|row| row.neighborhood_identity()),
    );
    let continuation_identities = candidate
        .continuations()
        .iter()
        .map(|row| row.continuation_identity().to_string())
        .collect::<Vec<_>>();
    let local_frame_identities = candidate.local_frame_identities().to_vec();
    let precision_basis_identities = candidate.precision_basis_identities().to_vec();

    let (kind, cause, human_reason) =
        classify_group(candidate, fragment_consumption_proof, &source_senses);
    counters.classified_walk(kind);
    let walk_outcome_identity = walk_outcome_identity(
        request_identity,
        continuation_index_identity,
        candidate.source_loop_identity(),
        kind_name(kind),
        &fragment_identities,
        &split_vertex_identities,
        &continuation_identities,
    );
    PlanarBooleanWalkOutcomeRow::new(
        walk_outcome_identity,
        candidate.closed_walk_candidate_identity().to_string(),
        candidate.source_loop_identity().to_string(),
        source_face_identities,
        source_loop_carrier_identities,
        source_senses,
        fragment_identities,
        split_vertex_identities,
        neighborhood_identities,
        continuation_identities,
        local_frame_identities,
        precision_basis_identities,
        kind,
        cause,
        human_reason.to_string(),
    )
}

fn classify_group(
    candidate: &PlanarBooleanClosedWalkCandidate,
    fragment_consumption_proof: &crate::workload_platform::planar_boolean_loop_reconstruction::PlanarBooleanFragmentConsumptionProof,
    source_senses: &[crate::workload_platform::planar_boolean_events::PlanarBooleanSourceIntervalSense],
) -> (
    PlanarBooleanWalkOutcomeKind,
    PlanarBooleanWalkOutcomeCause,
    &'static str,
) {
    if !consumption_proof_matches_candidate(candidate, fragment_consumption_proof) {
        return (
            PlanarBooleanWalkOutcomeKind::Denied,
            PlanarBooleanWalkOutcomeCause::DeniedProofMismatch,
            "closure classification requires an exact fragment-consumption proof match for every assembled walk candidate",
        );
    }
    if carries_unconsumed_residue(candidate, fragment_consumption_proof) {
        return (
            PlanarBooleanWalkOutcomeKind::Residual,
            PlanarBooleanWalkOutcomeCause::ResidualCoverageMismatch,
            "closure classification detected fragment or split-vertex residue recorded on the candidate without matching continuation coverage",
        );
    }

    let mut neighborhoods =
        BTreeMap::<&str, Vec<&PlanarBooleanClosedWalkCandidateContinuation>>::new();
    for row in candidate.continuations() {
        neighborhoods
            .entry(row.split_vertex_identity())
            .or_default()
            .push(row);
    }

    for neighborhood_rows in neighborhoods.values() {
        let unique_slots = neighborhood_rows
            .iter()
            .map(|row| (row.fragment_identity(), row.fragment_endpoint_role()))
            .collect::<BTreeSet<_>>();
        if unique_slots.len() < 2 {
            return (
                PlanarBooleanWalkOutcomeKind::Open,
                PlanarBooleanWalkOutcomeCause::OpenInsufficientSlots,
                "closure classification requires two unique fragment endpoint slots at every split vertex",
            );
        }
        if unique_slots.len() > 2 {
            return (
                PlanarBooleanWalkOutcomeKind::Unsupported,
                PlanarBooleanWalkOutcomeCause::UnsupportedBranchMultiplicity,
                "closure classification denies multi-branch continuation neighborhoods before loop promotion",
            );
        }

        let fragment_count = neighborhood_rows
            .iter()
            .map(|row| row.fragment_identity())
            .collect::<BTreeSet<_>>()
            .len();
        if fragment_count == 1 {
            return (
                PlanarBooleanWalkOutcomeKind::SelfColliding,
                PlanarBooleanWalkOutcomeCause::SelfCollisionSingleFragment,
                "closure classification localizes a self-colliding split vertex when both slots refer to the same fragment",
            );
        }

        let mut senses_per_slot =
            BTreeMap::<(&str, PlanarBooleanFragmentContinuationEndpointRole), BTreeSet<_>>::new();
        for row in neighborhood_rows {
            senses_per_slot
                .entry((row.fragment_identity(), row.fragment_endpoint_role()))
                .or_default()
                .insert(row.source_sense());
        }
        let expected_senses = source_senses.iter().copied().collect::<BTreeSet<_>>();
        if senses_per_slot
            .values()
            .any(|senses| senses != &expected_senses)
        {
            return (
                PlanarBooleanWalkOutcomeKind::Unsupported,
                PlanarBooleanWalkOutcomeCause::UnsupportedOrientationCoverage,
                "closure classification requires orientation-stable continuation coverage for every fragment endpoint slot",
            );
        }
    }

    (
        PlanarBooleanWalkOutcomeKind::Closed,
        PlanarBooleanWalkOutcomeCause::ClosedTwoSlotCoverage,
        "closure classification preserved a two-slot continuation neighborhood at every split vertex",
    )
}

fn carries_unconsumed_residue(
    candidate: &PlanarBooleanClosedWalkCandidate,
    fragment_consumption_proof: &crate::workload_platform::planar_boolean_loop_reconstruction::PlanarBooleanFragmentConsumptionProof,
) -> bool {
    let actual_fragment_identities = unique_strings(
        candidate
            .continuations()
            .iter()
            .map(|row| row.fragment_identity()),
    );
    if actual_fragment_identities != candidate.fragment_identities() {
        return true;
    }

    let actual_split_vertex_identities = unique_strings(
        candidate
            .continuations()
            .iter()
            .map(|row| row.split_vertex_identity()),
    );
    if actual_split_vertex_identities != candidate.split_vertex_identities() {
        return true;
    }

    let Some(proof_row) = fragment_consumption_proof
        .proof_for_candidate_identity(candidate.closed_walk_candidate_identity())
    else {
        return true;
    };

    proof_row.fragment_identities() != actual_fragment_identities
        || proof_row.split_vertex_identities() != actual_split_vertex_identities
}

fn unique_strings<'a>(values: impl Iterator<Item = &'a str>) -> Vec<String> {
    values
        .map(ToString::to_string)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn kind_name(kind: PlanarBooleanWalkOutcomeKind) -> &'static str {
    match kind {
        PlanarBooleanWalkOutcomeKind::Closed => "closed",
        PlanarBooleanWalkOutcomeKind::Open => "open",
        PlanarBooleanWalkOutcomeKind::Residual => "residual",
        PlanarBooleanWalkOutcomeKind::Unsupported => "unsupported",
        PlanarBooleanWalkOutcomeKind::SelfColliding => "self-colliding",
        PlanarBooleanWalkOutcomeKind::Denied => "denied",
    }
}
