use hadwiger_research::facade::*;

pub fn contact_witness(id: &str) -> TileEquivalenceWitness {
    TileEquivalenceWitness::builder(id, TileEquivalenceScope::ContactConstraint)
        .with_left_contact_signature(
            TileContactGraphSignature::from_edges(
                "tile-a",
                [("center", "north"), ("center", "east")],
            )
            .unwrap(),
        )
        .with_right_contact_signature(
            TileContactGraphSignature::from_edges(
                "tile-b",
                [("center", "east"), ("center", "north")],
            )
            .unwrap(),
        )
        .finish()
        .unwrap()
}

pub fn contact_equivalence(handle: &HadwigerResearchHandle) -> TilingCandidateEquivalenceProof {
    classify_tiling_candidate_equivalence_checked(
        handle,
        TilingCandidateEquivalenceRequest::from_tile_equivalence_witness(
            "suppression-equivalence",
            contact_witness("suppression-contact-class"),
        )
        .unwrap(),
    )
    .unwrap()
}
