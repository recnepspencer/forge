use worth_query_declaration::facade::application_schema::ApplicationOperationProgramTarget;

pub(super) fn has_graph_mutation(program: &[ApplicationOperationProgramTarget]) -> bool {
    program
        .iter()
        .any(|target| !matches!(target, ApplicationOperationProgramTarget::Emit { .. }))
}
