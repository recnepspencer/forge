use worth_query_execution::facade::provider_session::WorthQuerySessionBoundReadsAndEffects;

fn prepare_without_commit_authority(staged: WorthQuerySessionBoundReadsAndEffects<'_>) {
    let _ = staged.prepare_for_commit();
}

fn main() {}
