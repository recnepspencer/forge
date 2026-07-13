use worth_query::facade::foundation::AdmittedEffectIntent;
use worth_query::facade::runtime::QueryWorkflowDeclaration;

fn normalized() -> worth_query::facade::foundation::NormalizedEffectIntent {
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
