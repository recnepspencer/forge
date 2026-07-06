pub(super) fn shared_area_admission_outcome_identity(
    request_identity: &str,
    component_identity: &str,
) -> String {
    format!("shared-area-admission-outcome:{request_identity}:{component_identity}")
}

pub(super) fn mixed_boundary_area_outcome_identity(
    request_identity: &str,
    island_identity: &str,
) -> String {
    format!("mixed-boundary-area-outcome:{request_identity}:{island_identity}")
}

pub(super) fn outcome_set_identity(
    request_identity: &str,
    outcome_kind: &str,
    count: usize,
) -> String {
    format!("shared-area-outcome-set:{request_identity}:{outcome_kind}:{count}")
}
