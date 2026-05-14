use forge_query::facade::{AdmittedEffectIntent, QueryWorkflowDeclaration};

fn normalized() -> forge_query::facade::NormalizedEffectIntent {
    unimplemented!()
}

fn workflow_declaration() -> QueryWorkflowDeclaration {
    unimplemented!()
}

fn main() {
    let _ = AdmittedEffectIntent {
        normalized: normalized(),
        workflow_declaration: workflow_declaration(),
        admitted_digest: String::new(),
    };
}
