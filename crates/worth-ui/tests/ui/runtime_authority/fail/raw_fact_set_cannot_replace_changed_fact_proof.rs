use worth_ui::facade::{WorthUiChangedRuntimeFacts, WorthUiRuntimeFactSet};

fn requires_changed_fact_proof(_proof: WorthUiChangedRuntimeFacts) {}

fn main() {
    requires_changed_fact_proof(WorthUiRuntimeFactSet::empty());
}
