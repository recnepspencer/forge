/// Ordering posture for plugin contributions inside a slot.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PluginSlotOrdering {
    StableByPluginThenDeclaration,
    HostPriorityThenPlugin,
    Declaration,
}

impl PluginSlotOrdering {
    pub fn stable_by_plugin_then_declaration() -> Self {
        Self::StableByPluginThenDeclaration
    }

    pub fn host_priority_then_plugin() -> Self {
        Self::HostPriorityThenPlugin
    }

    pub fn declaration() -> Self {
        Self::Declaration
    }

    pub(crate) fn digest_basis(self) -> &'static str {
        match self {
            Self::StableByPluginThenDeclaration => "stable_by_plugin_then_declaration",
            Self::HostPriorityThenPlugin => "host_priority_then_plugin",
            Self::Declaration => "declaration",
        }
    }
}
