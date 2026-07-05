pub(super) fn candidate_identity(request_identity: &str, shared_area_outcome_identity: &str) -> String {
    format!("overlap-region-candidate:{request_identity}:{shared_area_outcome_identity}")
}

pub(super) fn denied_candidate_identity(request_identity: &str, rejected_identity: &str) -> String {
    format!("denied-overlap-region-candidate:{request_identity}:{rejected_identity}")
}

pub(super) fn admitted_region_identity(request_identity: &str, candidate_identity: &str) -> String {
    format!("admitted-overlap-region:{request_identity}:{candidate_identity}")
}

pub(super) fn boundary_only_outcome_identity(request_identity: &str, pure_boundary_outcome_identity: &str) -> String {
    format!("boundary-only-overlap-outcome:{request_identity}:{pure_boundary_outcome_identity}")
}

pub(super) fn set_identity(request_identity: &str, set_kind: &str, row_count: usize) -> String {
    format!("overlap-region-boundary-set:{request_identity}:{set_kind}:{row_count}")
}
