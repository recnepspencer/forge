use worth_signal::facade::{CanonicalChangedRegions, InstalledSignalScopedChange};

use super::super::{BridgeDeliveredCorrespondenceChangeSet, BridgeInstalledSemanticCorrespondence};

#[derive(Debug, Clone)]
struct BridgePreparedScopedSignalTarget {
    node: worth_signal::facade::NodeId,
    aspect: worth_signal::facade::Aspect,
    changed_regions: CanonicalChangedRegions,
}

#[derive(Debug, Clone)]
pub struct BridgePreparedScopedSignalInvalidation {
    graph_instance_id: u64,
    targets: Vec<BridgePreparedScopedSignalTarget>,
}

impl BridgePreparedScopedSignalInvalidation {
    pub const fn graph_instance_id(&self) -> u64 {
        self.graph_instance_id
    }

    pub fn target_count(&self) -> usize {
        self.targets.len()
    }

    pub fn changed_regions(&self) -> impl ExactSizeIterator<Item = &CanonicalChangedRegions> {
        self.targets.iter().map(|target| &target.changed_regions)
    }

    pub(crate) fn retains_target(
        &self,
        graph_instance_id: u64,
        node: worth_signal::facade::NodeId,
        aspect: worth_signal::facade::Aspect,
    ) -> bool {
        self.graph_instance_id == graph_instance_id
            && self
                .targets
                .iter()
                .any(|target| target.node == node && target.aspect == aspect)
    }

    pub(crate) fn has_unique_target_bindings(&self) -> bool {
        let mut bindings = std::collections::BTreeSet::new();
        self.targets
            .iter()
            .all(|target| bindings.insert((target.node, target.aspect)))
    }
}

pub(crate) fn prepare_scoped_signal_invalidation_for_targets(
    correspondence: &BridgeInstalledSemanticCorrespondence,
    targets: &[super::super::InstalledCorrespondenceTarget],
    change_set: &BridgeDeliveredCorrespondenceChangeSet,
    capabilities: Vec<worth_signal::facade::InstalledSignalAspectCapability>,
) -> (
    Vec<InstalledSignalScopedChange>,
    BridgePreparedScopedSignalInvalidation,
) {
    let mut scoped_changes = Vec::with_capacity(capabilities.len());
    let mut prepared_targets = Vec::with_capacity(capabilities.len());
    for (capability, target) in capabilities.into_iter().zip(targets) {
        let regions = super::super::locality_lowering::lower_installed_target_regions(
            change_set.dependency(),
            target,
            change_set.changes(),
        );
        prepared_targets.push(BridgePreparedScopedSignalTarget {
            node: capability.node(),
            aspect: capability.aspect(),
            changed_regions: regions.clone(),
        });
        scoped_changes.push(InstalledSignalScopedChange::new(
            capability,
            regions.into_vec(),
        ));
    }
    (
        scoped_changes,
        BridgePreparedScopedSignalInvalidation {
            graph_instance_id: correspondence.basis().signal_graph_instance_id,
            targets: prepared_targets,
        },
    )
}
