use crate::capability::{
    AdmittedCapability, ComponentDescriptor, ComponentId, FrozenThemeTokenEntry, SurfaceDescriptor,
    SurfaceId, ThemeTokenId,
};
use crate::source::{
    WorthUiArtifactHandle, WorthUiArtifactIdentitySeed, WorthUiArtifactInputReference,
    WorthUiBoundSurfaceSemantics, WorthUiBoundThemeTokenSemantics,
    WorthUiBoundViewBindingReference, WorthUiDurableStateEligibility, WorthUiMosaicStructureFacts,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum WorthUiArtifactNode {
    Import(WorthUiArtifactImportNode),
    Page(WorthUiArtifactPageNode),
    Component(WorthUiArtifactComponentNode),
    Surface(WorthUiArtifactSurfaceNode),
    Binding(WorthUiArtifactBindingNode),
    Token(WorthUiArtifactThemeTokenNode),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiArtifactImportNode {
    handle: WorthUiArtifactHandle,
    target: WorthUiArtifactInputReference,
    identity_seed: WorthUiArtifactIdentitySeed,
    durable_state_eligibility: WorthUiDurableStateEligibility,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiArtifactPageNode {
    handle: WorthUiArtifactHandle,
    name_text: String,
    template_parameters: Vec<(String, String)>,
    structure: WorthUiMosaicStructureFacts,
    identity_seed: WorthUiArtifactIdentitySeed,
    durable_state_eligibility: WorthUiDurableStateEligibility,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiArtifactComponentNode {
    handle: WorthUiArtifactHandle,
    component: AdmittedCapability<ComponentId>,
    descriptor: ComponentDescriptor,
    structure: WorthUiMosaicStructureFacts,
    identity_seed: WorthUiArtifactIdentitySeed,
    durable_state_eligibility: WorthUiDurableStateEligibility,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiArtifactSurfaceNode {
    handle: WorthUiArtifactHandle,
    surface: AdmittedCapability<SurfaceId>,
    descriptor: SurfaceDescriptor,
    structure: WorthUiMosaicStructureFacts,
    semantics: WorthUiBoundSurfaceSemantics,
    identity_seed: WorthUiArtifactIdentitySeed,
    durable_state_eligibility: WorthUiDurableStateEligibility,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiArtifactBindingNode {
    handle: WorthUiArtifactHandle,
    view_binding_reference: WorthUiBoundViewBindingReference,
    structure: WorthUiMosaicStructureFacts,
    identity_seed: WorthUiArtifactIdentitySeed,
    durable_state_eligibility: WorthUiDurableStateEligibility,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiArtifactThemeTokenNode {
    handle: WorthUiArtifactHandle,
    theme_token: AdmittedCapability<ThemeTokenId>,
    entry: FrozenThemeTokenEntry,
    semantics: WorthUiBoundThemeTokenSemantics,
    identity_seed: WorthUiArtifactIdentitySeed,
    durable_state_eligibility: WorthUiDurableStateEligibility,
}

macro_rules! artifact_node_common_accessors {
    ($name:ident) => {
        impl $name {
            pub(crate) fn handle(&self) -> &WorthUiArtifactHandle {
                &self.handle
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

artifact_node_common_accessors!(WorthUiArtifactImportNode);
artifact_node_common_accessors!(WorthUiArtifactPageNode);
artifact_node_common_accessors!(WorthUiArtifactComponentNode);
artifact_node_common_accessors!(WorthUiArtifactSurfaceNode);
artifact_node_common_accessors!(WorthUiArtifactBindingNode);
artifact_node_common_accessors!(WorthUiArtifactThemeTokenNode);

impl WorthUiArtifactNode {
    pub(crate) fn handle(&self) -> &WorthUiArtifactHandle {
        match self {
            Self::Import(node) => node.handle(),
            Self::Page(node) => node.handle(),
            Self::Component(node) => node.handle(),
            Self::Surface(node) => node.handle(),
            Self::Binding(node) => node.handle(),
            Self::Token(node) => node.handle(),
        }
    }
}

impl WorthUiArtifactPageNode {
    pub(crate) fn new(
        handle: WorthUiArtifactHandle,
        name_text: impl Into<String>,
        template_parameters: Vec<(String, String)>,
        structure: WorthUiMosaicStructureFacts,
        identity_seed: WorthUiArtifactIdentitySeed,
        durable_state_eligibility: WorthUiDurableStateEligibility,
    ) -> Self {
        Self {
            handle,
            name_text: name_text.into(),
            template_parameters,
            structure,
            identity_seed,
            durable_state_eligibility,
        }
    }

    pub(crate) fn name_text(&self) -> &str {
        &self.name_text
    }

    pub(crate) fn template_parameters(&self) -> &[(String, String)] {
        &self.template_parameters
    }

    pub(crate) fn structure(&self) -> &WorthUiMosaicStructureFacts {
        &self.structure
    }
}

impl WorthUiArtifactImportNode {
    pub(crate) fn new(
        handle: WorthUiArtifactHandle,
        target: WorthUiArtifactInputReference,
        identity_seed: WorthUiArtifactIdentitySeed,
        durable_state_eligibility: WorthUiDurableStateEligibility,
    ) -> Self {
        Self {
            handle,
            target,
            identity_seed,
            durable_state_eligibility,
        }
    }

    pub(crate) fn target(&self) -> &crate::source::WorthUiArtifactInputReference {
        &self.target
    }
}

impl WorthUiArtifactComponentNode {
    pub(crate) fn new(
        handle: WorthUiArtifactHandle,
        component: AdmittedCapability<ComponentId>,
        descriptor: ComponentDescriptor,
        structure: WorthUiMosaicStructureFacts,
        identity_seed: WorthUiArtifactIdentitySeed,
        durable_state_eligibility: WorthUiDurableStateEligibility,
    ) -> Self {
        Self {
            handle,
            component,
            descriptor,
            structure,
            identity_seed,
            durable_state_eligibility,
        }
    }

    pub(crate) fn component(&self) -> &AdmittedCapability<ComponentId> {
        &self.component
    }

    pub(crate) fn descriptor(&self) -> &ComponentDescriptor {
        &self.descriptor
    }

    pub(crate) fn structure(&self) -> &WorthUiMosaicStructureFacts {
        &self.structure
    }
}

impl WorthUiArtifactSurfaceNode {
    pub(crate) fn new(
        handle: WorthUiArtifactHandle,
        surface: AdmittedCapability<SurfaceId>,
        descriptor: SurfaceDescriptor,
        structure: WorthUiMosaicStructureFacts,
        semantics: WorthUiBoundSurfaceSemantics,
        identity_seed: WorthUiArtifactIdentitySeed,
        durable_state_eligibility: WorthUiDurableStateEligibility,
    ) -> Self {
        Self {
            handle,
            surface,
            descriptor,
            structure,
            semantics,
            identity_seed,
            durable_state_eligibility,
        }
    }

    pub(crate) fn surface(&self) -> &AdmittedCapability<SurfaceId> {
        &self.surface
    }

    pub(crate) fn descriptor(&self) -> &SurfaceDescriptor {
        &self.descriptor
    }

    pub(crate) fn structure(&self) -> &WorthUiMosaicStructureFacts {
        &self.structure
    }

    pub(crate) fn semantics(&self) -> &WorthUiBoundSurfaceSemantics {
        &self.semantics
    }
}

impl WorthUiArtifactBindingNode {
    pub(crate) fn new(
        handle: WorthUiArtifactHandle,
        view_binding_reference: WorthUiBoundViewBindingReference,
        structure: WorthUiMosaicStructureFacts,
        identity_seed: WorthUiArtifactIdentitySeed,
        durable_state_eligibility: WorthUiDurableStateEligibility,
    ) -> Self {
        Self {
            handle,
            view_binding_reference,
            structure,
            identity_seed,
            durable_state_eligibility,
        }
    }

    pub(crate) fn view_binding_reference(&self) -> &WorthUiBoundViewBindingReference {
        &self.view_binding_reference
    }

    pub(crate) fn structure(&self) -> &WorthUiMosaicStructureFacts {
        &self.structure
    }
}

impl WorthUiArtifactThemeTokenNode {
    pub(crate) fn new(
        handle: WorthUiArtifactHandle,
        theme_token: AdmittedCapability<ThemeTokenId>,
        entry: FrozenThemeTokenEntry,
        semantics: WorthUiBoundThemeTokenSemantics,
        identity_seed: WorthUiArtifactIdentitySeed,
        durable_state_eligibility: WorthUiDurableStateEligibility,
    ) -> Self {
        Self {
            handle,
            theme_token,
            entry,
            semantics,
            identity_seed,
            durable_state_eligibility,
        }
    }

    pub(crate) fn theme_token(&self) -> &AdmittedCapability<ThemeTokenId> {
        &self.theme_token
    }

    pub(crate) fn entry(&self) -> &FrozenThemeTokenEntry {
        &self.entry
    }

    pub(crate) fn semantics(&self) -> &WorthUiBoundThemeTokenSemantics {
        &self.semantics
    }
}
