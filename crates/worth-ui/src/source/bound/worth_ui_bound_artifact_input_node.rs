use crate::capability::{
    AdmittedCapability, ComponentDescriptor, ComponentId, FrozenThemeTokenEntry, SurfaceDescriptor,
    SurfaceId, ThemeTokenId,
};
use crate::source::{
    WorthUiArtifactInputImportNode, WorthUiArtifactInputProvenance, WorthUiBoundSurfaceSemantics,
    WorthUiBoundThemeTokenSemantics, WorthUiBoundViewBindingReference, WorthUiMosaicStructureFacts,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum WorthUiBoundArtifactInputNode {
    Import(WorthUiArtifactInputImportNode),
    Component(WorthUiBoundArtifactInputComponentNode),
    Surface(WorthUiBoundArtifactInputSurfaceNode),
    Binding(WorthUiBoundArtifactInputBindingNode),
    Token(WorthUiBoundArtifactInputThemeTokenNode),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiBoundArtifactInputComponentNode {
    component: AdmittedCapability<ComponentId>,
    descriptor: ComponentDescriptor,
    authored_identity: Option<String>,
    structure: WorthUiMosaicStructureFacts,
    provenance: WorthUiArtifactInputProvenance,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiBoundArtifactInputSurfaceNode {
    surface: AdmittedCapability<SurfaceId>,
    descriptor: SurfaceDescriptor,
    authored_identity: Option<String>,
    structure: WorthUiMosaicStructureFacts,
    semantics: WorthUiBoundSurfaceSemantics,
    provenance: WorthUiArtifactInputProvenance,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiBoundArtifactInputBindingNode {
    view_binding_reference: WorthUiBoundViewBindingReference,
    authored_identity: Option<String>,
    structure: WorthUiMosaicStructureFacts,
    provenance: WorthUiArtifactInputProvenance,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiBoundArtifactInputThemeTokenNode {
    theme_token: AdmittedCapability<ThemeTokenId>,
    entry: FrozenThemeTokenEntry,
    authored_identity: Option<String>,
    semantics: WorthUiBoundThemeTokenSemantics,
    provenance: WorthUiArtifactInputProvenance,
}

impl WorthUiBoundArtifactInputComponentNode {
    pub(crate) fn new(
        component: AdmittedCapability<ComponentId>,
        descriptor: ComponentDescriptor,
        authored_identity: Option<String>,
        structure: WorthUiMosaicStructureFacts,
        provenance: WorthUiArtifactInputProvenance,
    ) -> Self {
        Self {
            component,
            descriptor,
            authored_identity,
            structure,
            provenance,
        }
    }

    pub(crate) fn component(&self) -> &AdmittedCapability<ComponentId> {
        &self.component
    }

    pub(crate) fn descriptor(&self) -> &ComponentDescriptor {
        &self.descriptor
    }

    pub(crate) fn authored_identity(&self) -> Option<&str> {
        self.authored_identity.as_deref()
    }

    pub(crate) fn structure(&self) -> &WorthUiMosaicStructureFacts {
        &self.structure
    }

    pub(crate) fn provenance(&self) -> &WorthUiArtifactInputProvenance {
        &self.provenance
    }
}

impl WorthUiBoundArtifactInputSurfaceNode {
    pub(crate) fn new(
        surface: AdmittedCapability<SurfaceId>,
        descriptor: SurfaceDescriptor,
        authored_identity: Option<String>,
        structure: WorthUiMosaicStructureFacts,
        semantics: WorthUiBoundSurfaceSemantics,
        provenance: WorthUiArtifactInputProvenance,
    ) -> Self {
        Self {
            surface,
            descriptor,
            authored_identity,
            structure,
            semantics,
            provenance,
        }
    }

    pub(crate) fn surface(&self) -> &AdmittedCapability<SurfaceId> {
        &self.surface
    }

    pub(crate) fn descriptor(&self) -> &SurfaceDescriptor {
        &self.descriptor
    }

    pub(crate) fn authored_identity(&self) -> Option<&str> {
        self.authored_identity.as_deref()
    }

    pub(crate) fn structure(&self) -> &WorthUiMosaicStructureFacts {
        &self.structure
    }

    pub(crate) fn semantics(&self) -> &WorthUiBoundSurfaceSemantics {
        &self.semantics
    }

    pub(crate) fn provenance(&self) -> &WorthUiArtifactInputProvenance {
        &self.provenance
    }
}

impl WorthUiBoundArtifactInputBindingNode {
    pub(crate) fn new(
        view_binding_reference: WorthUiBoundViewBindingReference,
        authored_identity: Option<String>,
        structure: WorthUiMosaicStructureFacts,
        provenance: WorthUiArtifactInputProvenance,
    ) -> Self {
        Self {
            view_binding_reference,
            authored_identity,
            structure,
            provenance,
        }
    }

    pub(crate) fn view_binding_reference(&self) -> &WorthUiBoundViewBindingReference {
        &self.view_binding_reference
    }

    pub(crate) fn authored_identity(&self) -> Option<&str> {
        self.authored_identity.as_deref()
    }

    pub(crate) fn structure(&self) -> &WorthUiMosaicStructureFacts {
        &self.structure
    }

    pub(crate) fn provenance(&self) -> &WorthUiArtifactInputProvenance {
        &self.provenance
    }
}

impl WorthUiBoundArtifactInputThemeTokenNode {
    pub(crate) fn new(
        theme_token: AdmittedCapability<ThemeTokenId>,
        entry: FrozenThemeTokenEntry,
        authored_identity: Option<String>,
        semantics: WorthUiBoundThemeTokenSemantics,
        provenance: WorthUiArtifactInputProvenance,
    ) -> Self {
        Self {
            theme_token,
            entry,
            authored_identity,
            semantics,
            provenance,
        }
    }

    pub(crate) fn theme_token(&self) -> &AdmittedCapability<ThemeTokenId> {
        &self.theme_token
    }

    pub(crate) fn entry(&self) -> &FrozenThemeTokenEntry {
        &self.entry
    }

    pub(crate) fn authored_identity(&self) -> Option<&str> {
        self.authored_identity.as_deref()
    }

    pub(crate) fn semantics(&self) -> &WorthUiBoundThemeTokenSemantics {
        &self.semantics
    }

    pub(crate) fn provenance(&self) -> &WorthUiArtifactInputProvenance {
        &self.provenance
    }
}
