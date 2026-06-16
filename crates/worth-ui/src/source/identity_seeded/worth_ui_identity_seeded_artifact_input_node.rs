use crate::source::{
    WorthUiArtifactIdentitySeed, WorthUiArtifactInputImportNode, WorthUiArtifactInputProvenance,
    WorthUiBoundArtifactInputBindingNode, WorthUiBoundArtifactInputComponentNode,
    WorthUiBoundArtifactInputPageNode, WorthUiBoundArtifactInputSurfaceNode,
    WorthUiBoundArtifactInputThemeTokenNode, WorthUiBoundViewBindingReference,
    WorthUiDurableStateEligibility,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum WorthUiIdentitySeededArtifactInputNode {
    Import(WorthUiIdentitySeededArtifactInputImportNode),
    Page(WorthUiIdentitySeededArtifactInputPageNode),
    Component(WorthUiIdentitySeededArtifactInputComponentNode),
    Surface(WorthUiIdentitySeededArtifactInputSurfaceNode),
    Binding(WorthUiIdentitySeededArtifactInputBindingNode),
    Token(WorthUiIdentitySeededArtifactInputThemeTokenNode),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiIdentitySeededArtifactInputImportNode {
    node: WorthUiArtifactInputImportNode,
    identity_seed: WorthUiArtifactIdentitySeed,
    durable_state_eligibility: WorthUiDurableStateEligibility,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiIdentitySeededArtifactInputPageNode {
    node: WorthUiBoundArtifactInputPageNode,
    identity_seed: WorthUiArtifactIdentitySeed,
    durable_state_eligibility: WorthUiDurableStateEligibility,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiIdentitySeededArtifactInputComponentNode {
    node: WorthUiBoundArtifactInputComponentNode,
    identity_seed: WorthUiArtifactIdentitySeed,
    durable_state_eligibility: WorthUiDurableStateEligibility,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiIdentitySeededArtifactInputSurfaceNode {
    node: WorthUiBoundArtifactInputSurfaceNode,
    identity_seed: WorthUiArtifactIdentitySeed,
    durable_state_eligibility: WorthUiDurableStateEligibility,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiIdentitySeededArtifactInputBindingNode {
    node: WorthUiBoundArtifactInputBindingNode,
    identity_seed: WorthUiArtifactIdentitySeed,
    durable_state_eligibility: WorthUiDurableStateEligibility,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiIdentitySeededArtifactInputThemeTokenNode {
    node: WorthUiBoundArtifactInputThemeTokenNode,
    identity_seed: WorthUiArtifactIdentitySeed,
    durable_state_eligibility: WorthUiDurableStateEligibility,
}

macro_rules! seeded_node_impl {
    ($name:ident, $inner:ty) => {
        impl $name {
            pub(crate) fn new(
                node: $inner,
                identity_seed: WorthUiArtifactIdentitySeed,
                durable_state_eligibility: WorthUiDurableStateEligibility,
            ) -> Self {
                Self {
                    node,
                    identity_seed,
                    durable_state_eligibility,
                }
            }

            pub(crate) fn identity_seed(&self) -> &WorthUiArtifactIdentitySeed {
                &self.identity_seed
            }

            pub(crate) fn durable_state_eligibility(&self) -> &WorthUiDurableStateEligibility {
                &self.durable_state_eligibility
            }
        }
    };
}

seeded_node_impl!(
    WorthUiIdentitySeededArtifactInputPageNode,
    WorthUiBoundArtifactInputPageNode
);
seeded_node_impl!(
    WorthUiIdentitySeededArtifactInputComponentNode,
    WorthUiBoundArtifactInputComponentNode
);
seeded_node_impl!(
    WorthUiIdentitySeededArtifactInputSurfaceNode,
    WorthUiBoundArtifactInputSurfaceNode
);
seeded_node_impl!(
    WorthUiIdentitySeededArtifactInputBindingNode,
    WorthUiBoundArtifactInputBindingNode
);
seeded_node_impl!(
    WorthUiIdentitySeededArtifactInputThemeTokenNode,
    WorthUiBoundArtifactInputThemeTokenNode
);

impl WorthUiIdentitySeededArtifactInputImportNode {
    pub(crate) fn new(
        node: WorthUiArtifactInputImportNode,
        identity_seed: WorthUiArtifactIdentitySeed,
        durable_state_eligibility: WorthUiDurableStateEligibility,
    ) -> Self {
        Self {
            node,
            identity_seed,
            durable_state_eligibility,
        }
    }

    pub(crate) fn identity_seed(&self) -> &WorthUiArtifactIdentitySeed {
        &self.identity_seed
    }

    pub(crate) fn durable_state_eligibility(&self) -> &WorthUiDurableStateEligibility {
        &self.durable_state_eligibility
    }

    pub(crate) fn target(&self) -> &crate::source::WorthUiArtifactInputReference {
        self.node.target()
    }

    pub(crate) fn provenance(&self) -> &WorthUiArtifactInputProvenance {
        self.node.provenance()
    }
}

impl WorthUiIdentitySeededArtifactInputPageNode {
    pub(crate) fn name_text(&self) -> &str {
        self.node.name_text()
    }

    pub(crate) fn template_parameters(&self) -> &[(String, String)] {
        self.node.template_parameters()
    }

    pub(crate) fn provenance(&self) -> &WorthUiArtifactInputProvenance {
        self.node.provenance()
    }

    pub(crate) fn bound_node(&self) -> &WorthUiBoundArtifactInputPageNode {
        &self.node
    }
}

impl WorthUiIdentitySeededArtifactInputComponentNode {
    pub(crate) fn component(
        &self,
    ) -> &crate::capability::AdmittedCapability<crate::capability::ComponentId> {
        self.node.component()
    }

    pub(crate) fn provenance(&self) -> &WorthUiArtifactInputProvenance {
        self.node.provenance()
    }

    pub(crate) fn bound_node(&self) -> &WorthUiBoundArtifactInputComponentNode {
        &self.node
    }
}

impl WorthUiIdentitySeededArtifactInputSurfaceNode {
    pub(crate) fn surface(
        &self,
    ) -> &crate::capability::AdmittedCapability<crate::capability::SurfaceId> {
        self.node.surface()
    }

    pub(crate) fn provenance(&self) -> &WorthUiArtifactInputProvenance {
        self.node.provenance()
    }

    pub(crate) fn bound_node(&self) -> &WorthUiBoundArtifactInputSurfaceNode {
        &self.node
    }
}

impl WorthUiIdentitySeededArtifactInputBindingNode {
    pub(crate) fn view_binding_reference(&self) -> &WorthUiBoundViewBindingReference {
        self.node.view_binding_reference()
    }

    pub(crate) fn provenance(&self) -> &WorthUiArtifactInputProvenance {
        self.node.provenance()
    }

    pub(crate) fn bound_node(&self) -> &WorthUiBoundArtifactInputBindingNode {
        &self.node
    }
}

impl WorthUiIdentitySeededArtifactInputThemeTokenNode {
    pub(crate) fn theme_token(
        &self,
    ) -> &crate::capability::AdmittedCapability<crate::capability::ThemeTokenId> {
        self.node.theme_token()
    }

    pub(crate) fn provenance(&self) -> &WorthUiArtifactInputProvenance {
        self.node.provenance()
    }

    pub(crate) fn bound_node(&self) -> &WorthUiBoundArtifactInputThemeTokenNode {
        &self.node
    }
}
