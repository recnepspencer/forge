use worth_query::facade::{QueryWorkflowDeclaration, WorkflowAuthorityTargetFamily};

fn main() {
    let declaration: QueryWorkflowDeclaration = todo!();
    declaration.report().authority_target_family = WorkflowAuthorityTargetFamily::BridgeWriteback;
}
