use worth_query_host::facade::primary_graph::{
    WorthQueryAdmittedDisclosedApplicationResult, WorthQueryApplicationCommitReceipt,
};
use worth_query_host::facade::publication::domain_computation::{
    publish_application_commit, publish_application_result,
    WorthQueryPublishedApplicationCommitAttemptReleasePosture,
    WorthQueryPublishedApplicationQueryReleasePosture,
    WorthQueryPublishedApplicationQueryResultBufferRelease,
};

fn owner_derivation_is_the_publication_lane<Query, QueryResult>(
    commit: WorthQueryApplicationCommitReceipt,
    result: WorthQueryAdmittedDisclosedApplicationResult<Query, QueryResult>,
) {
    let commit = publish_application_commit(commit).into_receipt();
    let _commit_release: WorthQueryPublishedApplicationCommitAttemptReleasePosture =
        commit.inspect().attempt_release();
    let result = publish_application_result(result);
    let query_release = result.receipt().inspect().terminal_release();
    let _application_basis: WorthQueryPublishedApplicationQueryReleasePosture =
        query_release.application_basis();
    let _graph_read_basis: WorthQueryPublishedApplicationQueryReleasePosture =
        query_release.graph_read_basis();
    let _result_buffer: WorthQueryPublishedApplicationQueryResultBufferRelease =
        query_release.result_buffer();
    let _released_capacity = query_release.released_graph_capacity_reservation_count();
    let _all_released = query_release.resources_released();
}

fn main() {}
