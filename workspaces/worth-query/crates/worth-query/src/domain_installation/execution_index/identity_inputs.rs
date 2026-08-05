use super::WorthQueryInstalledDomainArtifact;

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
