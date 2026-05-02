use forge_query::facade::{
    ForgeQueryGraphCompositionLifecycleOutcomeEntry,
    ForgeQueryGraphCompositionLifecycleOutcomeKind,
    ForgeQueryGraphCompositionLifecycleOutcomes,
};

fn main() {
    let _ = ForgeQueryGraphCompositionLifecycleOutcomes {
        entries: vec![ForgeQueryGraphCompositionLifecycleOutcomeEntry {
            component_index: 0,
            outcome_kind: ForgeQueryGraphCompositionLifecycleOutcomeKind::Created,
            declared_collection: String::new(),
            declared_symbol: None,
        }],
        lifecycle_digest: String::new(),
        counter_snapshot: String::new(),
    };
}
