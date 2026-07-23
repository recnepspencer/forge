use worth_query::facade::{domain, foundation, runtime};

fn refresh_closed<D, O, F>(
    closed: domain::WorthQueryDisposedWorkflowProjection<
        D,
        O,
        F,
        foundation::ObservationLaneWitness,
    >,
    workspace: &mut runtime::WorthQueryWorkspace,
) {
    let _ = closed.refresh(workspace);
}

fn refresh_pending<D, O, F>(
    pending: domain::WorthQueryReplacementCleanupPendingWorkflowProjection<
        D,
        O,
        F,
        foundation::ObservationLaneWitness,
    >,
    workspace: &mut runtime::WorthQueryWorkspace,
) {
    let _ = pending.refresh(workspace);
}

fn main() {}
