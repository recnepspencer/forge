use worth_ui::facade::{WorthUiChangedRuntimeFacts, WorthUiRuntimeFactSet};

fn main() {
    let _forged = WorthUiChangedRuntimeFacts {
        facts: WorthUiRuntimeFactSet::empty(),
        proof: unreachable!(),
    };
}
