use worth_ui::facade::{
    WorthUiValidationPreparedReload, WorthUiValidationReloadEvidence,
};

fn main() {
    let _prepared = WorthUiValidationPreparedReload {
        evidence: evidence(),
        ready: None,
        candidate_plan: None,
    };
}

fn evidence() -> WorthUiValidationReloadEvidence {
    todo!()
}
