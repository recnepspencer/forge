use worth_ui::facade::{WorthUiAdmittedReplacementCandidate, WorthUiRuntimeHost};

fn attempt(host: WorthUiRuntimeHost, admitted: WorthUiAdmittedReplacementCandidate) {
    let _ = host.prepare_execution_plan_input(admitted);
}

fn main() {}
