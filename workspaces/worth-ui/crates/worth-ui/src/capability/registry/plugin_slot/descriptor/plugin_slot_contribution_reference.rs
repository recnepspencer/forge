use crate::capability::PluginSlotId;

/// Typed reference from a future plugin contribution to its host-owned slot.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PluginSlotContributionReference {
    slot_id: PluginSlotId,
}

impl PluginSlotContributionReference {
    pub fn slot(slot_id: PluginSlotId) -> Self {
        Self { slot_id }
    }

    pub fn slot_id(&self) -> &PluginSlotId {
        &self.slot_id
    }

    pub(crate) fn digest_basis(&self) -> String {
        self.slot_id.as_str().to_owned()
    }
}
