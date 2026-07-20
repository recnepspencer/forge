use std::any::TypeId;
use std::collections::HashMap;

use super::WorthQueryInstalledDomainArtifact;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthQueryInstalledOperationRequiredDomain {
    pub(crate) role: String,
    pub(crate) domain_marker: TypeId,
}

pub(super) fn operation_required_domain_index(
    artifacts: &[WorthQueryInstalledDomainArtifact],
) -> HashMap<(TypeId, TypeId, TypeId), Vec<WorthQueryInstalledOperationRequiredDomain>> {
    let mut index: HashMap<_, Vec<WorthQueryInstalledOperationRequiredDomain>> = HashMap::new();
    for artifact in artifacts {
        for binding in &artifact.operation_required_domains {
            index
                .entry((
                    artifact.marker_type,
                    binding.operation_marker(),
                    binding.family_marker(),
                ))
                .or_default()
                .push(WorthQueryInstalledOperationRequiredDomain {
                    role: binding.role().to_string(),
                    domain_marker: binding.domain_marker(),
                });
        }
    }
    for bindings in index.values_mut() {
        bindings.sort_by(|left, right| left.role.cmp(&right.role));
    }
    index
}
