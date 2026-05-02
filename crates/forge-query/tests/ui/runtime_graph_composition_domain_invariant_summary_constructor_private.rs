use forge_query::facade::ForgeQueryGraphCompositionDomainInvariantSummary;

fn main() {
    let _ = ForgeQueryGraphCompositionDomainInvariantSummary {
        target_combination_families: vec!["mixed_existing_and_symbolic_entity_identity_edges".into()],
        lifecycle_families: vec!["mixed_existing_target_verified_retarget".into()],
        program_digest: "digest".to_string(),
        breadth_digest: "breadth".to_string(),
        counter_snapshot: "components=2".to_string(),
        summary_digest: "summary".to_string(),
    };
}
