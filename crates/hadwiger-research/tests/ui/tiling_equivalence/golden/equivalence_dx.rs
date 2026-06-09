use hadwiger_research::facade::{
    classify_tiling_candidate_equivalence_checked, HadwigerResearchHandle,
    TileEquivalenceWitness, TilingCandidateEquivalenceProof, TilingCandidateEquivalenceRequest,
    TilingEquivalenceError,
};

fn classify_contact_equivalence(
    handle: &HadwigerResearchHandle,
    witness: TileEquivalenceWitness,
) -> Result<TilingCandidateEquivalenceProof, TilingEquivalenceError> {
    classify_tiling_candidate_equivalence_checked(
        handle,
        TilingCandidateEquivalenceRequest::from_tile_equivalence_witness(
            "equivalence-dx",
            witness,
        )?,
    )
}

fn main() {}
