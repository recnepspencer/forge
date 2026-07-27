use worth_query_execution::facade::provider_session::WorthQueryProposedStateInspection;

fn commit_suggestion(inspection: WorthQueryProposedStateInspection<'_>) {
    let _ = inspection.commit();
}

fn main() {}
