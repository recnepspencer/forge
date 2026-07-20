use super::WorthQueryInstalledDomainArtifact;

pub(super) fn graph_obligation_identity_parts(
    artifacts: &[WorthQueryInstalledDomainArtifact],
) -> Vec<String> {
    let mut identity_parts = artifacts
        .iter()
        .flat_map(|artifact| {
            artifact
                .graph_obligation_definitions
                .iter()
                .map(|obligation| {
                    format!("{}:{}", artifact.domain_owner, obligation.canonical_part())
                })
        })
        .collect::<Vec<_>>();
    identity_parts.sort();
    identity_parts
}

pub(super) fn package_provenance_identity_parts(
    artifacts: &[WorthQueryInstalledDomainArtifact],
) -> Vec<String> {
    let mut identity_parts = artifacts
        .iter()
        .map(|artifact| {
            artifact
                .substrate_provenance
                .identity()
                .as_str()
                .to_string()
        })
        .collect::<Vec<_>>();
    identity_parts.sort();
    identity_parts
}
