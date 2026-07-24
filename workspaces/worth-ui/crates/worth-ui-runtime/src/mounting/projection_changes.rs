use worth_ui_host_contract::{UiMountedInstanceIdentity, UiSemanticSurfaceIdentity};

#[derive(Clone, Default)]
pub(crate) struct UiMountedProjectionChanges {
    changed_instances:
        crate::runtime::persistent_index::UiPersistentOrdSet<UiMountedInstanceIdentity>,
    retired_instances:
        crate::runtime::persistent_index::UiPersistentOrdSet<UiMountedInstanceIdentity>,
    changed_surfaces:
        crate::runtime::persistent_index::UiPersistentOrdSet<UiSemanticSurfaceIdentity>,
    removed_surfaces:
        crate::runtime::persistent_index::UiPersistentOrdSet<UiSemanticSurfaceIdentity>,
    order_changed: bool,
    coalesced: u64,
    overflowed: bool,
}

#[derive(Clone)]
pub(crate) struct UiMountedProjectionChangeSnapshot {
    semantic_revision: u64,
    binding_revision: u64,
    changes: UiMountedProjectionChanges,
}

impl UiMountedProjectionChanges {
    pub(crate) fn mark_changed_instance(&mut self, instance: UiMountedInstanceIdentity) {
        self.retired_instances.remove_with_work(&instance);
        if !self.changed_instances.insert(instance) {
            self.record_coalesced();
        }
    }

    pub(crate) fn mark_retired_instance(&mut self, instance: UiMountedInstanceIdentity) {
        self.changed_instances.remove_with_work(&instance);
        if !self.retired_instances.insert(instance) {
            self.record_coalesced();
        }
    }

    pub(crate) fn mark_order_changed(&mut self, order: &[UiMountedInstanceIdentity]) {
        if self.order_changed {
            self.record_coalesced();
        }
        self.order_changed = true;
        for instance in order {
            self.mark_changed_instance(*instance);
        }
    }

    pub(crate) fn mark_changed_surface(&mut self, surface: UiSemanticSurfaceIdentity) {
        self.removed_surfaces.remove_with_work(&surface);
        if !self.changed_surfaces.insert(surface) {
            self.record_coalesced();
        }
    }

    pub(crate) fn mark_removed_surface(&mut self, surface: UiSemanticSurfaceIdentity) {
        self.changed_surfaces.remove_with_work(&surface);
        if !self.removed_surfaces.insert(surface) {
            self.record_coalesced();
        }
    }

    pub(crate) fn snapshot(
        &self,
        semantic_revision: u64,
        binding_revision: u64,
    ) -> UiMountedProjectionChangeSnapshot {
        UiMountedProjectionChangeSnapshot {
            semantic_revision,
            binding_revision,
            changes: self.clone(),
        }
    }

    fn record_coalesced(&mut self) {
        match self.coalesced.checked_add(1) {
            Some(coalesced) => self.coalesced = coalesced,
            None => self.overflowed = true,
        }
    }
}

impl UiMountedProjectionChangeSnapshot {
    pub(crate) fn changed_instances(&self) -> impl Iterator<Item = UiMountedInstanceIdentity> + '_ {
        self.changes.changed_instances.iter().copied()
    }

    pub(crate) fn retired_instances(&self) -> impl Iterator<Item = UiMountedInstanceIdentity> + '_ {
        self.changes.retired_instances.iter().copied()
    }

    pub(crate) fn changed_surfaces(&self) -> impl Iterator<Item = UiSemanticSurfaceIdentity> + '_ {
        self.changes.changed_surfaces.iter().copied()
    }

    pub(crate) fn removed_surfaces(&self) -> impl Iterator<Item = UiSemanticSurfaceIdentity> + '_ {
        self.changes.removed_surfaces.iter().copied()
    }

    pub(crate) fn order_changed(&self) -> bool {
        self.changes.order_changed
    }

    pub(crate) fn coalesced(&self) -> u64 {
        self.changes.coalesced
    }

    pub(crate) fn overflowed(&self) -> bool {
        self.changes.overflowed
    }

    pub(crate) fn has_semantic_changes(&self) -> bool {
        !self.changes.changed_instances.is_empty()
            || !self.changes.retired_instances.is_empty()
            || self.changes.order_changed
    }

    pub(crate) fn matches(&self, semantic_revision: u64, binding_revision: u64) -> bool {
        self.semantic_revision == semantic_revision && self.binding_revision == binding_revision
    }
}
