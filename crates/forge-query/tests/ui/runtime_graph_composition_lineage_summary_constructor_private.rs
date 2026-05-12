use forge_query::facade::{
    ForgeQueryContinuityMutationFamily, ForgeQueryContinuityOutcomeClass,
    ForgeQueryGraphCompositionLineageEntry, ForgeQueryGraphCompositionLineageSummary,
};

fn main() {
    let _ = ForgeQueryGraphCompositionLineageSummary {
        entries: vec![ForgeQueryGraphCompositionLineageEntry {
            component_index: 0,
            family: ForgeQueryContinuityMutationFamily::SplitExistingTarget,
            outcome_class: ForgeQueryContinuityOutcomeClass::ContinuesAsSplitSuccessors,
            prior_authoritative_identity: String::new(),
            successor_authoritative_identities: Vec::new(),
            target_collection: None,
            lineage_digest: String::new(),
            continuity_resolution_digest: String::new(),
        }],
        counter_snapshot: String::new(),
        aggregate_lineage_digest: String::new(),
        aggregate_continuity_resolution_digest: String::new(),
        lineage_summary_digest: String::new(),
    };
}
