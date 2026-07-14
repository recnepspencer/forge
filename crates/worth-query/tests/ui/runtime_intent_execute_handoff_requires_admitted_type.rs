use worth_query::facade::runtime::{WorthQueryIntentDeclaration, WorthQueryRuntime};

fn must_fail(runtime: &mut WorthQueryRuntime, declaration: WorthQueryIntentDeclaration) {
    let _ = runtime.execute_admitted_intent_handoff(declaration);
}

fn main() {}
