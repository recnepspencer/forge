use crate::workload_platform::planar_boolean_loop_reconstruction::{
    PlanarBooleanClosedWalkCandidate, PlanarBooleanClosedWalkCandidateAssembly,
    PlanarBooleanClosedWalkCandidateSet, PlanarBooleanFragmentConsumptionProof,
    PlanarBooleanFragmentConsumptionProofRow, PlanarBooleanFragmentContinuationCounters,
    PlanarBooleanFragmentContinuationIndex, PlanarBooleanFragmentContinuationRow,
    PlanarBooleanWalkOutcomeSet, PlanarBooleanWalkOutcomeSetInput,
};

use super::continuation_index::prepared_loop_continuation_subject;
use super::replay_support::retained_replay_receipt_chain;
use super::runtime_subject::{LoopFixtureEntryOrder, PreparedLoopReconstructionSubject};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum AdversarialLoopReconstructionScenario {
    OpenWalk,
    ResidualFragmentClaim,
    DeniedProofMismatch,
    LineageContradiction,
}

pub(crate) struct PreparedAdversarialLoopReconstructionSubject {
    pub(crate) continuation_index: PlanarBooleanFragmentContinuationIndex,
    pub(crate) walk_candidate_assembly: PlanarBooleanClosedWalkCandidateAssembly,
}

impl PreparedAdversarialLoopReconstructionSubject {
    pub(crate) fn classify(&self) -> PlanarBooleanWalkOutcomeSet {
        PlanarBooleanWalkOutcomeSet::classify(
            PlanarBooleanWalkOutcomeSetInput::from_closed_walk_candidates(
                self.walk_candidate_assembly.closed_walk_candidates(),
                self.walk_candidate_assembly.fragment_consumption_proof(),
            ),
        )
    }
}

pub(crate) fn adversarial_loop_reconstruction_subject(
    scenario: AdversarialLoopReconstructionScenario,
) -> PreparedAdversarialLoopReconstructionSubject {
    let prepared = prepared_loop_continuation_subject(LoopFixtureEntryOrder::Canonical);
    let continuation_index = match scenario {
        AdversarialLoopReconstructionScenario::OpenWalk => rebuild_continuation_index(
            &prepared.subject,
            &prepared.continuation_index,
            prepared.continuation_index.rows()[1..].to_vec(),
        ),
        AdversarialLoopReconstructionScenario::LineageContradiction => {
            let rows = prepared
                .continuation_index
                .rows()
                .iter()
                .enumerate()
                .map(|(index, row)| {
                    if index == 0 {
                        clone_row_with_face(row, "contradictory-source-face")
                    } else {
                        row.clone()
                    }
                })
                .collect::<Vec<_>>();
            rebuild_continuation_index(&prepared.subject, &prepared.continuation_index, rows)
        }
        AdversarialLoopReconstructionScenario::ResidualFragmentClaim
        | AdversarialLoopReconstructionScenario::DeniedProofMismatch => {
            prepared.continuation_index.clone()
        }
    };
    let honest_assembly = PlanarBooleanClosedWalkCandidateAssembly::assemble(
        crate::workload_platform::planar_boolean_loop_reconstruction::PlanarBooleanClosedWalkCandidateSetInput::from_continuation_index(
            &continuation_index,
        ),
    );
    let walk_candidate_assembly = match scenario {
        AdversarialLoopReconstructionScenario::ResidualFragmentClaim => {
            assembly_with_residual_fragment_claim(&honest_assembly)
        }
        AdversarialLoopReconstructionScenario::DeniedProofMismatch => {
            assembly_with_fragment_mismatch(&honest_assembly)
        }
        AdversarialLoopReconstructionScenario::OpenWalk
        | AdversarialLoopReconstructionScenario::LineageContradiction => honest_assembly,
    };
    PreparedAdversarialLoopReconstructionSubject {
        continuation_index,
        walk_candidate_assembly,
    }
}

fn rebuild_continuation_index(
    subject: &PreparedLoopReconstructionSubject,
    original: &PlanarBooleanFragmentContinuationIndex,
    rows: Vec<PlanarBooleanFragmentContinuationRow>,
) -> PlanarBooleanFragmentContinuationIndex {
    let mut counters = PlanarBooleanFragmentContinuationCounters::default();
    for _ in 0..original.counters().split_vertices_consumed() {
        counters.consumed_split_vertex();
    }
    for _ in 0..original.counters().overlap_chains_consumed() {
        counters.consumed_overlap_chain();
    }
    for _ in 0..rows.len() {
        counters.indexed_fragment_continuation();
    }
    PlanarBooleanFragmentContinuationIndex::new(
        original.continuation_index_identity().to_string(),
        original.request_identity().to_string(),
        original.source_provenance_bundle_identity().to_string(),
        subject
            .vertices
            .split_vertex_identity_set_identity()
            .to_string(),
        subject.fragments.fragment_set_identity().to_string(),
        subject.overlap_chains.chain_set_identity().to_string(),
        rows,
        original.ordering_basis().clone(),
        counters,
    )
}

fn assembly_with_fragment_mismatch(
    assembly: &PlanarBooleanClosedWalkCandidateAssembly,
) -> PlanarBooleanClosedWalkCandidateAssembly {
    let candidate = assembly
        .closed_walk_candidates()
        .rows()
        .first()
        .expect("fixture should assemble one walk candidate");
    let proof = PlanarBooleanFragmentConsumptionProof::new(
        assembly
            .fragment_consumption_proof()
            .fragment_consumption_proof_identity()
            .to_string(),
        assembly
            .fragment_consumption_proof()
            .request_identity()
            .to_string(),
        assembly
            .fragment_consumption_proof()
            .continuation_index_identity()
            .to_string(),
        vec![PlanarBooleanFragmentConsumptionProofRow::new(
            candidate.closed_walk_candidate_identity().to_string(),
            vec!["wrong-fragment".to_string()],
            candidate.split_vertex_identities().to_vec(),
            candidate
                .continuations()
                .iter()
                .map(|continuation| continuation.continuation_identity().to_string())
                .collect(),
        )],
    );
    PlanarBooleanClosedWalkCandidateAssembly::new(assembly.closed_walk_candidates().clone(), proof)
}

fn assembly_with_residual_fragment_claim(
    assembly: &PlanarBooleanClosedWalkCandidateAssembly,
) -> PlanarBooleanClosedWalkCandidateAssembly {
    let candidate = assembly
        .closed_walk_candidates()
        .rows()
        .first()
        .expect("fixture should assemble one walk candidate");
    let mut fragment_identities = candidate.fragment_identities().to_vec();
    fragment_identities.push("residual-fragment".to_string());
    let candidate = PlanarBooleanClosedWalkCandidate::new(
        candidate.closed_walk_candidate_identity().to_string(),
        candidate.source_loop_identity().to_string(),
        candidate.source_face_identities().to_vec(),
        candidate.source_loop_carrier_identities().to_vec(),
        candidate.source_senses().to_vec(),
        fragment_identities.clone(),
        candidate.split_vertex_identities().to_vec(),
        candidate.continuations().to_vec(),
        candidate.local_frame_identities().to_vec(),
        candidate.precision_basis_identities().to_vec(),
    );
    let candidate_set = PlanarBooleanClosedWalkCandidateSet::new(
        assembly
            .closed_walk_candidates()
            .closed_walk_candidate_set_identity()
            .to_string(),
        assembly
            .closed_walk_candidates()
            .request_identity()
            .to_string(),
        assembly
            .closed_walk_candidates()
            .continuation_index_identity()
            .to_string(),
        vec![candidate.clone()],
        assembly.closed_walk_candidates().counters(),
    );
    let proof = PlanarBooleanFragmentConsumptionProof::new(
        assembly
            .fragment_consumption_proof()
            .fragment_consumption_proof_identity()
            .to_string(),
        assembly
            .fragment_consumption_proof()
            .request_identity()
            .to_string(),
        assembly
            .fragment_consumption_proof()
            .continuation_index_identity()
            .to_string(),
        vec![PlanarBooleanFragmentConsumptionProofRow::new(
            candidate.closed_walk_candidate_identity().to_string(),
            fragment_identities,
            candidate.split_vertex_identities().to_vec(),
            candidate
                .continuations()
                .iter()
                .map(|continuation| continuation.continuation_identity().to_string())
                .collect(),
        )],
    );
    PlanarBooleanClosedWalkCandidateAssembly::new(candidate_set, proof)
}

fn clone_row_with_face(
    row: &PlanarBooleanFragmentContinuationRow,
    source_face_identity: &str,
) -> PlanarBooleanFragmentContinuationRow {
    PlanarBooleanFragmentContinuationRow::new(
        row.continuation_identity().to_string(),
        row.neighborhood_identity().to_string(),
        row.split_vertex_identity().to_string(),
        row.fragment_identity().to_string(),
        row.source_loop_identity().to_string(),
        source_face_identity.to_string(),
        row.source_edge_identity().to_string(),
        row.carrier_identity().to_string(),
        row.source_loop_carrier_identity().to_string(),
        row.fragment_endpoint_role(),
        row.source_sense(),
        row.endpoint_parameter_bits(),
        row.fragment_parameter_range_bits(),
        row.local_frame_identity().to_string(),
        row.precision_basis_identity().to_string(),
        row.event_group_identities().to_vec(),
        row.boundary_roles().to_vec(),
    )
}
