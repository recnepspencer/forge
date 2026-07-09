use worth_query::facade::{
    BasisFamily, EffectAuthorityLane, EffectFamily, EffectLifecycleCounters,
    EffectOperationInput, NormalizedEffectIntent, WorkflowContextBinding, WorkflowDeclarationRequest,
};

fn binding() -> WorkflowContextBinding {
    unimplemented!()
}

fn request() -> WorkflowDeclarationRequest {
    unimplemented!()
}

fn input() -> EffectOperationInput {
    unimplemented!()
}

fn placeholder<T>() -> T {
    unimplemented!()
}

fn main() {
    let _ = NormalizedEffectIntent {
        family: EffectFamily::Mutation,
        authority_lane: EffectAuthorityLane::Relational,
        basis_family: BasisFamily::BranchHead,
        basis_authority: placeholder(),
        basis_lifecycle: placeholder(),
        capability_digest: String::new(),
        scoped_basis_digest: String::new(),
        expected_lower_runtime_binding_digest: None,
        workflow_binding: binding(),
        workflow_request: request(),
        operation_input: input(),
        source_path: "raw_effect.mutation",
        normalized_digest: String::new(),
        counters: EffectLifecycleCounters::default(),
    };
}
