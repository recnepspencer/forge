use worth_query::facade::{domain, foundation, runtime};

fn replace_with_rebind_witness<D: 'static, O, F>(
    live: domain::WorthQueryLiveBoundDomainProjection<D, O, F, foundation::ObservationLaneWitness>,
    candidate: domain::WorthQueryCurrentDomainProjection<
        D,
        O,
        F,
        foundation::ObservationLaneWitness,
    >,
    witness: domain::WorthQueryRebindWitness,
    workspace: &mut runtime::WorthQueryWorkspace,
) {
    let _ = live.replace_with(candidate, witness, workspace);
}

fn main() {}
