use worth_query_execution::facade::provider_session::WorthQueryGraphReadCompletion;

fn escalate(completion: WorthQueryGraphReadCompletion) {
    let _ = completion.proposed_state();
}

fn main() {}
