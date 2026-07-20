use std::any::TypeId;
use std::collections::HashMap;

use super::WorthQueryInstalledDomainArtifact;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthQueryInstalledOperationGraphBinding {
    pub(crate) role: String,
    pub(crate) graph_marker: TypeId,
}

pub(super) fn operation_graph_participation_index(
    artifacts: &[WorthQueryInstalledDomainArtifact],
) -> HashMap<(TypeId, TypeId, TypeId), Vec<WorthQueryInstalledOperationGraphBinding>> {
    let mut index: HashMap<
        (TypeId, TypeId, TypeId),
        Vec<WorthQueryInstalledOperationGraphBinding>,
    > = HashMap::new();
    for artifact in artifacts {
        for binding in &artifact.operation_graph_participations {
            index
                .entry((
                    artifact.marker_type,
                    binding.operation_marker(),
                    binding.family_marker(),
                ))
                .or_default()
                .push(WorthQueryInstalledOperationGraphBinding {
                    role: binding.role().to_string(),
                    graph_marker: binding.graph_marker(),
                });
        }
    }
    for bindings in index.values_mut() {
        bindings.sort_by(|left, right| left.role.cmp(&right.role));
    }
    index
}
