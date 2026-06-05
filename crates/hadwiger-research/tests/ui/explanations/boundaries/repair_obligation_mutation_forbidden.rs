use hadwiger_research::facade::HadwigerRejectionExplanation;

fn mutate(explanation: &mut HadwigerRejectionExplanation) {
    let _ = explanation.repair_obligations_mut();
}

fn main() {}
