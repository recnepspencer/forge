use std::collections::HashMap;

use worth_ui_host_contract::{UiMountedPaintCommand, UiMountedPaintCommandIdentity};

/// Bounded current-command owner for ordinary headless presentation work.
///
/// This owner deliberately has no `Clone` surface. Complete materialization is
/// reconstruction work; a delta can snapshot only the identities it changes.
pub(super) struct UiHeadlessRetainedCommandStore {
    by_identity: HashMap<UiMountedPaintCommandIdentity, UiMountedPaintCommand>,
}

impl UiHeadlessRetainedCommandStore {
    pub(super) fn with_capacity(capacity: usize) -> Self {
        Self {
            by_identity: HashMap::with_capacity(capacity),
        }
    }

    pub(super) fn len(&self) -> usize {
        self.by_identity.len()
    }

    pub(super) fn contains_key(&self, identity: &UiMountedPaintCommandIdentity) -> bool {
        self.by_identity.contains_key(identity)
    }

    pub(super) fn get(
        &self,
        identity: &UiMountedPaintCommandIdentity,
    ) -> Option<&UiMountedPaintCommand> {
        self.by_identity.get(identity)
    }

    pub(super) fn insert(
        &mut self,
        identity: UiMountedPaintCommandIdentity,
        command: UiMountedPaintCommand,
    ) -> Option<UiMountedPaintCommand> {
        self.by_identity.insert(identity, command)
    }

    pub(super) fn remove(
        &mut self,
        identity: &UiMountedPaintCommandIdentity,
    ) -> Option<UiMountedPaintCommand> {
        self.by_identity.remove(identity)
    }

    pub(super) fn values(&self) -> impl Iterator<Item = &UiMountedPaintCommand> {
        self.by_identity.values()
    }

    pub(super) fn identities(&self) -> impl Iterator<Item = UiMountedPaintCommandIdentity> + '_ {
        self.by_identity.keys().copied()
    }
}
