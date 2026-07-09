pub(super) const PHASE_SIXTEEN_READERS: usize = 3;
pub(super) const PHASE_SIXTEEN_SUBMITTERS: usize = 2;
pub(super) const PHASE_SIXTEEN_SUBMISSION_ROUNDS: usize = 3;

#[derive(Clone)]
pub(super) struct PlannedSubmission {
    pub(super) ordinal: usize,
    pub(super) identity: String,
    pub(super) title: String,
}

#[derive(Clone, Copy)]
pub(super) enum SubmitterInterleaving {
    Ascending,
    Descending,
}

pub(super) fn planned_phase_sixteen_submissions() -> Vec<PlannedSubmission> {
    (0..PHASE_SIXTEEN_SUBMITTERS)
        .flat_map(planned_submissions_for_submitter)
        .collect()
}

pub(super) fn planned_submissions_for_submitter(submitter: usize) -> Vec<PlannedSubmission> {
    (0..PHASE_SIXTEEN_SUBMISSION_ROUNDS)
        .map(|round| {
            let ordinal = submitter * PHASE_SIXTEEN_SUBMISSION_ROUNDS + round;
            PlannedSubmission {
                ordinal,
                identity: format!("phase16-task-{ordinal}"),
                title: format!("Phase 16 Task {ordinal}"),
            }
        })
        .collect()
}

pub(super) fn submitter_thread_ordinals(interleaving: SubmitterInterleaving) -> Vec<usize> {
    let mut ordinals = (0..PHASE_SIXTEEN_SUBMITTERS).collect::<Vec<_>>();
    if matches!(interleaving, SubmitterInterleaving::Descending) {
        ordinals.reverse();
    }
    ordinals
}
