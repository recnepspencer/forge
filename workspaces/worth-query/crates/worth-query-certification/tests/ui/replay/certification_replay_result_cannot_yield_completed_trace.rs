use worth_query::facade::{certification, domain, foundation};

fn borrow_completed_trace<'a, D, O, F, L: foundation::BasisOperationLane>(
    replay: &'a certification::WorthQueryCertificationReplayResult<D, O, F, L>,
) -> &'a domain::WorthQueryCompletedWorkflowTrace<D, O, F, L> {
    replay.replay_trace()
}

fn leak_completed_trace<D, O, F, L: foundation::BasisOperationLane>(
    replay: certification::WorthQueryCertificationReplayResult<D, O, F, L>,
) -> domain::WorthQueryCompletedWorkflowTrace<D, O, F, L> {
    replay.into_replay_trace()
}

fn main() {}
