pub(super) fn normalization_outcome_identity(
    request_identity: &str,
    shared_area_outcome_identity: &str,
) -> String {
    format!("pre-region-normalization:{request_identity}:{shared_area_outcome_identity}")
}

pub(super) fn normalization_set_identity(request_identity: &str, row_count: usize) -> String {
    format!("pre-region-normalization-set:{request_identity}:{row_count}")
}
