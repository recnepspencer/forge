use worth_query_execution::facade::primary_graph::{
    WorthQueryApprovedElevation, WorthQueryMandatoryReview, WorthQueryRequestedElevation,
    WorthQueryReviewedElevation,
};

fn raw_receipts_are_not_publication_projections(
    requested: &WorthQueryRequestedElevation,
    approved: &WorthQueryApprovedElevation,
    mandatory: &WorthQueryMandatoryReview,
    reviewed: &WorthQueryReviewedElevation,
) {
    let _ = requested.commit_receipt();
    let _ = approved.authorization_publication_receipt();
    let _ = mandatory.close_commit_receipt();
    let _ = reviewed.review_commit_receipt();
}

fn main() {}
