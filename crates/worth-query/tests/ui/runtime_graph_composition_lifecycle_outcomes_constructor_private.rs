use worth_query::facade::{
    WorthQueryGraphCompositionLifecycleOutcomeEntry,
    WorthQueryGraphCompositionLifecycleOutcomeKind,
    WorthQueryGraphCompositionLifecycleOutcomes,
};

fn main() {
    let _ = WorthQueryGraphCompositionLifecycleOutcomes {
        entries: vec![WorthQueryGraphCompositionLifecycleOutcomeEntry {
            component_index: 0,
            outcome_kind: WorthQueryGraphCompositionLifecycleOutcomeKind::Created,
            declared_collection: String::new(),
            declared_symbol: None,
        }],
        lifecycle_digest: String::new(),
        counter_snapshot: String::new(),
    };
}
