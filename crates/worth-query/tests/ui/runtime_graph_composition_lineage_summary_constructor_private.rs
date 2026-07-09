use worth_query::facade::{
    WorthQueryContinuityMutationFamily, WorthQueryContinuityOutcomeClass,
    WorthQueryGraphCompositionLineageEntry, WorthQueryGraphCompositionLineageSummary,
};

fn main() {
    let _ = WorthQueryGraphCompositionLineageSummary {
        entries: vec![WorthQueryGraphCompositionLineageEntry {
            component_index: 0,
            family: WorthQueryContinuityMutationFamily::SplitExistingTarget,
            outcome_class: WorthQueryContinuityOutcomeClass::ContinuesAsSplitSuccessors,
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
