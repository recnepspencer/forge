use worth_ui::facade::source::WorthUiWatchedCandidateSubmission;

fn extract(submission: WorthUiWatchedCandidateSubmission) {
    let _ = submission.into_candidate();
}

fn main() {}
