/// Diagnostic marker for an attempted unbounded plugin mutation hook.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PluginSlotGlobalMutationHook {
    kind: PluginSlotGlobalMutationHookKind,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum PluginSlotGlobalMutationHookKind {
    OpaqueCallback,
}

impl PluginSlotGlobalMutationHook {
    pub fn opaque_callback_for_diagnostics() -> Self {
        Self {
            kind: PluginSlotGlobalMutationHookKind::OpaqueCallback,
        }
    }

    pub(crate) fn digest_basis(&self) -> &'static str {
        match self.kind {
            PluginSlotGlobalMutationHookKind::OpaqueCallback => "opaque_callback",
        }
    }
}
