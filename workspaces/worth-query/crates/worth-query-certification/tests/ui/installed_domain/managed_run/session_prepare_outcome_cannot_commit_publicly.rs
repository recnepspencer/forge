use worth_query_execution::facade::provider_session::WorthQuerySessionPrepareOutcome;

fn commit_without_phase_11(prepared: WorthQuerySessionPrepareOutcome<'_>) {
    let _ = prepared.commit();
}

fn main() {}
