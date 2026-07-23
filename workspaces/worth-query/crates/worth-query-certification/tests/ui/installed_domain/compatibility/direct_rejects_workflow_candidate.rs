use worth_query::facade::{domain, foundation, runtime};

fn cross_projection_authority<D: 'static, O, F>(
    live: domain::WorthQueryLiveBoundDomainProjection<D, O, F, foundation::ObservationLaneWitness>,
    candidate: domain::WorthQueryCurrentWorkflowProjection<
        D,
        O,
        F,
        foundation::ObservationLaneWitness,
    >,
    witness: domain::WorthQueryReplacementWitness,
    workspace: &mut runtime::WorthQueryWorkspace,
) {
    let _ = live.replace_with(candidate, witness, workspace);
}

fn main() {}
