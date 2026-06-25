use super::{WorthUiRuntimeFactFamily, WorthUiRuntimeFactId};

impl WorthUiRuntimeFactId {
    pub fn composition_root(identity: impl Into<String>) -> Self {
        Self::new(WorthUiRuntimeFactFamily::CompositionRoot, identity)
    }

    pub fn composition_node(identity: impl Into<String>) -> Self {
        Self::new(WorthUiRuntimeFactFamily::CompositionNode, identity)
    }

    pub fn composition_edge(identity: impl Into<String>) -> Self {
        Self::new(WorthUiRuntimeFactFamily::CompositionEdge, identity)
    }

    pub fn composition_participation(identity: impl Into<String>) -> Self {
        Self::new(WorthUiRuntimeFactFamily::CompositionParticipation, identity)
    }

    pub fn composition_policy(identity: impl Into<String>) -> Self {
        Self::new(WorthUiRuntimeFactFamily::CompositionPolicy, identity)
    }

    pub fn composition_context(identity: impl Into<String>) -> Self {
        Self::new(WorthUiRuntimeFactFamily::CompositionContext, identity)
    }

    pub fn composition_context_override(identity: impl Into<String>) -> Self {
        Self::new(
            WorthUiRuntimeFactFamily::CompositionContextOverride,
            identity,
        )
    }

    pub fn composition_context_propagation(identity: impl Into<String>) -> Self {
        Self::new(
            WorthUiRuntimeFactFamily::CompositionContextPropagation,
            identity,
        )
    }

    pub fn composition_topology(identity: impl Into<String>) -> Self {
        Self::new(WorthUiRuntimeFactFamily::CompositionTopology, identity)
    }

    pub fn composition_root_mount_authority(identity: impl Into<String>) -> Self {
        Self::new(
            WorthUiRuntimeFactFamily::CompositionRootMountAuthority,
            identity,
        )
    }
}
