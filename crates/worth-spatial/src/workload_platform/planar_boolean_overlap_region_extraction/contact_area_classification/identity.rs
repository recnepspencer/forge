pub(super) fn shared_boundary_contact_outcome_identity(
    request_identity: &str,
    component_identity: &str,
) -> String {
    format!(
        "shared-boundary-contact-outcome:{request_identity}:{component_identity}"
    )
}

pub(super) fn pure_boundary_only_outcome_identity(
    request_identity: &str,
    island_identity: &str,
) -> String {
    format!("pure-boundary-only-outcome:{request_identity}:{island_identity}")
}

pub(super) fn outcome_set_identity(request_identity: &str, outcome_kind: &str, count: usize) -> String {
    format!("overlap-contact-outcome-set:{request_identity}:{outcome_kind}:{count}")
}
