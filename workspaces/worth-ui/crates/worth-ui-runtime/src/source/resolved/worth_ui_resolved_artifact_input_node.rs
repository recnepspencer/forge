use crate::capability::{
    AdmittedCapability, ComponentDescriptor, ComponentId, FrozenThemeTokenEntry,
    FrozenViewBindingEntry, SurfaceDescriptor, SurfaceId, ThemeTokenId, ViewBindingId,
};
use worth_ui_dsl::WorthUiArtifactInputProvenance;
use worth_ui_dsl::WorthUiAuthoredStructuralBody;

use crate::source::WorthUiRuntimeSemanticImport;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum WorthUiResolvedArtifactInputNode {
    Import(WorthUiRuntimeSemanticImport),
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
    structure: WorthUiAuthoredStructuralBody,
    provenance: WorthUiArtifactInputProvenance,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiResolvedArtifactInputSurfaceNode {
    surface: AdmittedCapability<SurfaceId>,
    descriptor: SurfaceDescriptor,
    authored_identity: Option<String>,
    structure: WorthUiAuthoredStructuralBody,
    provenance: WorthUiArtifactInputProvenance,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiResolvedArtifactInputBindingNode {
    view_binding: AdmittedCapability<ViewBindingId>,
    entry: FrozenViewBindingEntry,
    authored_identity: Option<String>,
    structure: WorthUiAuthoredStructuralBody,
    provenance: WorthUiArtifactInputProvenance,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiResolvedArtifactInputThemeTokenNode {
    theme_token: AdmittedCapability<ThemeTokenId>,
    entry: FrozenThemeTokenEntry,
    binding_target: WorthUiResolvedThemeTokenBindingTarget,
    authored_identity: Option<String>,
    provenance: WorthUiArtifactInputProvenance,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum WorthUiResolvedThemeTokenBindingTarget {
    FileAuthoredReference(String),
    RustAuthoredRegisteredTarget(ThemeTokenId),
}

impl WorthUiResolvedArtifactInputComponentNode {
    pub(crate) fn new(
        component: AdmittedCapability<ComponentId>,
        descriptor: ComponentDescriptor,
        authored_identity: Option<String>,
        structure: WorthUiAuthoredStructuralBody,
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

    pub(crate) fn structure(&self) -> &WorthUiAuthoredStructuralBody {
        &self.structure
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
        structure: WorthUiAuthoredStructuralBody,
        provenance: WorthUiArtifactInputProvenance,
    ) -> Self {
        Self {
            surface,
            descriptor,
            authored_identity,
            structure,
            provenance,
        }
    }

    pub(crate) fn surface(&self) -> &AdmittedCapability<SurfaceId> {
        &self.surface
    }

    pub(crate) fn descriptor(&self) -> &SurfaceDescriptor {
        &self.descriptor
    }

    pub(crate) fn structure(&self) -> &WorthUiAuthoredStructuralBody {
        &self.structure
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
        structure: WorthUiAuthoredStructuralBody,
        provenance: WorthUiArtifactInputProvenance,
    ) -> Self {
        Self {
            view_binding,
            entry,
            authored_identity,
            structure,
            provenance,
        }
    }

    pub(crate) fn view_binding(&self) -> &AdmittedCapability<ViewBindingId> {
        &self.view_binding
    }

    pub(crate) fn entry(&self) -> &FrozenViewBindingEntry {
        &self.entry
    }

    pub(crate) fn structure(&self) -> &WorthUiAuthoredStructuralBody {
        &self.structure
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
        binding_target: WorthUiResolvedThemeTokenBindingTarget,
        authored_identity: Option<String>,
        provenance: WorthUiArtifactInputProvenance,
    ) -> Self {
        Self {
            theme_token,
            entry,
            binding_target,
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

    pub(crate) fn binding_target(&self) -> &WorthUiResolvedThemeTokenBindingTarget {
        &self.binding_target
    }

    pub(crate) fn authored_identity(&self) -> Option<&str> {
        self.authored_identity.as_deref()
    }

    pub(crate) fn provenance(&self) -> &WorthUiArtifactInputProvenance {
        &self.provenance
    }
}

impl WorthUiResolvedThemeTokenBindingTarget {
    pub(crate) fn from_ingress(
        authored_target_text: &str,
        registered_target: &ThemeTokenId,
        provenance: &WorthUiArtifactInputProvenance,
    ) -> Self {
        match provenance {
            WorthUiArtifactInputProvenance::ParsedSourceDeclaration { .. } => {
                Self::FileAuthoredReference(authored_target_text.to_owned())
            }
            WorthUiArtifactInputProvenance::RustAuthoredDeclaration { .. } => {
                Self::RustAuthoredRegisteredTarget(registered_target.clone())
            }
        }
    }

    pub(crate) fn reference_text(&self) -> &str {
        match self {
            Self::FileAuthoredReference(reference) => reference,
            Self::RustAuthoredRegisteredTarget(target) => target.as_str(),
        }
    }
}
