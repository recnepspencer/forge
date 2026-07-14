use worth_ui::facade::{WorthUiAdmittedReplacementCandidate, WorthUiRuntime};

fn attempt(host: WorthUiRuntime, admitted: WorthUiAdmittedReplacementCandidate) {
    let _ = host.prepare_execution_plan_input(admitted);
}

fn main() {}
