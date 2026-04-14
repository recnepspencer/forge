use forge_store::{DurableMutationRequest, EmbeddedStoreHandle};

fn misuse(mut embedded: EmbeddedStoreHandle) {
    let request = DurableMutationRequest::new("illegal-cross-mode", |_runtime| {
        unreachable!("compile-fail boundary should reject this before execution")
    });
    let _ = embedded.execute_mutation(request);
}

fn main() {}
