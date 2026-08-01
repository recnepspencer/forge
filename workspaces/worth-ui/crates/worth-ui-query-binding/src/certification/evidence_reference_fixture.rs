/// Issues opaque references from entities seeded through one real Query world.
pub fn query_evidence_references<const N: usize>(
    seeds: [&str; N],
) -> [crate::UiQueryEvidenceReference; N] {
    let mut workspace = super::collection_projection_workspace();
    seeds.map(|seed| {
        let entity = super::insert_projection_status(&mut workspace, seed, seed);
        let identity = entity.evidence_identity();
        crate::UiQueryEvidenceReference::query_issued(&identity)
    })
}
