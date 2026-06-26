use crate::capability::{
    AdmittedCapability, ComponentDescriptor, ComponentId, FrozenThemeTokenEntry,
    FrozenViewBindingEntry, SurfaceDescriptor, SurfaceId, ThemeTokenId, ViewBindingId,
};
use crate::source::{
    WorthUiArtifactInputBodyAtom, WorthUiArtifactInputImportNode, WorthUiArtifactInputProvenance,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum WorthUiResolvedArtifactInputNode {
    Import(WorthUiArtifactInputImportNode),
    Component(WorthUiResolvedArtifactInputComponentNode),
    Surface(WorthUiResolvedArtifactInputSurfaceNode),
    Binding(WorthUiResolvedArtifactInputBindingNode),
    Token(WorthUiResolvedArtifactInputThemeTokenNode),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiResolvedArtifactInputComponentNode {
    component: AdmittedCapability<ComponentId>,
    descriptor: ComponentDescriptor,
    authored_identity: Option<String>,
    body_atoms: Vec<WorthUiArtifactInputBodyAtom>,
    provenance: WorthUiArtifactInputProvenance,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiResolvedArtifactInputSurfaceNode {
    surface: AdmittedCapability<SurfaceId>,
    descriptor: SurfaceDescriptor,
    authored_identity: Option<String>,
    body_atoms: Vec<WorthUiArtifactInputBodyAtom>,
    provenance: WorthUiArtifactInputProvenance,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiResolvedArtifactInputBindingNode {
    view_binding: AdmittedCapability<ViewBindingId>,
    entry: FrozenViewBindingEntry,
    authored_identity: Option<String>,
    body_atoms: Vec<WorthUiArtifactInputBodyAtom>,
    provenance: WorthUiArtifactInputProvenance,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiResolvedArtifactInputThemeTokenNode {
    theme_token: AdmittedCapability<ThemeTokenId>,
    entry: FrozenThemeTokenEntry,
    authored_identity: Option<String>,
    provenance: WorthUiArtifactInputProvenance,
}

impl WorthUiResolvedArtifactInputComponentNode {
    pub(crate) fn new(
        component: AdmittedCapability<ComponentId>,
        descriptor: ComponentDescriptor,
        authored_identity: Option<String>,
        body_atoms: Vec<WorthUiArtifactInputBodyAtom>,
        provenance: WorthUiArtifactInputProvenance,
    ) -> Self {
        Self {
            component,
            descriptor,
            authored_identity,
            body_atoms,
            provenance,
        }
    }

    pub(crate) fn component(&self) -> &AdmittedCapability<ComponentId> {
        &self.component
    }

    pub(crate) fn descriptor(&self) -> &ComponentDescriptor {
        &self.descriptor
    }

    pub(crate) fn body_atoms(&self) -> &[WorthUiArtifactInputBodyAtom] {
        &self.body_atoms
    }

    pub(crate) fn authored_identity(&self) -> Option<&str> {
        self.authored_identity.as_deref()
    }

    pub(crate) fn provenance(&self) -> &WorthUiArtifactInputProvenance {
        &self.provenance
    }
}

impl WorthUiResolvedArtifactInputSurfaceNode {
    pub(crate) fn new(
        surface: AdmittedCapability<SurfaceId>,
        descriptor: SurfaceDescriptor,
        authored_identity: Option<String>,
        body_atoms: Vec<WorthUiArtifactInputBodyAtom>,
        provenance: WorthUiArtifactInputProvenance,
    ) -> Self {
        Self {
            surface,
            descriptor,
            authored_identity,
            body_atoms,
            provenance,
        }
    }

    pub(crate) fn surface(&self) -> &AdmittedCapability<SurfaceId> {
        &self.surface
    }

    pub(crate) fn descriptor(&self) -> &SurfaceDescriptor {
        &self.descriptor
    }

    pub(crate) fn body_atoms(&self) -> &[WorthUiArtifactInputBodyAtom] {
        &self.body_atoms
    }

    pub(crate) fn authored_identity(&self) -> Option<&str> {
        self.authored_identity.as_deref()
    }

    pub(crate) fn provenance(&self) -> &WorthUiArtifactInputProvenance {
        &self.provenance
    }
}

impl WorthUiResolvedArtifactInputBindingNode {
    pub(crate) fn new(
        view_binding: AdmittedCapability<ViewBindingId>,
        entry: FrozenViewBindingEntry,
        authored_identity: Option<String>,
        body_atoms: Vec<WorthUiArtifactInputBodyAtom>,
        provenance: WorthUiArtifactInputProvenance,
    ) -> Self {
        Self {
            view_binding,
            entry,
            authored_identity,
            body_atoms,
            provenance,
        }
    }

    pub(crate) fn view_binding(&self) -> &AdmittedCapability<ViewBindingId> {
        &self.view_binding
    }

    pub(crate) fn entry(&self) -> &FrozenViewBindingEntry {
        &self.entry
    }

    pub(crate) fn body_atoms(&self) -> &[WorthUiArtifactInputBodyAtom] {
        &self.body_atoms
    }

    pub(crate) fn authored_identity(&self) -> Option<&str> {
        self.authored_identity.as_deref()
    }

    pub(crate) fn provenance(&self) -> &WorthUiArtifactInputProvenance {
        &self.provenance
    }
}

impl WorthUiResolvedArtifactInputThemeTokenNode {
    pub(crate) fn new(
        theme_token: AdmittedCapability<ThemeTokenId>,
        entry: FrozenThemeTokenEntry,
        authored_identity: Option<String>,
        provenance: WorthUiArtifactInputProvenance,
    ) -> Self {
        Self {
            theme_token,
            entry,
            authored_identity,
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

    pub(crate) fn provenance(&self) -> &WorthUiArtifactInputProvenance {
        &self.provenance
    }
}
