use forge_store_layout_indexes::CopyOnWriteLayoutMutationRequest;

fn forge() -> CopyOnWriteLayoutMutationRequest<'static> {
    CopyOnWriteLayoutMutationRequest {
        strategy: todo!(),
        plan: todo!(),
        materialization: todo!(),
        current_security: todo!(),
    }
}

fn main() {
    let _ = forge();
}
