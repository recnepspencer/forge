use std::collections::HashMap;

use worth_ui_host_contract::{UiMountedPaintCommand, UiMountedPaintCommandIdentity};

/// The ordinary retained-command owner deliberately has no `Clone` surface.
/// Complete replacement is reconstruction work; delta code may only address
/// exact identities through this bounded store.
pub(super) struct UiNativeRetainedCommandStore {
    by_identity: HashMap<UiMountedPaintCommandIdentity, UiMountedPaintCommand>,
}

impl UiNativeRetainedCommandStore {
    pub(super) fn with_capacity(capacity: usize) -> Self {
        Self {
            by_identity: HashMap::with_capacity(capacity),
        }
    }

    pub(super) fn len(&self) -> usize {
        self.by_identity.len()
    }

    pub(super) fn contains(&self, identity: &UiMountedPaintCommandIdentity) -> bool {
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

    pub(super) fn as_map(&self) -> &HashMap<UiMountedPaintCommandIdentity, UiMountedPaintCommand> {
        &self.by_identity
    }
}
