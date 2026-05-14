use forge_query::facade::runtime::{ForgeQueryIntentDeclaration, ForgeQueryRuntime};

fn must_fail(runtime: &mut ForgeQueryRuntime, declaration: ForgeQueryIntentDeclaration) {
    let _ = runtime.execute_admitted_intent_handoff(declaration);
}

fn main() {}
