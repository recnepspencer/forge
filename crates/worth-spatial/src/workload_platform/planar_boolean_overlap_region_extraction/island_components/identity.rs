use super::rows::PlanarBooleanOverlapIslandCandidateKind;

pub(crate) fn island_identity(neighborhood_identity: &str) -> String {
    format!("overlap-island:{neighborhood_identity}")
}

pub(crate) fn candidate_identity(
    cell_identity: &str,
    kind: PlanarBooleanOverlapIslandCandidateKind,
) -> String {
    format!("overlap-island-candidate:{kind:?}:{cell_identity}")
}

pub(crate) fn candidate_set_identity(
    request_identity: &str,
    candidate_identities: impl IntoIterator<Item = String>,
) -> String {
    let joined = candidate_identities.into_iter().collect::<Vec<_>>().join("|");
    format!("overlap-island-candidate-set:{request_identity}:{joined}")
}

pub(crate) fn component_set_identity(request_identity: &str, kind: &str, count: usize) -> String {
    format!("overlap-component-set:{kind}:{request_identity}:{count}")
}

pub(crate) fn partition_identity(request_identity: &str, island_count: usize) -> String {
    format!("overlap-island-partition:{request_identity}:{island_count}")
}
