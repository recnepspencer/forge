use crate::capability::ViewBindingId;
use crate::runtime::{
    WorthUiQueryBindingChangedFacts, WorthUiQueryBindingComparison,
    WorthUiQueryBindingComparisonOutcome, WorthUiRuntimeFactId, WorthUiRuntimeFactSet,
};

pub struct WorthUiQueryBindingRuntimeFactLowering;

impl WorthUiQueryBindingRuntimeFactLowering {
    pub fn from_comparison(
        comparison: &WorthUiQueryBindingComparison,
    ) -> WorthUiQueryBindingChangedFacts {
        let mut changed_facts = WorthUiRuntimeFactSet::empty();
        for entry in comparison.entries() {
            if entry.outcome() == WorthUiQueryBindingComparisonOutcome::PreserveMeaning {
                continue;
            }
            let view_binding_id = ViewBindingId::new(entry.identity().view_binding_id())
                .expect("query binding identities preserve validated view binding ids");
            changed_facts.insert(WorthUiRuntimeFactId::query_binding(&view_binding_id));
        }
        WorthUiQueryBindingChangedFacts::from_comparison_facts(
            changed_facts,
            comparison.active_artifact_digest(),
            comparison.candidate_artifact_digest(),
        )
    }
}
