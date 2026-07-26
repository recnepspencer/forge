use worth_query_execution::facade::domain_computation::WorthQueryGraphProviderStep;

fn retain_checkpoint_payload(step: &mut WorthQueryGraphProviderStep) {
    step.retain_checkpoint(Box::new(vec![1_u8, 2, 3]));
}

fn main() {}
