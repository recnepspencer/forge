use std::collections::BTreeSet;

use crate::workload_platform::planar_boolean_loop_reconstruction::{
    PlanarBooleanFragmentMembershipRow, PlanarBooleanLoopCandidate,
    PlanarBooleanLoopOverlapChainLineageRow,
};

use super::counters::PlanarBooleanReconstructedLoopBoundaryCounters;
use super::denial::{
    PlanarBooleanReconstructedLoopBoundaryDenial, PlanarBooleanReconstructedLoopBoundaryDenialKind,
};
use super::identity::{
    admitted_reconstructed_loop_identity, admitted_reconstructed_loop_set_identity,
    born_loop_identity, born_loop_set_identity,
};
use super::input::PlanarBooleanReconstructedLoopBoundaryInput;
use super::product::{
    PlanarBooleanAdmittedReconstructedLoopSet, PlanarBooleanBornLoopSet,
    PlanarBooleanReconstructedLoopBoundary,
};
use super::row::{PlanarBooleanAdmittedReconstructedLoop, PlanarBooleanBornLoop};

pub(crate) fn admit_reconstructed_loop_boundary(
    input: PlanarBooleanReconstructedLoopBoundaryInput<'_>,
) -> Result<PlanarBooleanReconstructedLoopBoundary, PlanarBooleanReconstructedLoopBoundaryDenial> {
    let request_identity = input.loop_candidates().request_identity().to_string();
    let provenance = input.source_provenance();
    let mut counters = PlanarBooleanReconstructedLoopBoundaryCounters::default();
    let mut admitted = Vec::new();
    let mut born = Vec::new();

    for candidate in input.loop_candidates().rows() {
        counters.consumed_loop_candidate();
        let fragment_memberships = memberships_for_candidate(candidate, provenance, &mut counters)?;
        let contributing_lineages =
            overlapping_lineages_for_candidate(candidate, provenance, &mut counters);
        let born_source_loops = born_source_loop_identities(&contributing_lineages);

        if born_source_loops.len() > 1 {
            let chain_identities = contributing_lineages
                .iter()
                .map(|lineage| lineage.chain_identity().to_string())
                .collect::<Vec<_>>();
            born.push(PlanarBooleanBornLoop::new(
                born_loop_identity(
                    &request_identity,
                    candidate.loop_candidate_identity(),
                    &born_source_loops,
                    &chain_identities,
                ),
                candidate.loop_candidate_identity().to_string(),
                born_source_loops,
                chain_identities,
                candidate.local_frame_identity().to_string(),
                candidate.precision_basis_identity().to_string(),
                candidate.fragment_identities().to_vec(),
                candidate.split_vertex_identities().to_vec(),
            ));
            counters.emitted_born_loop();
            continue;
        }

        let Some(first_membership) = fragment_memberships.first() else {
            counters.denied_candidate();
            return Err(PlanarBooleanReconstructedLoopBoundaryDenial::new(
                PlanarBooleanReconstructedLoopBoundaryDenialKind::UntrackedBornLoopEmergence,
                candidate.loop_candidate_identity().to_string(),
                counters,
                "reconstructed loop admission requires source fragment memberships for every candidate fragment",
            ));
        };
        admitted.push(PlanarBooleanAdmittedReconstructedLoop::new(
            admitted_reconstructed_loop_identity(
                &request_identity,
                candidate.loop_candidate_identity(),
                candidate.source_loop_identity(),
                candidate.fragment_identities(),
            ),
            candidate.loop_candidate_identity().to_string(),
            candidate.source_loop_identity().to_string(),
            first_membership.source_face_identity().to_string(),
            candidate.local_frame_identity().to_string(),
            candidate.precision_basis_identity().to_string(),
            candidate.fragment_identities().to_vec(),
            candidate.split_vertex_identities().to_vec(),
        ));
        counters.emitted_admitted_reconstructed_loop();
    }

    admitted.sort_by(|left, right| {
        left.source_loop_identity()
            .cmp(right.source_loop_identity())
            .then_with(|| {
                left.reconstructed_loop_identity()
                    .cmp(right.reconstructed_loop_identity())
            })
    });
    born.sort_by(|left, right| {
        left.loop_candidate_identity()
            .cmp(right.loop_candidate_identity())
            .then_with(|| left.born_loop_identity().cmp(right.born_loop_identity()))
    });

    Ok(PlanarBooleanReconstructedLoopBoundary::new(
        PlanarBooleanAdmittedReconstructedLoopSet::new(
            admitted_reconstructed_loop_set_identity(&request_identity, &admitted),
            request_identity.clone(),
            admitted,
        ),
        PlanarBooleanBornLoopSet::new(
            born_loop_set_identity(&request_identity, &born),
            request_identity,
            born,
        ),
        counters,
    ))
}

fn memberships_for_candidate<'a>(
    candidate: &PlanarBooleanLoopCandidate,
    provenance: &'a crate::workload_platform::planar_boolean_loop_reconstruction::PlanarBooleanLoopSourceProvenanceBundle,
    counters: &mut PlanarBooleanReconstructedLoopBoundaryCounters,
) -> Result<Vec<&'a PlanarBooleanFragmentMembershipRow>, PlanarBooleanReconstructedLoopBoundaryDenial>
{
    let mut memberships = Vec::new();
    for fragment_identity in candidate.fragment_identities() {
        let Some(membership) = provenance
            .fragment_membership_map()
            .membership_for_fragment_identity(fragment_identity)
        else {
            counters.denied_candidate();
            return Err(PlanarBooleanReconstructedLoopBoundaryDenial::new(
                PlanarBooleanReconstructedLoopBoundaryDenialKind::UntrackedBornLoopEmergence,
                candidate.loop_candidate_identity().to_string(),
                *counters,
                "reconstructed loop admission requires fragment membership recovery for every loop candidate fragment",
            ));
        };
        counters.consumed_fragment_membership();
        if membership.source_loop_identity() != candidate.source_loop_identity() {
            counters.denied_candidate();
            return Err(PlanarBooleanReconstructedLoopBoundaryDenial::new(
                PlanarBooleanReconstructedLoopBoundaryDenialKind::ContradictoryIslandOwnership,
                candidate.loop_candidate_identity().to_string(),
                *counters,
                "reconstructed loop admission denies fragment memberships that disagree with the candidate source loop identity",
            ));
        }
        memberships.push(membership);
    }
    Ok(memberships)
}

fn overlapping_lineages_for_candidate<'a>(
    candidate: &PlanarBooleanLoopCandidate,
    provenance: &'a crate::workload_platform::planar_boolean_loop_reconstruction::PlanarBooleanLoopSourceProvenanceBundle,
    counters: &mut PlanarBooleanReconstructedLoopBoundaryCounters,
) -> Vec<&'a PlanarBooleanLoopOverlapChainLineageRow> {
    let fragment_identities = candidate
        .fragment_identities()
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    provenance
        .overlap_chain_lineage_map()
        .rows()
        .iter()
        .filter(|row| {
            row.fragment_identities()
                .iter()
                .any(|identity| fragment_identities.contains(identity))
        })
        .inspect(|_| counters.consumed_overlap_chain_lineage())
        .collect()
}

fn born_source_loop_identities(
    lineages: &[&PlanarBooleanLoopOverlapChainLineageRow],
) -> Vec<String> {
    lineages
        .iter()
        .flat_map(|lineage| lineage.source_loop_identities().iter().cloned())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}
