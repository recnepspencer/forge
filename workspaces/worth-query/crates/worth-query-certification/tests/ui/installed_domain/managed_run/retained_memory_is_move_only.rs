use worth_query_execution::facade::provider_session::WorthQueryGraphProviderRetainedMemory;

fn duplicate(
    memory: &WorthQueryGraphProviderRetainedMemory,
) -> WorthQueryGraphProviderRetainedMemory {
    memory.clone()
}

fn main() {}
