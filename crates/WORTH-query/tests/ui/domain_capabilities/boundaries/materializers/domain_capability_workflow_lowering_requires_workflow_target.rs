use worth_query::facade::{
    materialize_lowered_mutation_intent_declaration, WorthQueryDomainCapabilityTransitionOutcome,
    WorthQueryLowerRuntimeBoundaryBoundContributionTarget,
    WorthQueryMaterializationReadyWorkflowContribution, LoweredMutationIntentDeclaration,
};

fn main() {
    let _workflow_lowering_materializer: fn(
        WorthQueryMaterializationReadyWorkflowContribution<
            WorthQueryLowerRuntimeBoundaryBoundContributionTarget,
        >,
    ) -> WorthQueryDomainCapabilityTransitionOutcome<LoweredMutationIntentDeclaration> =
        materialize_lowered_mutation_intent_declaration;
}
