use forge_query::facade::{
    materialize_lowered_mutation_intent_declaration, ForgeQueryDomainCapabilityTransitionOutcome,
    ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
    ForgeQueryMaterializationReadyWorkflowContribution, LoweredMutationIntentDeclaration,
};

fn main() {
    let _workflow_lowering_materializer: fn(
        ForgeQueryMaterializationReadyWorkflowContribution<
            ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
        >,
    ) -> ForgeQueryDomainCapabilityTransitionOutcome<LoweredMutationIntentDeclaration> =
        materialize_lowered_mutation_intent_declaration;
}
