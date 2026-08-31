use crate::capability::CapabilitySnapshot;
use crate::graph::UiGraphSnapshot;

pub(super) fn appearance_axis_demand(
    snapshot: &UiGraphSnapshot,
    capabilities: &CapabilitySnapshot,
) -> (
    bool,
    crate::runtime::appearance::UiAppearanceStateAxisDemand,
) {
    let mut has_consumers = false;
    let mut demand = crate::runtime::appearance::UiAppearanceStateAxisDemand::default();
    for node in snapshot.nodes() {
        let Some(attachment) = node.appearance_role_attachment() else {
            continue;
        };
        if node.component_reference() != Some(attachment.target()) {
            continue;
        }
        let Some(role) = capabilities.appearance_roles().get(attachment.role()) else {
            continue;
        };
        if role.aspect_contract() != attachment.aspect_contract()
            || role.revision() != attachment.revision()
        {
            continue;
        }
        has_consumers = true;
        for (_, partition) in role.partitions() {
            for axis in partition.axes() {
                demand.include(axis.axis());
            }
        }
    }
    (has_consumers, demand)
}
