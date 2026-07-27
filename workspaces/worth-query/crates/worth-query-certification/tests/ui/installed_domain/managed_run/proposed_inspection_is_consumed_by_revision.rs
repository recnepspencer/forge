use worth_query_execution::facade::provider_session::{
    WorthQueryLoweredProvisionalEffectProgram, WorthQueryProposedStateInspection,
};

fn reuse_after_revision(
    inspection: WorthQueryProposedStateInspection<'_>,
    program: WorthQueryLoweredProvisionalEffectProgram,
) {
    let _newer = match inspection.revise(program) {
        Ok(attempt) => attempt,
        Err(_) => return,
    };
    let _stale_generation = inspection.generation();
}

fn main() {}
